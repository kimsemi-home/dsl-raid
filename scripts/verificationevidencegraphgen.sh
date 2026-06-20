#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-evidence-graph.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-evidence-graph-json))' |
    python3 -m json.tool
}

validate_graph() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/evidence_graph_check.py" "$repo/$out" "$repo"
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated evidence graph is stale: run scripts/verificationevidencegraphgen.sh generate" \
      "verification evidence graph generated output ok"
    validate_graph ;;
  *) echo "usage: scripts/verificationevidencegraphgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
