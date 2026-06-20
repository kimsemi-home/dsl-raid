#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-release-provenance.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-release-provenance-json))' |
    python3 -m json.tool
}

validate_release_provenance() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/release_provenance_check.py" \
    "$repo/$out" \
    "$repo/docs/generated/verification-github-actions.json"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated release provenance is stale: run scripts/verificationreleaseprovenancegen.sh generate" \
      "verification release provenance generated output ok"
    validate_release_provenance ;;
  *) echo "usage: scripts/verificationreleaseprovenancegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
