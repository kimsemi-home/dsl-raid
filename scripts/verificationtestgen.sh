#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/lisp-runtime.sh"

repo="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
mode="${1:-check}"
out="${2:-tests/golden/verification-graph.generated.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-test-manifest-json))' |
    python3 -m json.tool
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out"
    ;;
  check)
    tmp="$(mktemp)"
    trap 'rm -f "$tmp"' EXIT
    generate > "$tmp"
    diff -u "$repo/$out" "$tmp"
    echo "verification test manifest generated output ok"
    ;;
  *)
    echo "usage: scripts/verificationtestgen.sh [generate|check] [out]" >&2
    exit 2
    ;;
esac
