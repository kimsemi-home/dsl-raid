#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-reliability.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-reliability-json))' |
    python3 -m json.tool
}

validate_reliability() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
tiers, statuses = {"T0", "T1", "T2", "T3", "T4"}, {"candidate", "shadow", "assisted", "bounded", "trusted"}
ceilings, effects = {"low", "medium", "high"}, {"authority-blocked", "human-review", "bounded-auto", "trusted-context"}
errors, seen = [], set()
for row in data.get("records", []):
    if row["id"] in seen:
        errors.append(f"duplicate reliability {row['id']}")
    seen.add(row["id"])
    if not row["agent"].startswith("agent:"):
        errors.append(f"{row['id']} bad agent")
    if row["tier"] not in tiers or row["status"] not in statuses:
        errors.append(f"{row['id']} bad tier or status")
    if row["confidence_ceiling"] not in ceilings or row["authority_effect"] not in effects:
        errors.append(f"{row['id']} bad ceiling or effect")
    if row["tier"] in {"T0", "T1"} and row["authority_effect"] in {"bounded-auto", "trusted-context"}:
        errors.append(f"{row['id']} cold-start cannot automate")
    if row["tier"] == "T4" and (row["verification_rate"] < 95 or row["failure_rate"] > 5):
        errors.append(f"{row['id']} trusted tier lacks history")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
    if not row.get("restrictions"):
        errors.append(f"{row['id']} missing restrictions")
if not data.get("records"):
    errors.append("reliability manifest has no records")
if not data.get("closure_rules"):
    errors.append("reliability manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification reliability check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated reliability registry is stale: run scripts/verificationreliabilitygen.sh generate" \
      "verification reliability generated output ok"
    validate_reliability ;;
  *) echo "usage: scripts/verificationreliabilitygen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
