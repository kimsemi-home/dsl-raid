#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-precommit-closure.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-precommit-json))' |
    python3 -m json.tool
}

validate_precommit() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/precommit_check.py" "$repo/$out" "$repo"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification precommit closure is stale: run scripts/verificationprecommitgen.sh generate" \
      "verification precommit closure generated output ok"
    validate_precommit ;;
  *) echo "usage: scripts/verificationprecommitgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
