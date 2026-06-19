#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-meta-model.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-meta-model-json))' |
    python3 -m json.tool
}

validate_meta_model() {
  python3 - "$repo/$out" <<'PY'
import json, sys
data = json.load(open(sys.argv[1]))
required = {"Actor", "Artifact", "Evidence", "Claim", "Decision",
            "Change", "Review", "Risk", "Version", "Migration", "Audit"}
terms = data.get("terms", [])
errors, names, ids = [], {row.get("term") for row in terms}, set()
if not required <= names:
    errors.append("missing required terms: " + ",".join(sorted(required - names)))
for row in terms:
    if row["id"] in ids:
        errors.append(f"duplicate term id {row['id']}")
    ids.add(row["id"])
    if row["owner"].startswith("agent:"):
        errors.append(f"{row['id']} owner cannot be an agent")
    if not row.get("authority_gate"):
        errors.append(f"{row['id']} missing authority gate")
if not data.get("closure_rules"):
    errors.append("meta-model manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification meta-model check ok")
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
      "generated verification meta-model is stale: run scripts/verificationmetamodelgen.sh generate" \
      "verification meta-model generated output ok"
    validate_meta_model
    ;;
  *) echo "usage: scripts/verificationmetamodelgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
