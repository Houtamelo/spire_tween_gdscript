# Reuse looping text effects with BBCode

Assign `SpireRichTextEffectPreset` resources to a `SpireRichTextLabel` to animate
tagged sections of text. Each preset combines typed effect resources with
independent timing. Presets preview in the editor and coexist with ordinary
character tweens. Rendering always applies presets first and code last.

## Author a preset

1. Add a `SpireRichTextLabel`, or change an existing `RichTextLabel` to that type.
2. Enable `bbcode_enabled`.
3. Add a `SpireRichTextEffectPreset` to `effect_presets`; set `bbcode` to `enchanted`.
4. Add a `SpireTextEffect` to its `effects` array; select `kind` Offset.
   Set `from` to `(0, -6)`, `to` to `(0, 6)`, and `loop_mode` to Yoyo.
5. Set `text` to `Hello [enchanted]world[/enchanted]!`.
6. Save the preset as an external `.tres` resource to reuse it across labels.

Edits to assigned effects update their labels. Shared resources contain
configuration; each label owns its adapters and caches. The GDScript tag adapter
is embedded in the native library. Artists do not need to attach a script.

Try [the example scene](../spire_tween_gdscript/examples/text_effect_presets/preset_demo.tscn).
Its buttons punch, fade, and reset the same text through the normal code API.
The **Play shared Blueprint** button uses the same `enchanted_offset.tres`
resource as the `enchanted` preset. **Reset** stops that Blueprint too.
Edit the preset resources or their effect entries to change the authored animation.

