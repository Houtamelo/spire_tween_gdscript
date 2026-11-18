# Reusable looping effects for rich text

Status: **implemented** following the user's implementation authorization on
2026-11-07. This file retains its original draft filename. The
[authoring guide](../rich_text_effect_presets.md) documents the shipped types,
fields, timing semantics, and example scene.

Provide reusable Resource presets that artists assign to `SpireRichTextLabel`
and apply to sections of text using BBCode. Each preset combines independently
configured looping effects. Existing character animation methods remain usable,
including on characters affected by a preset.

## Artist workflow

1. Create a `SpireRichTextEffectPreset` resource and set `bbcode`, for example
   `enchanted`.
2. Add effect-entry resources to its `effects` array. For example, combine an
   offset effect with a color effect.
3. Configure each entry's duration, easing, loop behavior, and effect parameters.
4. Assign the preset to the label's `effect_presets` array and enable
   the label's existing `bbcode_enabled` property.
5. Write `Hello [enchanted]world[/enchanted]!` in the label's `text`.
6. Save the preset as a `.tres` resource to reuse it across labels and scenes.

Artists do not need to author scripts for supplied effects. The addon supplies
the small GDScript adapter needed for custom tag registration.

## Agreed decisions

| Topic | Decision |
|---|---|
| Purpose | Presets provide looping text effects. |
| Selection | A preset's `bbcode` identifies tagged sections of a label's text. |
| Composition | One preset can combine multiple effects. |
| Authoring types | Each effect has a separate Resource type; do not use one large Resource with an effect enum and unrelated hidden fields. |
| Timing | Every effect entry has independent duration, easing, and loop behavior. |
| Evaluation | Evaluate preset contributions directly during `_process_custom_fx()`, reusing Spire's easing and effect mathematics. Presets do not create ordinary runtime tweens. |
| Engine adapter | Supply a GDScript subclass with a `bbcode` variable. Configuration and effect evaluation remain in Rust. |
| Code authoring | Preserve the existing character methods and their ordinary tween handles. Code and presets can affect the same label and characters. |
| State separation | Preset evaluation does not overwrite the character state controlled by the code API. Combine their contributions during rendering. |
| Rendering order | Apply presets first, then apply code-controlled character state last, deterministically. |

Direct evaluation is a distinct playback model. Existing runtime tweens retain
their current loop-boundary behavior, including completing at most one loop per
processing update. This feature does not change that runtime contract.

## Resources and ownership

The implementation uses the following ownership layout.

| Object | Authored data | Runtime responsibility |
|---|---|---|
| `SpireRichTextEffectPreset` | `bbcode: GString`; ordered collection of effect-entry resources. | Describe a reusable tagged appearance. |
| Effect-entry resources | Effect-specific parameters and independent timing fields. | Describe one contribution; evaluation uses shared Rust mathematics. |
| `SpireRichTextLabel` | `effect_presets` array. | Own preset registration and change subscriptions, rebuild content when required, and retain the character rendering effect. |
| Internal `SpireRichTextEffect` adapter | None (generated from the preset). | Store the tag and compiled entries per label; evaluate their combined contribution for the current glyph. |
| Internal `SpireCharRenderer` | None. | Own code-controlled character state and apply it after preset effects. |
| Label-bound method tweens | None. | Update character state through the label, request redraws, and follow the label's lifetime and processing settings. |

Presets and effect-entry resources contain configuration. Do not store label
references, mutable playback clocks, character state, or mutable random generators
inside shared authoring resources. Two labels can reference the same preset
without sharing playback state.

The same effect-entry resources also configure `TweenBlueprint.character_effect`.
Each Blueprint play validates and copies the resource fields into ordinary
Char range tweens; range selection, start-delay curves, and finite loop counts stay on the
Blueprint. Preset rendering and Blueprint playback share resource compilation.

Create an adapter per label/preset registration. Separate occurrences of a tag
use the rendering context supplied by Godot; do not place occurrence-specific
state in a shared preset. Any compiled sampling data belongs to the adapter or
an appropriate label-owned cache.

Each effect is a concrete Rust class inheriting Godot's `Resource`. A macro
provides the shared timing fields and change-notifying setters. A resource-filtered
array exposes the supported entry types without requiring Rust extension classes
to inherit from one another. Real Inspector and save/reload checks verify it.

### Effect catalogue

