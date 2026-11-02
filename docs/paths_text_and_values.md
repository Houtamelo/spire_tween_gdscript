# Waypoint paths, text effects, and integer margins

These templates use the existing tween pipeline. Native constructors return
unregistered `SpireTween<LerpMethodData<f64>>` builders. GDScript constructors
register and return `SpireMethodFloat`. The float is progress, normally zero to
one. Easing, delays, speed scale, bindings, manual stepping, signals, and sequence
composition work as for other method tweens. Stop never writes to the target.
Invalidation emits `stopped` without `finished`.

## Waypoint paths

```gdscript
var options := SpirePathOptions.new()
options.path_type = Spire.PATH_TYPE_CATMULL_ROM
options.resolution = 20
options.orient = true
DoNode3D.path_with_options(actor,
    PackedVector3Array([Vector3(4, 0, 0), Vector3(4, 0, -6)]), 2.0, options
).set_ease(Spire.EASE_LINEAR)
```

| Receiver | Native trait | GDScript namespace | Waypoints |
|---|---|---|---|
| Node2D | `DoPath2D` | `DoNode2D` | `Vec<Vector2>` / `PackedVector2Array` |
| Node3D | `DoPath3D` | `DoNode3D` | `Vec<Vector3>` / `PackedVector3Array` |
| Control | `DoPathControl` | `DoControl` | `Vec<Vector2>` / `PackedVector2Array` |

Native methods: `do_path(waypoints, duration, PathOptions)` and
`do_global_path(waypoints, duration, PathOptions)`.

GDScript methods: `path(node, waypoints, duration)` and
`global_path(node, waypoints, duration)` use linear defaults.
`path_with_options(node, waypoints, duration, options)` and
`global_path_with_options(node, waypoints, duration, options)` accept a typed
`SpirePathOptions`. godot-rust cannot use nullable object handles as optional
defaults, so these are separate entry points.

`path` uses local coordinates, consistent with Spire's `position` shortcut;
`global_path` uses world coordinates. The node's current position is prepended
on first processing, after any delay. Waypoints and options are copied at
construction. Local paths move with transformed parents. Control Containers may
subsequently reposition their children. Paths set transforms directly and do
not provide collision-aware physics movement.

`PathOptions` in Rust and `SpirePathOptions` in GDScript expose:

| Field | Default | Meaning |
|---|---|---|
| `path_type` | `Linear` | Linear, CatmullRom, CubicBezier; GDScript `PATH_TYPE_*`. |
| `resolution` | `10` | Samples per curved segment; clamps to 3..4096. Three samples prevent a nonconstant cubic loop from looking stationary. Linear lengths are exact. |
| `closed` | `false` | Return to captured start when necessary. |
| `lock_position` | `0` | Axis mask X=1, Y=2, Z=4; retain captured coordinates. Applied before measuring distance. |
| `lock_rotation` | `0` | Same mask for captured world Euler components. 2D uses Z. |
| `orient` | `false` | Enable orientation. |
| `mode` | `Full3D` | Ignore, Full3D, SideScroller2D (XY/Z rotation), TopDown2D (XZ/Y rotation); GDScript `PATH_MODE_*`. Both plane modes use XY for 2D receivers. |
| `look_ahead` | `0.01` | Fraction of total length sampled ahead; clamps to 0..1. Zero uses a small tangent probe. |
| `use_look_at_position` | `false` | Look at the fixed point instead of the tangent. |
| `look_at_position` | `Vector3.ZERO` | Point in constructor local/global space; 2D uses X/Y. |
| `look_at_target` | `null` | Moving Node2D/Control for 2D, Node3D for 3D. World position reread each step; overrides fixed point. |
| `forward_direction` | `Vector3.FORWARD` | Full3D model forward. 2D uses XY, defaulting to +X when XY is zero. 3D plane modes use +X for side-scrolling and -Z for top-down. |
| `up` | `Vector3.UP` | Up direction for Full3D orientation. |
| `stable_z_rotation` | `false` | Also lock world Euler Z. |

