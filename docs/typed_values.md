# Tweening Vector4, Quaternion, and Rect2

These Godot values have built-in property and method tween families. They use
ordinary easing, node bindings, loops, sequences, and completion/cancellation
signals. GDScript constructors register immediately; Rust constructors return
unregistered builders.

## GDScript constructors

| Value | Property constructor | Method constructor | Handle classes |
|---|---|---|---|
| `Vector4` | `Spire.do_property_vec4(owner, path, to, duration)` | `Spire.do_call_vec4(callable, from, to, duration)` | `SpirePropertyVec4`, `SpireMethodVec4` |
| `Quaternion` | `Spire.do_property_quaternion(owner, path, to, duration)` | `Spire.do_call_quaternion(callable, from, to, duration)` | `SpirePropertyQuaternion`, `SpireMethodQuaternion` |
| `Rect2` | `Spire.do_property_rect2(owner, path, to, duration)` | `Spire.do_call_rect2(callable, from, to, duration)` | `SpirePropertyRect2`, `SpireMethodRect2` |

Each property wrapper also exposes `build(owner, path, to, duration)`; each method
wrapper exposes `build(callable, from, to, duration)`. Method callables receive one
argument of the corresponding type. Generic property paths support script fields
and indexed subproperties. Property constructors on `Spire` return null for a null
owner; `build` requires a valid owner.

```gdscript
var bounds: Rect2 = Rect2(0, 0, 64, 64)
var shader_vector: Vector4 = Vector4.ZERO

func animate(node: Node3D, sprite: Sprite2D, material: ShaderMaterial):
    Spire.do_property_rect2(self, "bounds", Rect2(20, 30, 128, 96), 0.5)
    Spire.do_property_vec4(self, "shader_vector", Vector4(1, 2, 3, 4), 1.0)
    DoNode3D.quaternion(node, Quaternion(Vector3.UP, PI / 2), 0.5)
    DoSprite2D.region_rect(sprite, Rect2(32, 0, 64, 64), 0.5)
    # Initialize a vec4 uniform explicitly in headless mode.
    material.set_shader_parameter(&"weights", Vector4.ZERO)
    DoShaderMaterial.parameter_vec4(material, &"weights", Vector4.ONE, 1.0)
```

The generated shortcuts are:

- `DoNode3D.quaternion(node, to, duration)` → `SpirePropertyQuaternion`.
- `DoSprite2D.region_rect(sprite, to, duration)` → `SpirePropertyRect2`.
  `region_rect_position` and `region_rect_size` return `SpirePropertyVec2` and
  preserve the other rectangle component. Enable `Sprite2D.region_enabled` to
  display the animated region.
- `DoVisualShaderNodeVec4Constant.constant(resource, to, duration)` →
  `SpirePropertyVec4`, plus `constant_x/y/z/w` scalar shortcuts. Godot stores this
  shader vector in a Quaternion; the adapter interprets its four components as
  numeric data and performs no rotation interpolation or normalization.
- `DoShaderMaterial.parameter_vec4(material, parameter, to, duration)` →
  `SpirePropertyVec4`. As with other shader adapters, a missing/incompatible
  uniform returns null with a diagnostic; replacing the shader cancels the tween.

These shortcuts also work through the matching typed `Spire.do_property_*`
lookup. Rectangle component paths are `region_rect:position` and `region_rect:size`.
Quaternion components have no generated scalar shortcuts because independent
component animation does not preserve unit rotations.

## Rust constructors

Use the existing generic constructors with the new types:

```rust,ignore
use godot::prelude::*;
use spire_tween::prelude::*;

node.do_quaternion(Quaternion::from_axis_angle(Vector3::UP, 1.0), 0.5)
    .register();
sprite.do_region_rect(Rect2::from_components(0.0, 0.0, 64.0, 64.0), 0.5)
    .register();
owner.do_property("custom_value", Evaluator::Static(Vector4::ONE), 1.0)
    .register();
owner.do_method("receive_rect", Rect2::default(), target_rect, 1.0)
    .register();
```

