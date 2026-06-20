#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-language-expansion.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-language-expansion-json))' |
    python3 -m json.tool
}

validate_language_expansion() {
  python3 "$repo/scripts/lib/language_expansion_check.py" "$repo/$out"
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated language expansion is stale: run scripts/verificationlanguagegen.sh generate" \
      "verification language expansion generated output ok"
    validate_language_expansion ;;
  *) echo "usage: scripts/verificationlanguagegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
