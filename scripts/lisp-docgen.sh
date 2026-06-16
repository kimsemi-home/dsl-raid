#!/usr/bin/env bash
set -euo pipefail

repo="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
mode="${1:-check}"
out="${2:-docs/generated/lisp-pipeline.md}"
target="$repo/$out"

run_sbcl() {
  sbcl --noinform --non-interactive \
    --eval '(require :asdf)' \
    --eval '(asdf:load-asd (merge-pathnames "lisp/dslraid.asd" (uiop:getcwd)))' \
    --eval '(let ((*standard-output* (make-broadcast-stream))) (asdf:load-system :dslraid))' \
    --eval '(write-string (dslraid:emit-language-pipeline-markdown))'
}

cd "$repo"

case "$mode" in
  generate)
    mkdir -p "$(dirname "$target")"
    run_sbcl > "$target"
    echo "generated $out"
    ;;
  check)
    tmp="$(mktemp)"
    trap 'rm -f "$tmp"' EXIT
    run_sbcl > "$tmp"
    if ! diff -u "$target" "$tmp"; then
      echo "generated Lisp pipeline doc is stale: run scripts/lisp-docgen.sh generate" >&2
      exit 1
    fi
    echo "lisp generated doc ok"
    ;;
  *)
    echo "usage: scripts/lisp-docgen.sh [generate|check] [out]" >&2
    exit 2
    ;;
esac
