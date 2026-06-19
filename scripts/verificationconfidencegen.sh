#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-confidence.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-confidence-json))' |
    python3 -m json.tool
}

validate_confidence() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("ceilings", []):
    if row["id"] in seen: errors.append(f"duplicate ceiling {row['id']}")
    seen.add(row["id"])
    if row["self_confidence"] != "ignored":
        errors.append(f"self confidence is authoritative {row['id']}")
    if row["ceiling"] not in {"low", "medium", "high"}:
        errors.append(f"bad ceiling {row['id']}")
    if row["decided_by"].startswith("agent:"):
        errors.append(f"agent decides confidence {row['id']}")
    if not row.get("requires"): errors.append(f"empty requires {row['id']}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"unknown evidence {row['id']} {item}")
if not data.get("ceilings"): errors.append("confidence manifest has no ceilings")
if not data.get("closure_rules"): errors.append("confidence manifest has no closure rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification confidence check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification confidence is stale: run scripts/verificationconfidencegen.sh generate" \
      "verification confidence generated output ok"
    validate_confidence ;;
  *) echo "usage: scripts/verificationconfidencegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
