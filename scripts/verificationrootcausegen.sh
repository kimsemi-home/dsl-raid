#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-root-cause.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-root-cause-json))' |
    python3 -m json.tool
}

validate_root_cause() {
  PYTHONDONTWRITEBYTECODE=1 python3 "$repo/scripts/lib/root_cause_check.py" \
    "$repo/$out" "$repo/docs/generated/verification-evidence.json" "$repo"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification root cause is stale: run scripts/verificationrootcausegen.sh generate" \
      "verification root cause generated output ok"
    validate_root_cause ;;
  *) echo "usage: scripts/verificationrootcausegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
