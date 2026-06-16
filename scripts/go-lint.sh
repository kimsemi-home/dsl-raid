#!/usr/bin/env bash
set -euo pipefail

version="${GOLANGCI_LINT_VERSION:-v2.12.2}"
root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
config="$root/.golangci.yml"

has_go_file() {
  find "$root" \
    \( -path "$root/.git" -o -path "$root/target" -o -path "$root/apps/viewer/node_modules" \) -prune \
    -o -name '*.go' -print -quit | grep -q .
}

discover_modules() {
  find "$root" \
    \( -path "$root/.git" -o -path "$root/target" -o -path "$root/apps/viewer/node_modules" \) -prune \
    -o -name go.mod -print | sort
}

run_lint() {
  local module_dir="$1"
  echo "golangci-lint $version: $module_dir"
  (
    cd "$module_dir"
    go run "github.com/golangci/golangci-lint/v2/cmd/golangci-lint@$version" \
      run --config "$config" ./...
  )
}

main() {
  if ! has_go_file; then
    echo "golangci-lint: no Go files; skipping."
    return 0
  fi

  modules=""
  while IFS= read -r module; do
    modules="${modules}${module}
"
  done < <(discover_modules)
  if [ -z "$modules" ]; then
    echo "golangci-lint: Go files found, but no go.mod exists." >&2
    return 1
  fi

  printf "%s" "$modules" | while IFS= read -r module; do
    [ -n "$module" ] || continue
    run_lint "$(dirname "$module")"
  done
}

main "$@"
