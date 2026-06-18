#!/usr/bin/env bash
set -euo pipefail

if command -v sbcl >/dev/null 2>&1; then
  sbcl --version
  exit 0
fi

export DEBIAN_FRONTEND=noninteractive

apt_with_retry() {
  local label="$1"
  shift
  local attempt
  for attempt in 1 2 3; do
    echo "${label} (attempt ${attempt}/3)"
    if timeout 420 sudo "$@"; then
      return 0
    fi
    sudo apt-get clean
    sleep "$((attempt * 5))"
  done
  return 1
}

apt_with_retry \
  "apt update" \
  apt-get update \
  -o Acquire::Retries=5 \
  -o Acquire::http::Timeout=30 \
  -o Acquire::https::Timeout=30

apt_with_retry \
  "install sbcl" \
  apt-get install -y --no-install-recommends \
  -o Acquire::Retries=5 \
  -o Acquire::http::Timeout=30 \
  -o Acquire::https::Timeout=30 \
  sbcl

sbcl --version
