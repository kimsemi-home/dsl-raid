#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"

dslraid_enter_repo

source "$repo/scripts/lib/lisp-runtime.sh"

mode="${1:-check}"
out="${2:-docs/generated/agent-cluster-principles.md}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-agent-cluster-principles-markdown))'
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated agent principles doc is stale: run scripts/agentprinciplegen.sh generate" \
  "agent principles generated doc ok" \
  "usage: scripts/agentprinciplegen.sh [generate|check] [out]"
