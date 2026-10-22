# Property adapters and simple templates

These helpers use the ordinary property, method, and sequence processing paths.
GDScript constructors register immediately. Rust constructors return builders:
call `.register()` for automatic playback or `TweenPtr::new(builder)` for manual
stepping. Durations are seconds. Godot 4.4 or newer is required.

```gdscript
DoNode2D.jump(player, Vector2(300, 120), 48, 2, 0.8)
DoControl.uniform_scale(panel, 1.2, 0.25)
DoLabel.counter(score_label, 0, 12000, 1.0) # "12,000"
DoEnvironment.background_color(environment, Color.CORNFLOWER_BLUE, 1.0)
DoShaderMaterial.parameter_float(material, &"strength", 0.8, 1.0)
DoAudioServer.bus_volume_db(&"Music", -12.0, 0.5)
```

## Motion

`DoNode2D`, `DoNode3D`, and `DoControl` expose the following. The receiver is the
first GDScript argument. Rust uses the same names prefixed with `do_` on `Gd<C>`;
import `spire_tween::prelude::*`. Vector arguments are Vector2 for 2D/UI, Vector3
for 3D. These helpers do not add methods to native `WithBaseField` receivers;
call them on `self.to_gd()` in a Godot class implementation.

| Method and arguments after receiver | Return type | Behavior |
|---|---|---|
| `uniform_scale(to: float, duration)` | SpirePropertyVec2 / SpirePropertyVec3 | Expands a scalar to an absolute uniform scale. |
| `blendable_position(by: Vector, duration)` | SpirePropertyVec2 / SpirePropertyVec3 | Adds a local-position delta. |
| `blendable_global_position(by: Vector, duration)` | SpirePropertyVec2 / SpirePropertyVec3 | Adds a global-position delta. |
| `blendable_scale(by: Vector, duration)` | SpirePropertyVec2 / SpirePropertyVec3 | Adds a scale delta; it does not multiply the current scale. |
| `jump(to: Vector, height: float, jumps: int, duration)` | SpireMethodFloat | Jumps to a local position. |
| `global_jump(to: Vector, height: float, jumps: int, duration)` | SpireMethodFloat | Jumps to a global position. |

Jump captures the starting position on first processing. As in DOTween, horizontal
movement is linear, vertical displacement to the destination uses OutQuad, and
each hop is an OutQuad rise followed by its Yoyo reversal. Positive height is
upward: negative Y for 2D/UI and positive Y for 3D. Counts below one become one.
The returned tween's easing remaps the overall progress. Restart and Yoyo are
supported; Incremental is not. These helpers set positions directly and do not
perform collision-aware physics movement.

Control also exposes:

| Method | Arguments after receiver | Return type |
|---|---|---|
| `anchor_min` | `to: Vector2, duration` | SpireSequence |
| `anchor_max` | `to: Vector2, duration` | SpireSequence |
| `circle`, `global_circle` | `center: Vector2, from_angle, to_angle, radius, duration` | SpireMethodFloat |
| `ellipsis`, `global_ellipsis` | `center: Vector2, from_angle, to_angle, from_radius: Vector2, to_radius: Vector2, duration` | SpireMethodFloat |

Anchor min joins left/top; anchor max joins right/bottom. They preserve the
existing scalar anchor setters' offset/opposite-anchor behavior. Circle/ellipsis
angles are radians, with positive rotation clockwise. Unprefixed methods use local
Control coordinates; `global_*` methods use global coordinates. Container layout
may reposition a Control independently of the tween. Rust traits are
`DoControlAnchors` and `DoControlEllipsis`.

## Blendable and gradient colors

| GDScript namespace | Blendable method | Gradient method | Property |
|---|---|---|---|
| `DoCanvasItem`, `DoSpriteBase3D`, `DoLabel3D` | `blendable_modulate` | `gradient_modulate` | `modulate` |
| `DoLight3D` | `blendable_light_color` | `gradient_light_color` | `light_color` |
| `DoBaseMaterial3D` | `blendable_albedo_color` | `gradient_albedo_color` | `albedo_color` |

Blendable methods take `(receiver, to: Color, duration)` and return SpirePropertyColor.
They capture the initial color **at construction** and add `to - initial` over
time through ordinary Relative mode. Concurrent relative color tweens therefore
combine. A normal absolute color tween can overwrite their contributions.

