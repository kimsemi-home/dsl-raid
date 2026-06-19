#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-control-plane.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-control-plane-json))' |
    python3 -m json.tool
}

validate_control_plane() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
divergences = {"D0", "D1", "D2", "D3", "D4"}
severities = {"none", "review", "human-review", "authority-blocked"}
errors, seen = [], set()
for row in data.get("routes", []):
    if row["id"] in seen: errors.append(f"duplicate route {row['id']}")
    seen.add(row["id"])
    if row["control_plane"] == row["verifier"]:
        errors.append(f"{row['id']} control plane verifies itself")
    if not row["verifier"].startswith("sidecar:"):
        errors.append(f"{row['id']} verifier is not sidecar")
    if not row["shadow"].startswith("shadow:"):
        errors.append(f"{row['id']} missing shadow orchestrator")
    if row["divergence"] not in divergences or row["severity"] not in severities:
        errors.append(f"{row['id']} bad divergence or severity")
    if row["divergence"] in {"D3", "D4"} and row["severity"] == "review":
        errors.append(f"{row['id']} high divergence lacks gate")
    if row["divergence"] == "D4" and row["severity"] != "authority-blocked":
        errors.append(f"{row['id']} D4 must block authority")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("routes"): errors.append("control-plane manifest has no routes")
if not data.get("closure_rules"): errors.append("control-plane manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification control-plane check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated control-plane verifier is stale: run scripts/verificationcontrolgen.sh generate" \
      "verification control-plane generated output ok"
    validate_control_plane ;;
  *) echo "usage: scripts/verificationcontrolgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
