@tool
extends "template.gd"
## Character templates use native Char tweens. duration is seconds per character
## per leg; start_delay_curve samples multiply that duration, and may exceed 1.

@export_group("Character Selection")
@export_range(0, 1000, 1, "or_greater") var first_character: int = 0
@export_range(-1, 1000, 1, "or_greater") var last_character: int = -1
@export var start_delay_curve: Curve
@export var pivot := Vector2(10.0, -12.0)

var _saved_characters: Array[Dictionary] = []


func _capture_target_rest_state(node: Node) -> Callable:
	return node.capture_character_rest_state(first_character, last_character)


func _target_warnings(node: Node) -> PackedStringArray:
	var warnings := PackedStringArray()
	if not node is SpireRichTextLabel:
		warnings.append("`target` must be a SpireRichTextLabel.")
	elif first_character >= node.get_total_character_count() or last_character >= node.get_total_character_count():
		warnings.append("`first_character` and `last_character` must select existing logical characters; -1 selects through the end.")
	return warnings


func _parameter_warnings() -> PackedStringArray:
	var warnings := PackedStringArray()
	if first_character < 0 or last_character < -1 or (last_character != -1 and last_character < first_character):
		warnings.append("`first_character` must be non-negative; `last_character` must be -1 or at least `first_character`.")
	if _uses_pivot() and not _finite_vector(pivot):
		warnings.append("`pivot` must have finite components.")
	var label := get_resolved_target() as SpireRichTextLabel
	if label != null and start_delay_curve != null:
		var last := _last(label)
		for index in range(first_character, last + 1):
			var x := float(index - first_character) / maxf(1.0, last - first_character)
			var sample := start_delay_curve.sample(x)
			if not is_finite(sample) or sample < 0.0 or not is_finite(sample * duration):
				warnings.append("`start_delay_curve` must produce finite, non-negative character start delays.")
				break
	return warnings


func _uses_pivot() -> bool:
	return false


func _last(label: SpireRichTextLabel) -> int:
	return label.get_character_count() - 1 if last_character == -1 else last_character


func _delay_curve(default_span: float = 1.0) -> Curve:
	if start_delay_curve != null:
		return start_delay_curve.duplicate()
	var curve := Curve.new()
	curve.max_value = maxf(1.0, default_span)
	curve.add_point(Vector2.ZERO, 0.0, default_span)
	curve.add_point(Vector2(1.0, default_span), default_span, 0.0)
	return curve


func _capture(label: SpireRichTextLabel, channels: Array[StringName]) -> void:
	# refresh() force-resets every channel, including those owned by other effects.
	# Normal label processing synchronizes text; callers allow a draw before play.
	_saved_characters.clear()
	for index in range(first_character, _last(label) + 1):
		var values := {&"index": index}
		for channel in channels:
			values[channel] = label.call("get_char_" + channel, index)
		_saved_characters.append(values)


func _initialize(label: SpireRichTextLabel, channel: StringName, value: Variant) -> void:
	for index in range(first_character, _last(label) + 1):
		label.call("set_char_" + channel, index, value)


func _restore_saved() -> void:
	super._restore_saved()
	if _saved_target == null:
		return
	var label := _saved_target.get_ref() as SpireRichTextLabel
	if not is_instance_valid(label):
		return
	for values in _saved_characters:
		var index: int = values[&"index"]
		if not label.is_valid_char(index):
			continue
		for channel: StringName in values:
			if channel != &"index":
				label.call("set_char_" + channel, index, values[channel])


func _clear_saved_state() -> void:
	super._clear_saved_state()
	_saved_characters.clear()


func _configure(tween: SpireCharTween, looping: bool = false) -> SpireCharTween:
	tween.set_ease(ease)
	if looping:
		tween.set_loops(-1, Spire.LOOP_MODE_YOYO)
	return tween
