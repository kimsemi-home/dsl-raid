#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-learning-loop.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-learning-json))' |
    python3 -m json.tool
}

validate_learning() {
  PYTHONDONTWRITEBYTECODE=1 python3 "$repo/scripts/lib/learning_loop_check.py" \
    "$repo/$out" "$repo/docs/generated/verification-evidence.json" "$repo"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification learning loop is stale: run scripts/verificationlearninggen.sh generate" \
      "verification learning loop generated output ok"
    validate_learning ;;
  *) echo "usage: scripts/verificationlearninggen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
