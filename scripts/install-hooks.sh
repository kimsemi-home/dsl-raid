#!/usr/bin/env bash
set -euo pipefail

repo="$(git rev-parse --show-toplevel)"
cd "$repo"

chmod +x .githooks/pre-commit scripts/go-lint.sh scripts/check-source-lines.sh
chmod +x scripts/lisp-docgen.sh
git config core.hooksPath .githooks

echo "git hooks installed: core.hooksPath=.githooks"
