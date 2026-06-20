#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"
dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-knowledge-conversion.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-knowledge-conversion-json))' |
    python3 -m json.tool
}

validate_knowledge_conversion() {
  PYTHONDONTWRITEBYTECODE=1 \
    python3 "$repo/scripts/lib/knowledge_conversion_check.py" "$repo/$out" "$repo"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated knowledge conversion is stale: run scripts/verificationknowledgegen.sh generate" \
      "verification knowledge conversion generated output ok"
    validate_knowledge_conversion ;;
  *) echo "usage: scripts/verificationknowledgegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
