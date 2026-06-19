#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/lisp-runtime.sh"

repo="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
mode="${1:-check}"
out="${2:-docs/generated/verification-evidence.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-evidence-json))'
}

check_json() {
  python3 -m json.tool "$1" >/dev/null
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    check_json "$repo/$out"
    echo "generated $out"
    ;;
  check)
    tmp="$(mktemp)"
    trap 'rm -f "$tmp"' EXIT
    generate > "$tmp"
    check_json "$tmp"
    diff -u "$repo/$out" "$tmp"
    check_json "$repo/$out"
    echo "verification evidence generated output ok"
    ;;
  *)
    echo "usage: scripts/verificationevidencegen.sh [generate|check] [out]" >&2
    exit 2
    ;;
esac
