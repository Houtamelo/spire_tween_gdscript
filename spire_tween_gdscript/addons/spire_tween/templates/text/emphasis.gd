@tool
extends "../common/text_template.gd"

enum Style { PUNCH_OFFSET, PUNCH_ROTATION, PUNCH_SCALE, SHAKE, COLOR_FLASH }
enum ShakeChannel { OFFSET, ROTATION, SCALE }

@export_group("Text Emphasis")
@export_storage var style: Style = Style.PUNCH_OFFSET
@export var strength := Vector3(0.0, -12.0, 0.0)
@export var shake_channel: ShakeChannel = ShakeChannel.OFFSET
@export_range(1, 60, 1) var vibrato: int = 10
@export_range(0.0, 1.0, 0.01) var elasticity: float = 0.7
@export var flash_color := Color("ffd38a")


func _parameter_warnings() -> PackedStringArray:
	var warnings := super._parameter_warnings()
	if style != Style.COLOR_FLASH and not strength.is_finite(): warnings.append("`strength` must have finite components.")
	if style != Style.COLOR_FLASH and (vibrato < 1 or vibrato * duration > 65536.0): warnings.append("`vibrato` must be positive and `vibrato * duration` must not exceed 65536.")
	if style in [Style.PUNCH_OFFSET, Style.PUNCH_ROTATION, Style.PUNCH_SCALE] and (not is_finite(elasticity) or elasticity < 0.0 or elasticity > 1.0): warnings.append("`elasticity` must be finite and within 0..1.")
	if style == Style.COLOR_FLASH and not _finite_color(flash_color): warnings.append("`flash_color` must have finite components.")
	return warnings


func _validate_property(property: Dictionary) -> void:
	_show_fields(property, {"strength": style != Style.COLOR_FLASH, "vibrato": style != Style.COLOR_FLASH, "elasticity": style in [Style.PUNCH_OFFSET, Style.PUNCH_ROTATION, Style.PUNCH_SCALE], "shake_channel": style == Style.SHAKE, "flash_color": style == Style.COLOR_FLASH, "pivot": _uses_pivot()})


func _uses_pivot() -> bool:
	return style in [Style.PUNCH_ROTATION, Style.PUNCH_SCALE] or (style == Style.SHAKE and shake_channel != ShakeChannel.OFFSET)


func _build(node: Node) -> SpireSequence:
	var label := node as SpireRichTextLabel
	var channel: StringName = &"offset"
	if style == Style.PUNCH_ROTATION or (style == Style.SHAKE and shake_channel == ShakeChannel.ROTATION): channel = &"rotation"
	if style == Style.PUNCH_SCALE or (style == Style.SHAKE and shake_channel == ShakeChannel.SCALE): channel = &"scale"
	if style == Style.COLOR_FLASH: channel = &"color"
	var channels: Array[StringName] = [channel]
	if channel in [&"rotation", &"scale"]: channels.append(&"pivot")
	_capture(label, channels)
	if channels.has(&"pivot"): _initialize(label, &"pivot", pivot)
	var last := _last(label)
	var curve := _delay_curve(0.0)
	var sequence := Spire.sequence()
	var tween: SpireCharTween
	match style:
		Style.PUNCH_OFFSET:
			tween = label.punch_chars_offset(first_character, last, Vector2(strength.x, strength.y), duration, curve, vibrato, elasticity)
		Style.PUNCH_ROTATION:
			tween = label.punch_chars_rotation(first_character, last, strength, duration, curve, vibrato, elasticity)
		Style.PUNCH_SCALE:
			tween = label.punch_chars_scale(first_character, last, Vector2(strength.x, strength.y), duration, curve, vibrato, elasticity)
		Style.SHAKE:
			match shake_channel:
				ShakeChannel.OFFSET: tween = label.shake_chars_offset(first_character, last, duration, Vector2(strength.x, strength.y).abs(), curve, vibrato)
				ShakeChannel.ROTATION: tween = label.shake_chars_rotation(first_character, last, duration, strength.abs(), curve, vibrato)
				ShakeChannel.SCALE: tween = label.shake_chars_scale(first_character, last, duration, Vector2(strength.x, strength.y).abs(), curve, vibrato)
		Style.COLOR_FLASH:
			tween = label.color_chars(first_character, last, flash_color, duration * 0.5, curve)
	sequence.append(_configure(tween))
	if style == Style.COLOR_FLASH:
		var returning := Spire.sequence()
		for values in _saved_characters:
			var index: int = values[&"index"]
			returning.join(_configure(label.color_chars(index, index, values[&"color"], duration * 0.5, Spire.EASE_LINEAR)))
		sequence.append(returning)
	return sequence
