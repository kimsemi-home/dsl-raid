#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
limit="${SOURCE_LINE_LIMIT:-75}"
failed=0

check_file() {
  local file="$1"
  local lines
  lines="$(wc -l < "$file" | tr -d ' ')"
  if [ "$lines" -gt "$limit" ]; then
    printf 'line budget exceeded: %s has %s lines, limit is %s\n' "${file#$root/}" "$lines" "$limit" >&2
    failed=1
  fi
}

scan_root() {
  local path="$1"
  [ -e "$path" ] || return 0
  while IFS= read -r -d '' file; do
    check_file "$file"
  done < <(find "$path" \
    \( -path '*/node_modules/*' -o -path '*/dist/*' -o -path '*/target/*' \) -prune \
    -o -type f \( -name '*.rs' -o -name '*.ts' -o -name '*.tsx' -o -name '*.js' -o -name '*.jsx' -o -name '*.lisp' \) -print0)
}

scan_root "$root/crates"
scan_root "$root/apps/viewer/src"
scan_root "$root/lisp"

if [ "$failed" -ne 0 ]; then
  exit 1
fi

echo "source line budget ok: <= ${limit} lines"
