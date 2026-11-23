# Ready-to-use effect scenes

Instance a template scene beneath the node to animate, edit its root controls,
and connect a signal to `play()`. The templates are GDScript custom
`TweenBlueprint` builders. Their animations use Spire's native tweens; no user
script or Rust compilation is required when using an installed addon.

Try `res://examples/effect_templates/gallery.tscn` in the source example project.
Its 33 editable template instances have **Play / Restart** and **Stop & Restore**
buttons. The gallery's visual children sit beneath layout wrappers so their
positions and scales are not controlled by a Container.

## Use a template

1. Drag a `.tscn` from this folder beneath the target in the Scene dock.
2. Select the template root and edit its parameters. Set `target` explicitly
   when its first non-Blueprint ancestor is not the desired target.
3. Leave `trigger_condition` Manual and connect the desired signal to `play()`
   using Godot's Signals dock, or use an existing automatic trigger.
4. Use **Play Animation Preview** and **Stop Animation Preview** in the root's
   Inspector to test without running the scene. Completion, saving, or switching
   scenes restores the rest state. Preview the outermost Blueprint when composed.
5. Run the scene. `stop()` preserves the current pose; `restore()` stops playback
   and restores the channels captured by this template's last run.
6. Duplicate or save an inherited scene to author your own variation. Parameters
   stay on the instance root; normal use does not require editable children.

General visual templates support `Control` and `Node2D`. Text templates require
`SpireRichTextLabel`. For a Control positioned by a Container, animate a visual
child beneath a layout wrapper. Set that visual child's `pivot_offset` in the
editor for centered whole-target scale/rotation. The templates do not change
the target's layout, visibility flag, process mode, or pivot automatically.

For text, let the label process and draw before playback when capturing BBCode
colors. A Button-triggered run naturally does this. An immediate automatic
trigger needs a short `delay` allowing the first draw. After changing text,
allow the normal refresh/draw before playing; `refresh()` explicitly resets
all code-controlled character state.

## Catalog

Each file below is relative to this directory. Entrance scenes expose `exiting`
for the corresponding exit. Direction, intensity, colors, and rotation are
parameters where relevant.

| Directory | Scenes |
|---|---|
| `text_entrances/` | `typewriter`, `fade`, `slide`, `pop`, `bounce`, `flip`, `scatter` |
| `text_continuous/` | `wave`, `bob`, `breathing`, `wobble`, `trembling`, `color_cycle`, `alpha_pulse` |
| `text_emphasis/` | `punch_offset`, `punch_rotation`, `punch_scale`, `shake`, `color_flash` |
| `canvas_entrances/` | `fade`, `slide`, `pop`, `bounce`, `flip`, `spin` |
| `interaction/` | `hover_focus`, `press`, `selection` |
| `canvas_continuous/` | `pulse`, `float`, `sway`, `spin`, `color_cycle` |

`catalog.json` supplies titles, descriptions, paths, and looping flags for
examples and integration checks. Runtime templates do not load that catalog.

### Text entrances and exits

| Scene | Behavior and controls |
|---|---|
| Typewriter | Reveal or hide each character at the end of its local leg. Default `duration` is 0.045 seconds per character; default delays space characters one leg apart. |
| Fade | Animate selected character alpha from 0 to 1, or 1 to 0. |
| Slide | Combine alpha with movement from/to `offset` in pixels. |
| Pop | Combine alpha with a scale overshoot. |
| Bounce | Combine alpha with repeated scale landings. |
| Flip | Combine alpha with projected glyph `rotation`, an Euler vector in radians. |
| Scatter / assemble | Seeded per-character offsets and rotations; `offset` bounds displacement and `rotation` bounds Euler rotation. `scatter_seed` makes the arrangement reproducible. |

Entrance/exit templates author canonical character states: offset/rotation zero,
scale one, and alpha one are the visible endpoints. They preserve channels they
do not animate. `restore()` recovers their captured channel values. Rotation and
scale templates expose a glyph `pivot` in pixels relative to its pen origin.
Whole-target Flip is a 2D squash along `flip_axis`; text Flip supports the glyph
renderer’s projected X/Y/Z rotation.

Pop and Bounce use native `Ease` variants: `InPop`, `OutPop`, `InOutPop`,
`InBounce`, `OutBounce`, and `InOutBounce`. These are also available through
ordinary tween `set_ease()` calls, Blueprint easing fields, and text effect
resources. Pop anticipates/overshoots; Bounce stays within 0..1 for progress
within 0..1. The templates select Out for entrances and In for exits. Typewriter
keeps its discrete GDScript reveal function.

### Continuous effects

