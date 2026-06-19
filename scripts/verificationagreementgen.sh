#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-agreement.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-agreement-json))' |
    python3 -m json.tool
}

validate_agreement() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
risks, decisions, errors, seen = {"routine", "high", "audit"}, {"agree", "blocked"}, [], set()
for row in data.get("agreements", []):
    if row["id"] in seen:
        errors.append(f"duplicate agreement {row['id']}")
    seen.add(row["id"])
    if not row["producer"].startswith("agent:"):
        errors.append(f"{row['id']} bad producer")
    if len(row.get("reviewers", [])) < 2:
        errors.append(f"{row['id']} requires two reviewers")
    if any(item == row["producer"] for item in row.get("reviewers", [])):
        errors.append(f"{row['id']} reviewer cannot be producer")
    if row["risk"] not in risks or row["decision"] not in decisions:
        errors.append(f"{row['id']} bad risk or decision")
    if row["risk"] in {"high", "audit"} and not row["adversarial"]:
        errors.append(f"{row['id']} requires adversarial review")
    if row["adversarial"] and "reviewer:red-team" not in row.get("reviewers", []):
        errors.append(f"{row['id']} adversarial reviewer missing")
    if row["isolation"] != "sealed":
        errors.append(f"{row['id']} reviewer isolation is not sealed")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("agreements"):
    errors.append("agreement manifest has no agreements")
if not data.get("closure_rules"):
    errors.append("agreement manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification agreement check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated agreement manifest is stale: run scripts/verificationagreementgen.sh generate" \
      "verification agreement generated output ok"
    validate_agreement ;;
  *) echo "usage: scripts/verificationagreementgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
