#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-learning-loop.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-learning-json))' |
    python3 -m json.tool
}

validate_learning() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
text = pathlib.Path(sys.argv[1]).read_text()
errors, stages = [], {row["id"]: row for row in data.get("stages", [])}
orders = sorted(row.get("order") for row in stages.values())
if orders != list(range(1, len(orders) + 1)):
    errors.append("learning stages must be contiguous")
home_prefix = "/" + "Users" + "/"
if home_prefix in text:
    errors.append("learning loop leaked a private local path")
for row in stages.values():
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
statuses = {"open", "closed", "revalidating"}
for row in data.get("cycles", []):
    if row["status"] not in statuses:
        errors.append(f"{row['id']} bad status")
    if row["owner"].startswith("agent:"):
        errors.append(f"{row['id']} owner cannot be an agent")
    if not row["knowledge_update"].startswith("update:"):
        errors.append(f"{row['id']} missing knowledge update")
    if not row["revalidation"].startswith("revalidate:"):
        errors.append(f"{row['id']} missing revalidation")
    for stage in row.get("stages", []):
        if stage not in stages:
            errors.append(f"{row['id']} unknown stage {stage}")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("cycles"): errors.append("learning loop has no cycles")
if not data.get("closure_rules"): errors.append("learning loop has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification learning loop check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification learning loop is stale: run scripts/verificationlearninggen.sh generate" \
      "verification learning loop generated output ok"
    validate_learning ;;
  *) echo "usage: scripts/verificationlearninggen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
