#!/usr/bin/env bash
set -euo pipefail

repo="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$repo"
mode="${1:-check}"

case "$mode" in
  check|"") ;;
  *)
    echo "usage: scripts/privacycheck.sh [check]" >&2
    exit 2
    ;;
esac

roots=(
  docs examples schemas tests/golden .github/workflows scripts lisp crates
  apps/viewer/src apps/viewer/tests apps/viewer/public BUILD.bazel Makefile
  .gitlab-ci.yml Cargo.toml Cargo.lock README.md SECURITY.md CONTRIBUTING.md
  CODE_OF_CONDUCT.md
)

existing=()
for root in "${roots[@]}"; do
  if [[ -e "$root" ]]; then
    existing+=("$root")
  fi
done

if [[ "${#existing[@]}" -eq 0 ]]; then
  echo "privacy public-surface check ok"
  exit 0
fi

patterns=(
  '/Users/'
  'BEGIN (RSA|OPENSSH|PRIVATE) KEY'
  'AKIA[0-9A-Z]{16}'
  'ghp_[A-Za-z0-9_]{20,}'
  'xox[baprs]-'
)

args=()
for pattern in "${patterns[@]}"; do
  args+=("-e" "$pattern")
done

if rg -n --hidden \
  --glob '!target/**' \
  --glob '!apps/viewer/node_modules/**' \
  --glob '!apps/viewer/dist/**' \
  --glob '!.git/**' \
  --glob '!scripts/privacycheck.sh' \
  "${args[@]}" "${existing[@]}"; then
  echo "private data pattern found in public surface" >&2
  exit 1
fi

echo "privacy public-surface check ok"
