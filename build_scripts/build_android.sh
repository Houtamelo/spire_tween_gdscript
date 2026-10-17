#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd -- "$repo_root"

: "${ANDROID_NDK_HOME:?Set ANDROID_NDK_HOME to the installed NDK version directory.}"
if [[ ! -d "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt" ]]; then
    printf 'ANDROID_NDK_HOME does not contain an NDK toolchain: %s\n' "$ANDROID_NDK_HOME" >&2
    exit 1
fi

cargo run --locked -p spire_release -- build --platform android --out "${SPIRE_RELEASE_OUT:-dist/staging/android}"

# i686 and armv7 builds remain unverified and are not built by this script.
