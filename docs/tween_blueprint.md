# Author animations with TweenBlueprint

Add `TweenBlueprint` nodes to a scene to store animation configuration in the Inspector. Each call to `play()` builds a fresh native `SpireSequence`. Property, character, serial, and parallel animations require no script; GDScript supplies custom builders.

For ready-made effects, use the [33 supplied GDScript template scenes](effect_templates.md).

[Watch the editor authoring walkthrough](../readme_images/blueprint_authoring.mp4) (1:44). It shows creating a two-step animation without code, connecting a Button signal, using Undo/Redo, and saving and reopening the scene.

## Animate a property

1. Add a `TweenBlueprint` under the node to animate and set **Kind** to **Property**.
2. Leave **Target** empty to use its first non-blueprint ancestor, or assign a node.
3. In **Property Animation**, choose **To Animate**, set **End Value**, and set **Speed Or Duration** in seconds.
4. Leave **Start Enabled** off to capture the current value when the property tween begins. Enable it to author **Start Value** explicitly.
5. Choose a trigger, or call `play()` from GDScript. The default trigger is **Manual**.

The property picker suggests writable tweenable properties, including exported GDScript fields. You can also enter a property path such as `position:x`. Suggestions exclude built-in configuration fields that are poor animation candidates; the filter does not prevent manually entered supported properties.

Value editors follow the selected property's type and hints. The supported types are int, float, String, Color, Vector2, Vector2i, Vector3, Vector3i, Vector4, Quaternion, Rect2, and Rect2i. Rect2i uses a native Rect2 tween with rounded integer accessor conversion.

Changing or hiding a field preserves its stored value. Incompatible values remain available for repair rather than being silently converted. When both value fields are unset, initial property selection fills them from the target on the next deferred editor update. Editor inspection never writes to the animation target.

## Compose animations

Set **Kind** to **Chain Block** to run direct blueprint children serially, or **Parallel Block** to run them together. Non-blueprint children are ignored. Each child keeps its own delay, speed, loops, easing, and sequence handle; child sequences are adopted by the parent rather than ticked independently.

Children are built eagerly when the parent is played. Their implicit start values are read when their property tween actually starts, after delay and preceding serial steps. Replaying or looping a returned native sequence reuses its constructed children.

## Animate text characters

Set **Kind** to **Text** and target a `SpireRichTextLabel`. Use the editor's **Change Type** action to convert an existing `RichTextLabel`. Assign a `SpireTextEffect` resource to `text_effect`, also used by [looping text presets](rich_text_effect_presets.md). For example, select its `kind` Offset and edit `from`, `to`, `duration`, and `ease`. No script is required.

`text_start_index` and `text_end_index` select an inclusive range of logical indices, including spaces and excluding BBCode tags. The default last index, -1, follows the current text length. `text_delay_ease_source` selects **Enum** or **Curve**. Set `text_delay_ease_enum` for a built-in Ease or assign a scalar graph to `text_delay_ease_curve`. The selected function supplies each character's start delay as a multiple of the resource's `duration`, sampled at its normalized position within the selection. An unassigned Curve falls back to `text_delay_ease_enum`. These native Inspector fields support normal Undo/Redo and retain inactive settings when changing sources. The default is Enum with Linear: the last character starts when the first finishes its first leg. A flat custom graph at zero starts all characters together. Samples must be finite and non-negative. The resource's `character_phase_offset` applies to continuous preset rendering.

The resource supplies endpoints or strength, duration, easing, loop mode, pivot, and effect-specific parameters. With `property_start_enabled` off, each tween captures the character's current value when processing begins. Enable it to initialize basic effects from the resource's `from` value. Punch/shake kinds ignore `from`; their optional initial channel value remains in Blueprint's `property_start_value`, with a typed Inspector field. Explicit starts and resource pivots apply to the whole selection when the Blueprint begins, after its delay and preceding serial steps, before per-character staggering. A reveal can therefore start with the entire phrase hidden. Constructing a serial composition does not change its later targets early.

The effect resource's `duration` is the duration of one animation leg for each character. The character step finishes when the last selected character finishes, including its start delay and requested loops. Its total duration is the largest character completion time, measured from the start of the step. For example, `duration = 1` second with a linear 0-to-1 delay curve gives each character a one-second animation and the whole range a two-second duration for one leg. See [range timing](character_animation.md#range-timing) for the constructor workflow.

