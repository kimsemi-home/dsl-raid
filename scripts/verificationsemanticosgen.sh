#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"
dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-semantic-os.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-semantic-os-json))' |
    python3 -m json.tool
}

validate_semantic_os() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/semantic_os_check.py" \
    "$repo/$out" \
    "$repo"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated semantic os is stale: run scripts/verificationsemanticosgen.sh generate" \
      "verification semantic os generated output ok"
    validate_semantic_os ;;
  *) echo "usage: scripts/verificationsemanticosgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
