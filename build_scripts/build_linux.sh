cd ..

cargo build --target=x86_64-unknown-linux-gnu --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-unknown-linux-gnu/debug"
cargo build --target=x86_64-unknown-linux-gnu --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-unknown-linux-gnu/release"