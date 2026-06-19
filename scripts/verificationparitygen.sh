#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-backend-parity.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-backend-parity-json))' |
    python3 -m json.tool
}

validate_parity() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, os, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
nodes = [row["id"] for row in evidence["verification_nodes"]]
backends = {row["backend"]: row for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("projections", []):
    if row["id"] in seen:
        errors.append(f"duplicate parity projection {row['id']}")
    seen.add(row["id"])
    if row.get("graph_nodes") != nodes:
        errors.append(f"{row['id']} graph node mismatch")
    backend = backends.get(row["backend"])
    if not backend:
        errors.append(f"{row['id']} missing backend evidence")
        continue
    for key in ("output", "generator", "check"):
        if row[key] != backend[key]:
            errors.append(f"{row['id']} {key} mismatch")
    if not os.path.exists(row["output"]):
        errors.append(f"{row['id']} missing output {row['output']}")
    if not os.path.exists(row["generator"]):
        errors.append(f"{row['id']} missing generator {row['generator']}")
if len(data.get("projections", [])) != 4:
    errors.append("backend parity requires four execution projections")
if not data.get("closure_rules"):
    errors.append("backend parity manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification backend parity check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated backend parity is stale: run scripts/verificationparitygen.sh generate" \
      "verification backend parity generated output ok"
    validate_parity ;;
  *) echo "usage: scripts/verificationparitygen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
