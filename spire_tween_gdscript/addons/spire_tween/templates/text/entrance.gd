@tool
extends "../common/text_template.gd"

enum Style { TYPEWRITER, FADE, SLIDE, POP, BOUNCE, FLIP, SCATTER }

@export_group("Text Entrance / Exit")
@export_storage var style: Style = Style.FADE
@export var exiting := false
@export var offset := Vector2(0.0, 24.0)
@export var rotation := Vector3(0.0, PI * 0.5, 0.0)
@export var scatter_seed: int = 1729


func _parameter_warnings() -> PackedStringArray:
	var warnings := super._parameter_warnings()
	if style in [Style.SLIDE, Style.SCATTER] and not _finite_vector(offset):
		warnings.append("`offset` must have finite components.")
	if style in [Style.FLIP, Style.SCATTER] and not rotation.is_finite():
		warnings.append("`rotation` must have finite components.")
	return warnings


func _validate_property(property: Dictionary) -> void:
	_show_fields(property, {"offset": style in [Style.SLIDE, Style.SCATTER], "rotation": style in [Style.FLIP, Style.SCATTER], "scatter_seed": style == Style.SCATTER, "pivot": style in [Style.POP, Style.BOUNCE, Style.FLIP, Style.SCATTER]})


func _uses_pivot() -> bool:
	return style in [Style.POP, Style.BOUNCE, Style.FLIP, Style.SCATTER]


func _build(node: Node) -> SpireSequence:
	var label := node as SpireRichTextLabel
	var channels: Array[StringName] = [&"alpha"]
	if style in [Style.SLIDE, Style.SCATTER]: channels.append(&"offset")
	if style in [Style.POP, Style.BOUNCE]: channels.append_array([&"scale", &"pivot"])
	if style in [Style.FLIP, Style.SCATTER]: channels.append_array([&"rotation", &"pivot"])
	_capture(label, channels)
	if channels.has(&"pivot"): _initialize(label, &"pivot", pivot)
	var last := _last(label)
	var curve := _delay_curve(float(last - first_character) if style == Style.TYPEWRITER else 1.0)
	var sequence := Spire.sequence()
	_initialize(label, &"alpha", 1.0 if exiting else 0.0)
	var fade := _configure(label.fade_chars(first_character, last, 0.0 if exiting else 1.0, duration, curve))
	if style == Style.TYPEWRITER:
		fade.set_ease_func(Easing.typewriter)
	sequence.append(fade)
	match style:
		Style.SLIDE:
			_initialize(label, &"offset", Vector2.ZERO if exiting else offset)
			sequence.join(_configure(label.offset_chars(first_character, last, offset if exiting else Vector2.ZERO, duration, curve)))
		Style.POP, Style.BOUNCE:
			_initialize(label, &"scale", Vector2.ONE if exiting else Vector2.ZERO)
			var tween := label.scale_chars(first_character, last, Vector2.ZERO if exiting else Vector2.ONE, duration, curve)
			var selected_ease := Spire.EASE_IN_POP if exiting else Spire.EASE_OUT_POP
			if style == Style.BOUNCE: selected_ease = Spire.EASE_IN_BOUNCE if exiting else Spire.EASE_OUT_BOUNCE
			sequence.join(tween.set_ease(selected_ease))
		Style.FLIP:
			_initialize(label, &"pivot", pivot)
			_initialize(label, &"rotation", Vector3.ZERO if exiting else rotation)
			sequence.join(_configure(label.rotate_chars(first_character, last, rotation if exiting else Vector3.ZERO, duration, curve)))
		Style.SCATTER:
			_initialize(label, &"pivot", pivot)
			var random := RandomNumberGenerator.new()
			random.seed = scatter_seed
			for index in range(first_character, last + 1):
				var displacement := Vector2(random.randf_range(-absf(offset.x), absf(offset.x)), random.randf_range(-absf(offset.y), absf(offset.y)))
				var twist := rotation * random.randf_range(-1.0, 1.0)
				label.set_char_offset(index, Vector2.ZERO if exiting else displacement)
				label.set_char_rotation(index, Vector3.ZERO if exiting else twist)
				var x := float(index - first_character) / maxf(1.0, last - first_character)
				var delay_seconds := curve.sample(x) * duration
				sequence.join(_configure(label.offset_chars(index, index, displacement if exiting else Vector2.ZERO, duration, Spire.EASE_LINEAR)).set_delay(delay_seconds))
				sequence.join(_configure(label.rotate_chars(index, index, twist if exiting else Vector3.ZERO, duration, Spire.EASE_LINEAR)).set_delay(delay_seconds))
	return sequence
