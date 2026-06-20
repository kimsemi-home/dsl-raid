#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-confidence-decision.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-confidence-decision-json))' |
    python3 -m json.tool
}

validate_decisions() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-confidence.json" \
    "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, ceilings, evidence = [json.load(open(path)) for path in sys.argv[1:]]
ceiling_map = {row["id"]: row for row in ceilings.get("ceilings", [])}
outputs = {row["output"] for row in evidence.get("generated_backends", [])}
effects = {"bounded-auto", "release-eligible", "review-required", "blocked"}
errors, seen = [], set()
for row in data.get("decisions", []):
    rid, ceiling = row["id"], ceiling_map.get(row["ceiling"])
    if rid in seen: errors.append(f"duplicate confidence decision {rid}")
    seen.add(rid)
    if ceiling is None:
        errors.append(f"{rid} references unknown ceiling")
        continue
    if row["gate"].startswith("agent:"):
        errors.append(f"{rid} gate cannot be agent")
    if not set(row.get("requires", [])).issubset(set(ceiling.get("requires", []))):
        errors.append(f"{rid} requires signals outside ceiling")
    if row["decision"] == "raise" and row["status"] != "closed":
        errors.append(f"{rid} raise must be closed")
    if row["authority_effect"] not in effects:
        errors.append(f"{rid} bad authority effect")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{rid} unknown evidence {item}")
if not data.get("decisions"): errors.append("confidence decision has no decisions")
if not data.get("closure_rules"): errors.append("confidence decision has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification confidence decision check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated confidence decision is stale: run scripts/verificationconfidencedecisiongen.sh generate" \
      "verification confidence decision generated output ok"
    validate_decisions ;;
  *) echo "usage: scripts/verificationconfidencedecisiongen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
