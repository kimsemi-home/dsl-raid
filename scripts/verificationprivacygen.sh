#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-privacy.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-privacy-json))' |
    python3 -m json.tool
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated verification privacy manifest is stale: run scripts/verificationprivacygen.sh generate" \
  "verification privacy generated output ok" \
  "usage: scripts/verificationprivacygen.sh [generate|check] [out]"
