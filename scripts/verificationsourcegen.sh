#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-source-shape.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-source-shape-json))' |
    python3 -m json.tool
}

validate_source_shape() {
  python3 "$script_dir/lib/source_shape_check.py" "$repo/$out"
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated source shape is stale: run scripts/verificationsourcegen.sh generate" \
      "verification source shape generated output ok"
    validate_source_shape ;;
  *) echo "usage: scripts/verificationsourcegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
