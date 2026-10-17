#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd -- "$repo_root"
if [[ "$(uname -s)" != Darwin ]]; then
    printf 'macOS builds require a macOS host with Xcode command line tools.\n' >&2
    exit 1
fi
cargo run --locked -p spire_release -- build --platform macos --out "${SPIRE_RELEASE_OUT:-dist/staging/macos}"
