@tool
extends TweenBlueprint
## Base for the supplied scene templates. Use play(), stop(), and restore().
## Snapshots are captured when this step starts, including inside a composition.

const Easing := preload("easing.gd")

@export_group("Template")
@export_range(0.001, 10.0, 0.001, "or_greater", "suffix:s") var duration: float = 0.45
@export var ease: Spire.Ease = Spire.EASE_OUT_CUBIC

var _saved_target: WeakRef
var _saved_properties: Dictionary = {}
var _completed := false
var _editor_warnings := PackedStringArray()


func _init() -> void:
	set_process(Engine.is_editor_hint())


func _process(_delta: float) -> void:
	if not Engine.is_editor_hint():
		return
	# Includes in-place Curve edits and changes to the resolved target's text.
	var warnings := get_template_warnings()
	if warnings != _editor_warnings:
		_editor_warnings = warnings
		update_configuration_warnings()


func _get_configuration_warnings() -> PackedStringArray:
	return get_template_warnings()


func get_template_warnings() -> PackedStringArray:
	var warnings := PackedStringArray()
	if not is_finite(duration) or duration <= 0.0:
		warnings.append("`duration` must be finite and positive.")
	var node := get_resolved_target()
	if not is_instance_valid(node):
		warnings.append("`target` must resolve to a compatible node; instance this scene beneath its target or assign `target`.")
	else:
		warnings.append_array(_target_warnings(node))
	warnings.append_array(_parameter_warnings())
	return warnings


func _target_warnings(_node: Node) -> PackedStringArray:
	return PackedStringArray()


func _parameter_warnings() -> PackedStringArray:
	return PackedStringArray()


func _play_internal(sequence: SpireSequence) -> void:
	var warnings := get_template_warnings()
	if not warnings.is_empty():
		for warning in warnings:
			push_warning("%s: %s" % [name, warning])
		return
	sequence.append_ctor(_begin.bind(weakref(get_resolved_target())))
	sequence.finished.connect(_mark_completed)


func _begin(reference: WeakRef) -> SpireSequence:
	var node: Node = reference.get_ref()
	if not is_instance_valid(node):
		return Spire.sequence()
	# A delayed/serial step may have been reconfigured since play() was called.
	var warnings := get_template_warnings()
	warnings.append_array(_target_warnings(node))
	if not warnings.is_empty():
		for warning in warnings: push_warning("%s: %s" % [name, warning])
		return Spire.sequence()
	_saved_target = reference
	return _build(node)


func _build(_node: Node) -> SpireSequence:
	return Spire.sequence()


func _mark_completed() -> void:
	_completed = true


func _reset_state() -> void:
	# Restart an interrupted run from its baseline. A completed run permits new
	# authored/layout values to become the next run's baseline.
	if not _completed:
		_restore_saved()
	_clear_saved_state()


func _clear_saved_state() -> void:
	_saved_properties.clear()
	_saved_target = null
	_completed = false


## Stop and restore only the channels captured by this template's last run.
func restore() -> void:
	stop()
	_restore_saved()
	_clear_saved_state()


func _capture_rest_state() -> Callable:
	if not get_template_warnings().is_empty():
		return Callable()
	var restore_target := _capture_target_rest_state(get_resolved_target())
	if not restore_target.is_valid():
		return Callable()
	return func() -> void:
		restore_target.call()
		# A later runtime reset must not restore a lazy step's intermediate pose.
		_clear_saved_state()


func _capture_target_rest_state(_node: Node) -> Callable:
	return Callable()


func _restore_saved() -> void:
	if _saved_target == null:
		return
	var node: Node = _saved_target.get_ref()
	if not is_instance_valid(node):
		return
	for property: StringName in _saved_properties:
		node.set_indexed(NodePath(property), _saved_properties[property])


func _remember(node: Node, property: StringName) -> Variant:
	var value: Variant = node.get_indexed(NodePath(property))
	_saved_properties[property] = value
	return value


func _tween(node: Node, property: StringName, from: Variant, to: Variant, seconds: float = -1.0) -> SpireProperty:
	return Spire.do_property(node, NodePath(property), to, duration if seconds < 0.0 else seconds).from(from).set_ease(ease)


func _finite_vector(value: Vector2) -> bool:
	return is_finite(value.x) and is_finite(value.y)


func _finite_color(value: Color) -> bool:
	return is_finite(value.r) and is_finite(value.g) and is_finite(value.b) and is_finite(value.a)


func _show_fields(property: Dictionary, visible: Dictionary) -> void:
	if visible.has(property.name) and not visible[property.name]:
		property.usage &= ~PROPERTY_USAGE_EDITOR
