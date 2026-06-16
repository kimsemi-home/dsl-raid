#!/usr/bin/env bash
set -euo pipefail

repo="$(git rev-parse --show-toplevel)"
cd "$repo"

chmod +x .githooks/pre-commit scripts/*.sh scripts/lib/*.sh
git config core.hooksPath .githooks

echo "git hooks installed: core.hooksPath=.githooks"
