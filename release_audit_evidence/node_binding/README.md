# Explicit node binding verification

The existing local debug addon reproduced seven failing GDScript cases and seven manager assertion panics. Some panics were caught at the GDExtension boundary while their test still returned true; a zero exit status alone cannot establish success.

The fix preserves the tween's explicit node set and the manager's reverse weak index. Clearing removes both records, duplicate binds are idempotent, absent unbinds are harmless, and GDScript methods delegate to native binding methods. Clearing also accepts previously freed nodes and does not initialize the manager when used on an unregistered builder.

The focused suites run inside a real Godot 4.7.2 SceneTree:

```sh
godot --headless --path spire_tween_gdscript --fixed-fps 120 res://examples/tests/node_binding_runner.tscn
godot --headless --path spire_tween_rust --fixed-fps 120 res://examples/tests/node_binding_runner.tscn
```

Expected markers are `BINDING_RESULT total=13 failed=0` and `NATIVE_BINDING_RESULT total=11 failed=0`. Also reject engine errors, Rust panic reports, and unexpected warnings in the output. Both runners report individual cases and return nonzero on failed assertions. Native tests catch panics directly; GDScript boundary panics additionally require checking the output.

Coverage includes clearing multiple bindings without disturbing another tween; duplicate binds and repeated/absent unbinds; preserving a remaining node binding; unregister/register before and after deferred removal; completion and replay; append, join, insert, and nested children; freed-node clearing; automatic property/method owner lifetime; bound pause behavior; builder bindings; and node-driven force-complete/kill.

The existing 15 GDScript and 10 native ownership regressions are also rerun. The GDScript all-tests runner completes 77 cases with its existing invalid-enum script error. The native all-tests scene now instantiates `TestRunner`; its focused suites pass, followed by the previously recorded shake, time-scale, follow-3D, and freed-node errors in the older tests. Neither broad runner certifies an otherwise clean release.

Evidence files:

- `baseline-gdscript.log`: the pre-binding-fix local debug library.
- `candidate-*.log`: focused debug/release binding runs in isolated projects.
- `ownership-*.log`: ownership regression runs.
- `full-debug-*.log`: broader suite results and existing failures.
- `installed-*.log`: focused checks using the actual project addon libraries.
- `build-*.log`, `check-*.log`, `cargo-tests.log`: build, feature, and Cargo verification.
- `validation.json`, `artifact-sha256.json`: process exit codes, output gates, and refreshed Linux library hashes.
- `baseline-insert-only.log`, `candidate-insert-only.log`: a separate existing sequence defect described in [the report](sequence_insert_lifetime.md).

The isolated projects are under `/tmp/spire-binding-i861qtbo`, with Godot config/data/cache redirected there. For release-profile GDScript checks, the release library is deliberately copied to the isolated manifest's debug entry because a normal Godot editor executable selects that entry. The native manifest likewise uses a release-named path for both modes; each isolated run installs the intended profile at that path.
