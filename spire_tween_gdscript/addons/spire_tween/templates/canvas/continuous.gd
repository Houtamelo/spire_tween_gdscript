@tool
extends "../common/canvas_template.gd"
## A continuously looping native sequence; stop() preserves its current pose.

enum Style { PULSE, FLOAT, SWAY, SPIN, COLOR_CYCLE }

@export_group("Continuous Motion")
@export_storage var style: Style = Style.PULSE
@export var amplitude := Vector2(0.08, 0.08)
@export_range(-360.0, 360.0, 0.5, "radians_as_degrees") var angle: float = 0.12
@export var color_from := Color("67e8c2")
@export var color_to := Color("baa8ff")


func _parameter_warnings() -> PackedStringArray:
	var warnings := PackedStringArray()
	if style in [Style.PULSE, Style.FLOAT] and not _finite_vector(amplitude):
		warnings.append("`amplitude` must have finite components.")
	if style in [Style.SWAY, Style.SPIN] and not is_finite(angle):
		warnings.append("`angle` must be finite.")
	if style == Style.COLOR_CYCLE and (not _finite_color(color_from) or not _finite_color(color_to)):
		warnings.append("`color_from` and `color_to` must have finite components.")
	return warnings


func _validate_property(property: Dictionary) -> void:
	_show_fields(property, {"amplitude": style in [Style.PULSE, Style.FLOAT], "angle": style in [Style.SWAY, Style.SPIN], "color_from": style == Style.COLOR_CYCLE, "color_to": style == Style.COLOR_CYCLE})


func _build(node: Node) -> SpireSequence:
	var sequence := Spire.sequence()
	var property: StringName
	var from: Variant
	var to: Variant
	match style:
		Style.PULSE:
			property = &"scale"
			from = _remember(node, property)
			to = from * (Vector2.ONE + amplitude)
		Style.FLOAT:
			property = &"position"
			from = _remember(node, property)
			to = from + amplitude
		Style.SWAY:
			property = &"rotation"
			var baseline: float = _remember(node, property)
			from = baseline - angle
			to = baseline + angle
		Style.SPIN:
			property = &"rotation"
			from = _remember(node, property)
			to = from + TAU * (-1.0 if angle < 0.0 else 1.0)
		Style.COLOR_CYCLE:
			property = &"modulate"
			var baseline: Color = _remember(node, property)
			from = baseline * color_from
			to = baseline * color_to
	var tween := _tween(node, property, from, to)
	tween.set_loops(-1, Spire.LOOP_MODE_INCREMENTAL if style == Style.SPIN else Spire.LOOP_MODE_YOYO)
	sequence.append(tween)
	return sequence
