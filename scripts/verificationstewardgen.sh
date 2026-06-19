#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-backup-steward.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-backup-steward-json))' |
    python3 -m json.tool
}

validate_stewards() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
statuses = {"candidate", "active", "returned", "escalated"}
required_forbidden = {"security-boundary-change", "authority-model-change"}
errors, seen = [], set()
for row in data.get("assignments", []):
    if row["id"] in seen:
        errors.append(f"duplicate steward assignment {row['id']}")
    seen.add(row["id"])
    if row["status"] not in statuses:
        errors.append(f"{row['id']} bad status")
    if row["steward"].startswith("agent:"):
        errors.append(f"{row['id']} steward cannot be an agent")
    if row["steward"] == row["missing_owner"]:
        errors.append(f"{row['id']} steward must differ from owner")
    if not row["review_back"].startswith("review-back:"):
        errors.append(f"{row['id']} missing review-back")
    if required_forbidden - set(row.get("forbidden", [])):
        errors.append(f"{row['id']} missing forbidden high-risk action")
    if set(row.get("allowed", [])) & set(row.get("forbidden", [])):
        errors.append(f"{row['id']} has conflicting allowed action")
    if row["status"] == "active" and "evidence-literate" not in row["criteria"]:
        errors.append(f"{row['id']} active steward lacks evidence literacy")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("assignments"):
    errors.append("backup steward manifest has no assignments")
if not data.get("closure_rules"):
    errors.append("backup steward manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification backup steward check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out"
    ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification backup steward is stale: run scripts/verificationstewardgen.sh generate" \
      "verification backup steward generated output ok"
    validate_stewards
    ;;
  *) echo "usage: scripts/verificationstewardgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
