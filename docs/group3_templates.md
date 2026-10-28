# Punch, shake, orientation, normalized UI, and 3D spirals

GDScript constructors register immediately. Rust constructors return builders:
call `.register()` for scheduled playback or `TweenPtr::new(builder)` for manual
stepping. These APIs return a float method tween (`SpireMethodFloat` in GDScript,
`SpireTween<LerpMethodData<f64>>` in Rust). Godot 4.4+ is supported.

```gdscript
DoControl.punch_scale(button, Vector2(0.15, 0.15), 0.4)
DoNode3D.shake(camera_rig, 0.6, Vector3(0.1, 0.1, 0.1))
DoNode3D.look_at(enemy, player.global_position, 0.3)
DoScrollContainer.vertical_normalized_position(scroll, 1.0, 0.25) # top
```

## Punch and shake

| GDScript class | Constructors | Strength/punch types |
|---|---|---|
| `DoNode2D`, `DoControl` | `punch_position`, `punch_scale`, `punch_rotation`, `shake_scale`, `shake_rotation` | Vector2 position/scale; float rotation |
| `DoNode3D` | `punch_position`, `punch_scale`, `punch_rotation`, `shake`, `shake_scale`, `shake_rotation` | Vector3 |
| `DoCamera3D` | `shake`, `shake_rotation` | Vector3; position uses captured camera-relative XY and ignores Z strength |
| `DoNode3D` | `blendable_punch_rotation` | Vector3; always blendable |

Positions are local. Rotation strengths are **radians**, following Godot. Scale
strengths are additive deltas. Existing 2D/Control position `shake` APIs remain.

```gdscript
DoNode3D.punch_position(node, punch, duration,
    vibrato = 10, elasticity = 1.0, blendable = true)
DoNode3D.shake(node, duration, strength,
    vibrato = 10, randomness = 90.0, fade_out = true,
    randomness_mode = Spire.SHAKE_RANDOMNESS_MODE_FULL, blendable = true)
DoNode3D.blendable_punch_rotation(node, punch, duration,
    vibrato = 10, elasticity = 1.0)
```

The scale/rotation counterparts use the respective punch/shake signature with
the types in the table. Punch uses alternating decaying waypoints, not a sine
wave. Elasticity controls the opposite-side amplitude: zero returns to baseline
between outward peaks; one uses full opposite peaks. Segment durations increase
with their one-based index. Both effects use `floor(vibrato * duration)` segments,
clamped to 2–65536. Punch computes knots analytically without waypoint allocations.
Shake retains its waypoint buffer.

Starts are captured on first processing and retained on replay. Restart and
Incremental repeat the effect; Yoyo retraces it. Pause/resume retains the path;
replay after stop regenerates shake randomness. Values are sampled modulo one:
keep the driver's range at `0 -> 1`. Global/default or explicit easing remaps
the whole path, including live global setting changes.

Blendable vector/scalar effects replace their previous contribution on the current
value. Non-blendable effects write captured start plus offset. 3D rotations compose
local quaternion contributions; different-axis rotations are order-dependent.
Absolute writers can discard tracked contributions. Avoid simultaneously crossing
zero on a scale axis and animating rotation: Godot's reflected-transform
decomposition is ambiguous.

`stop()` leaves the target untouched. `force_complete()` is a no-op while stopped;
play before requesting successful completion. Successful completion restores via
an immediate native finish hook, before later finish handlers. Clearing native
finish connections disables restoration; clearing stopped connections disables
shake regeneration on replay. Owner destruction cancels without finished.

Invalid duration becomes one second; nonfinite punch components become zero;
shake components must also be nonnegative. Elasticity clamps to `[0,1]` and
randomness to `[0,180]` degrees (nonfinite defaults: 1 and 90). Corrections and
invalid segment counts warn. Positive short effects silently use two segments.

Rust traits: `DoPunch2D`, `DoPunchControl`, `DoPunch3D`, `DoShakeProperties2D`,
`DoShakePropertiesControl`, `DoShake3D`; methods have the `do_` prefix and require
every option explicitly. Generic accessors remain Rust-only:

```rust,ignore
do_punch_vec3(getter, setter, punch, duration, vibrato, elasticity, blendable)
do_shake_vec3(getter, setter, duration, strength, vibrato, randomness,
    fade_out, randomness_mode, blendable)
```

The getter is `Fn() -> Vector3`, setter `FnMut(Vector3)`. Captured lifetimes cannot
be discovered: use `.bound_to(node)` for every manually freed node they access.
Binding guards processing and pre-completion validity. If a stopped callback
frees a captured object before finish restoration, guard the generic accessors
with `is_instance_valid()` as well. Owned node adapters perform that restoration
guard automatically.

## Orientation

`DoNode3D` exposes these constructors. Rust uses `DoOrientation3D` with `do_`
prefixed names. Angles are radians; Euler conversion uses YXZ order. Starts are
captured on first processing. Stop leaves orientation; completion reaches its
destination without restoring the initial rotation.

| Constructor after `node` | Behavior |
|---|---|
| `rotate(to: Vector3, duration, mode = Spire.ROTATE_MODE_FAST)` | Local Euler target |
| `global_rotate(to: Vector3, duration, mode = Spire.ROTATE_MODE_FAST)` | World Euler target |
| `rotate_quaternion(to: Quaternion, duration)` | Local normalized shortest-arc slerp |
| `global_rotate_quaternion(to: Quaternion, duration)` | World normalized shortest-arc slerp |
| `blendable_rotation(by: Vector3, duration)` | Local quaternion deltas multiplied on the right |
| `blendable_global_rotation(by: Vector3, duration)` | World quaternion deltas multiplied on the left |

