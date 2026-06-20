#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-failure-recovery.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-failure-recovery-json))' |
    python3 -m json.tool
}

validate_recovery() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-failure-conditions.json" \
    "$repo/docs/generated/verification-incident-learning.json" \
    "$repo/docs/generated/verification-learning-loop.json" \
    "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, failure, incident, learning, evidence = [json.load(open(p)) for p in sys.argv[1:]]
responses = {row["id"]: row["response"] for row in failure["conditions"]}
outputs = {row["output"] for row in evidence["generated_backends"]}
incidents = {row["id"] for row in incident["cycles"]} | {row["incident"] for row in learning["cycles"]}
updates = {row["knowledge_update"] for row in incident["cycles"]} | {row["knowledge_update"] for row in learning["cycles"]}
rechecks = {row["revalidation"] for row in incident["cycles"]} | {row["revalidation"] for row in learning["cycles"]}
statuses = {"release-blocked-until-revalidated", "learning-open", "closed"}
errors, seen = [], set()
for row in data.get("recoveries", []):
    rid, fid = row["id"], row["failure"]
    if rid in seen: errors.append(f"duplicate recovery {rid}")
    seen.add(rid)
    if fid not in responses: errors.append(f"{rid} unknown failure {fid}")
    elif row["response"] != responses[fid]: errors.append(f"{rid} response mismatch")
    if row["incident"] not in incidents: errors.append(f"{rid} unknown incident")
    if row["learning_update"] not in updates: errors.append(f"{rid} unknown update")
    if row["revalidation"] not in rechecks: errors.append(f"{rid} unknown revalidation")
    if not row["release_gate"].startswith("gate:"): errors.append(f"{rid} bad gate")
    if row["status"] not in statuses: errors.append(f"{rid} bad status")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"{rid} unknown evidence {item}")
if not data.get("recoveries"): errors.append("failure recovery manifest has no records")
if not data.get("closure_rules"): errors.append("failure recovery manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification failure recovery check ok")
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
      "generated failure recovery is stale: run scripts/verificationfailurerecoverygen.sh generate" \
      "verification failure recovery generated output ok"
    validate_recovery
    ;;
  *) echo "usage: scripts/verificationfailurerecoverygen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
