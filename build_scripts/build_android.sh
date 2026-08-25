# If we are not in the repository root, go to the parent directory and check again
if [ ! -f "Cargo.toml" ]; then
    cd ..
    if [ ! -f "Cargo.toml" ]; then
        echo "Error: Could not find Cargo.toml, please run this script from the repository root."
        exit 1
    fi
fi

export ANDROID_NDK_HOME="/home/houtamelo/bin/android-sdk/ndk"

# Most common target.
cargo ndk -t aarch64-linux-android build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/aarch64-linux-android/debug"
cargo ndk -t aarch64-linux-android build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/aarch64-linux-android/release"

cargo ndk -t x86_64-linux-android build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-linux-android/debug"
cargo ndk -t x86_64-linux-android build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-linux-android/release"

#Currently doesn't work
# cargo ndk -t i686-linux-android build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/i686-linux-android/debug"
# cargo ndk -t i686-linux-android build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/i686-linux-android/release"
# cargo ndk -t armv7-linux-androideabi build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/armv7-linux-androideabi/debug"
# cargo ndk -t armv7-linux-androideabi build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/armv7-linux-androideabi/release"