Orientation preserves scale and faces increasing waypoint order, including during
Yoyo playback. Degenerate directions or parallel up vectors retain the previous
orientation. Rotation locks follow Godot's YXZ Euler decomposition. An effective
look-at target is lifetime-bound; freeing it cancels playback. An unused target
does not bind when orientation is disabled or mode is Ignore.

### Curves and timing

Catmull-Rom uses the standard uniform cubic through the waypoints. Open endpoint
neighbors are extrapolated; closed neighbors wrap across the seam. Cubic Bézier
inputs use triples:

```text
[destination, handle_at_previous_waypoint, handle_at_destination, ...]
```

The captured start has no separate triple. Each segment evaluates
`(1-t)^3*start + 3(1-t)^2*t*handle1 + 3(1-t)*t^2*handle2 + t^3*end`.
An implicit closing edge is straight because it has no supplied handles. Supply
an explicit closing triple for a curved seam.

Curves build a chord-length table, invert distance, then evaluate the actual
cubic. Higher resolution improves speed uniformity; arc length is approximate.
Linear time is proportional to exact segment length. Empty, repeated, and
zero-length paths remain stationary and complete normally.

Restart reuses the captured path; Yoyo retraces positions. Incremental translates
copies by `end - start`; closed paths repeat in place. Eased progress outside
0..1 enters preceding/following translated copies. Force-complete uses the existing
method tween completion behavior and skips remaining loops.

Invalid duration becomes one second. Resolution, masks, and look-ahead normalize
to valid bounds. Nonfinite waypoints use the preceding point; incomplete Bézier
triples are discarded. Invalid orientation vectors use defaults, invalid fixed
points use zero, and incompatible target types are ignored. Warnings include
the rejected value.

## Text replacement and scrambling

Text effects are options on ordinary string property tweens. Existing calls to
`do_text` / `DoLabel.text` keep their plain-text behavior and property-tween
return type. BBCode handling is opt-in, including for RichTextLabel.

```gdscript
label.bbcode_enabled = true
DoRichTextLabel.text(label, "[b]ACCESS GRANTED[/b]", 1.0,
    true, Spire.SCRAMBLE_MODE_UPPERCASE)

# Configure an existing string property tween; returns the same handle.
Spire.do_property_string(label, "text", "[i]Ready[/i]", 0.5).set_text_options(true)
```

```rust
label.do_text("[b]ACCESS GRANTED[/b]".into(), 1.0)
    .with_text_options(TextOptions {
        rich_text_enabled: true,
        scramble_mode: ScrambleMode::Uppercase,
        ..Default::default()
    })
    .register();
```

| API | Configuration |
|---|---|
| Native string property tween | `with_text_options(TextOptions)`, `set_text_options(TextOptions)`, `get_text_options() -> &TextOptions` |
| GDScript class `text(node, to, duration, ...)` constructors | Optional `rich_text_enabled`, `scramble_mode`, `scramble_chars` arguments |
| `Spire.do_property_string(owner, property_path, to, duration, ...)` | Same optional arguments; returns `SpirePropertyString` |
| `SpirePropertyString.set_text_options(...)` | Same optional arguments; returns the same handle |

`TextOptions` fields are `rich_text_enabled: bool`, `scramble_mode: ScrambleMode`,
and `scramble_chars: GString`. Defaults are false, None, and empty in both
languages. Configure the receiver's `bbcode_enabled` separately. The separate
`do_text_effect` / `text_effect` APIs and their native receiver traits have been
removed. String method tweens and generic Variant interpolation remain plain text.

Without scrambling, a growing target prefix replaces source characters while
total length moves between source and target lengths. `abcd` to `WXYZ` halfway
gives `WXcd`. Characters are Unicode scalars; combined graphemes may contain
several scalars. The existing shrinking-length rounding is preserved.

Rich mode handles Godot BBCode. Paired tags carry styles independently of
character counts; slices close active tags before switching styles. `[lb]`,
`[rb]`, and self-closing Unicode control tags form one unit each. Complete image
tags and resource paths are atomic units. Quoted attributes can contain `]`.
Balanced custom effect tags are retained; the receiver must install the effect.
Unclosed/mismatched tags are literal input. Entangled markup is unsupported.

