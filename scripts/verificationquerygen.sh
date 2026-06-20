#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-query-surface.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-query-surface-json))' |
    python3 -m json.tool
}

validate_query_surface() {
  python3 "$repo/scripts/lib/query_surface_check.py" "$repo/$out"
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated query surface is stale: run scripts/verificationquerygen.sh generate" \
      "verification query surface generated output ok"
    validate_query_surface ;;
  *) echo "usage: scripts/verificationquerygen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
