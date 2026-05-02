# If we are not in the repository root, go to the parent directory and check again
if [ ! -f "Cargo.toml" ]; then
    cd ..
    if [ ! -f "Cargo.toml" ]; then
        echo "Error: Could not find Cargo.toml, please run this script from the repository root."
        exit 1
    fi
fi

cargo build -p spire_tween --features standalone --target=x86_64-unknown-linux-gnu --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-unknown-linux-gnu/debug"
cargo build -p spire_tween --features standalone --target=x86_64-unknown-linux-gnu --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-unknown-linux-gnu/release"