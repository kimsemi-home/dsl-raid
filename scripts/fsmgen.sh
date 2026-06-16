#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"

mode="${1:-check}"
input="${2:-examples/runscope/runscope.raid.json}"
out="${3:-docs/generated/fsm-catalog.md}"

generate() {
  fsm_tmp="$(mktemp)"
  cargo run -p dslraid-cli --quiet -- doc fsm-catalog generate "$input" --out "$fsm_tmp" \
    >/dev/null
  cat "$fsm_tmp"
  rm -f "$fsm_tmp"
}

dslraid_enter_repo
dslraid_generated_case \
  "$mode" "$out" \
  "generated FSM catalog is stale: run scripts/fsmgen.sh generate" \
  "fsm generated doc ok" \
  "usage: scripts/fsmgen.sh [generate|check] [input] [out]"
