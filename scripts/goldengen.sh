#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-.github/workflows/golden.yml}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-golden-yaml))'
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated golden workflow is stale: run scripts/goldengen.sh generate" \
  "golden workflow generated output ok" \
  "usage: scripts/goldengen.sh [generate|check] [out]"
