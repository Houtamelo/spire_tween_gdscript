# Group 3 implementation verification

The batch adds punch, spatial/property shake, quaternion/Euler/look-at rotation,
normalized pivot/scroll, and an explicitly specified 3D spiral. The API inventory
records the Pro spiral numerical-parity limit rather than claiming equivalence
to an undocumented algorithm. See [API guide](../../docs/group3_templates.md)
and [math contracts](../../docs/designs/group3_math.md).

## Verified results

Linux debug libraries were built with the optional `api-4-4` feature and executed
headlessly on Godot 4.4 stable and 4.7.2 stable. Default API configuration remains
unchanged. Each engine passed:

- Full native suite: `NATIVE_TEST_RESULT total=124 failed=0`.
- Full GDScript suite: `GDSCRIPT_TEST_RESULT total=215 failed=0 skipped=0`.
- No ObjectDB/resource shutdown leak diagnostics or unexpected script errors/panics.

The 15 new native and 15 new GDScript cases cover the public adapters, typed
generic accessors, numeric waypoints, local/global rotations, look-at constraints,
live UI dimensions/ranges, input corrections, lifecycle, and cancellation.
The library's 16 unit tests pass, including the existing planar fixtures and new
spatial waypoints independently evaluated from the mathematical equations.

The generated pixel-scroll constructors have additional assertions in the
existing scroll cases. The latest GDScript full runs include those assertions;
the latest 15-case native feature runs also pass them on both engines. These
test-only additions do not change suite totals.

The default-API build with GDScript bridge support, native rustdoc broken-link
check, generator execution, and byte-for-byte generator reproducibility pass.
Existing rustdoc warnings from enum diagrams are unchanged.

Normalization warnings include the rejected input. The focused real-engine
diagnostic checks also cover both sides of the randomness/elasticity bounds,
nonfinite fallbacks, vector/quaternion inputs, and valid boundary values that
must not emit warnings. Both Godot versions pass
`TEMPLATE_WARNINGS_RESULT checks=15 failed=0`. A control using the prior addon
binary fails because its randomness warning omits the rejected value. Run after
installing the current addon and test scripts:

```sh
bash scripts/verify_template_warnings.sh \
  dist/engine-4.4/editor/Godot_v4.4-stable_linux.x86_64 \
  dist/group2-20261022/gdscript
```

## Regressions demonstrated before fixes

Three real-engine negative controls failed their new assertions before repair:

1. Global quaternion rotation with a nonuniform target scale used `S*R` via
   `Basis::scaled`, changing orientation and scale. Column scaling `R*S` preserves
   both. The source test checks a nontrivial target under a rotated parent.
2. For an unregistered generic effect with a freed explicit binding, forced
   completion invoked getter/setter twice and emitted finished. The control
   observed `[2,2,1,1]` for reads/writes/stopped/finished, against `[0,0,1,0]`.
   `SpireTween::force_complete` now checks explicit binding validity before
   invoking the tween's completion implementation. Registered GDScript bindings
   also retain their existing manager-driven cancellation behavior.
3. Slerping a constrained look-at destination let an unselected Euler component
   drift at intermediate times: Z was `0.4410521` instead of `0.4`. Shortest
   Euler routes with masked deltas preserve constrained components at every step.

Owned effect restoration checks owner validity after stopped handlers. Generic
accessors remain responsible for guarding objects freed by user callbacks during
stopped notification, since those closure captures are not discoverable.

## Reproduction

From the repository root:

```sh
cargo run --locked -p gdscript_bindgen
cargo build --locked -p spire_tween_rust --features api-4-4
cargo build --locked -p spire_tween \
  --features api-4-4,include_gdext_lib,include_gdscript_bridge
cargo check --locked -p spire_tween \
  --features include_gdext_lib,include_gdscript_bridge
cargo test --locked -p spire_tween --lib
RUSTDOCFLAGS='-D rustdoc::broken_intra_doc_links' \
  cargo doc --locked -p spire_tween --no-deps
```

Native projects copied the newly built `libspire_tween_rust.so` into their addon
library directory. GDScript projects copied the new `libspire_tween.so` plus the
source test scripts/scenes. Isolated fixture directories avoid overwriting a
library mapped by another engine process.

```sh
timeout 660 dist/engine-4.4/editor/Godot_v4.4-stable_linux.x86_64 \
  --headless --verbose --log-file /tmp/spire-group3-final-native-4.4-engine.log \
  --path dist/group3-20261026/native --max-fps 120 res://test_runner.tscn \
  -- --no-cache
timeout 120 dist/engine-4.4/editor/Godot_v4.4-stable_linux.x86_64 \
  --headless --verbose --log-file /tmp/spire-group3-final-gdscript-4.4-engine.log \
  --path dist/group2-20261022/gdscript --fixed-fps 120 \
  res://examples/tests/test_runner.tscn -- --all-tests --no-cache
```

For 4.7.2 use its engine and the `native-4.7.2` / `gdscript-4.7.2` fixtures.
Final logs are `/tmp/spire-group3-final-{native,gdscript}-{4.4,4.7.2}.log`.
Native uses real time; GDScript uses fixed FPS. Native's intentional panic
controls are delimited by `EXPECTED_API_FAILURE_BEGIN/END`; GDScript's invalid
enum controls by `EXPECTED_INVALID_ENUM_BEGIN/END`.

No pipeline Python was introduced.