Continuous scenes keep playing until stopped. `duration` is seconds per leg;
a forward-and-back cycle takes twice that duration. Spin takes `duration` per
full turn and keeps rotating in the direction of `angle`.

Text Wave applies phase-delayed offset oscillation. Bob moves the selection
together. Breathing changes glyph scale; Wobble changes glyph rotation.
Trembling continuously shakes glyph offsets. Color Cycle transitions between
`color_from` and `color_to`; Alpha Pulse oscillates from `minimum_alpha` to one.
These are code-controlled character templates. Tagged BBCode effects continue
to use the separate `SpireRichTextEffectPreset` workflow and compose beneath
this code-controlled layer.

Restoring a color/alpha channel writes its captured value. To hand that channel
back to a changing BBCode preset, the label's `reset()` clears the code layer;
that also clears other code-controlled channels on that label.
Editor preview uses a separate exact-state capture, preserving inherited colors
and channel overrides when it restores.

Whole-target Pulse multiplies baseline scale by `Vector2.ONE + amplitude`;
Float adds pixel `amplitude` to position; Sway oscillates around baseline
rotation by `angle` radians. Color Cycle multiplies baseline modulation by the
configured colors. Whole-target scale and color preserve the target's baseline.

### Emphasis and interaction

Punch Offset, Rotation, and Scale apply restoring native impulses, controlled
by `strength`, `vibrato`, and `elasticity`. Offset and Scale use X/Y of
`strength`; Rotation uses all three radians components. Shake exposes an
offset/rotation/scale `shake_channel` and decays over its leg. Color Flash
returns each character to its own captured color, including BBCode colors.

Hover/Focus holds a scale-and-tint emphasis after its transition. Connect
mouse/focus entry to `play()` and the corresponding exit to `restore()`.
Repeated entry does not compound the enlargement. When combining mouse and
keyboard focus, the containing UI decides whether either input still warrants
emphasis. Press is a short squash-and-recovery; Selection enlarges and springs
back. Their `duration` covers both legs and `scale_multiplier`/`tint` configure
the emphasized pose.

## Timing, composition, and restoration

For text, **`duration` is each character's duration per leg**, not the complete
range duration. `start_delay_curve` samples multiply `duration`. A linear 0-to-1
curve starts the final character when the first character ends its first leg;
a flat zero curve starts them together. Samples must be finite and non-negative
and may exceed one. A single character samples at zero. An unassigned curve
uses the template's defaults: linear entrances, a per-character-spacing ramp
for Typewriter, traveling delays for Wave/Wobble/Color Cycle, and zero delays
for Bob/Breathing/Trembling/Alpha Pulse/emphasis. Assign a curve to override.
`first_character`/`last_character` select inclusive logical indices; -1 selects
through the end. Spaces count, BBCode syntax does not.

For Color Flash, `duration` is the full per-character flash-and-return duration;
the initial flash leg uses half that duration, and curve offsets multiply that
half-leg. The return waits for all initial flashes and restores individual
colors simultaneously. Scatter uses singleton native Char tweens to supply
different endpoints while retaining the range's configured delay samples.

Compose template nodes under native Chain Block or Parallel Block Blueprints.
Each template resolves its own target. Capturing values and initializing an
entrance happen when its lazy sequence step starts, after preceding steps and
its inherited `delay`. A continuous child keeps a serial composition at that
step until it is stopped; place it deliberately.

`play()` restarts an interrupted run from its captured baseline. After natural
completion, the next run captures current values, allowing external layout or
gameplay changes. To replay an exit from its former visible baseline, call
`restore()` first. Hover/Focus keeps its original held baseline across repeated
entry events. `restore()` is immediate; it is not a separate exit animation.

Restoration writes only the properties or selected character channels captured
by that template. Parallel effects should animate different channels. When
independent animations write the same channel, the containing scene determines
their ordering and restoration. Missing character indices are skipped during
restoration. Freeing the target or leaving the tree stops its bound playback.

Selecting a template in the editor does not animate its target. Configuration
warnings name the exact exported fields. Inactive controls remain serialized
and are hidden from the Inspector. Runtime snapshots and tween handles are not
exported or saved. Scene-root parameters and native Blueprint timing/triggers
use ordinary Inspector edits and Undo/Redo.

## Validate

From the source checkout:

```bash
bash scripts/test_scene_templates.sh /path/to/godot /tmp/spire-template-evidence
```

The runner uses a private project and Xvfb, checks all 33 scenes with real
rendering, and then exercises the actual editor Inspector, Undo/Redo, and scene
persistence. It reuses `target/debug/libspire_tween.so`; set
`SPIRE_TEMPLATE_LIBRARY` to use another compatible addon binary.
