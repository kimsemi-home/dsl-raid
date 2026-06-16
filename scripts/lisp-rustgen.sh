#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"

dslraid_enter_repo

mode="${1:-check}"
input="${2:-examples/runscope/runscope.lisp.raid.json}"
out="${3:-generated/runtime_fsm.rs}"

generate() {
  cargo run -p dslraid-cli --quiet -- codegen "$input" --target rust
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated Rust backend is stale: run scripts/lisp-rustgen.sh generate" \
  "lisp rust backend ok" \
  "usage: scripts/lisp-rustgen.sh [generate|check] [input] [out]"
