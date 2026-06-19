#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-sidecar.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-sidecar-json))' |
    python3 -m json.tool
}

validate_sidecar() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("receipts", []):
    if row["id"] in seen: errors.append(f"duplicate receipt {row['id']}")
    seen.add(row["id"])
    if row["producer"] == row["verifier"]:
        errors.append(f"producer verifies itself {row['id']}")
    if row["output"] not in outputs:
        errors.append(f"unknown output {row['id']} {row['output']}")
    if row.get("independent") is not True:
        errors.append(f"not independent {row['id']}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"unknown evidence {row['id']} {item}")
if not data.get("receipts"): errors.append("sidecar manifest has no receipts")
if not data.get("closure_rules"): errors.append("sidecar manifest has no closure rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification sidecar check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification sidecar is stale: run scripts/verificationsidecargen.sh generate" \
      "verification sidecar generated output ok"
    validate_sidecar ;;
  *) echo "usage: scripts/verificationsidecargen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
