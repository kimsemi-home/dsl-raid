#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-objective-coverage.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-objective-coverage-json))' |
    python3 -m json.tool
}

validate_objective_coverage() {
  PYTHONDONTWRITEBYTECODE=1 python3 "$script_dir/lib/objective_coverage_check.py" "$repo/$out"
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated objective coverage is stale: run scripts/verificationobjectivegen.sh generate" \
      "verification objective coverage generated output ok"
    validate_objective_coverage ;;
  *) echo "usage: scripts/verificationobjectivegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
