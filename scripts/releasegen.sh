#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-.github/workflows/release.yml}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-release-yaml))'
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated release workflow is stale: run scripts/releasegen.sh generate" \
  "release workflow generated output ok" \
  "usage: scripts/releasegen.sh [generate|check] [out]"