Use the eleven effect families already supported by the character API. Each row
below represents a separate Resource type. Exact class names are listed in the
[authoring guide](../rich_text_effect_presets.md#effect-resources).

| Entry | Parameters in addition to timing |
|---|---|
| Color | `from: Color`, `to: Color`. |
| Fade | `from: float`, `to: float`; preserve RGB. |
| Offset | `from: Vector2`, `to: Vector2`, in local pixels. |
| Rotation | `from: Vector3`, `to: Vector3`, in radians; rotation mode and pivot. |
| Scale | `from: Vector2`, `to: Vector2`; pivot. |
| Punch offset | Punch vector, vibrato, elasticity. |
| Punch rotation | Punch Euler vector, vibrato, elasticity; pivot. |
| Punch scale | Punch scale delta, vibrato, elasticity; pivot. |
| Shake offset | Strength, vibrato, randomness, randomness mode, fade-out behavior. |
| Shake rotation | Corresponding rotation shake parameters; pivot. |
| Shake scale | Corresponding scale shake parameters; pivot. |

Explicit `from`/`to` values describe the preset's own contribution. They do not
capture and subsequently mutate the state controlled by code. Color writes RGBA;
fade writes alpha and preserves incoming RGB, following the rendering order below.

The existing API's blendable flag is not automatically a preset field. Presets
apply a freshly sampled contribution during drawing; they do not accumulate
successive writes into a target property. Effect layering needs its own explicit
rules rather than an unused copy of a runtime-tween option.

## Evaluation and timing

Each entry samples the rendering context and its own configuration. Transform
and color assembly must be shared with existing character rendering where
practical; avoid a second implementation of the same projection or effect math.

Godot supplies elapsed time, absolute character ranges, and an index relative to
the current custom-effect block. Use the block-relative index for character
phases so the start of a tagged section does not depend on preceding text.
Characters belonging to an inseparable shaped cluster must remain visually
coherent, including across outline and fill callbacks.

Timing follows these rules:

- `duration` is finite and strictly positive. Each entry has an independent
  `ease` and `loop_mode`.
- Provide a finite, signed `character_phase_offset` in seconds. Zero keeps
  characters synchronized; nonzero values create a travelling phase. This is a
  recurring phase offset, not a new runtime tween per character.
- Expose the existing Restart, Yoyo, and Incremental modes. Incremental advances
  basic from/to values by the original delta; punch/shake repeat their paths,
  matching existing effect-adapter semantics.
- `duration` denotes one leg. Restart samples the beginning at an exact boundary;
  Yoyo samples the end at the first boundary and reverses the position supplied
  to easing. Large time differences sample the current appearance directly;
  there is no catch-up loop or tween completion event.

Use Godot's effect time so playback follows the label's processing settings.
Do not override `process_mode`. Godot resets effect time when content is rebuilt;
preserving phase across a content rebuild is not implied by this architecture.
Resizing/reflow should preserve phase when it does not require rebuilding content.

Punch and shake must reuse Spire's independently authored mathematics. Shake
sampling must not draw new random values on every glyph, outline, or fill
callback. A generated path must give stable samples for the same occurrence,
character/cluster, entry, and playback phase. Adapters retain paths per entry and
absolute character-range start; rebuilding registrations creates fresh paths.

## Coexistence and composition

Keep code-authored character state independent of preset evaluation. A label
with `[bob]Hello[/bob]` can continue evaluating its bobbing preset while a normal
`scale_char(0, ...)` tween enlarges the first character. The code tween retains
its existing controls, lifecycle checks, and signals.

Every drawing pass starts from Godot's incoming glyph properties. Combine sampled
preset contributions and stored code contributions without writing the rendered
result back into the code-controlled state. This prevents feedback accumulation
and prevents stopping a code tween from disabling a preset.

Current implementation facts relevant to the design:

- The character renderer multiplies its transform onto the incoming transform.
- Explicit code-controlled color and alpha currently replace the incoming values
  for their respective channels; they are retained until reset or overwritten.
- Godot 4.4 gathers nested effects from the inner item toward its parents and
  passes transform/color results from one effect to the next. The current code
  effect encloses the label's complete content.

The agreed rendering order is **presets first, code last**:

1. Evaluate tagged presets using Godot's nested-tag order. Apply entries within
   each preset in their authored array order.
2. Apply the stored code-controlled character state to the resulting glyph.
3. Draw the glyph.

Each stage receives the previous stage's result and performs its normal
operation. Transforms compose; color entries write RGBA; fade entries write
alpha while preserving RGB. The code layer runs last and retains its existing
transform and color/alpha operations. Ordinary tween updates keep their current
processing behavior; this contract specifies how their stored values are applied
during drawing.

Use this fixed order consistently. Do not add blend-policy settings, per-channel
priority rules, or conflict-resolution machinery. Last application determines
the result according to the operation being applied.

Preserve existing getter semantics. Transform getters report the code layer's
stored values; color getters retain their existing fallback to the last drawn
baseline. Preset integration must avoid applying color/alpha twice when a code
tween captures its starting value.

## Registration, editing, and validation

Integration behavior:

- Register supported presets before rebuilding/parsing label content. Merely
  installing tags after the text was parsed is insufficient.
- Preserve the `text` and `bbcode_enabled` properties. With BBCode disabled, tags
  remain literal text. Continue using Godot for translation and text shaping.
- Preserve user-installed custom effects. At runtime, keep generated adapters in
  `custom_effects` without duplicates, replacing stale adapters when configuration
  changes. In the editor, restore the authored array after parsing so generated
  adapters are not serialized. That editor preview behavior relies on Godot
  retaining the parsed effect scopes after restoration.
- Watch assigned presets and entries for Resource change signals. Changing an
  entry refreshes every label that references it. Snapshot comparisons also detect
  array membership changes. Connections and caches follow each label's lifetime.
- Rebuild registration when presets or their `bbcode` fields change. Do not rebuild
  text on every animation sample.
- Provide live editor rendering of configured presets. Keep authored resources
  separate from transient adapter/cached state so inspection and preview do not
  serialize playback state or dirty scenes each frame.
- Report invalid tags, duplicate tags, unsupported entry resources, invalid
  durations, and nonfinite parameters with the exact exported property names.
  Skip invalid entries and emit diagnostics on configuration changes rather than
  once per glyph. Reserve built-in tags and let installed custom effects take
  precedence. Otherwise the first preset with a given tag wins. Permit null array
  slots while authoring.

The adapter script is bundled with `include_str!` in the native library and
compiled through Godot's GDScript API. Distribution requires no extra loose script.

## Required verification

Implementation must include real Godot integration tests on the supported engine
versions, plus rendered and Inspector validation where the behavior is visual.

| Area | Required evidence |
|---|---|
| Resource authoring | Create every entry type in the Inspector; save/reload embedded and external resources; retain all values and ordering. |
| Tag registration | Two presets, repeated and nested tags, enabled/disabled BBCode, and existing custom effects render as specified. Invalid/duplicate tags follow the selected diagnostic policy. |
| Independent timing | Entries in one preset visibly use different durations, easing, and loop behavior. Boundary and large-step samples match the selected preset sampling contract. |
| Resource sharing | Two labels use one preset without sharing mutable playback state; edits update both; replacing/freeing one leaves the other valid. |
| Code coexistence | Real character tweens run on tagged glyphs, including overlapping geometry, color, and alpha. Verify deterministic preset-first/code-last rendering across repeated frames, along with getters, stop, completion, replay, and reset. |
| Text/layout lifecycle | Text edits, translation, resize, clipping, missing character indices, shaped clusters, and outline/fill callbacks remain coherent. |
| Processing | Pause/resume follows label settings; preset evaluation does not alter existing runtime tween timing. |
| Shake and punch | Samples agree with existing math; repeated drawing is stable; selected path regeneration and loop behavior are covered. |
| Editor and distribution | Configure and view effects in the real editor, reopen the scene, and load a packaged addon containing the adapter script. |

## Implementation entry points

- [Resource types and setters](../../spire_tween/src/tweens/templates/text_animator/presets/resources.rs).
- [Registration](../../spire_tween/src/tweens/templates/text_animator/presets/registry.rs).
- [Native preset renderer](../../spire_tween/src/tweens/templates/text_animator/presets/mod.rs).
- [Typed compilation and validation](../../spire_tween/src/tweens/templates/text_animator/presets/compile.rs).
- [Sampling](../../spire_tween/src/tweens/templates/text_animator/presets/sample.rs).
- [Integration driver](../../scripts/test_text_effect_presets.sh).

## Implementation references

- [Existing character API and lifecycle](../character_animation.md).
- [Label state and content rebuilding](../../spire_tween/src/tweens/templates/text_animator/gd.rs).
- [Existing drawing callback](../../spire_tween/src/tweens/templates/text_animator/render.rs).
- [Character tween constructors](../../spire_tween/src/tweens/templates/text_animator/tween.rs).
- [Punch and related mathematical contracts](group3_math.md).
- [Shake sampling contract](shake_sampler_math.md).
- [Godot 4.4 custom tag lookup](https://github.com/godotengine/godot/blob/4.4-stable/scene/gui/rich_text_effect.cpp): `RichTextEffect::get_bbcode()` reads the script instance's `bbcode` property.
- [Godot 4.7 custom tag lookup](https://github.com/godotengine/godot/blob/4.7-stable/scene/gui/rich_text_effect.cpp): the same script-instance requirement applies.
- [Godot 4.4 effect order and drawing](https://github.com/godotengine/godot/blob/4.4-stable/scene/gui/rich_text_label.cpp): `_fetch_item_fx_stack()` and `_draw_line()`.
- [Godot effect context and time](https://docs.godotengine.org/en/stable/classes/class_charfxtransform.html).
