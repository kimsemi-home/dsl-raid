#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-feedback.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-feedback-json))' |
    python3 -m json.tool
}

validate_feedback() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/verification_feedback_check.py" \
    "$repo/$out" "$repo/docs/generated/verification-evidence.json"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification feedback is stale: run scripts/verificationfeedbackgen.sh generate" \
      "verification feedback generated output ok"
    validate_feedback ;;
  *) echo "usage: scripts/verificationfeedbackgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
