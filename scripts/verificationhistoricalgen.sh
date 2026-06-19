#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-historical-interpreter.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-historical-json))' |
    python3 -m json.tool
}

validate_historical() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" "$repo/docs/generated/verification-context-map.json" <<'PY'
import json, sys
data, evidence, context = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
edges = {row["id"] for row in context["translations"]}
errors = []
for row in data.get("interpretations", []):
    if row["interpreted_under"] == row["reinterpreted_under"]:
        errors.append(f"{row['id']} has identical ontology versions")
    for field in ("interpreted_under", "reinterpreted_under"):
        if not row[field].startswith("ontology:"):
            errors.append(f"{row['id']} bad {field}")
    if row["translation_edge"] not in edges:
        errors.append(f"{row['id']} missing bridge {row['translation_edge']}")
    if row["policy"] != "keep-original":
        errors.append(f"{row['id']} can only keep original evidence")
    if row["evidence"] not in outputs:
        errors.append(f"{row['id']} evidence is not generated output")
    for item in row.get("supporting_evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown support {item}")
if not data.get("interpretations"):
    errors.append("historical interpreter has no interpretations")
if not data.get("closure_rules"):
    errors.append("historical interpreter has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification historical interpreter check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification historical interpreter is stale: run scripts/verificationhistoricalgen.sh generate" \
      "verification historical interpreter generated output ok"
    validate_historical ;;
  *) echo "usage: scripts/verificationhistoricalgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
