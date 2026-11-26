# Rebuild SpireTween from this source snapshot

This archive contains the addon, binding generator, native test code, GDScript
templates, release tooling, documentation, and pinned build inputs from the
same source snapshot as the accompanying binary addon. `source.json` records its
revision and source inventory; `SHA256SUMS` verifies delivered bytes.

Install Rust through rustup, extract this archive, and run from its root:

```bash
cargo build --locked -p spire_tween --features api-4-4,include_gdext_lib,include_gdscript_bridge
cargo build --locked -p spire_tween --features api-4-4,include_gdext_lib,include_gdscript_bridge --release
```

Cargo uses the nightly toolchain in `rust-toolchain.toml`. Build on the target
platform or install the appropriate Rust target and native SDK for cross builds.
The outputs are under `target/debug` and `target/release`. Match their locations
to the `.gdextension` manifest in your Godot project. The supplied binary addon
already contains a manifest and both profiles for each supported desktop platform.

The workspace layout and lockfile are retained. Third-party example art, example
project assets, videos, audit evidence, binaries and caches are excluded. The
[public repository](https://github.com/Houtamelo/spire_tween_gdscript) contains the
complete example projects and engine integration fixtures. Release automation
uses Git to establish source provenance; use a repository checkout for that flow.

The source remains MIT licensed; see `LICENSE`. Build dependencies retain their
own licenses. Dependency notices and required dependency sources accompany the
binary addon.
