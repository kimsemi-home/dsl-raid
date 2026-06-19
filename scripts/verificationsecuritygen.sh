#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-security-audit.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-security-audit-json))' |
    python3 -m json.tool
}

validate_security() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
risks, kinds, errors, seen = {"low", "medium", "high"}, {"permission-change"}, [], set()
for row in data.get("boundaries", []):
    if row["id"] in seen:
        errors.append(f"duplicate boundary {row['id']}")
    seen.add(row["id"])
    if row["change_kind"] not in kinds:
        errors.append(f"{row['id']} bad change kind")
    if row["risk"] not in risks:
        errors.append(f"{row['id']} bad risk")
    if row["risk"] == "high" and not row["approval"].startswith("human:"):
        errors.append(f"{row['id']} high risk requires human approval")
    if row["authority"].startswith("agent:"):
        errors.append(f"{row['id']} authority cannot be an agent")
    if row["semantic_hash"] not in outputs:
        errors.append(f"{row['id']} unknown semantic hash")
    if row["conformance"] not in outputs:
        errors.append(f"{row['id']} unknown conformance")
    if not row.get("affected_paths"):
        errors.append(f"{row['id']} missing affected paths")
    if not row.get("rollback", "").startswith("rollback:"):
        errors.append(f"{row['id']} missing rollback")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("boundaries"):
    errors.append("security audit manifest has no boundaries")
if not data.get("closure_rules"):
    errors.append("security audit manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification security audit check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated security audit is stale: run scripts/verificationsecuritygen.sh generate" \
      "verification security audit generated output ok"
    validate_security ;;
  *) echo "usage: scripts/verificationsecuritygen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
