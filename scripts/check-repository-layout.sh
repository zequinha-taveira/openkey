#!/usr/bin/env bash
set -euo pipefail

root_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root_dir"

expected_dirs=(
  protocol/core
  protocol/protocols
  firmware/platform
  firmware/platform/mcu/rp23xx
  firmware/crypto
  firmware/storage
  firmware/usb
  firmware/config
  firmware/boot
  simulator
  boards
  host/sdk-python
  host/cli
  host/configurator
  host/provisioner
  host/updater
  host/diagnostics
  host/gui
  tests
  fuzz
  tools
  docs
)

for dir in "${expected_dirs[@]}"; do
  if [[ ! -d "$dir" ]]; then
    echo "layout error: missing expected directory: $dir" >&2
    exit 1
  fi
done

for forbidden in legacy openkey-rs crates; do
  if [[ -e "$forbidden" ]]; then
    echo "layout error: parallel or legacy tree is not allowed: $forbidden" >&2
    exit 1
  fi
done

# cargo metadata validates that every workspace member path exists and is parseable.
cargo metadata --no-deps --format-version 1 >/dev/null

# Guard against the most common stale path names in active top-level project docs.
if rg -n --glob '*.md' --glob '!docs/reference/adr/**' --glob '!docs/reference/version-history.md' \
  'firmware/protocols/|host/simulator|host/sdk/' \
  README.md SECURITY.md docs/reference docs/explanation docs/planning >/dev/null; then
  echo "layout error: stale canonical path found in active documentation" >&2
  exit 1
fi

echo "repository layout: OK"
