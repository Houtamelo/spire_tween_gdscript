# AnimatableBody collision validation

Existing Node2D/Node3D position and rotation tweens drive scripted
`AnimatableBody2D/3D` motion with `sync_to_physics = true` and a physics-clock
root tween/sequence. No production implementation or public API changes were
needed. The workflow is documented in
[`docs/animatable_bodies.md`](../../docs/animatable_bodies.md).

## Coverage

Five new cases are registered in each real-engine suite:

| Case | Assertions |
|---|---|
| `test_animatable_body_2d_translation` | Local-position tween pushes a rigid body, produces `body_entered` identifying the moving body, reaches its endpoint, and supports pause/resume. |
| `test_animatable_body_3d_translation` | World-position tween below a non-origin parent pushes a rigid body, produces the expected contact, and reaches its endpoint. |
| `test_animatable_body_2d_rotation_sequence` | A physics-clock sequence rotates an offset door collider into a rigid body and pushes it. |
| `test_animatable_body_3d_rotation_sequence` | The equivalent 3D rotation/contact test. |
| `test_animatable_body_stop_and_free` | Stop halts new writes without completion; freeing a restarted target cancels playback without completion in both dimensions. |

Each collision test includes a second rigid body at the same starting position
with collision layers/masks disabled. It must remain stationary. Independent
SubViewport worlds prevent interactions with the test runner's other scenes.
Targets are sampled after additional physics frames to allow synchronization.
Tests use automatic manager/sequence updates, not `custom_step`.

The new expected suite totals in `release/config.json` are 171 native and 263
GDScript cases. Engine physics runs at 60 ticks/second with the project's
`DEFAULT` backend settings in both tested engine versions. Backend-specific or
cross-platform equivalence is not claimed.

## Results

Focused Godot 4.4 runs passed all five native and all five GDScript cases.
Both full suites also passed on each engine, with exit code 0:

| Linux x86_64 engine | Native | GDScript |
|---|---|---|
| Godot 4.4 stable | 171 passed, 0 failed | 263 passed, 0 failed, 0 skipped |
| Godot 4.7.2 stable | 171 passed, 0 failed | 263 passed, 0 failed, 0 skipped |

All five new native and GDScript cases were discovered on both engines. No
ObjectDB/resource shutdown leaks or orphan StringNames were reported. Script
exceptions and native failure-path panics stayed inside the existing intentional
failure markers. Minimum-API native compilation, default-API native compilation,
targeted rustfmt and diff whitespace checks passed. Existing compiler warnings
remain unchanged; the new native test module adds none.

## Negative control

In a separate ignored fixture, omit only the moving animatable bodies' collision
shapes. All four translation/rotation cases fail their contact and displacement
assertions while their transform endpoint checks still succeed. The stop/free
case passes. The runner reports `GDSCRIPT_TEST_RESULT total=5 failed=4 skipped=0`
and exits 1. This verifies that endpoint interpolation alone cannot satisfy the
collision tests. The source tests retain their shapes.

## Reproduction and artifacts

Build the native suite against the minimum supported bindings:

```sh
cargo build --locked -p spire_tween_rust --features api-4-4
```

Install the native library in an isolated native fixture and the existing
GDScript bridge library in an isolated GDScript fixture. The latter's production
source is unchanged; synchronize the updated test scripts/scenes into it.

```sh
timeout 660 "$GODOT" --headless --verbose --log-file /tmp/animatable-native-engine.log \
  --path "$NATIVE_PROJECT" --max-fps 120 res://test_runner.tscn -- --no-cache
timeout 120 "$GODOT" --headless --verbose --log-file /tmp/animatable-gdscript-engine.log \
  --path "$GDSCRIPT_PROJECT" --fixed-fps 120 \
  res://examples/tests/test_runner.tscn -- --all-tests --no-cache
```

Use writable temporary `XDG_CONFIG_HOME`, `XDG_CACHE_HOME`, and `XDG_DATA_HOME`.
The native suite uses the real clock because existing tests check elapsed time;
the GDScript suite uses fixed 120 FPS. Both use actual engine physics steps.
For a focused case, pass `--test=test_animatable_body_2d_translation --no-cache`.

Ignored fixtures: `dist/animatable-20261103/{native,gdscript}-{4.4,4.7.2}/`.
Full logs: `/tmp/spire-animatable-{native,gdscript}-{4.4,4.7.2}.log` and companion
`-engine.log` files. Focused logs use the same prefix plus their test name.
Negative-control logs: `/tmp/spire-animatable-control.log` and companion engine
log. Build, default-API compilation, and backend inspection logs use the
`/tmp/spire-animatable-` prefix as well.
