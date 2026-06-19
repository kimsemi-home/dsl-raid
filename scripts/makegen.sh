#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-Makefile}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-makefile))'
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated Makefile is stale: run scripts/makegen.sh generate" \
  "Makefile generated output ok" \
  "usage: scripts/makegen.sh [generate|check] [out]"
