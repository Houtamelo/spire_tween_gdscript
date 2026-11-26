# Build and verify a SpireTween release

The `Release candidate` GitHub Actions workflow builds Windows x86_64, Linux x86_64, and universal macOS libraries from one checkout. It tests both profiles against Godot 4.4, 4.5.2, 4.6.3, and 4.7.2, then assembles and tests the extracted desktop archive. All automation uses shell and the `spire_release` Rust tool; CI does not use Python.

## Run the workflow

Push or open a pull request to build and validate the desktop archive. The final `verified-desktop-package` artifact appears only after the build, full native/GDScript suites, deliberate failure controls, installation checks, and exported-game checks succeed. Intermediate artifacts are diagnostics, not release approvals. Nothing publishes to itch.io or GitHub Releases automatically.

Use **Actions → Release candidate → Run workflow** for optional Android builds. Android output is explicitly experimental and excluded from the desktop archive until device/emulator testing is implemented and passes.

macOS frameworks use free ad-hoc signatures in all archives. Building, testing and packaging require no Apple developer account, paid membership, certificates or repository secrets. The workflow does not submit software to Apple for notarization. Downloaded libraries may require local approval, quarantine handling or re-signing before Godot loads them; see the [godot-rust distribution guidance](https://godot-rust.github.io/book/toolchain/export-mac-and-ios.html). Downloaded-package behavior still requires verification on macOS.

## Build locally

The dated toolchain in `rust-toolchain.toml` and root `Cargo.lock` are the release snapshot. Development Cargo builds retain their existing default Godot API; the release tool explicitly selects `api-4-4`. Release builds use `target/release-pipeline`, keeping ordinary development binaries separate. Set `SPIRE_RELEASE_TARGET_DIR` to override that directory (relative paths resolve against the repository root). Windows CI uses the short `$RUNNER_TEMP/st` path to keep generated ThinLTO object paths below MSVC's legacy path limit; use a similarly short absolute directory for local Windows builds in a deep checkout.

```bash
# Install target standard libraries first when cross-compiling.
./build_scripts/build_linux.sh
./build_scripts/build_windows_64.sh             # Windows/MSVC host
XWIN_DIR=/path/to/xwin ./build_scripts/build_windows_64.sh  # Linux cross-build
./build_scripts/build_macos.sh                  # Mac with Xcode command line tools

# Optional Android native libraries; NDK and cargo-ndk versions are in config.json.
ANDROID_NDK_HOME=/path/to/ndk ./build_scripts/build_android.sh
```

Scripts stage only intended addon libraries, native test libraries, license/source notices, provenance, and logs in `dist/staging/<platform>`. Existing output directories are rejected to prevent mixing old/new artifacts. Set `SPIRE_RELEASE_OUT` to choose another empty directory.

macOS builds compile ARM64 and Intel in both profiles, merge each pair with `lipo`, package universal frameworks, and ad-hoc sign them. Their deployment target is macOS 11.0; the workflow's macOS 15 runners do not establish runtime compatibility with macOS 11.0. Linux runtime requirements must be measured from the actual linked artifacts, not inferred from the Rust target name.

Windows release builds statically link the Visual C++ runtime. Artifact verification reads the actual PE import tables and rejects a remaining `VCRUNTIME`/`MSVCP` DLL dependency. This avoids requiring a separate Visual C++ runtime installation for the addon.

## Verify a platform build

```bash
cargo run --locked -p spire_release -- download --version 4.4 --out dist/engine --download-templates
cargo run --locked -p spire_release -- test \
  --stage dist/staging/linux --godot "$(cat dist/engine/godot-path)" \
  --templates "$PWD/dist/engine/export/templates" --out dist/validation

SPIRE_RELEASE_STAGE="$PWD/dist/staging/linux" \
SPIRE_RELEASE_GODOT="$(cat dist/engine/godot-path)" \
  cargo test --locked -p spire_release --test pipeline -- --ignored
```

Godot/editor/template downloads are checked against committed upstream SHA-512 checksums. Update `config.json`, `godot-downloads.json`, and the archive-test version matrix together when updating the supported range.

Tests copy project source into isolated fixtures. Asset import runs in a separate project containing no extension or scripts; the runtime project receives asset caches, explicit resource paths, the script class list, and the extension list. This avoids the known cold headless GDExtension import crash and never relies on checked-in editor caches. Both aggregate suites use fixed 120 FPS simulation time. The native unscaled-time test runs separately with real pacing because it deliberately measures the wall clock; all 189 native cases remain required (188 simulation cases plus one real-time case). An empty-filter failure control checks that exclusion cannot falsely pass. Every child has a wall-clock deadline and must emit its expected completion marker as well as the expected exit code. Intentional failure controls must exit nonzero.

The Linux validation jobs also run the scene-template, Blueprint-preview, and rendered text/preset suites in isolated graphical editor sessions. Glyph observation uses the native test extension; those helpers are excluded from customer addon libraries. Historical aggregate shutdown warnings were repaired in the test suite; new shutdown diagnostics still require investigation.

Linux graphical validation installs Mesa's OpenGL and Vulkan drivers and pins
Lavapipe for editor-side Vulkan probes, including when the project renders through
OpenGL. An intentional missing-driver run must fail before the regular template
checks. Universal macOS exports enable both desktop and ASTC texture imports.
Framework metadata includes all bundle keys required by Godot, and export checks
reject any rewrite of that metadata, preserving the release version and minimum
macOS deployment target.

## Assemble the customer archive

```bash
cargo run --locked -p spire_release -- package \
  --staging dist/staging --reports dist/reports --out dist/archive
cargo run --locked -p spire_release -- test-archive \
  --archive dist/archive/SpireTween-VERSION-REVISION-desktop.zip \
  --godot "$(cat dist/engine/godot-path)" \
  --templates "$PWD/dist/engine/export/templates" --out dist/archive-validation
```

The package uses an allowlist: addon runtime binaries/manifest, the supplied GDScript scene templates with their shared scripts, UID sidecars, catalog and README, project license, changelog, README/release instructions, dependency notices, unmodified MPL dependency sources, provenance and SHA-256 manifests. Template files are staged and checksummed on each platform, verified against the source inventory, and included once in the assembled archive. It excludes example assets (including Girl Knight), native test libraries, caches, Rust intermediates, PDBs and import libraries. Debug runtime libraries remain included because the editor needs them.

For a local development candidate without the complete validation matrix, add `--candidate`. Pass `--platforms linux` or `--platforms linux,windows` together with `--candidate` for a partial archive; its filename and manifest identify only the included platforms. Commercial packaging always requires all three desktop platforms. Rust standard-library copyright/license notices are included alongside the Cargo dependency inventory.

Without `--candidate`, packaging additionally requires a clean common source revision and matching successful full-suite/export reports for every configured engine/platform/architecture. The package job runs a real-artifact integration test that accepts ad-hoc macOS frameworks and rejects missing Mac validation evidence. Run it locally with `SPIRE_RELEASE_STAGING` and `SPIRE_RELEASE_REPORTS` pointing to the complete build/report directories: `cargo test --locked -p spire_release --test desktop -- --ignored`.

Archive extraction verifies every internal checksum and rejects extra files, then installs and runs both library profiles and exported games. Each smoke run loads every scene in the installed template catalog, checks its script, plays it, and verifies restoration. The integration failure control removes a shared script while updating the checksums, and requires the runtime check to reject the incomplete addon. The workflow delivers the exact archive that passed those checks. Toolchain/dependency/source pinning records reproducible inputs; bit-identical compilation across different machines is not claimed.

Release archives and macOS framework metadata use the version in
`spire_tween/Cargo.toml`. See [the changelog](../CHANGELOG.md) for the next release
and migration notes.

The approved distribution model for 2.0.0 is one paid desktop addon covering
Windows, Linux, and macOS, including all 33 templates, plus a free runnable
showcase demo.

One purchase includes a lifetime license and all future SpireTween updates,
including major versions, at no additional charge.

Purchases also include a companion source archive matching the delivered addon's
source revision. It contains the Rust sources, generated bindings, GDScript
templates, documentation, and pinned build inputs needed to rebuild that version.
The public repository and existing MIT licensing remain in place. Third-party
example art, compiled libraries, and development caches are excluded from this
source download.

The storefront is itch.io, using the publisher's existing account. The publisher
will provide the final asset URL after publication; leave purchase links unset
until then.

The primary support channel for bug reports and usage questions is
[GitHub Issues](https://github.com/Houtamelo/spire_tween_gdscript/issues).
The itch.io community is a secondary channel checked from time to time.

The author checks GitHub daily and prioritizes bug fixes over all other
SpireTween work. Most issues are expected to be resolved within 72 hours, though
the author reserves the right to take up to 14 days.

The workflow assembles the addon and its companion source archive, compiles the
extracted source, and exports a standalone showcase for each desktop platform.
The final delivery includes these downloads only after the required checks pass.

## Source and showcase downloads

`package` also creates `SpireTween-VERSION-REVISION-source.zip`. The outer
`SHA256SUMS` covers both source and binary archives. The source ZIP has its own
checksum manifest and provenance; see [source build instructions](SOURCE.md).

```bash
SPIRE_RELEASE_SOURCE_ARCHIVE=/path/to/SpireTween-VERSION-REVISION-source.zip \
  cargo test --locked -p spire_release --test source -- --ignored

cargo run --locked -p spire_release -- demo \
  --stage dist/staging/linux --godot /path/to/godot \
  --templates /path/to/export/templates --out dist/showcase
SPIRE_RELEASE_DEMO_ARCHIVE=/path/to/SpireTween-VERSION-REVISION-demo-linux.zip \
  cargo test --locked -p spire_release --test demo -- --ignored
```

Source validation compares delivered files to the build's source inventory and
runs locked Cargo compilation from the extracted workspace. A failure control
removes a core source file while regenerating checksums. Showcase validation
exercises Play and Restore on all 33 gallery cards in the exported application;
its failure control removes the real PCK dependency with valid checksums.

Showcases use Godot 4.4 export templates and the same staged addon as the paid
download. Each platform exports on its native OS; the macOS application is
universal and is also exercised on Intel. Downloads contain runtime files,
licenses, dependency sources/notices, engine notices from the export engine, and
provenance. Editor caches and project sources remain outside the demo download.
See [showcase instructions](DEMO.md).
