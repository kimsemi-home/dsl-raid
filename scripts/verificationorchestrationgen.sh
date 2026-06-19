#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-orchestration.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-orchestration-json))' |
    python3 -m json.tool
}

validate_orchestration() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("routes", []):
    if row["id"] in seen: errors.append(f"duplicate route {row['id']}")
    seen.add(row["id"])
    if not row["control_plane"].startswith("control-plane:"):
        errors.append(f"bad control plane {row['id']}")
    if not row["agent"].startswith("agent:"): errors.append(f"bad agent {row['id']}")
    if not row["policy"].startswith("policy:"): errors.append(f"bad policy {row['id']}")
    if row["authority"].startswith("agent:"):
        errors.append(f"agent authority {row['id']}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"unknown evidence {row['id']} {item}")
    for item in row.get("outputs", []):
        if item not in outputs: errors.append(f"unknown output {row['id']} {item}")
    if not row.get("evidence") or not row.get("outputs"):
        errors.append(f"missing evidence or outputs {row['id']}")
if not data.get("routes"): errors.append("orchestration manifest has no routes")
if not data.get("closure_rules"): errors.append("orchestration has no closure rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification orchestration check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification orchestration is stale: run scripts/verificationorchestrationgen.sh generate" \
      "verification orchestration generated output ok"
    validate_orchestration ;;
  *) echo "usage: scripts/verificationorchestrationgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
