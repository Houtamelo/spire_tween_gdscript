# Animate individual SpireRichTextLabel characters

`SpireRichTextLabel` extends Godot's `RichTextLabel` with animation of individual
logical character indices. Rust callers use the `SpireDoRichTextCharacters` extension trait on the label or its `Gd` handle.
For reusable looping effects selected with BBCode tags, see
[rich-text effect presets](rich_text_effect_presets.md).
They support fade, color, offset, three-axis rotation,
scale, and punch/shake of offset, rotation and scale. Each constructor uses an
ordinary Spire method-progress tween, including easing, delays, loops, manual
stepping, node lifetime checks, signals and sequence composition.

For a rendered showcase of all eleven constructors, run
`bash scripts/render_text_characters.sh` from the repository root. The
[render-test guide](../spire_tween_gdscript/examples/render_tests/README.md)
describes its GIF output, pixel/state assertions and isolated virtual display.

## Quick start

```gdscript
@onready var label: SpireRichTextLabel = %Dialogue

func animate() -> void:
    label.bbcode_enabled = true
    label.text = "[color=gold]Hello[/color]"

    # Drawing supplies the effective BBCode color and shaping clusters.
    await label.draw

    var sequence := Spire.sequence()
    sequence.set_default_children_ease(Spire.EASE_OUT_CUBIC)
    for index in label.get_character_count():
        label.set_char_offset(index, Vector2(0, -20))
        var tween := label.offset_char(index, Vector2.ZERO, 0.4)
        tween.set_delay(index * 0.04)
        sequence.join(tween)

    # Angles are radians. X/Y tilts use orthographic projection.
    label.rotate_char(0, Vector3(0, PI / 3, 0), 0.5)
```

```rust
use godot::prelude::*;
use spire_tween::prelude::*;

// label: Gd<SpireRichTextLabel>
label
    .do_offset_char(0, Vector2::new(0.0, -20.0), 0.4)
    .with_ease(EaseKind::Basic(Ease::OutCubic))
    .register();
```

Native constructors return unregistered `SpireTween<LerpMethodData<f64>>`
builders. GDScript constructors register and return `SpireMethodFloat`. That
float is tween progress, not the character's current property value. Native state
accessors are associated functions, for example
`SpireRichTextLabel::get_char_offset(label.clone(), 0)`.

## Node type and ownership

Add a `SpireRichTextLabel` directly, or right-click an existing `RichTextLabel`
in the Scene dock and choose **Change Type → SpireRichTextLabel**. Existing
`RichTextLabel` properties, themes, layout and ordinary `DoRichTextLabel` tweens
remain available. GDScript inheritors use `extends SpireRichTextLabel`.

The label owns an internal `SpireCharRenderer` Resource containing its character
state and creates no helper nodes.
`Node.duplicate()` and scene instantiation create fresh, independent animation
state; active tweens remain bound to the original label. Character buffers are populated at runtime ready, during automatic processing,
or by an explicit `refresh()`. Accessing the lazy effect Resource alone does not
populate those buffers.
The label maintains a `RichTextEffect` scope through `push_customfx()`;
callers do not add effect tags. The label's `text` and `bbcode_enabled` properties
remain unchanged, including literal brackets when BBCode is disabled. Godot
continues to handle wrapping, fonts, BBCode and bidirectional layout.

Supply content through `RichTextLabel.text`, including ordinary Spire text
tweens. Manual `push_*`, `add_text`, `append_text` and `clear` formatting-stack
edits are not tracked; the label manages that stack. Rebuilding can also reset
selection and scroll state, as a normal text assignment does.

Freeing the label invalidates its character tweens through their linked callables.
The label owns its rendering effect; tweens update that effect through the label.
Preset registration and change subscriptions also belong to the label and are
released when it is freed.

Godot continuously processes custom rich-text effects while their scope remains
in the label, including between tweens. The label checks source changes once per
processing frame. Character buffers are reused across refreshes; the
production draw callback does not allocate a per-glyph buffer.

