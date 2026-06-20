#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-.github/workflows/pages.yml}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-pages-yaml))'
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated Pages workflow is stale: run scripts/pagesworkflowgen.sh generate" \
  "Pages workflow generated output ok" \
  "usage: scripts/pagesworkflowgen.sh [generate|check] [out]"
