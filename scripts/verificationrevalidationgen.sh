#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-revalidation-gate.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-revalidation-gate-json))' |
    python3 -m json.tool
}

validate_revalidation() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
statuses = {"valid", "due-soon", "due", "grace", "expired", "frozen", "revalidated", "superseded", "retired"}
restricted = {"due-soon", "due", "grace", "expired", "frozen"}
effects = {"normal-authority", "review-required", "authority-limited", "authority-frozen"}
errors, seen = [], set()
for row in data.get("gates", []):
    if row["id"] in seen:
        errors.append(f"duplicate gate {row['id']}")
    seen.add(row["id"])
    if row["status"] not in statuses:
        errors.append(f"{row['id']} bad status")
    if row["authority_effect"] not in effects:
        errors.append(f"{row['id']} bad authority effect")
    if row["owner"].startswith("agent:"):
        errors.append(f"{row['id']} owner cannot be an agent")
    if "T" not in row["due_at"]:
        errors.append(f"{row['id']} due_at must be timestamp-like")
    if row["status"] == "valid" and row["authority_effect"] != "normal-authority":
        errors.append(f"{row['id']} valid gate should have normal authority")
    if row["status"] in restricted and row["authority_effect"] == "normal-authority":
        errors.append(f"{row['id']} restricted status cannot be normal")
    if row["authority_effect"] != "normal-authority" and "none" in row["blocks"]:
        errors.append(f"{row['id']} restricted authority needs real blocks")
    if set(row["blocks"]) & set(row["allowed"]):
        errors.append(f"{row['id']} blocks and allowed overlap")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("gates"):
    errors.append("revalidation gate manifest has no gates")
if not data.get("closure_rules"):
    errors.append("revalidation gate manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification revalidation gate check ok")
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
      "generated verification revalidation gate is stale: run scripts/verificationrevalidationgen.sh generate" \
      "verification revalidation gate generated output ok"
    validate_revalidation
    ;;
  *) echo "usage: scripts/verificationrevalidationgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
