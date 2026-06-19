#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-debt.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-debt-json))' |
    python3 -m json.tool
}

validate_debt() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
kinds = {"review", "verification", "evidence", "automation", "governance"}
statuses, errors, seen = {"open", "mitigated", "closed"}, [], set()
for row in data.get("records", []):
    if row["id"] in seen:
        errors.append(f"duplicate debt {row['id']}")
    seen.add(row["id"])
    if row["debt_kind"] not in kinds:
        errors.append(f"{row['id']} bad debt kind")
    if row["status"] not in statuses:
        errors.append(f"{row['id']} bad status")
    if row["owner"].startswith("agent:"):
        errors.append(f"{row['id']} owner cannot be an agent")
    if not row["repayment"].startswith("repay:"):
        errors.append(f"{row['id']} missing repayment")
    if row["status"] == "open" and not row["revalidation"].startswith("revalidate:"):
        errors.append(f"{row['id']} open debt missing revalidation")
    if row["source"] not in outputs:
        errors.append(f"{row['id']} unknown source {row['source']}")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("records"):
    errors.append("debt manifest has no records")
if not data.get("closure_rules"):
    errors.append("debt manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification debt check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification debt is stale: run scripts/verificationdebtgen.sh generate" \
      "verification debt generated output ok"
    validate_debt ;;
  *) echo "usage: scripts/verificationdebtgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
