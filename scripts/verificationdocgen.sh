#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-graph.md}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-graph-markdown))'
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated verification graph doc is stale: run scripts/verificationdocgen.sh generate" \
  "verification graph generated doc ok" \
  "usage: scripts/verificationdocgen.sh [generate|check] [out]"
