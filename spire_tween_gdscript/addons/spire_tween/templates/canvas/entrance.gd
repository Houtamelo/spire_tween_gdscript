@tool
extends "../common/canvas_template.gd"
## Entrance/exit variations share a scene root with an editable target.

enum Style { FADE, SLIDE, POP, BOUNCE, FLIP, SPIN }

@export_group("Entrance / Exit")
@export_storage var style: Style = Style.FADE
@export var exiting := false
@export var offset := Vector2(0.0, 48.0)
@export_range(-360.0, 360.0, 1.0, "radians_as_degrees") var angle: float = PI
@export_enum("Horizontal", "Vertical") var flip_axis: int = 0


func _parameter_warnings() -> PackedStringArray:
	var warnings := PackedStringArray()
	if style == Style.SLIDE and not _finite_vector(offset):
		warnings.append("`offset` must have finite components.")
	if style == Style.SPIN and not is_finite(angle):
		warnings.append("`angle` must be finite.")
	return warnings


func _validate_property(property: Dictionary) -> void:
	_show_fields(property, {"offset": style == Style.SLIDE, "angle": style == Style.SPIN, "flip_axis": style == Style.FLIP})


func _build(node: Node) -> SpireSequence:
	var sequence := Spire.sequence()
	var alpha: float = _remember(node, &"modulate:a")
	sequence.append(_tween(node, &"modulate:a", alpha if exiting else 0.0, 0.0 if exiting else alpha))
	if style == Style.SLIDE:
		var position: Vector2 = _remember(node, &"position")
		var displaced := position + offset
		sequence.join(_tween(node, &"position", position if exiting else displaced, displaced if exiting else position))
	elif style in [Style.POP, Style.BOUNCE, Style.FLIP, Style.SPIN]:
		var scale: Vector2 = _remember(node, &"scale")
		var collapsed := Vector2.ZERO
		if style == Style.FLIP:
			collapsed = scale * (Vector2(0.0, 1.0) if flip_axis == 0 else Vector2(1.0, 0.0))
		var movement := _tween(node, &"scale", scale if exiting else collapsed, collapsed if exiting else scale)
		if style == Style.BOUNCE:
			movement.set_ease(Spire.EASE_IN_BOUNCE if exiting else Spire.EASE_OUT_BOUNCE)
		elif style == Style.POP:
			movement.set_ease(Spire.EASE_IN_POP if exiting else Spire.EASE_OUT_POP)
		sequence.join(movement)
		if style == Style.SPIN:
			var rotation: float = _remember(node, &"rotation")
			sequence.join(_tween(node, &"rotation", rotation if exiting else rotation - angle, rotation + angle if exiting else rotation))
	return sequence
