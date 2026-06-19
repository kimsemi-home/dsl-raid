#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-quarantine.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-quarantine-json))' |
    python3 -m json.tool
}

validate_quarantine() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
required = {"artifact-commit", "confidence-increase", "automatic-approval"}
errors, seen = [], set()
for row in data.get("bundles", []):
    if row["id"] in seen: errors.append(f"duplicate bundle {row['id']}")
    seen.add(row["id"])
    if row["status"] not in {"isolated", "released", "closed"}:
        errors.append(f"bad status {row['id']}")
    missing = required - set(row.get("blocks", []))
    if missing: errors.append(f"missing blocks {row['id']} {sorted(missing)}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"unknown evidence {row['id']} {item}")
if not data.get("bundles"): errors.append("quarantine manifest has no bundles")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification quarantine check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification quarantine is stale: run scripts/verificationquarantinegen.sh generate" \
      "verification quarantine generated output ok"
    validate_quarantine ;;
  *) echo "usage: scripts/verificationquarantinegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
