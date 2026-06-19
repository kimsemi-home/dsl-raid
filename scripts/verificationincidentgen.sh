#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-incident-learning.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-incident-json))' |
    python3 -m json.tool
}

validate_incident() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
errors = []
for row in data.get("cycles", []):
    if row["id"] in {""}: errors.append("empty incident id")
    if row["owner"].startswith("agent:"):
        errors.append(f"{row['id']} owner cannot be an agent")
    if row["status"] not in {"open", "closed", "revalidating"}:
        errors.append(f"{row['id']} bad status")
    if row["status"] == "closed":
        for field in ("knowledge_update", "revalidation", "prevention"):
            if not row.get(field): errors.append(f"{row['id']} missing {field}")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("cycles"):
    errors.append("incident learning manifest has no cycles")
if not data.get("closure_rules"):
    errors.append("incident learning manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification incident learning check ok")
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
      "generated verification incident learning is stale: run scripts/verificationincidentgen.sh generate" \
      "verification incident learning generated output ok"
    validate_incident
    ;;
  *) echo "usage: scripts/verificationincidentgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