`ROTATE_MODE_FAST` chooses the shortest signed route per Euler component.
`ROTATE_MODE_FAST_BEYOND_360` preserves winding and multiple revolutions.
`ROTATE_MODE_WORLD_AXIS_ADD` interprets the argument as relative rotation about
world axes; `ROTATE_MODE_LOCAL_AXIS_ADD` composes about object-local axes. Add
modes always interpret the argument as a delta. Quaternion multiplication order
matters. Existing Euler property constructors retain their behavior.

Scale is preserved. Global writes rebuild an orthogonal world basis with its
current scale; nonuniformly scaled parents can cause shear/decomposition effects.
Use Restart/Yoyo for quaternion/look-at templates; an Incremental quaternion loop
contract is not provided. These templates retain their float-driver return types;
the general [typed Quaternion property/method family](typed_values.md) also
supports rotation-arc extrapolation. Nonfinite Euler components become zero;
quaternion targets normalize, with zero/nonfinite targets becoming identity. Warns
on corrections. Invalid duration becomes one second.

### Look-at

```gdscript
DoNode3D.look_at(node, towards: Vector3, duration,
    up = Vector3.UP, axis = Spire.ROTATION_AXIS_ALL, use_model_front = false)
DoNode3D.dynamic_look_at(node, towards: Vector3, duration,
    up = Vector3.UP, axis = Spire.ROTATION_AXIS_ALL, use_model_front = false)
DoNode3D.look_at_target(node, target: Node3D, duration,
    up = Vector3.UP, axis = Spire.ROTATION_AXIS_ALL, use_model_front = false)
```

Targets are world-space. Static look-at captures its destination once; dynamic
look-at recomputes it as the source moves. The node-target version also tracks
target motion and binds its lifetime and configured bound-node pause rules. The
initial orientation remains the interpolation baseline.

Godot -Z faces the target, or +Z with `use_model_front=true`. Constraints `X`, `Y`,
`Z`, `XY`, `XZ`, `YZ` (with the `ROTATION_AXIS_` prefix) preserve unselected initial
world Euler components throughout playback; `ALL` is unrestricted. Look-at uses
shortest Euler routes rather than quaternion slerp. Coincident targets
or parallel up/direction retain the last valid destination and warn once. Invalid
up defaults to Vector3.UP with a warning. Owner/bound-target destruction cancels.

## Normalized UI

`DoControl.pivot(node, to: Vector2, duration)`, `pivot_x(node, to: float, duration)`
and `pivot_y(...)` tween ratios of live Control size. Zero is top/left; one is
bottom/right. The initial ratio is captured on first processing; writes use the
current size. Component variants preserve the other pixel component. Zero-sized
axes start at ratio zero. Existing pixel `pivot_offset` constructors remain.

`DoScrollContainer.normalized_position(node, to: Vector2, duration)`,
`horizontal_normalized_position(node, to: float, duration)` and
`vertical_normalized_position(...)` use live scrollbar min/max/page values. The
effective upper bound is `max(min, max-page)`. X: left=0, right=1. Y: bottom=0,
top=1, matching Unity ScrollRect. Empty ranges stay at min. Axis variants preserve
the other scrollbar. Container layout may change ranges between steps.

Both families clamp targets and eased ratios to `[0,1]`. Corrections warn.
Nonfinite pivot/X-scroll defaults to zero; nonfinite Y-scroll defaults to one.
Invalid duration becomes one second. Owner destruction cancels. Rust uses
`DoNormalizedPivot` and `DoNormalizedScroll`, with `do_` prefixed names.
Ordinary pixel scroll constructors are also available as
`DoScrollContainer.scroll_horizontal` and `scroll_vertical`.

## 3D spiral

```gdscript
DoNode3D.spiral(node, duration, axis = Vector3.BACK,
    mode = Spire.SPIRAL_MODE_EXPAND, speed = 1.0, frequency = 10.0,
    depth = 0.0, snapping = false)
```

Rust uses `DoSpiral3D::do_spiral` with every option explicit. Position is local;
axis is in parent coordinates. Capture start on first processing. Define
`travel=duration*speed*eased_progress`, `angle=frequency*travel`. Expand uses radius
travel; `SPIRAL_MODE_EXPAND_THEN_CONTRACT` uses a triangular radius peaking halfway
then returning to zero. Axial displacement is `depth*travel`. Snapping rounds the
result. Frequency is radians per unit radial travel; lower frequency creates
wider turns at a given angle. Speed/depth may be signed. Stop leaves position;
completion evaluates the endpoint, including residual axial displacement.

Invalid/zero axis becomes +Z, invalid duration becomes one second, invalid or
nonpositive frequency becomes 10, nonfinite speed/depth become 1/0. Warns on
corrections. This covers the publicly documented Pro controls; its undocumented
numerical algorithm was not inspected and exact Pro numerical parity is not
asserted. Existing 2D spiral families retain their APIs.

See [mathematical contracts and sources](designs/group3_math.md) and the
[earlier adapter guide](adapter_templates.md).
