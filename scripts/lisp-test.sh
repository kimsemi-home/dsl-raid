#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"

dslraid_enter_repo

source "$repo/scripts/lib/lisp-runtime.sh"

mode="${1:-check}"

run_tests() {
  dslraid_lisp_eval \
    '(progn
       (load "lisp/tests/golden.lisp")
       (dslraid::run-golden-smoke)
       (write-line "lisp language tests ok"))'
}

case "$mode" in
  check)
    run_tests
    ;;
  *)
    echo "usage: scripts/lisp-test.sh [check]" >&2
    exit 2
    ;;
esac
