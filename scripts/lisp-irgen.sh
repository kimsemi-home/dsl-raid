#!/usr/bin/env bash
set -euo pipefail

repo="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
mode="${1:-check}"
out="${2:-examples/runscope/runscope.lisp.raid.json}"
target="$repo/$out"

run_sbcl() {
  sbcl --noinform --non-interactive \
    --eval '(require :asdf)' \
    --eval '(asdf:load-asd (merge-pathnames "lisp/dslraid.asd" (uiop:getcwd)))' \
    --eval '(let ((*standard-output* (make-broadcast-stream))) (asdf:load-system :dslraid))' \
    --eval '(write-string (dslraid:emit-project-json "runscope" "RunScope" (dslraid:runscope-fsms)))'
}

validate_ir() {
  cargo run --quiet -p dslraid-cli -- validate "$target" \
    --schema schemas/dslraid-core.schema.json \
    --format text \
    --deny warning >/dev/null
}

cd "$repo"

case "$mode" in
  generate)
    mkdir -p "$(dirname "$target")"
    run_sbcl > "$target"
    validate_ir
    echo "generated $out"
    ;;
  check)
    tmp="$(mktemp)"
    trap 'rm -f "$tmp"' EXIT
    run_sbcl > "$tmp"
    if ! diff -u "$target" "$tmp"; then
      echo "generated Lisp IR is stale: run scripts/lisp-irgen.sh generate" >&2
      exit 1
    fi
    validate_ir
    echo "lisp generated ir ok"
    ;;
  *)
    echo "usage: scripts/lisp-irgen.sh [generate|check] [out]" >&2
    exit 2
    ;;
esac
