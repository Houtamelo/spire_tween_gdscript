# Build and verify a SpireTween release

The `Release candidate` GitHub Actions workflow builds Windows x86_64, Linux x86_64, and universal macOS libraries from one checkout. It tests both profiles against Godot 4.4, 4.5.2, 4.6.3, and 4.7.2, then assembles and tests the extracted desktop archive. All automation uses shell and the `spire_release` Rust tool; CI does not use Python.

## Run the workflow

Push or open a pull request to run an unsigned candidate. The final `verified-desktop-package` artifact appears only after the build, full native/GDScript suites, deliberate failure controls, installation checks, and exported-game checks succeed. Intermediate artifacts are diagnostics, not release approvals. Nothing publishes to itch.io or GitHub Releases automatically.

Use **Actions → Release candidate → Run workflow** for a signed desktop archive or optional Android builds. Android output is explicitly experimental and excluded from the desktop archive until device/emulator testing is implemented and passes.

Unsigned macOS candidates use ad-hoc signatures. They are suitable for development/CI, and do not establish the normal downloaded-customer experience. A commercial desktop archive requires Developer ID signing and accepted notarization of both frameworks.

## Configure Apple signing once

Repository secrets:

| Secret | Value |
|---|---|
| `APPLE_CERT_BASE64` | Base64-encoded Developer ID Application PKCS12 certificate and private key |
| `APPLE_CERT_PASSWORD` | PKCS12 password |
| `APPLE_SIGNING_IDENTITY` | Full Developer ID Application identity |
| `APPLE_ID` | Apple ID used for notarization |
| `APPLE_TEAM_ID` | Developer team ID |
| `APPLE_APP_PASSWORD` | App-specific password for notarization |

Apple Developer Program enrollment and obtaining the certificate remain account-owner actions. The signing script uses a temporary keychain, checks each signature and accepted notarization response, and removes the certificate/keychain afterward. It fails if a required secret is absent. Frameworks cannot be stapled like app/disk-image/installer bundles; their signed-code notarization tickets are retrieved online. Customer download/quarantine behavior still requires verification on macOS with signed artifacts.

## Build locally

The dated toolchain in `rust-toolchain.toml` and root `Cargo.lock` are the release snapshot. Development Cargo builds retain their existing default Godot API; the release tool explicitly selects `api-4-4`. Release builds use `target/release-pipeline`, keeping ordinary development binaries separate.

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

macOS builds compile ARM64 and Intel in both profiles, merge each pair with `lipo`, package universal frameworks, and ad-hoc sign them. Their deployment target is macOS 11.0; the workflow's macOS 14/15 runners do not establish runtime compatibility with macOS 11.0. Linux runtime requirements must be measured from the actual linked artifacts, not inferred from the Rust target name.

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

Tests copy project source into isolated fixtures. Asset import runs in a separate project containing no extension or scripts; the runtime project receives asset caches, explicit resource paths, the script class list, and the extension list. This avoids the known cold headless GDExtension import crash and never relies on checked-in editor caches. Native aggregate tests use real time; GDScript aggregate tests use fixed 120 FPS. Every child has a wall-clock deadline and must emit its expected completion marker as well as the expected exit code. Intentional failure controls must exit nonzero.

The aggregate GDScript suite previously reported shutdown resource warnings despite passing functional assertions. Those warnings remain a separate audit item; successful CI does not automatically resolve them.

## Assemble the customer archive

```bash
cargo run --locked -p spire_release -- package \
  --staging dist/staging --reports dist/reports --out dist/archive --candidate
cargo run --locked -p spire_release -- test-archive \
  --archive dist/archive/SpireTween-VERSION-REVISION-candidate.zip \
  --godot "$(cat dist/engine/godot-path)" \
  --templates "$PWD/dist/engine/export/templates" --out dist/archive-validation
```

The package uses an allowlist: addon runtime binaries/manifest, project license, README/release instructions, dependency notices, unmodified MPL dependency sources, provenance and SHA-256 manifests. It excludes example assets (including Girl Knight), native test libraries, caches, Rust intermediates, PDBs and import libraries. Debug runtime libraries remain included because the editor needs them.

Without `--candidate`, packaging additionally requires a clean common source revision, signed/notarized macOS artifacts, and matching successful full-suite/export reports for every configured engine/platform/architecture. Archive extraction verifies every internal checksum and rejects extra files, then installs and runs both library profiles and exported games. The workflow delivers the exact archive that passed those checks. Toolchain/dependency/source pinning records reproducible inputs; bit-identical compilation across different machines is not claimed.

The storefront URL, commercial version/changelog, and support terms remain separate release decisions.
