# Independent blueprint targets

Later verification after removing the addon Signal trigger is recorded in [Native signal connections](../blueprint_no_signal_trigger_2026_11_05/README.md).

Removed the target-override feature at the user's request. `play_on`, the `override_children_targets` property/setter, incoming-target propagation, inherited overrides, and transient per-build target state are gone. `play()` is parameterless; each blueprint uses its configured target or first non-blueprint ancestor.

The new regression first failed against the prior implementation. It now verifies that the removed API and Inspector field are absent, nested children retain independent configured targets, fallback skips blueprint ancestors and their configured targets, and freeing a container's unused target does not cancel its children. The rendered editor checks the same target rules and preserves the existing typed-field, Undo/Redo, and save/reload checks.

| Check | Result |
|---|---|
| Godot 4.4 runtime | 15 passed |
| Godot 4.7.2 runtime | 15 passed |
| Godot 4.4 rendered editor | Passed |
| Godot 4.7.2 rendered editor | Passed |
| Godot 4.7.2 aggregate GDScript suite | 291 passed, 0 failed, 0 skipped |
| Rust API compile test | Parameterless play and stop compile |

Reproduce with `bash scripts/test_blueprint.sh /path/to/godot --rendered` and `cargo test -p spire_tween --test must_compile --features include_gdext_lib,include_gdscript_bridge,api-4-4`. The focused tests use an isolated Linux fixture, Xvfb, and an API-4.4-compatible debug addon. Focused logs contain no engine errors or Rust panics; warnings exercise invalid inputs. The aggregate suite includes existing intentional error-handling cases.

This report supersedes the target-override behavior recorded in [the earlier runtime handoff](../blueprint_2026_11_05/README.md). The authoring guide and design documents now specify independent targets. This is development verification; no release or commit was performed.
