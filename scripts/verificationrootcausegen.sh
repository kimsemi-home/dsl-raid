#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-root-cause.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-root-cause-json))' |
    python3 -m json.tool
}

validate_root_cause() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
statuses = {"candidate-set", "confirmed", "rejected", "closed"}
errors = []
for row in data.get("cases", []):
    if row["status"] not in statuses:
        errors.append(f"{row['id']} bad status")
    if not row.get("candidates"):
        errors.append(f"{row['id']} missing candidates")
    if not row.get("validation_evidence"):
        errors.append(f"{row['id']} missing validation evidence")
    if row["status"] != "confirmed" and row["confidence_ceiling"] == "high":
        errors.append(f"{row['id']} unconfirmed root cause cannot be high confidence")
    if row["authority"].startswith("agent:"):
        errors.append(f"{row['id']} authority cannot be an agent")
    for field in ("validation_evidence", "evidence"):
        for item in row.get(field, []):
            if item not in outputs:
                errors.append(f"{row['id']} unknown {field} {item}")
if not data.get("cases"):
    errors.append("root cause manifest has no cases")
if not data.get("closure_rules"):
    errors.append("root cause manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification root cause check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification root cause is stale: run scripts/verificationrootcausegen.sh generate" \
      "verification root cause generated output ok"
    validate_root_cause ;;
  *) echo "usage: scripts/verificationrootcausegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
