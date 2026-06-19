#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-BUILD.bazel}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-bazel))'
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated Bazel file is stale: run scripts/bazelgen.sh generate" \
  "Bazel generated output ok" \
  "usage: scripts/bazelgen.sh [generate|check] [out]"
