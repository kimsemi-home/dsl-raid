#!/usr/bin/env bash

dslraid_lisp_eval() {
  local form="$1"
  local cache_dir
  cache_dir="$(mktemp -d)"
  set +e
  XDG_CACHE_HOME="$cache_dir" sbcl --noinform --non-interactive \
    --eval '(require :asdf)' \
    --eval '(asdf:load-asd (merge-pathnames "lisp/dslraid.asd" (uiop:getcwd)))' \
    --eval '(let ((*standard-output* (make-broadcast-stream))) (asdf:load-system :dslraid :force t))' \
    --eval "$form"
  local status="$?"
  set -e
  rm -rf "$cache_dir"
  return "$status"
}