Gradient methods take `(receiver, gradient: Gradient, duration, include_alpha =
true)` and return SpireMethodFloat. They sample the supplied resource live, honoring
its interpolation mode and color space. Easing remaps the sample coordinate.
Disabling alpha preserves the target's current alpha on every write. This differs
from DOTween's RGB-only gradient helper. Null gradients log an error and return
null. Gradient sampling supports Restart/Yoyo; Incremental does not traverse a
fresh gradient per loop and is not supported.

Rust uses the same method names prefixed with `do_`, with an explicit
`include_alpha: bool`. Traits are `DoCanvasItemColors`, `DoSpriteBase3DColors`,
`DoLabel3DColors`, `DoLight3DColors`, and `DoMaterialColors`.

`DoLine2D.endpoint_colors(line, start: Color, end: Color, duration)` returns a
SpireMethodFloat and animates the gradient's first/last colors. On first processing it
copies the existing gradient, preserving interior stops and interpolation
settings, and assigns that private copy to the line. With no gradient or an empty gradient it
starts from `default_color`; a single stop supplies both initial endpoint colors. Other users of the original resource are unchanged.
The copy remains assigned after completion or cancellation. Supports Restart/Yoyo,
not Incremental. Rust: `DoLineColors::do_endpoint_colors`.

## Resources, materials, and shader uniforms

Generated property shortcuts work on both Nodes and Resources. Resource tweens
retain their resource, and editing a shared resource affects every user. They do
not duplicate resources or change rendering modes automatically.

| Namespace | Properties |
|---|---|
| `DoEnvironment` | `background_color`, plus generated RGBA components |
| `DoBaseMaterial3D` | `albedo_color`, RGBA components, `fade` alias for alpha; `uv1_offset`, `uv1_scale`, `uv2_offset`, `uv2_scale`, plus XYZ components |
| `DoLabelSettings` | `font_size`, `font_color`, `outline_size`, `outline_color`, plus color components |
| `DoLine2D` | `default_color` and its RGBA components; `width` |

Each constructor takes `(resource, to, duration)` and returns the corresponding
typed property tween. Environment background color is visible when its background
mode is `BG_COLOR`. Material alpha visibility depends on the material's transparency
mode. UV properties use Godot's Vector3 representation. Rust uses generated
`SpireDoEnvironment`, `SpireDoBaseMaterial3D`, `SpireDoLabelSettings`, and
`SpireDoLine2D` traits with `do_` names, including native `WithBaseField` support.

ShaderMaterial has named, typed adapters:

```gdscript
DoShaderMaterial.parameter_float(material, &"gain", 2.0, 1.0)
DoShaderMaterial.parameter_vec3(material, &"offset", Vector3.ONE, 1.0)
DoShaderMaterial.parameter_color_a(material, &"tint", 0.5, 1.0)
```

Suffixes are `float`, `int`, `vec2`, `vec2i`, `vec3`, `vec3i`, and `color`.
`parameter_color_a` preserves current RGB on every write. Missing/incompatible
uniforms return null with a diagnostic before registration. If its stored value later
becomes incompatible, playback cancels. Supported uniforms must be compatible
with the indicated Godot type; Vector4/quaternion interpolation is separate work.
Starts are read on first processing, so shader values can change after construction.
The helper checks the declared uniform type. Shader defaults are obtained from the
renderer when no override exists; when defaults are unavailable (including headless
mode), first assign an initial value with `set_shader_parameter`. Replacing the
Shader resource cancels existing uniform tweens.
Rust equivalents are `do_shader_parameter::<T>(material, name, to, duration)` and
`do_shader_parameter_alpha(...)`, returning `anyhow::Result` builders.

## Text styles and counters

`DoLabel.font_size`, `.font_color`, `.outline_size`, and `.outline_color` animate
the effective Label style. With LabelSettings assigned, they edit that resource;
shared settings affect other labels too. Otherwise they read the effective theme
value and write a local theme override. Color component helpers such as
`font_color_a` preserve the other channels. These helpers do not alter modulate.
Generic `do_property_*` also recognizes these style names and their color component
paths, using the same accessors as the named constructors.

