#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-.gitlab-ci.yml}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-gitlab-yaml))'
}

dslraid_generated_case \
  "$mode" \
  "$out" \
  "generated GitLab CI is stale: run scripts/gitlabgen.sh generate" \
  "GitLab CI generated output ok" \
  "usage: scripts/gitlabgen.sh [generate|check] [out]"
