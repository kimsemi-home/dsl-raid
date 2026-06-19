#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-incompleteness-ledger.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-incompleteness-json))' |
    python3 -m json.tool
}

validate_incomplete() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
kinds = {"unknown", "gap", "drift", "assumption"}
effects = {"authority-blocked", "review-required", "human-review"}
statuses = {"open", "triaged", "closed"}
errors, seen = [], set()
for row in data.get("unknowns", []):
    if row["id"] in seen:
        errors.append(f"duplicate unknown {row['id']}")
    seen.add(row["id"])
    if row["unknown_kind"] not in kinds:
        errors.append(f"{row['id']} bad unknown kind")
    if row["status"] not in statuses:
        errors.append(f"{row['id']} bad status")
    if row["authority_effect"] not in effects:
        errors.append(f"{row['id']} bad authority effect")
    if row["owner"].startswith("agent:"):
        errors.append(f"{row['id']} owner cannot be an agent")
    if row["status"] == "open" and not row["next_action"].startswith("classify:"):
        errors.append(f"{row['id']} open unknown missing classification")
    if row["source"] not in outputs:
        errors.append(f"{row['id']} unknown source {row['source']}")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("unknowns"):
    errors.append("incompleteness ledger has no unknowns")
if not data.get("closure_rules"):
    errors.append("incompleteness ledger has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification incompleteness ledger check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated incompleteness ledger is stale: run scripts/verificationincompletegen.sh generate" \
      "verification incompleteness ledger generated output ok"
    validate_incomplete ;;
  *) echo "usage: scripts/verificationincompletegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
