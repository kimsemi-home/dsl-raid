#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-cold-start-gate.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-cold-start-gate-json))' |
    python3 -m json.tool
}

validate_cold_start() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
stages = {"candidate", "shadow", "assisted", "bounded"}
effects = {"proposal-only", "shadow-only", "human-review", "bounded-operation"}
errors, seen = [], set()
for row in data.get("gates", []):
    if row["id"] in seen:
        errors.append(f"duplicate cold-start gate {row['id']}")
    seen.add(row["id"])
    if row["stage"] not in stages or row["authority_effect"] not in effects:
        errors.append(f"{row['id']} bad stage or effect")
    if row["promotion_owner"].startswith("agent:"):
        errors.append(f"{row['id']} promotion owner cannot be agent")
    if set(row["allowed"]) & set(row["blocked"]):
        errors.append(f"{row['id']} allowed and blocked overlap")
    if row["stage"] in {"candidate", "shadow"} and row["authority_effect"] == "bounded-operation":
        errors.append(f"{row['id']} cold stage cannot have bounded operation")
    if row["stage"] in {"candidate", "shadow"} and not set(row["blocked"]) & {"production-change", "release"}:
        errors.append(f"{row['id']} cold stage must block production or release")
    if not row.get("promotion_requires"):
        errors.append(f"{row['id']} missing promotion requirements")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("gates"):
    errors.append("cold-start manifest has no gates")
if not data.get("closure_rules"):
    errors.append("cold-start manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification cold-start gate check ok")
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
      "generated cold-start gate is stale: run scripts/verificationcoldstartgen.sh generate" \
      "verification cold-start gate generated output ok"
    validate_cold_start
    ;;
  *) echo "usage: scripts/verificationcoldstartgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
