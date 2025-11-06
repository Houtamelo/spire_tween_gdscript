cd ..

export CLANG_PATH="/home/houtamelo/bin/android-sdk/ndk/28.1.13356709/toolchains/llvm/prebuilt/linux-x86_64/bin/clang"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="/home/houtamelo/bin/android-sdk/ndk/28.1.13356709/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android35-clang"
export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="/home/houtamelo/bin/android-sdk/ndk/28.1.13356709/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android35-clang"
export CARGO_TARGET_I686_LINUX_ANDROID_LINKER="/home/houtamelo/bin/android-sdk/ndk/28.1.13356709/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android35-clang"

cargo build --target=aarch64-linux-android --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/aarch64-linux-android/debug"
cargo build --target=aarch64-linux-android --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/aarch64-linux-android/release"
cargo build --target=x86_64-linux-android --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-linux-android/debug"
cargo build --target=x86_64-linux-android --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-linux-android/release"
cargo build --target=i686-linux-android --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/i686-linux-android/debug"
cargo build --target=i686-linux-android --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/i686-linux-android/release"

#Currently doesn't work
#export CARGO_TARGET_ARMV7_LINUX_ANDROID_LINKER="/home/houtamelo/bin/android-sdk/ndk/28.1.13356709/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi35-clang"
#cargo build --target=armv7-linux-androideabi
#cargo build --target=armv7-linux-androideabi --release
