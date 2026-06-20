#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-experiment-decision.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-experiment-decision-json))' |
    python3 -m json.tool
}

validate_decisions() {
  python3 - "$repo" "$repo/$out" \
    "$repo/docs/generated/verification-experiment-loop.json" <<'PY'
import json, pathlib, sys
repo, path, experiment_path = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
data, experiments = json.load(open(path)), json.load(open(experiment_path))
records = {row["id"]: row for row in experiments.get("experiments", [])}
decisions = {row["experiment"]: row for row in data.get("decisions", [])}
errors, seen = [], set()
for row in data.get("decisions", []):
    rid, exp = row["id"], records.get(row["experiment"])
    if rid in seen:
        errors.append(f"duplicate decision {rid}")
    seen.add(rid)
    if exp is None:
        errors.append(f"{rid} references unknown experiment")
        continue
    if row["act"] != exp["act"]:
        errors.append(f"{rid} act does not match experiment")
    if row["decision"] == "promote" and not exp.get("promoted"):
        errors.append(f"{rid} promotes an unpromoted experiment")
    if row["decision"] == "promote" and row["status"] != "closed":
        errors.append(f"{rid} promotion must be closed")
    if exp["status"] not in {"checked", "promoted"}:
        errors.append(f"{rid} experiment was not checked")
    for item in row.get("evidence", []) + [row["act"]]:
        if not (repo / item).exists():
            errors.append(f"{rid} missing evidence {item}")
    if not row["gate"].startswith("gate:"):
        errors.append(f"{rid} gate must be semantic")
for exp in records.values():
    if exp.get("promoted") and exp["id"] not in decisions:
        errors.append(f"{exp['id']} missing decision")
if not data.get("decisions"):
    errors.append("experiment decision has no decisions")
if not data.get("closure_rules"):
    errors.append("experiment decision has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification experiment decision check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated experiment decision is stale: run scripts/verificationexperimentdecisiongen.sh generate" \
      "verification experiment decision generated output ok"
    validate_decisions ;;
  *) echo "usage: scripts/verificationexperimentdecisiongen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
