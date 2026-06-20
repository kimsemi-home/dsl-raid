#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-actions-receipt.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-actions-receipt-json))' |
    python3 -m json.tool
}

validate_receipts() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/actions_receipt_check.py" "$repo/$out" "$repo"
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated actions receipt is stale: run scripts/verificationreceiptgen.sh generate" \
      "verification actions receipt generated output ok"
    validate_receipts ;;
  *) echo "usage: scripts/verificationreceiptgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
