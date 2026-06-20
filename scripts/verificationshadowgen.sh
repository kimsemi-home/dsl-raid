#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-shadow-orchestrator.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-shadow-json))' |
    python3 -m json.tool
}

validate_shadow() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" \
    "$repo/docs/generated/verification-control-plane.json" <<'PY'
import json, sys
data, evidence, control = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
expected = {row["shadow"] for row in control["routes"]}
actions = {"observe", "human-review", "authority-blocked"}
errors, seen, actual = [], set(), set()
for row in data.get("routes", []):
    if row["id"] in seen: errors.append(f"duplicate shadow route {row['id']}")
    seen.add(row["id"]); actual.add(row["shadow"])
    if row["primary"] == row["shadow"]: errors.append(f"{row['id']} not separated")
    if not row["primary"].startswith("control-plane:"): errors.append(f"{row['id']} bad primary")
    if not row["shadow"].startswith("shadow:"): errors.append(f"{row['id']} bad shadow")
    if row["action"] not in actions: errors.append(f"{row['id']} bad action")
    if row["divergence"] == "D3" and row["action"] == "observe":
        errors.append(f"{row['id']} D3 must review")
    if row["divergence"] == "D4" and row["action"] != "authority-blocked":
        errors.append(f"{row['id']} D4 must block")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"{row['id']} unknown evidence {item}")
if expected - actual: errors.append(f"missing shadows {sorted(expected - actual)}")
if not data.get("routes"): errors.append("shadow manifest has no routes")
if not data.get("closure_rules"): errors.append("shadow manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification shadow orchestrator check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated shadow orchestrator is stale: run scripts/verificationshadowgen.sh generate" \
      "verification shadow orchestrator generated output ok"
    validate_shadow ;;
  *) echo "usage: scripts/verificationshadowgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
