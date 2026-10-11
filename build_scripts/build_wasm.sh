#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd -- "$repo_root"

if ! command -v emcc >/dev/null 2>&1; then
    printf 'Emscripten is required; activate its SDK so emcc is on PATH.\n' >&2
    exit 1
fi

threaded_flags="-C link-args=-pthread -C target-feature=+atomics -C link-args=-sSIDE_MODULE=2 -Zlink-native-libraries=no -Cllvm-args=-enable-emscripten-cxx-exceptions=0"
nonthreaded_flags="-C link-args=-sSIDE_MODULE=2 -Zlink-native-libraries=no -Cllvm-args=-enable-emscripten-cxx-exceptions=0"

for profile in debug release; do
    profile_flags=()
    if [[ "$profile" == release ]]; then
        profile_flags+=(--release)
    fi
    output="spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/$profile"

    RUSTFLAGS="$threaded_flags" cargo build -p spire_tween --features include_gdext_lib,include_gdscript_bridge -Zbuild-std --target wasm32-unknown-emscripten "${profile_flags[@]}" --artifact-dir="$output"
    mv -f -- "$output/spire_tween.wasm" "$output/spire_tween.threads.wasm"

    RUSTFLAGS="$nonthreaded_flags" cargo build -p spire_tween --features include_gdext_lib,nothreads,include_gdscript_bridge -Zbuild-std --target wasm32-unknown-emscripten "${profile_flags[@]}" --artifact-dir="$output"
done
