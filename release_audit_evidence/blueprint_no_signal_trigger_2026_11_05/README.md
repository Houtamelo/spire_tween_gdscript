# Native signal connections replace the addon Signal trigger

Removed TriggerCondition::Signal, its exported constant, trigger_signal property/setter, Inspector signal picker, signal reflection, arbitrary-signal subscription branch, and related diagnostics. The remaining trigger ordinals (0–6) are unchanged; trigger_source remains shared mouse-trigger configuration.

The regression failed against the old implementation and now verifies that the removed API and Inspector fields are absent. A new real-engine case packs and restores a persistent Button.pressed → TweenBlueprint.play connection, then invokes it in Manual mode. Mouse tests cover source changes, switching to Manual, exit/reentry cleanup, and 2D/3D input signatures. No addon signal-binding replacement was introduced.

| Check | Result |
|---|---|
| Godot 4.4 runtime | 15 passed |
| Godot 4.7.2 runtime | 15 passed |
| Godot 4.4 / 4.7.2 rendered editor | Both passed |
| Godot 4.7.2 aggregate GDScript suite | 291 passed, 0 failed, 0 skipped |
| Rust API compile test | Passed |

Reproduce using `bash scripts/test_blueprint.sh /path/to/godot --rendered` and `cargo test -p spire_tween --test must_compile --features include_gdext_lib,include_gdscript_bridge,api-4-4`. Focused runs use isolated Linux fixtures, an API-4.4-compatible debug addon, and Xvfb. Their logs contain no engine errors or Rust panics. The aggregate suite includes existing intentional error-handling cases. Source hashes are recorded alongside the logs.
