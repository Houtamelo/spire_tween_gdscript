#!/usr/bin/env bash
cd ..

export CC_i686_pc_windows_msvc="clang-cl"
export CXX_i686_pc_windows_msvc="clang-cl"
export AR_i686_pc_windows_msvc="llvm-lib"
export CL_FLAGS="-Wno-unused-command-line-argument -fuse-ld=lld-link /imsvc/home/houtamelo/xwin/crt/include /imsvc/home/houtamelo/xwin/sdk/include/ucrt /imsvc/home/houtamelo/xwin/sdk/include/um /imsvc/home/houtamelo/xwin/sdk/include/shared"
export RUSTFLAGS="-Lnative=/home/houtamelo/xwin/crt/lib/x86 -Lnative=/home/houtamelo/xwin/sdk/lib/um/x86 -Lnative=/home/houtamelo/xwin/sdk/lib/ucrt/x86"
export CFLAGS_i686_pc_windows_msvc="$CL_FLAGS"
export CXXFLAGS_i686_pc_windows_msvc="$CL_FLAGS"

cargo build --target=i686-pc-windows-msvc --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/i686-pc-windows-msvc/debug"
cargo build --target=i686-pc-windows-msvc --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/i686-pc-windows-msvc/release"