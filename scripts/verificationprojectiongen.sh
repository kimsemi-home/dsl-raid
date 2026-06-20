#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-execution-projection.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-execution-projection-json))' |
    python3 -m json.tool
}

validate_execution_projection() {
  python3 "$repo/scripts/lib/execution_projection_check.py" "$repo/$out"
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated execution projection is stale: run scripts/verificationprojectiongen.sh generate" \
      "verification execution projection generated output ok"
    validate_execution_projection ;;
  *) echo "usage: scripts/verificationprojectiongen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
