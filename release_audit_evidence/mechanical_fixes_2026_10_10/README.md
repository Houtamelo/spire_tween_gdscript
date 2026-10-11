# Mechanical release-audit fixes — 2026-10-10

## Changes

- All four build scripts now have Bash shebangs, executable permissions, strict
  error handling, and repository discovery based on the script location.
- Android preserves a supplied version-specific `ANDROID_NDK_HOME` and validates
  the toolchain directory. Linux-to-Windows builds use `XWIN_DIR`, LLVM tools,
  and SDK flags with encoded Rust paths; native Windows uses its MSVC environment.
- Web commands have intact feature lists, real release profiles, the GDScript
  bridge in both nonthreaded profiles, and an actionable missing-Emscripten error.
- GDScript `--all-tests --no-cache` selects every scene and bypasses cache reads
  and writes. The editor's existing run-mode default is preserved.
- The missing ellipsis scene is included; its standalone demo is retained without
  starting a competing test inside the aggregate runner. Its TestBase priorities
  now initialize properly.
- Cache hashing frees its temporary node. The delayed-call validity suite now
  implements the aggregate runner interface instead of failing method lookup.
- Exact copies of the project MIT license are present in both addon directories.
- README installation/build commands reflect GDExtension loading and explicit
  addon features. SDK settings are documented; benchmarks are historical Godot
  4.5.1 measurements; current Linux libraries' measured GLIBC minimum is 2.34.

## Verification

- Actual Linux script builds debug and release libraries successfully.
- Actual Linux-to-Windows script builds debug and release DLLs using the installed
  xwin SDK and LLVM. This is compilation evidence, not Windows runtime testing.
- Eight persistent Python checks use real Bash/Cargo startup and failure paths,
  validating missing/invalid SDKs, missing Emscripten, fail-fast behavior,
  preservation of existing artifacts after compiler failure, and license copies.
- Eleven real-Godot CLI/cache/discovery checks pass for four flag combinations
  in each of debug and release: 88 checks on Godot 4.7.2, with no script errors.
- The README delayed-call installation example prints its expected message.
- The uncached aggregate run starts and finishes all 97 intended tests. All five
  ellipsis tests run exactly once; the cache bytes remain unchanged. The existing
  invalid-enum null-call error remains, alongside deliberately rejected invalid
  tweens/sequence additions. Exit zero is still not a full-suite success claim.
- The standalone ellipsis demo still starts and finishes exactly once.

The initial aggregate attempt used a frame cap incompatible with its real-clock
manual-stepping test; the completed run uses an external timeout instead.

## Concurrent-source limitation

The Android arm64 debug build initially succeeded. Its release build then hit
an in-progress sequence RustFn trait signature error. That signature was fixed
in the user's working tree; a retry hit new non-exhaustive `Ctor` matches in
`BlockItem` and `InsertItem` (sequence/data.rs). Those ongoing source edits were
preserved. A complete current-source Android debug/release matrix is therefore
not verified by this task. The SDK/fail-fast behavior is tested separately.

No Emscripten compiler is installed, so Web success builds were not run. No
Windows/Android runtime, graphical editor session, or export was attempted.
API selection, platform promises, artifact contents, third-party artwork,
dependency notices, toolchain/lockfile pinning, and full runner reliability remain
separate release work.

## Commands

```sh
./build_scripts/build_linux.sh
XWIN_DIR=/path/to/xwin ./build_scripts/build_windows_64.sh
ANDROID_NDK_HOME=/path/to/ndk/version ./build_scripts/build_android.sh
python3 -m unittest discover -s build_scripts/tests -v
godot --headless --path spire_tween_gdscript --script res://examples/tests/runner_cli_checks.gd -- --all-tests --no-cache
godot --headless --path spire_tween_gdscript res://examples/tests/test_runner.tscn -- --all-tests --no-cache
```

Engine fixtures and XDG directories were isolated under
`/tmp/spire-mechanical-fixes-6oelx20r`. Existing installed Linux artifacts were
exercised through symlinks in the fixture; source/library hashes accompany logs.
