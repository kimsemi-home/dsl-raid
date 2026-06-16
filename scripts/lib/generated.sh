#!/usr/bin/env bash

dslraid_repo_root() {
  git rev-parse --show-toplevel 2>/dev/null || pwd
}

dslraid_enter_repo() {
  repo="$(dslraid_repo_root)"
  cd "$repo"
}

dslraid_generated_case() {
  local mode="$1" out="$2" stale="$3" ok="$4" usage="$5"
  case "$mode" in
    generate)
      mkdir -p "$(dirname "$repo/$out")"
      generate > "$repo/$out"
      echo "generated $out"
      ;;
    check)
      dslraid_generated_check "$out" "$stale" "$ok"
      ;;
    *)
      echo "$usage" >&2
      exit 2
      ;;
  esac
}

dslraid_generated_check() {
  local out="$1" stale="$2" ok="$3" tmp
  tmp="$(mktemp)"
  generate > "$tmp"
  if ! diff -u "$repo/$out" "$tmp"; then
    echo "$stale" >&2
    rm -f "$tmp"
    exit 1
  fi
  rm -f "$tmp"
  echo "$ok"
}
