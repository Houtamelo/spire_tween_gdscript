extends Control

@onready var text: SpireRichTextLabel = $Text
var code_tweens: Array[SpireCharTween] = []

func _forget_finished() -> void:
	for index in range(code_tweens.size() - 1, -1, -1):
		if code_tweens[index].get_state() == Spire.STATE_STOPPED:
			code_tweens.remove_at(index)

func _on_punch_pressed() -> void:
	_forget_finished()
	for index in text.get_character_count():
		code_tweens.append(text.punch_char_offset(index, Vector2(0, -16), 0.6, 6, 0.6))

func _on_fade_pressed() -> void:
	_forget_finished()
	for index in text.get_character_count():
		code_tweens.append(text.fade_char(index, 0.0, 0.6))

func _on_reset_pressed() -> void:
	$Text/SharedEffect.stop()
	for tween in code_tweens: tween.stop()
	code_tweens.clear()
	text.reset()
