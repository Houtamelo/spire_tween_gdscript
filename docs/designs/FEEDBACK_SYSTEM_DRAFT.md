# Goal

Provide artists with simple & ergonomic ways to insert "game-feel" improvements in their scenes.

# Proposal

Create:

1. A simplified version of MoreMountains's Feedback system, targeting the specific needs of Spire of Slime.
2. Use said Feedback system to create a set of plug-and-play presets that cover most of our artists' needs.
3. Artists can still directly use the feedback system for more advanced use cases.

# Implementation idea

Piggy-back on the robust SpireTween library already made by Houtamelo.
It covers most of the primitives we need, including tween sequencing. Feedback nodes fill the serialization gap by storing editable configuration in scenes and creating runtime tweens when played.

The first implementation has fixed restart behavior. There is no `RetriggerMode` enum or corresponding Inspector setting. Every call to `play()` stops that feedback's previous run, resets its effect
state, and constructs a fresh runtime sequence. This applies to manual calls, automatic triggers, and calls from a containing feedback.

The [implementation contract](FEEDBACK_IMPLEMENTATION_CONTRACT.md) records the refined plan's concrete native/GDScript mapping, serialized names, helper signatures, reset rules, and trigger conventions. Those are selected planning proposals for review. In that mapping, the existing native `TweenBlueprint` supplies the common behavior and four thin GDScript classes expose the conceptual feedback roles below; the code examples remain illustrations of the original GDScript design rather than a literal Rust class hierarchy.

## Initial atomic feedbacks

Start with two atomic feedbacks. Blocks and sequences compose these into the curated presets.

| Atomic feedback          | Configuration                                                                                                                            | Scheduled behavior                                                 |
|--------------------------|------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------|
| Generic Property Tweener | `property_name: StringName`, an optional start value, an end value, duration, and easing. Value fields use the selected property's type. | Tweens the named property on the resolved target using SpireTween. |
| Method Caller            | `method_name: StringName`. The selected method must be callable without arguments.                                                       | Calls that method once when the sequence reaches its callback.     |

### Generic Property Tweener

Export `property_name` as a `StringName`:

```gdscript
@export var property_name: StringName
```

In `@tool` mode, resolve the effective target and find the named property in its `get_property_list()`. Use its type and relevant Inspector hints to expose typed value fields for that property.

The Inspector follows the existing Start Position example:

- An optional Start Value group, disabled by default.
- A start value with the selected property's type, shown when Start Value is enabled.
- An end value with the same type.
- Duration and easing, with the same semantics as the position example.
- The inherited `_delay` setting.

Without an explicit start value, read the property's current value when the tween begins, after the feedback's _delay and any preceding sequence steps. When Start Value is enabled, pass the configured
start value to the tween's `from()` method.

