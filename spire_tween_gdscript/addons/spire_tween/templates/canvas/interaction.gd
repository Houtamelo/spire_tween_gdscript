@tool
extends "../common/canvas_template.gd"
## Connect existing UI signals to play() and restore() using Godot's Signals dock.

enum Style { HOVER_FOCUS, PRESS, SELECTION }

@export_group("Interaction")
@export_storage var style: Style = Style.HOVER_FOCUS
@export var scale_multiplier := Vector2(1.06, 1.06)
@export var tint := Color.WHITE


func _parameter_warnings() -> PackedStringArray:
	var warnings := PackedStringArray()
	if not _finite_vector(scale_multiplier):
		warnings.append("`scale_multiplier` must have finite components.")
	if not _finite_color(tint):
		warnings.append("`tint` must have finite components.")
	return warnings


func _reset_state() -> void:
	# Mouse and focus entry may both arrive while emphasis is already held.
	if style == Style.HOVER_FOCUS and _completed:
		_restore_saved()
	super._reset_state()


func _build(node: Node) -> SpireSequence:
	var sequence := Spire.sequence()
	var scale: Vector2 = _remember(node, &"scale")
	var color: Color = _remember(node, &"modulate")
	var returning := style != Style.HOVER_FOCUS
	var leg := duration * 0.5 if returning else duration
	sequence.append(_tween(node, &"scale", scale, scale * scale_multiplier, leg))
	sequence.join(_tween(node, &"modulate", color, color * tint, leg))
	if returning:
		sequence.append(_tween(node, &"scale", scale * scale_multiplier, scale, leg).set_ease(Spire.EASE_OUT_POP))
		sequence.join(_tween(node, &"modulate", color * tint, color, leg))
	return sequence
