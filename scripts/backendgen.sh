#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/backend-targets.md}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.emit::emit-backend-targets-markdown))'
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated backend target doc is stale: run scripts/backendgen.sh generate" \
  "backend target generated doc ok" \
  "usage: scripts/backendgen.sh [generate|check] [out]"
