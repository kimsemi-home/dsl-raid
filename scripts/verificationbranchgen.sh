#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-branch-protection.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-branch-protection-json))' |
    python3 -m json.tool
}

validate_branch_protection() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/branch_protection_check.py" "$repo/$out" "$repo"
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated branch protection is stale: run scripts/verificationbranchgen.sh generate" \
      "verification branch protection generated output ok"
    validate_branch_protection ;;
  *) echo "usage: scripts/verificationbranchgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
