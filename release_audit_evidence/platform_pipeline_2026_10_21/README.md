# Native CI build failure investigation, 2026-10-21

The first [native run](https://github.com/Houtamelo/spire_tween_gdscript/actions/runs/39464267352), at `737db323497b0fdbf9568a94b3c9a542f4597d25`, passed the plan and Linux build jobs. macOS and Windows failed during the build jobs, so native runtime validation, packaging, extracted-archive validation and delivery did not run.

## macOS

All eight Rust compilations completed (addon/native harness, debug/release, ARM64/Intel). Universal-binary verification then failed:

```text
lipo -verify_arch arm64 x86_64 /path/to/spire_tween.framework/spire_tween
unknown architecture specification flag: /path/to/spire_tween.framework/spire_tween
```

[Apple's parser](https://github.com/apple-oss-distributions/cctools/blob/main/misc/lipo.c) consumes every argument following `-verify_arch` as an architecture. Commit `55173dc` moves the input path before the option. A native regression compiles real ARM64/Intel Mach-O objects, combines them, accepts the universal object, and rejects each thin object. It runs on the Mac build runner before the addon builds. A local LLVM-lipo probe passes equivalent positive/negative checks; Apple's own native test remains the authoritative check for the original parser failure.

## Windows

The addon built in both profiles and the native harness built in debug. The release harness failed with the actual Visual Studio 2022 MSVC linker:

```text
LINK : fatal error LNK1181: cannot open input file '.obj'
```

The compiler failure message elides 139 object-file arguments. The bare `.obj` diagnostic does not identify the missing or misparsed argument. Path-length, response-file parsing and empty-input hypotheses are not confirmed by this log. [Rust issue 157457](https://github.com/rust-lang/rust/issues/157457) was ruled out as evidence: its reporter traced that issue to a custom linker wrapper, which this pipeline does not use. Previous local Windows cross-builds used LLD, so they do not certify the failing native MSVC invocation.

Commit `55173dc` enables `rustc_codegen_ssa::back::link=info` on Windows and preserves failed Cargo build logs as workflow artifacts. It does not change the Windows linker or compilation flags. The [diagnostic run](https://github.com/Houtamelo/spire_tween_gdscript/actions/runs/39565814673) uses these changes.

The diagnostic run passed the full macOS build. Apple's regression passed in 0.88 seconds, including both thin-object rejection controls; the universal frameworks were built and staged successfully. Linux also passed. Windows reproduced the release-harness link failure.

The full failing command has 165 arguments and **zero empty arguments**. Its three longest object paths are 272, 270 and 264 characters including the four-character extended-path prefix (268, 266 and 260 without it). The successful link invocations stay below this boundary. This supports the path-length hypothesis; it does not yet establish MSVC's internal reason for printing `.obj`.

Commit `3dbda53` changes only the Windows Cargo output root to `$RUNNER_TEMP/st`, through `SPIRE_RELEASE_TARGET_DIR`. The [verification run](https://github.com/Houtamelo/spire_tween_gdscript/actions/runs/39569544819) passes the native Windows build in all four addon/harness debug/release combinations. The failing release-harness link now succeeds with the same argument count (165), zero empty arguments, response-file mode, Rust toolchain and Visual Studio MSVC `14.44.35207` linker. Its longest object path is 211 characters, instead of 272 including the extended-path prefix (268 without it). Static CRT and ThinLTO settings are unchanged. This verifies the shorter output root as the repair for this build; MSVC's internal choice to print `.obj` is not explained by these observations.

The downloaded Windows artifacts also pass the real verification/tampering integration test on Linux, including PE import inspection and deliberate external-VCRUNTIME, corrupt-file, missing-file and unexpected-file controls. This is artifact validation, not Windows runtime validation. Native Godot suites, exports and the final desktop archive remain separate workflow stages.

Local checks for `55173dc`: four CLI subprocess/filesystem integration tests, scoped rustfmt, actionlint 1.7.12 and `git diff --check` pass. The Apple regression is recorded above as a native CI result, not a local pass.

Local checks for the output-directory override: five CLI subprocess/filesystem integration tests pass, including rejection of an empty override before creating staging output. An actual Linux build with a relative override passes all three artifact/install/archive integration tests against Godot 4.4, including missing-library, tampering and unexpected-file failure controls. Scoped rustfmt, actionlint and diff checks pass.

Preserved logs: [macOS native build](macos-build.log), [failing Windows release harness](windows-native-release-failure.log), [successful Windows release harness](windows-native-release-success.log), and [successful Windows build provenance](windows-short-root-build.json). ANSI terminal escapes and trailing whitespace were removed from the logs; paths, compiler/linker arguments and result messages are preserved.