The same `SpireTextEffect` resource also configures finite
[Blueprint character steps](tween_blueprint.md#animate-text-characters). Assign
one to a step's `text_effect`, or share the same resource with a preset.
Blueprints construct Char range tweens from its fields; presets continue sampling
it during rendering. Blueprint character delays use `text_delay_ease_source`
with `text_delay_ease_enum` or `text_delay_ease_curve`, while
`character_phase_offset` remains a preset phase setting.

When assigning an array held in a GDScript variable, declare it as
`Array[SpireRichTextEffectPreset]`. You can also append presets directly to
`label.effect_presets`. Use `Array[SpireTextEffect]` for effect-entry arrays held
in GDScript variables.

```gdscript
var effect := SpireTextEffect.new()
effect.kind = Spire.TEXT_EFFECT_KIND_OFFSET
effect.from = Vector2(0, -6)
effect.to = Vector2(0, 6)
effect.loop_mode = Spire.LOOP_MODE_YOYO
preset.effects = [effect]
```

## Effect configuration

All entries use `SpireTextEffect`. Its `kind` selects the relevant Inspector
fields and the concrete types of `from` and `to`. Changing `kind` preserves
all stored values, including hidden fields. Incompatible endpoints keep their
Variant editor and produce configuration warnings until repaired. The Inspector's
Revert action supplies the selected kind's endpoint defaults: white for Color,
0/1 for Fade, zero for Offset/Rotation, and one for Scale.

The default kind is Color with white endpoints. `punch` and `strength` use
Vector3; Offset and Scale ignore Z, including during validation.

| `kind` | Parameters in addition to timing |
|---|---|
| `Color` | `from`, `to`: fill RGBA colors; alpha also affects outlines. |
| `Fade` | `from`, `to`: alpha values; preserves incoming RGB. |
| `Offset` | `from`, `to`: additional local-pixel offsets. |
| `Rotation` | `from`, `to`: YXZ Euler vectors in radians; `rotate_mode`, `pivot`. |
| `Scale` | `from`, `to`: scale multipliers; `pivot`. |
| `PunchOffset` | Pixel `punch`; `vibrato`, `elasticity`. |
| `PunchRotation` | Euler `punch` in radians; `vibrato`, `elasticity`, `pivot`. |
| `PunchScale` | Additive scale `punch`; `vibrato`, `elasticity`, `pivot`. |
| `ShakeOffset` | Pixel `strength`; `vibrato`, `randomness`, `randomness_mode`, `fade_out`. |
| `ShakeRotation` | Euler `strength` in radians; shake parameters and `pivot`. |
| `ShakeScale` | Additive scale `strength`; shake parameters and `pivot`. |

`pivot` is a pixel offset from the shaped cluster's glyph pen origin. Rotation
uses the [character API's](character_animation.md) rotation modes and orthographic
projection. Negative scale values reflect the glyph.

Every entry has independent timing:

| Field | Meaning |
|---|---|
| `duration` | Seconds per loop leg; finite and positive. Defaults to 1. |
| `ease` | Spire easing function. Defaults to Linear. |
| `loop_mode` | Restart, Yoyo, or Incremental. Defaults to Restart. |
| `character_phase_offset` | Seconds of phase advancement per logical character within this tag. Defaults to 0; may be negative. |

## Loop sampling

Sample time is `elapsed_time + relative_index * character_phase_offset`. Logical
spaces count toward the phase. Inseparable shaped clusters remain coherent;
outline and fill use the same transformation.

- **Restart:** repeats `from` to `to`; an exact loop boundary samples `from`.
- **Yoyo:** alternates forward and backward legs. One complete cycle takes twice
  `duration`. The first-leg boundary samples `to`. Easing is evaluated at the
  reversed position during a backward leg, matching existing character tweens.
- **Incremental:** basic effects add another `to - from` each leg. Punch and shake
  repeat their periodic paths, matching existing effect adapters.

These effects have no runtime tween handles or completion signals. Large frame
deltas sample the current phase directly. Ordinary tweens retain their existing
loop processing contract. Easing overshoot and Incremental loops may produce
values beyond configured endpoints.

Punch and shake reuse Spire's existing samplers. Shake generates a path on first
use for each entry/character range and repeats it. Sampling consumes no randomness;
repeated drawing and outline/fill callbacks agree. Rebuilding a registration
creates fresh shake paths.

## Combine presets and code

```gdscript
@onready var label: SpireRichTextLabel = %Dialogue

func emphasize() -> void:
    # The assigned [enchanted] preset keeps running.
    label.punch_char_offset(0, Vector2(0, -16), 0.6)
```

Godot evaluates nested tags from the inner tag toward its parents. Entries within
a preset run in array order. Code-controlled state runs last. Each stage performs
its normal operation: transforms compose, color writes RGBA, and fade writes
alpha. There are no priority or blending settings.

Preset rendering does not write the combined result into code-controlled state.
Transform getters report the code layer's stored values. Color getters retain
their fallback to the last drawn baseline. Wait for the first draw when a code
color/fade tween needs to capture the preset's current appearance. `reset()` clears
code-controlled deformations; configured presets continue rendering.

## Changes and persistence

Godot's effect clock follows the label's processing settings. The addon does not
override `process_mode`. Reflow preserves phase when content is not rebuilt.

Changes to source text, translation, BBCode mode, preset registration, or effect
configuration rebuild the rendering stack and restart its effect clocks. A
preset-only change preserves existing code-controlled character state. Source
changes keep the character API's existing reset/index behavior.

At runtime, generated adapters remain installed in `custom_effects`, after
user-authored effects. Refreshing or editing presets replaces the managed
registrations without adding duplicates. Duplicated or instantiated labels
replace copied adapters with their own instances.

In the editor, adapters are registered temporarily while parsing, then the label
restores its authored `custom_effects` list. Editor saves exclude generated
adapters and scripts. This editor preview behavior relies on Godot retaining the
parsed effect scopes when the array is restored.

GDScript field assignments, Inspector edits, and Undo/Redo emit Resource change
notifications. In-place additions/removals in `effects` and `effect_presets` are
also detected. Rust callers can use typed `set_*` methods when changing an assigned
effect. Direct Rust field assignments require `emit_changed()` afterward.

## Invalid configuration

`bbcode` must start with an ASCII letter or underscore and contain only ASCII
letters, digits, and underscores. Built-in tags are reserved. User-installed
custom effects take precedence over colliding presets; otherwise the first preset
with a tag wins. `effect_presets` is an `Array[SpireRichTextEffectPreset]`; Godot
rejects other resource types, as does the `Array[SpireTextEffect]` stored in
`effects`. Invalid effect configurations are skipped. Null array slots are
allowed while authoring.

Active numeric parameters must be finite; hidden settings do not affect validation.
`duration` and `vibrato` must be positive;
`floor(vibrato * duration)` must not exceed 65536. `elasticity` must be in 0..1,
`randomness` in 0..180, and shake `strength` components must be non-negative.
Editor warnings appear on the label. Runtime warnings name the exact exported
field and occur on configuration changes rather than each glyph callback.

Supply content through `text`. With `bbcode_enabled` off, tags remain literal.
Manual formatting-stack edits retain the character API's existing limitations.
The existing warning about scripts overriding `_process()` also applies: that
override disables automatic configuration refresh.

## Validation

Run `bash scripts/test_text_effect_presets.sh /path/to/godot /path/to/evidence`.
It builds an instrumented library and uses an isolated virtual display to check
rendering, Resource authoring, Inspector edits, Undo/Redo, and persistence. Set
`SPIRE_PRESET_FULL_SUITE=1` to also run the GDScript regression suite. Test sampling
methods are absent from production builds.
