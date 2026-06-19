#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-failure-conditions.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-failure-json))' |
    python3 -m json.tool
}

validate_failure() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
domains = {"ontology", "confidence", "reviewer", "control-plane", "lease", "translation", "evidence-quality", "feedback"}
severities, errors, seen = {"error", "warning", "info"}, [], set()
for row in data.get("conditions", []):
    if row["id"] in seen:
        errors.append(f"duplicate condition {row['id']}")
    seen.add(row["id"])
    if row["domain"] not in domains:
        errors.append(f"{row['id']} bad domain")
    if row["severity"] not in severities:
        errors.append(f"{row['id']} bad severity")
    if not row.get("blocks"):
        errors.append(f"{row['id']} missing blocks")
    if row["owner"].startswith("agent:"):
        errors.append(f"{row['id']} owner cannot be an agent")
    if not row["response"].startswith("response:"):
        errors.append(f"{row['id']} missing response")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("conditions"):
    errors.append("failure manifest has no conditions")
if not data.get("closure_rules"):
    errors.append("failure manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification failure conditions check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated failure conditions are stale: run scripts/verificationfailuregen.sh generate" \
      "verification failure conditions generated output ok"
    validate_failure ;;
  *) echo "usage: scripts/verificationfailuregen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
