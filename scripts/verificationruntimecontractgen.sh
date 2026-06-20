#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-runtime-contract.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-runtime-contract-json))' |
    python3 -m json.tool
}

validate_runtime_contract() {
  python3 "$repo/scripts/lib/runtime_contract_check.py" "$repo/$out"
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated runtime contract is stale: run scripts/verificationruntimecontractgen.sh generate" \
      "verification runtime contract generated output ok"
    validate_runtime_contract ;;
  *) echo "usage: scripts/verificationruntimecontractgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
