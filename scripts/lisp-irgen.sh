#!/usr/bin/env bash
set -euo pipefail

repo="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
mode="${1:-check}"
out="${2:-examples/runscope/runscope.lisp.raid.json}"
target="$repo/$out"

source "$repo/scripts/lib/lisp-runtime.sh"

run_irgen() {
  dslraid_lisp_eval '(write-string (dslraid:emit-project-json "runscope" "RunScope" (dslraid:runscope-fsms)))'
}

validate_ir() {
  if [ "${DSLRAID_SKIP_RUST_VALIDATE:-0}" = "1" ]; then
    return 0
  fi
  cargo run --quiet -p dslraid-cli -- validate "$target" \
    --schema schemas/dslraid-core.schema.json \
    --format text \
    --deny warning >/dev/null
}

cd "$repo"

case "$mode" in
  generate)
    mkdir -p "$(dirname "$target")"
    run_irgen > "$target"
    validate_ir
    echo "generated $out"
    ;;
  check)
    tmp="$(mktemp)"
    trap 'rm -f "$tmp"' EXIT
    run_irgen > "$tmp"
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
