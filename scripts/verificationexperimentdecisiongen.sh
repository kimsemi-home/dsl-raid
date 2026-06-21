#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-experiment-decision.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-experiment-decision-json))' |
    python3 -m json.tool
}

validate_decisions() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/verification_experiment_decision_check.py" \
    "$repo" "$repo/$out" \
    "$repo/docs/generated/verification-experiment-loop.json"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated experiment decision is stale: run scripts/verificationexperimentdecisiongen.sh generate" \
      "verification experiment decision generated output ok"
    validate_decisions ;;
  *) echo "usage: scripts/verificationexperimentdecisiongen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
