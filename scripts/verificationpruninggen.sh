#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-pruning.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-pruning-json))' |
    python3 -m json.tool
}

validate_pruning() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
actions = {"retain", "replace-with-tombstone", "delete-with-tombstone"}
statuses = {"candidate", "protected", "approved", "rejected"}
errors, seen = [], set()
for row in data.get("decisions", []):
    if row["id"] in seen:
        errors.append(f"duplicate pruning decision {row['id']}")
    seen.add(row["id"])
    if row["target"] not in outputs:
        errors.append(f"{row['id']} unknown target {row['target']}")
    if row["action"] not in actions:
        errors.append(f"{row['id']} bad action")
    if row["status"] not in statuses:
        errors.append(f"{row['id']} bad status")
    if row["status"] == "protected" and row["action"] != "retain":
        errors.append(f"{row['id']} protected evidence must be retained")
    if row["action"] != "retain" and not row["tombstone"].startswith("tombstone:"):
        errors.append(f"{row['id']} missing tombstone")
    if row["authority"].startswith("agent:"):
        errors.append(f"{row['id']} authority cannot be an agent")
    if not row.get("immutable_reasons"):
        errors.append(f"{row['id']} missing immutable reasons")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("decisions"):
    errors.append("pruning manifest has no decisions")
if not data.get("closure_rules"):
    errors.append("pruning manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification pruning check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification pruning is stale: run scripts/verificationpruninggen.sh generate" \
      "verification pruning generated output ok"
    validate_pruning ;;
  *) echo "usage: scripts/verificationpruninggen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
