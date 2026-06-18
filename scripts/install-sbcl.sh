#!/usr/bin/env bash
set -euo pipefail

if command -v sbcl >/dev/null 2>&1; then
  sbcl --version
  exit 0
fi

export DEBIAN_FRONTEND=noninteractive

APT_OPTS=(
  -o Acquire::Retries=3
  -o Acquire::http::Timeout=60
  -o Acquire::https::Timeout=60
  -o Acquire::ForceIPv4=true
  -o DPkg::Lock::Timeout=30
  -o Dpkg::Use-Pty=0
)

with_timeout() {
  if command -v timeout >/dev/null 2>&1; then
    timeout --kill-after=15s "${SBCL_APT_TIMEOUT:-600s}" sudo -n "$@"
  else
    sudo -n "$@"
  fi
}

apt_with_retry() {
  local label="$1"
  shift
  local attempt
  for attempt in 1 2; do
    echo "${label} (attempt ${attempt}/2)"
    if with_timeout "$@"; then
      return 0
    fi
    sudo apt-get clean
    sleep "$((attempt * 5))"
  done
  return 1
}

install_sbcl() {
  apt_with_retry \
    "install sbcl" \
    apt-get "${APT_OPTS[@]}" install -y --no-install-recommends \
    sbcl
}

if ! install_sbcl; then
  apt_with_retry "apt update" apt-get "${APT_OPTS[@]}" update
  install_sbcl
fi

sbcl --version
