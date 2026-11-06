# TweenBlueprint runtime and Inspector verification

Historical record before the target-override feature was removed. Current behavior and verification are recorded in [Independent blueprint targets](../blueprint_no_target_overrides_2026_11_05/README.md).

Validated on Linux x86_64 with one debug addon compiled against the Godot 4.4 API. The user selected runtime and built-in Inspector completion; preview/restoration and presets remain outside this handoff.

| Check | Result |
|---|---|
| Godot 4.4 runtime | 15 integration cases passed |
| Godot 4.7.2 runtime | 15 integration cases passed |
| Godot 4.4 rendered editor | Typed fields, hidden-field storage, Undo/Redo, save/reload, relative targets, and no automatic editor playback passed |
| Godot 4.7.2 rendered editor | Same checks passed |
| Aggregate GDScript runner, Godot 4.7.2 | 291 passed, 0 failed, 0 skipped |
| Rust public API compile test | TweenBlueprint and its enums are accessible from the Rust prelude; play/play_on/stop signatures compile |

Runtime cases exercise all 12 suggested types with duration and zero-duration playback, explicit/implicit start, fixed restart, serial/parallel composition, timing/easing, relative and speed-based modes, nested target overrides, invalid steps/settings, missing custom hooks/signals, freed bindings, handle ownership, automatic events, reentry, retargeting, and typed lifecycle ordering. Reentry here means leaving and re-entering the SceneTree; same-instance synchronous play reentry remains unsupported by design.

Reproduce the focused checks from the repository root:

```bash
bash scripts/test_blueprint.sh /path/to/godot-4.4 --rendered
bash scripts/test_blueprint.sh /path/to/godot-4.7.2 --rendered
cargo test -p spire_tween --test must_compile --features include_gdext_lib,include_gdscript_bridge,api-4-4
```

Rendered runs used Xvfb and Mesa llvmpipe; the two Inspector screenshots are attached. Focused logs contain no engine errors or Rust panics. Warnings in runtime logs are intentional invalid-input coverage. Display/input-method/VSync warnings are environmental. The aggregate suite also contains intentional error-handling cases that print engine errors while their assertions pass.

`source-sha256.txt` records the source files exercised by the focused tests. All test projects and runtime caches were isolated under `/tmp`; the development projects' addon binaries and caches were left untouched. This is development verification, not a release/package approval or a claim about the full native aggregate suite.
