#!/usr/bin/env bash
set -euo pipefail

repo="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
mode="${1:-check}"
input="${2:-examples/runscope/runscope.raid.json}"
out="${3:-docs/generated/fsm-catalog.md}"

run_dslraid() {
  cargo run -p dslraid-cli --quiet -- "$@"
}

cd "$repo"

case "$mode" in
  generate)
    run_dslraid doc fsm-catalog generate "$input" --out "$out"
    echo "generated $out"
    ;;
  check)
    run_dslraid doc fsm-catalog check "$input" --golden "$out"
    echo "fsm generated doc ok"
    ;;
  *)
    echo "usage: scripts/fsmgen.sh [generate|check] [input] [out]" >&2
    exit 2
    ;;
esac
