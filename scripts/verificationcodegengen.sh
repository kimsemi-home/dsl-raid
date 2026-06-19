#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-codegen.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-codegen-json))' |
    python3 -m json.tool
}

validate_map() {
  python3 - "$repo/$out" \
    "$repo/docs/generated/verification-ontology.json" \
    "$repo/docs/generated/verification-evidence.json" <<'PY'
import json
import sys

codegen, ontology, evidence = [json.load(open(path)) for path in sys.argv[1:]]
axis_map = {row["axis"]: row["backends"] for row in codegen["axes"]}
ontology_axes = ontology["codegen_axes"]
evidence_backends = {row["backend"]: row for row in evidence["generated_backends"]}
errors = []

if list(axis_map) != ontology_axes:
    errors.append("codegen axes do not match ontology order")

for axis in ontology_axes:
    if not axis_map.get(axis):
        errors.append(f"axis has no backend: {axis}")

for axis, backend_ids in axis_map.items():
    for backend_id in backend_ids:
        backend = evidence_backends.get(backend_id)
        if backend is None:
            errors.append(f"{axis} maps unknown backend {backend_id}")
        elif not backend.get("check"):
            errors.append(f"{backend_id} has no check command")

if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)

print("verification codegen semantic map ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out"
    ;;
  check)
    dslraid_generated_check \
      "$out" \
      "generated verification codegen map is stale: run scripts/verificationcodegengen.sh generate" \
      "verification codegen map generated output ok"
    validate_map
    ;;
  *)
    echo "usage: scripts/verificationcodegengen.sh [generate|check] [out]" >&2
    exit 2
    ;;
esac
