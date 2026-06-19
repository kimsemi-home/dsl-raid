#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-codegen.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-codegen-json))' |
    python3 -m json.tool
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated verification codegen map is stale: run scripts/verificationcodegengen.sh generate" \
  "verification codegen map generated output ok" \
  "usage: scripts/verificationcodegengen.sh [generate|check] [out]"