| Property mode | BBCode behavior | Scrambling |
|---|---|---|
| Absolute | Interpolates parsed characters with balanced tags | Supported |
| Relative | Appends/removes parsed characters at the end; preserves styles | Warns once and ignores scrambling |
| Speed-based | Character edits per second, excluding paired tags | Warns once and ignores scrambling |

Relative mode retains the existing length-delta semantics: equal-length samples
do not change the property. Speed-based mode charges one unit to replace, append,
or remove a character (or image); matching characters cost nothing. Formatting-only
targets apply immediately, and completed targets use their exact original markup.
Fractional edit budgets accumulate, and excess time is passed back to sequences.
Easing retains the ordinary property-tween behavior in each mode.

Unsupported scrambling does not change the requested property mode. The options
remain configured, so switching back to absolute mode enables scrambling again.
The warning is emitted once per options configuration when an unsupported mode
is processed or force-completed. BBCode handling remains active.

| ScrambleMode / GDScript `SCRAMBLE_MODE_*` | Unrevealed suffix |
|---|---|
| None | Source suffix, without randomness |
| All | ASCII uppercase, lowercase, digits |
| Uppercase | ASCII uppercase letters |
| Lowercase | ASCII lowercase letters |
| Numerals | Digits 0-9 |
| Custom | Unicode scalars from `scramble_chars` |

Scrambling displays the target's full character count between endpoints,
preserving formatting. Unrevealed units redraw each update. One-character
alphabets are valid; empty custom alphabets warn and use All. Brackets drawn in
rich mode are escaped. Random sequences are not promised to match DOTween or
earlier playback.

These remain ordinary property tweens: start capture, explicit starting values,
fixed/dynamic targets, loops, cancellation, and stop/force-complete use the same
pipeline as other properties. Restart/replay re-read the start unless explicitly
set; Yoyo reverses interpolation. String weights clamp to 0..1, so Incremental
has no extrapolating text meaning. Stop never changes the target property;
forced completion writes the target (or remaining relative delta). Cached parsed
endpoints are refreshed when the start, target, or text options change. Configure
options between updates; editing a tween while processing is unsupported.

## Integer margins

```gdscript
DoMarginContainer.margins(container, Vector4i(12, 12, 6, 6), 0.3)
```

Margins use `Vector4i(left, right, top, bottom)`. Signed components round
independently. This is not a rectangle's position/size or Control anchor offsets.

* Native `DoMargins::do_margins(to, duration)` / GDScript
  `DoMarginContainer.margins(node, to, duration)` capture effective theme constants
  on first processing and write local `margin_left/right/top/bottom` overrides.
* Native `do_property_margins(owner, path, to, duration)` / GDScript
  `Spire.do_property_margins` target any Vector4i property.
* Rust-only `do_get_set_margins(getter, setter, to, duration)` takes typed
  `Fn() -> Vector4i` / `FnMut(Vector4i)` accessors.

Reflected/theme constructors track owners. Accessor captures require explicit node
binding when accessing manually freed objects. Restart reuses the captured start;
Yoyo reverses; Incremental extrapolates with i32 saturation. Invalid types return
stopped tweens; invalid duration becomes one second. Both warn with rejected values.

## Integer values

Use the existing `i64` integer API: `Spire.do_property_int` and
`Spire.do_call_int` in GDScript, or native `do_property`, `do_method`, and
`do_get_set` with `i64` values. These support negative values and values beyond
the 32-bit integer range without adding a separate numeric family.

## Sources and tests

Independently authored from the public
[DOTween behavior documentation](https://dotween.demigiant.com/documentation.php),
standard cubic equations, and
[Godot's BBCode model](https://docs.godotengine.org/en/4.4/tutorials/ui/bbcode_in_richtextlabel.html).
No DOTween source was copied. Numerical identity for random sequences and
undocumented implementation details is not asserted.

Real-engine tests: `spire_tween_rust/src/tests/group4.rs` and
`spire_tween_gdscript/examples/tests/group4.gd`.
