#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/lisp-runtime.sh"

repo="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
mode="${1:-check}"
out="${2:-docs/generated/verification-ontology.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-ontology-json))' |
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
    echo "verification ontology generated output ok"
    ;;
  *)
    echo "usage: scripts/verificationontologygen.sh [generate|check] [out]" >&2
    exit 2
    ;;
esac
