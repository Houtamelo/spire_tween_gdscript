@tool
extends "../common/text_template.gd"

enum Style { WAVE, BOB, BREATHING, WOBBLE, TREMBLING, COLOR_CYCLE, ALPHA_PULSE }

@export_group("Continuous Text")
@export_storage var style: Style = Style.WAVE
@export var amplitude := Vector2(0.0, 6.0)
@export var rotation := Vector3(0.0, 0.0, 0.12)
@export var color_from := Color("67e8c2")
@export var color_to := Color("baa8ff")
@export_range(0.0, 1.0, 0.01) var minimum_alpha: float = 0.3
@export_range(1, 60, 1) var vibrato: int = 12


func _parameter_warnings() -> PackedStringArray:
	var warnings := super._parameter_warnings()
	if style in [Style.WAVE, Style.BOB, Style.BREATHING, Style.TREMBLING] and not _finite_vector(amplitude): warnings.append("`amplitude` must have finite components.")
	if style == Style.WOBBLE and not rotation.is_finite(): warnings.append("`rotation` must have finite components.")
	if style == Style.COLOR_CYCLE and (not _finite_color(color_from) or not _finite_color(color_to)): warnings.append("`color_from` and `color_to` must have finite components.")
	if style == Style.ALPHA_PULSE and (not is_finite(minimum_alpha) or minimum_alpha < 0.0 or minimum_alpha > 1.0): warnings.append("`minimum_alpha` must be finite and within 0..1.")
	if style == Style.TREMBLING and (vibrato < 1 or vibrato * duration > 65536.0): warnings.append("`vibrato` must be positive and `vibrato * duration` must not exceed 65536.")
	return warnings


func _validate_property(property: Dictionary) -> void:
	_show_fields(property, {"amplitude": style in [Style.WAVE, Style.BOB, Style.BREATHING, Style.TREMBLING], "rotation": style == Style.WOBBLE, "color_from": style == Style.COLOR_CYCLE, "color_to": style == Style.COLOR_CYCLE, "minimum_alpha": style == Style.ALPHA_PULSE, "vibrato": style == Style.TREMBLING, "pivot": style in [Style.BREATHING, Style.WOBBLE]})


func _uses_pivot() -> bool:
	return style in [Style.BREATHING, Style.WOBBLE]


func _build(node: Node) -> SpireSequence:
	var label := node as SpireRichTextLabel
	var channels: Array[StringName] = []
	match style:
		Style.WAVE, Style.BOB, Style.TREMBLING: channels = [&"offset"]
		Style.BREATHING: channels = [&"scale", &"pivot"]
		Style.WOBBLE: channels = [&"rotation", &"pivot"]
		Style.COLOR_CYCLE: channels = [&"color"]
		Style.ALPHA_PULSE: channels = [&"alpha"]
	_capture(label, channels)
	var last := _last(label)
	var curve := _delay_curve(1.0 if style in [Style.WAVE, Style.WOBBLE, Style.COLOR_CYCLE] else 0.0)
	var tween: SpireCharTween
	match style:
		Style.WAVE, Style.BOB:
			_initialize(label, &"offset", -amplitude)
			tween = label.offset_chars(first_character, last, amplitude, duration, curve)
		Style.BREATHING:
			_initialize(label, &"pivot", pivot)
			_initialize(label, &"scale", Vector2.ONE - amplitude)
			tween = label.scale_chars(first_character, last, Vector2.ONE + amplitude, duration, curve)
		Style.WOBBLE:
			_initialize(label, &"pivot", pivot)
			_initialize(label, &"rotation", -rotation)
			tween = label.rotate_chars(first_character, last, rotation, duration, curve)
		Style.TREMBLING:
			tween = label.shake_chars_offset(first_character, last, duration, amplitude.abs(), curve, vibrato, 90.0, false)
		Style.COLOR_CYCLE:
			_initialize(label, &"color", color_from)
			tween = label.color_chars(first_character, last, color_to, duration, curve)
		Style.ALPHA_PULSE:
			_initialize(label, &"alpha", minimum_alpha)
			tween = label.fade_chars(first_character, last, 1.0, duration, curve)
	return Spire.sequence().append(_configure(tween, true))
