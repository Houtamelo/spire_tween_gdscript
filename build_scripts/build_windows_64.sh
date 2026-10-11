#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd -- "$repo_root"

# Native Windows builds use the active MSVC developer environment.
case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*) ;;
    *)
        : "${XWIN_DIR:?Set XWIN_DIR to the root of an xwin SDK installation.}"
        for directory in crt/include sdk/include/ucrt sdk/include/um sdk/include/shared crt/lib/x86_64 sdk/lib/um/x86_64 sdk/lib/ucrt/x86_64; do
            if [[ ! -d "$XWIN_DIR/$directory" ]]; then
                printf 'Missing xwin SDK directory: %s\n' "$XWIN_DIR/$directory" >&2
                exit 1
            fi
        done

        export CC_x86_64_pc_windows_msvc="${CC_x86_64_pc_windows_msvc:-clang-cl}"
        export CXX_x86_64_pc_windows_msvc="${CXX_x86_64_pc_windows_msvc:-clang-cl}"
        export AR_x86_64_pc_windows_msvc="${AR_x86_64_pc_windows_msvc:-llvm-lib}"
        export CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER="${CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER:-lld-link}"
        include_flags="-Wno-unused-command-line-argument /imsvc\"$XWIN_DIR/crt/include\" /imsvc\"$XWIN_DIR/sdk/include/ucrt\" /imsvc\"$XWIN_DIR/sdk/include/um\" /imsvc\"$XWIN_DIR/sdk/include/shared\""
        export CC_SHELL_ESCAPED_FLAGS=1
        export CFLAGS_x86_64_pc_windows_msvc="${CFLAGS_x86_64_pc_windows_msvc:-} $include_flags"
        export CXXFLAGS_x86_64_pc_windows_msvc="${CXXFLAGS_x86_64_pc_windows_msvc:-} $include_flags"

        # Encoded rustflags keep SDK paths containing spaces as single arguments.
        encoded_flags="${CARGO_ENCODED_RUSTFLAGS:-}"
        if [[ -z "$encoded_flags" && -n "${RUSTFLAGS:-}" ]]; then
            flag_text="${RUSTFLAGS//$'\n'/ }"
            read -r -a existing_flags <<< "$flag_text"
            printf -v encoded_flags '%s\037' "${existing_flags[@]}"
            encoded_flags="${encoded_flags%$'\037'}"
        fi
        sdk_flags="-Lnative=$XWIN_DIR/crt/lib/x86_64"$'\037'"-Lnative=$XWIN_DIR/sdk/lib/um/x86_64"$'\037'"-Lnative=$XWIN_DIR/sdk/lib/ucrt/x86_64"
        export CARGO_ENCODED_RUSTFLAGS="${encoded_flags:+$encoded_flags$'\037'}$sdk_flags"
        ;;
esac

cargo build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --target=x86_64-pc-windows-msvc --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-pc-windows-msvc/debug"
cargo build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --target=x86_64-pc-windows-msvc --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-pc-windows-msvc/release"