`process_mode` stays under user control. Normal bound tweens pause when the label
cannot process, including during a SceneTree pause, and resume afterward.
Automatic text refresh follows the label's processing settings too.

The editor warns if an attached script, including a script ancestor, implements
`_process()`: this overrides the native automatic-refresh callback.
`super._process()` cannot invoke the Rust implementation. Put custom per-frame
logic on another node. Labels with assigned effect presets render those loops in
the editor. Labels without presets do not initialize animation state or change
the rendering stack merely through inspection.

## Indices, refresh and reset

Indices use Godot's `get_total_character_count()` and logical parsed text order,
excluding BBCode syntax.
Spaces, newlines and inline images occupy indices. Clipping, scrolling and
`visible_characters` do not remove indices. Invisible indices remain valid, even
though they may have no glyph on which to display an effect.

Godot's draw callback identifies shaped clusters. Logical characters belonging
to one inseparable cluster, such as a base letter and combining mark, share
animation channels once shaping is observed. Writes made before the first draw
are merged by channel in write order. This is not visual glyph numbering or a
new grapheme-segmentation algorithm.

| Operation | Behavior |
|---|---|
| Change text, BBCode setting or translation | Detected during the label's next processing callback; rebuild character data and reset deformations then. |
| Resize or reflow | Preserve character state. Godot supplies the new layout. |
| `refresh()` | Explicitly rebuild and reset, even if the source is unchanged. |
| `reset()` | Reset deformations without rebuilding text. |

Character getters, setters, tween updates, and rendering observers use the last
refreshed character buffer. They do not check the text for changes or rebuild
the rendering stack. Call `refresh()` before these operations when current state
is needed synchronously, including before the label enters the tree, immediately
after editing text, and before manual stepping or forced completion.

```gdscript
label.text = "New text"
label.refresh()
label.set_char_offset(0, Vector2(0, -20))
```

`reset()` only resets existing character data; it does not refresh changed text.

Refresh/reset do not cancel or restart active tweens. Surviving numeric indices
continue at their existing playback positions, even if they now contain different
characters. Writes to an index which no longer exists are ignored while the
tween's clock continues. It still completes normally and emits `finished`; if the same numeric
index reappears before completion, writes resume. Invalid constructor indices
also produce ordinary playing tweens whose missing-target writes do nothing.
Indices are never clamped or redirected to another character.

Punch/shake discard their previous contribution bookkeeping after a reset so
they cannot subtract an offset which the reset has already removed. Their clocks
continue. A new shake path is generated when that effect next samples rebuilt
state.

## Constructors

Rust names have a `do_` prefix; GDScript names omit it. All constructors begin
with `index: i64` / `int`. Basic constructors then take `to, duration`.

| Rust constructor | Target | Meaning |
|---|---|---|
| `do_fade_char` | `f64` / `float` | Alpha target in 0..1; preserves RGB. |
| `do_color_char` | `Color` | Fill RGBA; RGB allows HDR, target alpha clamps to 0..1. |
| `do_offset_char` | `Vector2` | Additional offset in label-local pixels. |
| `do_rotate_char` | `Vector3` | YXZ Euler angles in radians; also accepts `RotateMode`. |
| `do_scale_char` | `Vector2` | Per-axis scale multipliers; negative values mirror. |

Starting values are captured on first processing and retained for replay. Before
the first draw, color reads use the theme default. **Wait for the label's first
draw before starting color/fade tweens which must capture BBCode or other effect
colors.** Other channels do not require waiting. The baseline is the color passed
to our effect by Godot, including preceding effects, rather than an immutable
copy of the original markup color.
Glyphs which the engine has not visited with the effect still use the theme
fallback; awaiting the label's draw signal initializes the glyphs it draws.

Rotation modes match Spire's existing conventions: `Fast` takes the shortest
per-axis delta, `FastBeyond360` preserves turns, and `WorldAxisAdd` / `LocalAxisAdd`
compose before / after the captured orientation. GDScript defaults to
`Spire.ROTATE_MODE_FAST`. The projected glyph remains a flat 2D drawing: there is
no perspective, depth sorting or 3D lighting.

