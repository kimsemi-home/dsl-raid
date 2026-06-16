#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"

dslraid_enter_repo

source "$repo/scripts/lib/lisp-runtime.sh"

mode="${1:-check}"
out="${2:-docs/generated/language-diagnostics.md}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid:emit-language-diagnostics-markdown))'
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated language diagnostics doc is stale: run scripts/langdiaggen.sh generate" \
  "language diagnostics generated doc ok" \
  "usage: scripts/langdiaggen.sh [generate|check] [out]"