`TweenProperty::<T>::new_typed`, `TweenMethod::<T>::new`, `do_get_set`,
`do_property_array`, and `do_get_set_array` accept all three types. Rust typed
accessors remain Rust-only. Native wrapper classes are exported from
`spire_tween::prelude::gdscript` when the bridge feature is enabled. Registration,
type erasure (`AnyTween` / `AnyPropertyTween` / `AnyMethodTween`), and weak handles
work like existing value families.

## Interpolation and modes

| Value | Absolute interpolation | Relative update | Speed units |
|---|---|---|---|
| `Vector4` | Linear across `(x,y,z,w)` | Add the change in the interpolated vector | 4D Euclidean distance/second |
| `Rect2` | Linear across `(position.x,position.y,size.x,size.y)` | Add position and size deltas | 4D Euclidean distance/second across those four scalars |
| `Quaternion` | Normalized shortest-arc spherical interpolation | `current * previous.inverse() * next`, normalized | Shortest rotation angle in radians/second |

Property starts are captured on first processing unless `.from(value)`
(GDScript) or `.begin_from(value)` (Rust) overrides them. Method starts are supplied
at construction. Relative GDScript tweens use zero vectors/rectangles or identity
rotation as their baseline; Rust `.as_relative(baseline)` accepts an explicit
baseline. The original duration argument becomes speed after `.as_speed_based()`.
Speed-based stepping never overshoots and returns remaining time to sequences.

Quaternion inputs that are finite and nonzero are normalized automatically.
Zero/nonfinite quaternions emit a warning containing the rejected value and use
identity. This also applies to typed property forced completion. `q` and `-q`
represent the same orientation and have zero angular distance. Relative rotations
compose in processing order, so rotations around different axes need not commute.
The delta is postmultiplied in the property's coordinate frame; for Node3D's local
quaternion this is a local-axis contribution.

Quaternion property setters simply forward values. Completion paths that bypass
interpolation prepare the endpoint with `SpireLerp::prepare_completion_value`:
forced completion and speed-based completion when the endpoints already
coincide. The default implementation preserves the value, so existing/custom
lerpers retain their raw-endpoint completion behavior. Quaternion overrides it
to normalize the result or apply its warning/identity fallback.

Easing overshoot and Incremental loops extrapolate the original interpolation:
vectors and rectangles continue along their component differences; quaternions
continue along the original shortest rotation arc. Yoyo retraces it; Restart uses
the existing property start-reset rules. A quaternion endpoint specifies an
orientation, so use the existing `rotate(..., FastBeyond360)` template when an
explicit multi-turn Euler destination is required.

Rectangles retain signed sizes. Interpolation does not call `Rect2.abs()` or clamp
sizes; negative relative deltas and easing overshoot remain useful. Godot's
geometric rectangle operations generally assume nonnegative sizes.

Variant-based absolute and speed-based tweens also recognize these three types.
Variant absolute/method interpolation routes all supported built-in value
families through the same lerpers as their typed counterparts: integers, floats,
Vector2/2i/3/3i/4, Color, String, Quaternion, and Rect2. Integer-only endpoints
produce rounded integers; mixed integer/float endpoints produce floats.
StringName and NodePath endpoints use String interpolation and return String
values. Other types retain the Godot global lerp fallback. A custom interpolation
callable takes precedence; failed endpoint conversion logs an error and returns
the starting value.
Native Variant relative tweens can supply a typed baseline through
`.as_relative(baseline.to_variant())`; prefer the typed wrappers for GDScript
relative interpolation.

Stopping never writes the property. Forced completion writes the final value only
while playing/paused and with valid accessors, owner, and explicit bindings.
Invalid owners/callables cancel through the ordinary checked pipeline, emitting
`stopped` without `finished`.

The existing `rotate_quaternion` and `global_rotate_quaternion` convenience
templates retain their `SpireMethodFloat` return types. This typed family is an
additional general property/method API.

## References

The implementations use independently authored component and unit-rotation math
and Godot's value semantics. See Godot 4.4's
[Quaternion](https://docs.godotengine.org/en/4.4/classes/class_quaternion.html),
[Vector4](https://docs.godotengine.org/en/4.4/classes/class_vector4.html),
[Rect2](https://docs.godotengine.org/en/4.4/classes/class_rect2.html), and
[vec4 shader constant](https://docs.godotengine.org/en/4.4/classes/class_visualshadernodevec4constant.html)
references.
