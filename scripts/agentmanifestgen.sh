#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"

dslraid_enter_repo

mode="${1:-check}"
input="${2:-examples/runscope/runscope.agent-run.json}"
out="${3:-docs/generated/agent-run-manifest.md}"

generate() {
  python3 "$script_dir/lib/agent_manifest_doc.py" "$repo/$input"
}

dslraid_generated_case \
  "$mode" "$out" \
  "generated agent run manifest is stale: run scripts/agentmanifestgen.sh generate" \
  "agent run manifest generated doc ok" \
  "usage: scripts/agentmanifestgen.sh [generate|check] [input] [out]"