| Rust effect constructors | Strength |
|---|---|
| `do_punch_char_offset`, `do_shake_char_offset` | `Vector2`, pixel offsets |
| `do_punch_char_rotation`, `do_shake_char_rotation` | `Vector3`, radians |
| `do_punch_char_scale`, `do_shake_char_scale` | `Vector2`, additive scale deltas |

Punch arguments after the index are `punch, duration, vibrato, elasticity,
blendable`. GDScript defaults are `vibrato=10`, `elasticity=1.0`, `blendable=true`.
Shake arguments are `duration, strength, vibrato, randomness, fade_out,
randomness_mode, blendable`; defaults are `10`, `90.0`, `true`,
`Spire.SHAKE_RANDOMNESS_MODE_FULL`, `true`. Rust callers supply all arguments.
These reuse Spire's independently authored punch math and interpolated waypoint
shake sampler, including their existing looping and restoration behavior.

`stop()` preserves the last deformation. Successful completion removes the
punch/shake contribution. `force_complete()` follows the ordinary method-tween
contract and does nothing while stopped. Blendable effects replace only their
own additive contribution; competing absolute tweens use processing order.

Invalid duration becomes one second. Nonfinite target components use safe
defaults. Alpha/elasticity clamp to 0..1; randomness clamps to 0..180 degrees;
shake strengths are nonnegative. Warnings include rejected values.

## State access and pivots

| API in both languages | Value |
|---|---|
| `get_char_offset(index)`, `set_char_offset(index, value)` | `Vector2` pixels |
| `get_char_rotation(index)`, `set_char_rotation(index, value)` | `Vector3` radians |
| `get_char_scale(index)`, `set_char_scale(index, value)` | `Vector2` multipliers |
| `get_char_color(index)`, `set_char_color(index, value)` | Fill `Color` |
| `get_char_alpha(index)`, `set_char_alpha(index, value)` | Alpha only |
| `get_char_pivot(index)`, `set_char_pivot(index, value)` | `Vector2` pixel pivot offset |
| `is_valid_char(index)` | Whether the logical index exists |
| Rust `character_count()` / GDScript `get_character_count()` | Logical character count |

Invalid getter indices return zero offset/rotation/pivot, unit scale, white
color and alpha one. Invalid setters warn and do nothing.

Targets and direct setters normalize alpha to 0..1. Values produced by easing
or Incremental playback may overshoot, as with ordinary Spire color tweens;
those computed values are not treated as invalid caller input.

Rotation and scale default to the shaped cluster's glyph pen origin. Set a pivot
offset to rotate or scale around another point in its local pixel coordinates.
For pivot `p`, basis `A` and offset `o`, the additional transform is
`A * position + o + p - A * p`.

Godot 4.4's effect API does not expose the effective font size or glyph bounds
needed to calculate TMP's exact automatic center pivot. No approximate font
metrics are substituted. Automatic centering and perspective are potential
future rendering extensions. SpireRichTextLabel is the supported receiver;
Label/Label3D require a separate rendering integration.

## Provenance and validation

Independently authored against the public
[DOTween character-animation behavior](https://dotween.demigiant.com/documentation.php#textMeshProPerCharacter)
and Godot's [RichTextEffect](https://docs.godotengine.org/en/4.4/classes/class_richtexteffect.html),
[CharFXTransform](https://docs.godotengine.org/en/4.4/classes/class_charfxtransform.html)
and [RichTextLabel](https://docs.godotengine.org/en/4.4/classes/class_richtextlabel.html)
APIs. No DOTween source was copied. Missing-index no-op behavior and the documented
Godot rendering boundaries are explicit Spire contracts.

Real-engine fixtures are in `spire_tween_rust/src/tests/text_animator.rs` and
`spire_tween_gdscript/examples/tests/text_animator.gd`. Native rendering fixtures
record actual effect callbacks; they do not invoke synthetic glyph callbacks.
