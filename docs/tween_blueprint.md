# Author animations with TweenBlueprint

Add `TweenBlueprint` nodes to a scene to store animation configuration in the Inspector. Each call to `play()` builds a fresh native `SpireSequence`. Property, serial, and parallel animations require no script; GDScript supplies custom builders.

## Animate a property

1. Add a `TweenBlueprint` under the node to animate and set **Kind** to **Property**.
2. Leave **Target** empty to use its first non-blueprint ancestor, or assign a node.
3. Choose **Animated Property**, set **End Value**, and set **Speed Or Duration** in seconds.
4. Leave **Start Enabled** off to capture the current value when the property tween begins. Enable it to author **Start Value** explicitly.
5. Choose a trigger, or call `play()` from GDScript. The default trigger is **Manual**.

The property picker suggests writable tweenable properties, including exported GDScript fields. You can also enter a property path such as `position:x`. Suggestions exclude built-in configuration fields that are poor animation candidates; the filter does not prevent manually entered supported properties.

Value editors follow the selected property's type and hints. The supported types are int, float, String, Color, Vector2, Vector2i, Vector3, Vector3i, Vector4, Quaternion, Rect2, and Rect2i. Rect2i uses a native Rect2 tween with rounded integer accessor conversion.

Changing or hiding a field preserves its stored value. Incompatible values remain available for repair rather than being silently converted. When both value fields are unset, initial property selection fills them from the target on the next deferred editor update. Editor inspection never writes to the animation target.

## Compose animations

Set **Kind** to **Chain Block** to run direct blueprint children serially, or **Parallel Block** to run them together. Non-blueprint children are ignored. Each child keeps its own delay, speed, loops, easing, and sequence handle; child sequences are adopted by the parent rather than ticked independently.

Children are built eagerly when the parent is played. Their implicit start values are read when their property tween actually starts, after delay and preceding serial steps. Replaying or looping a returned native sequence reuses its constructed children.

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
| Ease Basic / Ease Curve | Native easing; a valid curve takes precedence over basic easing. |

Invalid delay and speed scale warn and normalize to 0 and 1 respectively. Invalid property paths, values, duration, or speed warn and skip the step so following valid siblings can run. Configuration warnings identify invalid authored settings in the Scene dock.

The Inspector reports a configuration warning when `speed_or_duration` is non-finite or non-positive. Runtime playback supports a zero duration: it applies the endpoint when the step is reached, honoring delay and serial ordering. Speed-based mode requires a positive speed.

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

This implementation covers runtime authoring and the built-in Inspector. Editor preview/restoration, supplied presets, and a dedicated Method Caller kind belong to the separate tooling plan; custom GDScript builders can already append native callbacks.

## Verify the workflow

On Linux, run the isolated runtime and editor tests without changing development-project binaries or caches:

```bash
bash scripts/test_blueprint.sh godot
bash scripts/test_blueprint.sh /path/to/godot --rendered
```

The rendered run uses Xvfb and saves an Inspector screenshot. Both modes retain their fixture and logs under the printed temporary path and reject engine errors in addition to failed assertions. `SPIRE_BLUEPRINT_LIBRARY` reuses an already built API-4.4-compatible library, and `SPIRE_BLUEPRINT_TEST_DIR` chooses the evidence directory. The runtime tests also participate in the aggregate GDScript runner.