`text_loops` sets the finite number of legs per character, using the resource's `loop_mode`, independently of the containing sequence's Restart loops. Each character repeats continuously after its initial delay. A character step builds one Char range tween. Blueprint `property_ease_source` selects an override for each character's local timeline: **Enum** uses `property_ease_enum`, and **Curve** uses `property_ease_curve`, falling back to `property_ease_enum` when unassigned. A resolved `property_ease_enum` of Default preserves the resource's `ease`. `text_blendable` controls additive punch/shake playback and appears only for those resource types. Property-only fields such as `property_speed_or_duration` and `property_end_value` are hidden for character steps.

The same effect resource can be shared across multiple Blueprints and preset collections. Effect fields are copied and validated on each `play()`; edits update the next run and do not mutate already-built tweens. Resource edits update the Blueprint's configuration warnings, and the Inspector supports normal resource editing, Undo/Redo, and scene save/reload.

Combine Offset and Fade children in a Parallel Block for a staggered reveal. Combine Offset, Rotation, Scale, and Color children for a composed effect. Use Property steps on the label's `text` for text replacement: writes to an active character which disappears are ignored while playback continues, without redirecting to another index. A fresh `play()` explicitly synchronizes the label before selecting its range against the current text.

The label initializes its character rendering state in both the editor and runtime without animating. For BBCode start colors, trigger after the label's first draw; a manually connected Button naturally does this. If playing immediately from After Ready or After Enter Tree, supply a delay allowing the first draw, as required by the underlying character constructors. Inspector queries do not reset character state or write animation values to the target.

Each blueprint resolves its own target independently: its configured Target, then its first non-blueprint ancestor. Containers do not pass targets to their children. A nested child's fallback skips all blueprint ancestors, including their configured targets. The Inspector uses the same resolution as runtime playback.

## Timing and playback

| Setting | Behavior |
|---|---|
| Delay | Seconds before the sequence starts. Default 0. |
| Speed Scale | Native clock multiplier; default 1. Explicit zero pauses progression. |
| Loops | Native sequence loop count; default 1, zero also plays once, negative means infinite. |
| Loop Mode | Forwarded to native Sequence, which supports Restart only; Yoyo and Incremental have no effect. |
| Tween Pause Mode / Tween Process Mode | Native pause policy and Idle, Physics, or Manual processing. These are separate from `Node.process_mode`. |
| Lerp Mode | Absolute duration, Relative offset over a duration, or Speed Based in native units per second. Explicit start applies only to Absolute mode. |
| `property_ease_source`, `property_ease_enum`, `property_ease_curve` | Select built-in or Curve easing. An unassigned Curve falls back to the selected Ease. Inactive values remain serialized. |

Invalid delay and speed scale warn and normalize to 0 and 1 respectively. Invalid property paths, values, duration, or speed warn and skip the step so following valid siblings can run. Configuration warnings identify invalid authored settings in the Scene dock.

The Inspector reports a configuration warning when `property_speed_or_duration` is non-finite or non-positive. Runtime playback supports a zero duration: it applies the endpoint when the step is reached, honoring delay and serial ordering. Speed-based mode requires a positive speed.

```gdscript
var sequence: SpireSequence = $Animation.play()
$Animation.stop()                 # Preserve the interrupted target values.
```

Every `play()` stops that blueprint's previous run, calls `_reset_state()`, and builds a new sequence. Built-in reset is a no-op, so an implicit start continues from the interrupted value. There is no retrigger-policy setting. Calling `play()` again on the same blueprint from a hook or callback before the current play call returns is unsupported.

Leaving the tree or freeing the blueprint stops its current run. Freeing the resolved target cancels bound playback. A returned handle can outlive the authoring node without retaining that node through signal callbacks.

The typed `tween_spawned`, `tween_loop_finished`, `tween_stopped`, and `tween_finished` signals carry the exact `SpireSequence`. Spawn is emitted after configuration and before building. Successful natural completion forwards loop-finished, stopped, then finished; cancellation and `stop()` do not emit finished.

## Automatic triggers

Only blueprints outside a containing composition connect automatic triggers. A managed child's stored trigger stays visible and read-only. Automatic triggers never run in the editor.

| Trigger | Source and behavior |
|---|---|
| Manual | No automatic subscription. |
| After Ready | Plays during the first runtime ready callback. |
| After Enter Tree | Plays on each tree entry once references are available. |
| Mouse Enter / Mouse Exit | Uses the source's native mouse signals. |
| Mouse Click Left / Right | Filters native input events for a pressed matching mouse button; ignores releases and other events. |

**Trigger Source** selects a separate source for mouse subscriptions; leaving it empty uses the resolved animation target. Mouse sources support Control, CollisionObject2D, and CollisionObject3D. Controls must accept mouse input; collision objects must enable picking and have a nonzero collision layer. The scene remains responsible for collision shapes, cameras, visibility, and viewport input configuration. The addon does not change these settings.

