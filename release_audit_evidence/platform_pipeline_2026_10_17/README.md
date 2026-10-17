# Platform release pipeline — local verification, 2026-10-17

The shell/Rust release pipeline is implemented. Native Windows/macOS CI and Apple signing are pending; a push to the existing public GitHub repository was rejected by automatic approval review because the exact source payload/destination had not been explicitly authorized. No remote branch or release was published.

Build source: `f87b076bca477cd6db7f44912daccb814b4f1927`. Pinned toolchain: `nightly-2026-08-11`, rustc `1.99.0-nightly (12c36e253 2026-08-10)`. Godot API: explicit `api-4-4`; ordinary development defaults remain unchanged. CI invokes shell and Rust only.

| Check | Result |
|---|---|
| Linux addon/native libraries, debug and release | Built and staged with provenance/hashes/notices |
| Godot 4.4 full GDScript suite | 152/152 per profile, exit 0 |
| Godot 4.4 full native suite | 134/134 per profile (66 focused + 68 ordinary), exit 0 |
| Passing/failing runner controls | Expected 0/1 exits and result markers in both profiles |
| Final staged Linux install/export smoke | Both profiles pass on Godot 4.4 and 4.7.2 |
| Rust release CLI filesystem/process checks | 4/4 pass |
| Real Linux artifact/archive integration checks | 3/3 pass, including missing-library and tampering failures |
| Windows x86_64 cross-build | Both addon/native profiles build with static CRT |
| Actual Windows PE imports and mutation failures | Pass on Linux; external VCRUNTIME/MSVCP imports are rejected |
| macOS ARM64/Intel addon/native `cargo check` | Both pass; does not establish linking or runtime support |
| Windows release-tool `cargo check` | Pass, including platform-specific process-tree cleanup |
| Fresh Linux/Windows-style checkouts | 802 source files have matching fingerprints after excluding platform library stubs |
| Workflow syntax / scoped Rust formatting | actionlint 1.7.12 and rustfmt pass |

The full suite ran before the final tooling-only provenance/licensing/archive refinements. `baseline-linux-build.json` preserves that exact build record; `linux-build.json` preserves final staging. All four tested Linux shared-library hashes match final staging exactly (`summary.json` records the comparison). The final archive was independently extracted, checksum-checked, installed, exported in both profiles, and executed on Linux using both configured endpoint engines. The local full report is not substituted for the clean-revision reports required by commercial packaging.

The local candidate is `dist/artifacts/SpireTween-1.4.0-f87b076bca47-candidate-linux-windows.zip`, 34,579,425 bytes, SHA-256 `57d0ce28e6e707107064034b4231cefababbb63ceed0497196cabe6065ccae88`. It contains Linux/Windows runtime libraries, notices, unmodified MPL sources, Rust library notices, installation/build instructions, checksums and provenance. It excludes macOS (not built here), examples/art, test binaries, PDBs and Rust intermediates. It is a development candidate, not a certified commercial archive. Native Windows execution is still pending.

Godot 4.4's extension-free asset importer emitted headless progress/editor-settings diagnostics during the initial full run. Runtime fixtures imported the needed assets successfully and all cases passed. The final tool redirects Linux data/config/cache paths into writable fixtures. No graphical Godot process or cold editor import with the addon loaded was used.

The known GDScript aggregate shutdown warnings persist: ObjectDB instances leaked and two resources still in use. The instance count was not enumerated in this run. Expected invalid-enum/invalid-built-in diagnostics also remain in the preserved full logs. These are distinct from functional test outcomes; this work does not resolve the separate shutdown warning investigation.

The GitHub workflow builds on native Windows/Linux/macOS hosts and validates four engine minors, both profiles, and both Mac architectures. Apple certificate enrollment/provisioning remains an owner task; the repository currently has none of the signing secrets listed in [release instructions](../../release/README.md). Optional Android builds remain experimental and unvalidated at runtime. The final assembled desktop archive must pass the native/extracted-archive jobs before `verified-desktop-package` is produced. Nothing auto-publishes to itch.io or GitHub Releases.
