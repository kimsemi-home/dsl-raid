#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-.github/workflows/verification.yml}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-workflow-yaml))'
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated verification workflow is stale: run scripts/workflowgen.sh generate" \
  "verification workflow generated output ok" \
  "usage: scripts/workflowgen.sh [generate|check] [out]"
