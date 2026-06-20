#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out_dir="${2:-scripts/releasecheck}"

group_ids() {
  dslraid_lisp_eval \
    '(dolist (id (dslraid.agent::verification-release-check-group-ids)) (format t "~A~%" id))'
}

generate_one() {
  local id="$1"
  dslraid_lisp_eval \
    "(write-string (dslraid.agent::emit-verification-release-check-script \"$id\"))"
}

generate_all() {
  mkdir -p "$repo/$out_dir"
  while IFS= read -r id; do
    [ -n "$id" ] || continue
    generate_one "$id" > "$repo/$out_dir/$id.sh"
    chmod +x "$repo/$out_dir/$id.sh"
    echo "generated $out_dir/$id.sh"
  done < <(group_ids)
}

check_all() {
  local failed=0 tmp
  while IFS= read -r id; do
    [ -n "$id" ] || continue
    tmp="$(mktemp)"
    generate_one "$id" > "$tmp"
    if ! diff -u "$repo/$out_dir/$id.sh" "$tmp"; then
      echo "generated release-check provider is stale: $out_dir/$id.sh" >&2
      failed=1
    fi
    rm -f "$tmp"
  done < <(group_ids)
  [ "$failed" -eq 0 ] || exit 1
  echo "release-check provider scripts generated output ok"
}

case "$mode" in
  generate) generate_all ;;
  check) check_all ;;
  *) echo "usage: scripts/releasecheckgen.sh [generate|check] [out-dir]" >&2; exit 2 ;;
esac
