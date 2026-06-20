#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-runtime-trace.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-runtime-trace-json))' |
    python3 -m json.tool
}

validate_runtime_trace() {
  PYTHONDONTWRITEBYTECODE=1 python3 \
    "$repo/scripts/lib/runtime_trace_check.py" "$repo" \
    "$repo/$out" "$repo/docs/generated/verification-evidence.json"
}

check_runtime_commands() {
  while IFS=$'\t' read -r design trace coverage; do
    cargo run -p dslraid-cli -- trace check "$trace" --design-ir "$design" >/dev/null
    cargo run -p dslraid-cli -- coverage check "$coverage" --design-ir "$design" >/dev/null
  done
  echo "verification runtime trace check ok"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated runtime trace map is stale: run scripts/verificationruntimegen.sh generate" \
      "verification runtime trace generated output ok"
    validate_runtime_trace | check_runtime_commands ;;
  *) echo "usage: scripts/verificationruntimegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
