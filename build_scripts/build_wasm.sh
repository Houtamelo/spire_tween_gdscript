# If we are not in the repository root, go to the parent directory and check again
if [ ! -f "Cargo.toml" ]; then
    cd ..
    if [ ! -f "Cargo.toml" ]; then
        echo "Error: Could not find Cargo.toml, please run this script from the repository root."
        exit 1
    fi
fi

RUSTFLAGS="-C link-args=-pthread \
-C target-feature=+atomics \
-C link-args=-sSIDE_MODULE=2 \
-Zlink-native-libraries=no \
-Cllvm-args=-enable-emscripten-cxx-exceptions=0"\
  cargo build -p spire_tween --features include_gdext_lib, include_gdscript_bridge -Zbuild-std --target wasm32-unknown-emscripten \
  --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/debug"

mv "spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/debug/spire_tween.wasm"\
  "spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/debug/spire_tween.threads.wasm"


RUSTFLAGS="-C link-args=-pthread \
-C target-feature=+atomics \
-C link-args=-sSIDE_MODULE=2 \
-Zlink-native-libraries=no \
-Cllvm-args=-enable-emscripten-cxx-exceptions=0"\
  cargo build -p spire_tween --features include_gdext_lib,include_gdscript_bridge -Zbuild-std --target wasm32-unknown-emscripten \
  --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/release"

mv "spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/release/spire_tween.wasm"\
  "spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/release/spire_tween.threads.wasm"

RUSTFLAGS="-C link-args=-sSIDE_MODULE=2 \
-Zlink-native-libraries=no \
-Cllvm-args=-enable-emscripten-cxx-exceptions=0"\
  cargo build -p spire_tween --features include_gdext_lib,nothreads,include_gdscript_bridge -Zbuild-std --target wasm32-unknown-emscripten\
  --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/debug"

RUSTFLAGS="-C link-args=-sSIDE_MODULE=2 \
-Zlink-native-libraries=no \
-Cllvm-args=-enable-emscripten-cxx-exceptions=0"\
  cargo build -p spire_tween --features include_gdext_lib,nothreads -Zbuild-std --target wasm32-unknown-emscripten\
  --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/release"
