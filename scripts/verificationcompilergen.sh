#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"
dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-governed-compiler.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-governed-compiler-json))' |
    python3 -m json.tool
}

validate_compiler_farm() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/governed_compiler_check.py" "$repo/$out" "$repo"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated governed compiler is stale: run scripts/verificationcompilergen.sh generate" \
      "verification governed compiler generated output ok"
    validate_compiler_farm ;;
  *) echo "usage: scripts/verificationcompilergen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
