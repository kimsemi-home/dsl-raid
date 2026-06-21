#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-sidecar.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-sidecar-json))' |
    python3 -m json.tool
}

validate_sidecar() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/verification_sidecar_check.py" \
    "$repo/$out" "$repo/docs/generated/verification-evidence.json"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification sidecar is stale: run scripts/verificationsidecargen.sh generate" \
      "verification sidecar generated output ok"
    validate_sidecar ;;
  *) echo "usage: scripts/verificationsidecargen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
