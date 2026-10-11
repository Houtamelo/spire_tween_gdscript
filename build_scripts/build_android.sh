#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd -- "$repo_root"

: "${ANDROID_NDK_HOME:?Set ANDROID_NDK_HOME to the installed NDK version directory.}"
if [[ ! -d "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt" ]]; then
    printf 'ANDROID_NDK_HOME does not contain an NDK toolchain: %s\n' "$ANDROID_NDK_HOME" >&2
    exit 1
fi

for target in aarch64-linux-android x86_64-linux-android; do
    cargo ndk -t "$target" build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/$target/debug"
    cargo ndk -t "$target" build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/$target/release"
done

# i686 and armv7 builds remain unverified and are not built by this script.
