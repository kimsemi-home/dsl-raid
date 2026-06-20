#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"
dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-executable-knowledge.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-executable-knowledge-json))' |
    python3 -m json.tool
}

validate_executable_knowledge() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/executable_knowledge_check.py" \
    "$repo/$out" \
    "$repo"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated executable knowledge is stale: run scripts/verificationexecutablegen.sh generate" \
      "verification executable knowledge generated output ok"
    validate_executable_knowledge ;;
  *) echo "usage: scripts/verificationexecutablegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
