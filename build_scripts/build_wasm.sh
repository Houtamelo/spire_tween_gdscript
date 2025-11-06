RUSTFLAGS="-C link-args=-pthread \
-C target-feature=+atomics \
-C link-args=-sSIDE_MODULE=2 \
-Zlink-native-libraries=no \
-Cllvm-args=-enable-emscripten-cxx-exceptions=0"\
  cargo build -Zbuild-std --target wasm32-unknown-emscripten \
  --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/debug"

mv "spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/debug/spire_tween.wasm"\
  "spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/debug/spire_tween.threads.wasm"
  

RUSTFLAGS="-C link-args=-pthread \
-C target-feature=+atomics \
-C link-args=-sSIDE_MODULE=2 \
-Zlink-native-libraries=no \
-Cllvm-args=-enable-emscripten-cxx-exceptions=0"\
  cargo build -Zbuild-std --target wasm32-unknown-emscripten \
  --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/release"

mv "spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/release/spire_tween.wasm"\
  "spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/release/spire_tween.threads.wasm"

RUSTFLAGS="-C link-args=-sSIDE_MODULE=2 \
-Zlink-native-libraries=no \
-Cllvm-args=-enable-emscripten-cxx-exceptions=0"\
  cargo build --features nothreads -Zbuild-std --target wasm32-unknown-emscripten\
  --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/debug"

RUSTFLAGS="-C link-args=-sSIDE_MODULE=2 \
-Zlink-native-libraries=no \
-Cllvm-args=-enable-emscripten-cxx-exceptions=0"\
  cargo build --features nothreads -Zbuild-std --target wasm32-unknown-emscripten\
  --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/wasm32-unknown-emscripten/release"