These are dynamically typed Inspector properties, rather than GDScript member declarations whose static types change at runtime. Use `_get_property_list()` to supply their metadata and `_get()` /
`_set()` to handle their values. Their storage must remain serializable, including while a target is temporarily unresolved, so saving a scene does not discard its configured
values. [Godot's dynamic property API](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get-property-list).

Refresh the metadata and Inspector when the property name, configured target, or ancestor fallback changes. Preserve compatible configured values. Missing properties, unresolved types,
and types unsupported by SpireTween should produce configuration warnings rather than usable-looking tween fields.

Editor inspection and runtime playback resolve each feedback's own configured target or first non-feedback ancestor. At runtime, validate that target before creating
the property tween. Editor inspection and validation do not modify the target's property.

### Method Caller

Export `method_name` as a `StringName`:

```gdscript
@export var method_name: StringName
```

In `@tool` mode, resolve the effective target, verify that the method exists, and inspect its signature. Methods with no parameters are valid, as are methods whose parameters all have defaults.
Methods requiring any argument are invalid for this feedback.

Use the matching entry from `get_method_list()` to inspect `args` and `default_args`. A method's required fixed argument count is `args.size() - default_args.size()`; it must be zero. Optional
variadic arguments do not require supplying arguments. [Godot's method signature metadata](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-get-method-list).

At runtime, revalidate the actual target and append a zero-argument method callback to the returned sequence. Optional parameters use the method's defaults. Invoke the method when that callback
executes, after the inherited _delay, rather than during sequence construction or editor validation.

The call is an instantaneous sequence step. Its return value is ignored; the sequence does not implicitly wait for asynchronous work started by the method. Missing methods or methods with required
parameters produce a configuration warning in the editor and a diagnostic at runtime, without scheduling the invalid call.

## Core types

The following code adapts the three existing scripts to the decisions in this document. These are proposed skeletons: TODOs identify behavior that still needs implementation or further decisions. The
document's contracts take precedence over the older source scripts.

Each call to `play()` stops the feedback's previous sequence, invokes `_reset_state()`, and constructs a fresh `SpireSequence`. The common `play()` implementation owns this fixed restart behavior;
effect-specific hooks only populate the new sequence. Only the outermost feedback subscribes to automatic triggers.

Built-in reset hooks do not rewind runtime target values. Custom effects may implement their own reset behavior. Editor preview baseline restoration is a separate responsibility. Child sequences are constructed eagerly when a container is played; the native Property Tweener still reads an implicit start only when that tween begins. Native sequence replay/loops do not call child feedback `play()` again.

Shared helpers called during editor inspection must be available in `@tool` mode. Automatic trigger subscriptions and playback operate at runtime; tool-mode reflection performs validation and
Inspector updates. [Godot's tool-script requirements](https://docs.godotengine.org/en/stable/tutorials/plugins/running_code_in_the_editor.html#how-to-use-tool).

### Feedback

Adapted from the original game prototype `feedback.gd`. Supplies the common configuration, playback contract, delayed sequence factory, and target resolver.

```gdscript
@tool
@abstract
class_name Feedback
extends Node

enum TRIGGER_CONDITION {
	MANUAL,
	MOUSE_ENTER,
	MOUSE_EXIT,
	MOUSE_CLICK_LEFT,
	MOUSE_CLICK_RIGHT,
	ENTER_TREE,
}

@export
var _trigger_condition: TRIGGER_CONDITION = TRIGGER_CONDITION.MANUAL:
	set(value):
		_trigger_condition = value
		notify_property_list_changed()

@export var _target: Node
@export var _delay: float = 0.0 # Seconds; must be non-negative.

var _latest_seq: SpireSequence = null

@abstract func _play_internal(seq: SpireSequence, resolved_target: Node) -> void
@abstract func _manages_children() -> bool
@abstract func _reset_state() -> void

func play() -> SpireSequence:
	if _latest_seq != null:
		_latest_seq.stop()

	_reset_state()
	return _setup_sequence_and_play()


func _setup_sequence_and_play() -> SpireSequence:
	var seq := Spire.sequence()
	if _delay > 0.0: seq.set_delay(_delay)

	var resolved_target := _resolve_target()
	_play_internal(seq, resolved_target)
	_latest_seq = seq
	return seq


func _ready() -> void:
	if Engine.is_editor_hint():
		notify_property_list_changed()
		# TODO: Add the parent-managed label and refresh on hierarchy changes.
		return

	if _is_managed_by_parent():
		return

	# TODO: Wire automatic events through `play()`
	match _trigger_condition:
		TRIGGER_CONDITION.MANUAL: pass
		TRIGGER_CONDITION.MOUSE_ENTER: pass
		TRIGGER_CONDITION.MOUSE_EXIT: pass
		TRIGGER_CONDITION.MOUSE_CLICK_LEFT: pass
		TRIGGER_CONDITION.MOUSE_CLICK_RIGHT: pass
		TRIGGER_CONDITION.ENTER_TREE: pass


func _validate_property(property: Dictionary) -> void:
	if property["name"] == "_trigger_condition" and _is_managed_by_parent():
		property["usage"] |= PROPERTY_USAGE_EDITOR | PROPERTY_USAGE_READ_ONLY


func _is_managed_by_parent() -> bool:
	var parent := get_parent()
	if parent is Feedback: return parent._manages_children()
	else: return false


func _get_first_non_feedback_ancestor() -> Node:
	var parent := get_parent()
	if parent is Feedback: return parent._get_first_non_feedback_ancestor()
	else: return parent


func _resolve_target() -> Node:
	if _target != null: return _target
	else: return _get_first_non_feedback_ancestor()


func _get_children_feedbacks() -> Array[Feedback]:
	var children_typed: Array[Feedback] = []
	for child in get_children():
		if child is Feedback:
			children_typed.append(child)

	return children_typed
```

Each feedback resolves its configured target or first non-feedback ancestor independently. Containers call each child's parameterless `play()`; their configured targets do not affect descendants. Editor inspection follows the same resolution rule.

### FeedbackBlock

Adapted from the original game prototype `feedback_block.gd`. Joins the child feedback sequences into one parallel block.

```gdscript
@tool
class_name FeedbackBlock
extends Feedback

func _manages_children() -> bool: return true

func _play_internal(seq: SpireSequence, resolved_target: Node) -> void:
	for child: Feedback in _get_children_feedbacks():
	    var child_seq := child.play()
	    if child_seq != null:
	        seq.join(child_seq)

# Each child's play() invokes its own reset hook before constructing a fresh run.
func _reset_state() -> void:
    pass
```

### FeedbackSequence

Adapted from the original game prototype `feedback_sequence.gd`. Appends the child feedback sequences in scene-tree order.

```gdscript
@tool
class_name FeedbackSequence
extends Feedback

func _manages_children() -> bool: return true

func _play_internal(seq: SpireSequence, resolved_target: Node) -> void:
	for child: Feedback in _get_children_feedbacks():
	    var child_seq := child.play()
	    if child_seq != null:
	        seq.append(child_seq)

# Each child's play() invokes its own reset hook before constructing a fresh run.
func _reset_state() -> void:
    pass
```

Containers enumerate direct feedback children in scene-tree order, including runtime-added children without scene ownership. Every child retains its own configured target or ancestor fallback; no target is forwarded by the container.

## Shared _delay

Every feedback inherits `@export var _delay: float = 0.0`, measured in seconds, with non-negative values. Apply it once to the fresh sequence returned by each `play()` call, using
`Spire.sequence().set_delay(_delay)` or a shared base-class helper with that behavior.

- A leaf feedback waits before its tween or callback starts.
- A block or sequence waits before starting its composed children.
- Children retain their own delays, so container and child delays compose through the hierarchy.
- _Delay applies to manual calls and composed playback as well as automatic triggers.

A run waiting for its initial _delay, or paused during playback, is still replaced by a subsequent `play()` call. The replacement receives its own full _delay.

## Composition

Both containers inherit `Feedback` and return a runtime sequence:

| Container          | Child playback                                     |
|--------------------|----------------------------------------------------|
| `FeedbackBlock`    | Joins child sequences so they play in parallel.    |
| `FeedbackSequence` | Appends child sequences so they play sequentially. |

Containers can nest. They call their children's `play()` methods to build the composition; each child follows the same fixed restart behavior. Each invocation supplies a fresh sequence, so a parent
retains the specific child sequence it adopted. If another call restarts that child, the original sequence is stopped; the original parent can continue without waiting for the replacement.

## Automatic trigger ownership

Only the outermost feedback in a composed feedback hierarchy subscribes to automatic triggers. Each accepted event calls the same `play()` entry point used by manual and composed playback.

- With `TRIGGER_CONDITION.MANUAL`, playback is controlled entirely through calls to `play()`.
- With another trigger condition, the outermost feedback connects to that event and calls `play()` when it occurs.
- Descendant feedbacks participate through the ancestor's composition. Their automatic trigger settings are inactive and shown as read-only in the Inspector. Their `play()` method still applies the
  fixed restart behavior.

Manual calls remain available on every feedback, including descendants managed by a containing feedback.

For playback from other signals, use Godot's built-in editor to connect the source signal to `play()` in Manual mode. There is no addon Signal trigger, signal-name property, or arbitrary-signal subscription logic.

## Restart behavior

The first implementation always behaves like the earlier `RESTART` option, without a selectable policy:

1. Stop the feedback's previous sequence, if one exists. This also handles playing, paused, and delayed runs; stopping an already stopped sequence is harmless.
2. Invoke `_reset_state()` before building the replacement.
3. Create a fresh sequence, apply the feedback's _delay once, and populate it through `_play_internal()`.

Stopping a container's previous sequence also stops its current child sequences. Each child reset hook is invoked when the container calls that child's `play()` to build its replacement.

Restarting replaces playback; it does not force the old run to its final values or emit successful completion for the cancelled run. Property values after `_reset_state()` depend on the effect's reset
implementation. The existing Generic Property Tweener rule still applies: without an explicit start value, it reads the property's current value when the replacement tween begins.

This playback rule is independent of loops within an individual SpireTween sequence. There is no retrigger policy inheritance or per-feedback policy override in the first implementation.

## Inspector behavior

| Feedback placement                                                                   | Trigger condition                      | Explanatory label                                      |
|--------------------------------------------------------------------------------------|----------------------------------------|--------------------------------------------------------|
| Outermost feedback                                                                   | Visible and editable.                  | None.                                                  |
| Feedback inside a `FeedbackBlock` or `FeedbackSequence`, including nested containers | Visible, greyed out, and not editable. | **Automatic triggers are managed by the parent**.       |

There is no retrigger setting in the Inspector. Keep a composed child's trigger condition visible so the parent-managed label explains why it cannot be edited.

Preserve the child's configured trigger condition while it is composed under a parent. Update the label and editability when the hierarchy changes; moving the feedback out of the
composition restores the normal outermost presentation.

Use `_validate_property()` to apply `PROPERTY_USAGE_READ_ONLY` to the existing exported trigger field while retaining its Inspector visibility. Refresh the property list when parent-management status
changes. [Godot's property validation API](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property).

## Position feedback example

This example illustrates the Generic Property Tweener's start/end semantics with a fixed position property, using the proposed `Feedback` helpers above. The source scripts are prototypes; the code in
this document includes agreed changes that still need to be applied to them.

```gdscript
@tool
extends Feedback

@export_custom(PROPERTY_HINT_GROUP_ENABLE, "")
@export_group("Start Position", "_start_position_")
var _start_position_enable: bool = false

@export
var _start_position_value: Vector2 = Vector2.ZERO

@export
var _end_position: Vector2

@export
var _duration: float = 1.0

@export
var _ease: Spire.Ease = Spire.EASE_LINEAR

func _manages_children() -> bool: return false

func _reset_state() -> void:
	pass

func _play_internal(seq: SpireSequence, resolved_target: Node) -> void:
	var target_node := resolved_target as Node2D

	if target_node == null:
		push_error("[Feedback] Position feedback requires a Node2D target.")
		return

	var pos_tween := DoNode2D.position(target_node, _end_position, _duration).set_ease(_ease)
	if _start_position_enable: pos_tween.from(_start_position_value)

	seq.append(pos_tween)
```