`DoRichTextLabel` exposes `normal_font_size`, `bold_font_size`, `italics_font_size`,
`bold_italics_font_size`, `mono_font_size`, `default_color`, `outline_color`, and
`outline_size`, plus color components. They write local theme overrides; explicit
BBCode formatting takes precedence. All take `(label, to, duration)` and return
typed property tweens. Rust uses the generated per-class traits. Outline size is
Godot text outline width, not Unity Outline effect-distance scaling.

`DoLabel.counter`, `DoRichTextLabel.counter`, and `DoLabel3D.counter` take
`(label, from: int, to: int, duration, thousands_separator = ",")` and return
SpireMethodInt. They interpolate an explicit integer and replace the whole text;
existing text is not parsed and RichTextLabel BBCode is replaced. Grouping is every
three digits and supports Unicode separators. An empty separator disables grouping.
Rust uses `do_counter` on the
`DoLabelCounter`, `DoRichTextLabelCounter`, or `DoLabel3DCounter` trait, with an
explicit `GString` separator. This does not provide locale-specific grouping rules.

## Named audio buses

`DoAudioServer.bus_volume_db(name: StringName, to, duration)` and
`.bus_volume_linear(...)` return SpirePropertyFloat. The existing bus name is resolved
on every update, so bus reordering does not redirect the tween. Removing or renaming
the bus cancels playback. Invalid names return null with an error. Linear gain is
unclamped: zero is silence, one is unity, and values above one amplify. The db
variant uses decibels. Mutating an audio effect's parameters uses the existing
`Spire.do_property_*` constructors on that AudioEffect resource.

Rust equivalents are `do_bus_volume_db` and `do_bus_volume_linear`, returning
`anyhow::Result` builders. They do not change mute, solo, or routing settings.

## Arrays of destinations

`Spire.do_property_array_vec2(owner, path, values: Array[Vector2], durations:
PackedFloat64Array)` and its `vec3` counterpart return SpireSequence. Easing applies
per segment; each segment reads its start when reached. Inputs are copied into
child tweens. Empty/mismatched arrays, negative/nonfinite durations, or a null
owner return null with an error before registration. Zero-duration segments are
allowed. These use ordinary sequence restart/replay behavior.

Rust's `do_property_array::<T>(owner: Gd<Object>, path: NodePath, values: Vec<T>,
durations: Vec<f64>)` returns an `anyhow::Result<SpireTween<Sequence>>` for the
supported property types. `do_get_set_array` provides typed Rust accessor closures
for the same sequence composition.

## Typed Rust accessor constructors

Generic getter/setter factories are **Rust-only**:

```rust,ignore
let read = player.clone();
let mut write = player.clone();
do_get_set(
    move || read.get_rotation() as f64,
    move |value: f64| write.set_rotation(value as f32),
    1.5,
    0.5,
).bound_to(player).register();
```

`do_get_set<T>(getter: Fn() -> T, setter: FnMut(T), to: T, duration)` builds an
ordinary property tween and defers the initial read. It supports the eight typed
property kinds: f64, i64, Vector2/2i/3/3i, Color, and GString. It accepts typed Rust
closures and converts to internal Callables; no generic getter/setter factory is
exported to GDScript.

`do_get_set_alpha(getter: Fn() -> Color, setter: FnMut(Color), to: f64, duration)`
preserves RGB on each write. `do_get_set_vec2_axis` and `do_get_set_vec3_axis` take
the vector getter/setter, a typed Vector2Axis/Vector3Axis, scalar target, and duration.
They reread the vector before writing so other axes are preserved.

Captured Node lifetimes cannot be inferred from Rust closures. Bind relevant nodes
for normal processing and keep closure captures valid when explicitly forcing
completion; use the object-specific helpers when automatic owner cancellation is
needed. The low-level `PropertyDataViaCallable` adapter validates both accessors
before processing/forced completion, including differing getter/setter targets.

## Completion and cancellation

Node-specific templates set an explicit owner, including when driven by a
sequence. A freed owner cancels playback before property access, including forced
completion. Cancellation emits `stopped`, and successful completion emits
`stopped` followed by `finished`. Explicit stop leaves the current value in place;
successful forced completion writes the endpoint. Retained Resources remain alive
until all their owning tween handles are released.
