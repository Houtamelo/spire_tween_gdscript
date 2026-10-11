# Release audit evidence

This directory records the 2026-10-07 audit of source commit `f518d78`. Read [the audit](../release_audit.md) for interpretation; zero exit status alone is not a passing result in the existing runners.

- `gdscript-tests.log`: existing local addon binary, all tests, Godot 4.7.2 at fixed 60 FPS.
- `gdscript-current-tests*.log`: freshly built source binary at fixed 60 and 120 FPS.
- `rust-tests.log`: debug Rust integration run; panics and eventual timeout.
- `rust-release-tests.log`: release Rust integration run at fixed 120 FPS; printed completion despite failures.
- `probe-current-*.log` and `probe-shipped46-*.log`: bounded native-engine reproductions.
- `minimal-*-surface.log`: current source versus existing addon method registration.
- `empty-control-import.log`, `empty-addon-*-import.log`, `editor-abort-gdb.log`, `godot46-import.log`: first-import control/variant evidence and debugger output.
- `minimal-shipped-44-import.log`: the existing API-4.6 addon rejected by the declared Godot 4.4.1 minimum.
- `godot46-export*.log`: matching-engine Linux release export and execution.
- `spire-audit-*.log`: Cargo compilation/test output.
- `current-linux-dependency-inventory.json`: current resolved Linux workspace packages and declared licenses. This includes build tools and harness packages; it is not a final shipped-binary notice list.
- `Cargo.lock.audit` and `artifact-provenance.json`: the audit's dependency snapshot, source revision, and hashes of the existing/fresh Linux binaries.
- `native-shake-run.log` and `repros/native_shake.rs`: deterministic native shake control/variant using real Godot objects and the public Rust API. `native-probe.Cargo.lock.audit` records that small probe's resolved dependencies.

The `repros` directory contains small Node scripts and scenes. To reuse a scene, place its matching `.gd` and `.tscn` at the root of a project with the addon installed/imported, then run it in Godot. The scripts print measured behavior; they intentionally reproduce defects and are not passing regression tests. **`probe_cycle` can crash the engine.** The binding panic reproductions require a debug addon build. `probe_leak` uses real object counts; its exact baseline varies by engine/project.

Original isolated projects, binaries, caches, and exports remain at `/tmp/spire-release-audit-3lu0eh8z`. Godot configuration/data/cache directories were redirected there with `XDG_CONFIG_HOME`, `XDG_DATA_HOME`, and `XDG_CACHE_HOME`. No production source was modified.

The audit built the addon with:

```sh
cargo build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --locked
cargo test -p spire_tween --features include_gdext_lib,include_gdscript_bridge --locked
cargo build -p spire_tween_rust --locked
cargo build -p spire_tween_rust --release --locked
```

The GDScript runner scene was copied to an isolated project and given `run_mode = 1` to force an all-tests run. The representative runtime commands were:

```sh
godot --headless --path "$AUDIT_PROJECT" --fixed-fps 60 res://examples/tests/test_runner.tscn
godot --headless --path "$AUDIT_PROJECT" --fixed-fps 120 res://examples/tests/test_runner.tscn
godot --headless --path "$AUDIT_RUST_PROJECT" --fixed-fps 120 res://test_runner.tscn
godot --headless --path "$AUDIT_PROBE_PROJECT" --fixed-fps 60 res://probe_leak.tscn
```

A first-import crash was reproduced with an empty project using `godot --headless --path "$AUDIT_PROJECT" --editor --import`, then with only the addon added. The empty control succeeded; the addon variant crashed on first import and succeeded on the second.
