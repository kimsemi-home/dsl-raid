#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-ssot-defect.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-ssot-defect-json))' |
    python3 -m json.tool
}

validate_ssot_defect() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" "$repo" <<'PY'
import json, pathlib, sys
data, evidence, repo = json.load(open(sys.argv[1])), json.load(open(sys.argv[2])), pathlib.Path(sys.argv[3])
outputs = {row["output"] for row in evidence["generated_backends"]}
statuses = {"drill", "candidate", "confirmed", "closed"}
errors = []
for row in data.get("defects", []):
    if row["status"] not in statuses:
        errors.append(f"{row['id']} bad status")
    if not (repo / row["ssot"]).exists():
        errors.append(f"{row['id']} missing ssot {row['ssot']}")
    if not row.get("affected_scope"):
        errors.append(f"{row['id']} missing affected scope")
    for field, prefix in (("freeze", "freeze:"), ("migration_plan", "migration:"), ("verification_plan", "verification:")):
        if not row[field].startswith(prefix):
            errors.append(f"{row['id']} bad {field}")
    if row["authority"].startswith("agent:"):
        errors.append(f"{row['id']} authority cannot be an agent")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("defects"):
    errors.append("ssot defect manifest has no defects")
if not data.get("closure_rules"):
    errors.append("ssot defect manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification ssot defect check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification ssot defect is stale: run scripts/verificationssotdefectgen.sh generate" \
      "verification ssot defect generated output ok"
    validate_ssot_defect ;;
  *) echo "usage: scripts/verificationssotdefectgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
