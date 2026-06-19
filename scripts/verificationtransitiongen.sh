#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-ontology-transition.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-ontology-transition-json))' |
    python3 -m json.tool
}

validate_transition() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
required = {"lane:legacy", "lane:migration", "lane:new", "lane:audit"}
errors = []
for row in data.get("transitions", []):
    if row["from_version"] == row["to_version"]:
        errors.append(f"{row['id']} has identical ontology versions")
    for field in ("from_version", "to_version"):
        if not row[field].startswith("ontology:"):
            errors.append(f"{row['id']} bad {field}")
    if not required.issubset(set(row.get("lanes", []))):
        errors.append(f"{row['id']} missing transition lanes")
    if row["cutover_gate"].startswith("agent:"):
        errors.append(f"{row['id']} cutover gate cannot be an agent")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("transitions"):
    errors.append("ontology transition has no transitions")
if not data.get("closure_rules"):
    errors.append("ontology transition has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification ontology transition check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification ontology transition is stale: run scripts/verificationtransitiongen.sh generate" \
      "verification ontology transition generated output ok"
    validate_transition ;;
  *) echo "usage: scripts/verificationtransitiongen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