Subscriptions disconnect on exit and rebind after a deferred configuration update. Invalid sources produce warnings and remain inactive.

For playback from other signals, leave the trigger set to **Manual** and use Godot's Signals dock to connect the source signal directly to the blueprint's `play()` method. These standard scene connections are saved by Godot; no addon signal selector or binding mode is required.

## Write a custom GDScript builder

Attaching a script selects custom behavior and hides Kind and built-in property controls. Override `_play_internal(sequence)`; an attached script without that override warns and produces an empty sequence. Use `get_resolved_target()` to obtain the script's configured target or ancestor fallback.

```gdscript
@tool
extends TweenBlueprint

@export var distance: float = 32.0
@export var duration: float = 0.2

func _play_internal(sequence: SpireSequence) -> void:
    var node := get_resolved_target() as Node2D
    if node == null:
        push_warning("This effect requires a Node2D target.")
        return
    sequence.append(
        DoNode2D.position(node, Vector2(distance, 0), duration).as_relative()
    )
```

`_reset_state()` is optional and may reset custom local state before construction. Hooks run with native blueprint borrows released, so they can access script and exported fields. Native virtual defaults cannot be called through GDScript `super`.

Custom GDScript builders can append native callbacks. A dedicated Method Caller kind remains outside this implementation.

## Preview in the editor

Select an outermost Blueprint and press **Play Animation Preview** in the Inspector. **Stop Animation Preview** stops it and invokes its restoration callback when valid. A parent-managed child cannot start an independent preview; preview its outermost composition instead. Native Property, Text, Chain Block and Parallel Block kinds and all supplied template scenes support preview.

Only one preview runs at a time. Starting another first stops the previous preview and invokes any restoration callback. Capture, playback and restoration are synchronous, with no frame delays. Natural completion also invokes restoration. Saving performs cleanup before scene serialization; switching scenes or removing the preview root cleans up the outgoing preview. Normal runtime `play()`/`stop()` behavior remains unchanged.

All participating children capture before any reset/build/play hook runs. Containers invoke captured child callbacks in reverse child order when the root stops. Intermediate child completion does not restore the composition. Native property capture handles overlapping Node2D/Node3D transforms and Control layout/transform properties through local rest values. Character capture preserves exact channel overrides, including inherited BBCode colors.

A custom script can provide restoration by overriding `_capture_rest_state() -> Callable`. Capture must not change targets. Its returned Callable must restore synchronously; it must not await signals or rely on `set_deferred()`/`call_deferred()`. An invalid Callable means there is nothing to restore and does not prevent playback. This is also the default for scripts that do not override the capture hook. Containers still play every child, invoking only valid restoration callbacks in reverse child order. Previews with no restoration action still participate in Stop, completion, replacement, saving and scene-exit cleanup.

```gdscript
func _capture_rest_state() -> Callable:
    var node := get_resolved_target() as Node2D
    if node == null:
        return Callable()
    var rest_transform := node.transform
    return func() -> void:
        if is_instance_valid(node):
            node.transform = rest_transform
```

For custom character effects, `SpireRichTextLabel.capture_character_rest_state(first, last)` returns a synchronous restoration Callable for an inclusive range; -1 selects through the end. Explicit character ranges require initialized, synchronized state. Whole-label capture `(0, -1)` also supports empty or uninitialized data without initializing it during capture; its callback rebuilds default state when no current snapshot was available. The callback owns refreshing, skips a freed label or different text/BBCode, and can restore channels after the captured source has been restored and rebuilt. Native Property previews of a label's `text` invoke that same callback, so composing text replacement and character animations restores both.

Custom implementors decide how to capture targets, mutable Resources and other state. Capturing a Resource reference does not copy its contents. Supplied templates also clear their lazy runtime snapshots during preview restoration, preventing an interrupted preview from affecting later runs. Editing animated properties during preview and Undo/Redo interception are outside this workflow.

## Verify the workflow

On Linux, run the isolated runtime and editor tests without changing development-project binaries or caches:

```bash
bash scripts/test_blueprint.sh godot
bash scripts/test_blueprint.sh /path/to/godot --rendered
bash scripts/test_blueprint_preview.sh /path/to/godot /tmp/spire-preview-evidence
```

The rendered run uses Xvfb and saves an Inspector screenshot. Both modes retain their fixture and logs under the printed temporary path and reject engine errors in addition to failed assertions. `SPIRE_BLUEPRINT_LIBRARY` reuses an already built API-4.4-compatible library, and `SPIRE_BLUEPRINT_TEST_DIR` chooses the evidence directory. The runtime tests also participate in the aggregate GDScript runner.
