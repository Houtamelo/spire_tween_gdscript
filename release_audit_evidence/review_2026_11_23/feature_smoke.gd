extends Node

var checks := 0
var failed := 0

func check(condition: bool, message: String) -> void:
	checks += 1
	if not condition:
		failed += 1
		push_error("RELEASE_FEATURE_ASSERTION: " + message)

func _ready() -> void:
	_run.call_deferred()

func _run() -> void:
	var node := Node2D.new()
	add_child(node)
	var blueprint := TweenBlueprint.new()
	blueprint.kind = Spire.BLUEPRINT_KIND_PROPERTY
	blueprint.property_to_animate = &"position"
	blueprint.property_end_value = Vector2(10, 20)
	blueprint.property_speed_or_duration = 1.0
	blueprint.property_ease_enum = Spire.EASE_LINEAR
	blueprint.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	node.add_child(blueprint)
	var sequence := blueprint.play()
	sequence.custom_step(0.5)
	check(node.position.is_equal_approx(Vector2(5, 10)), "native property Blueprint midpoint")
	sequence.force_complete()
	check(node.position.is_equal_approx(Vector2(10, 20)), "native property Blueprint endpoint")

	var color := SpireTextEffect.new()
	color.from = Color.RED
	color.to = Color.RED
	var preset := SpireRichTextEffectPreset.new()
	preset.bbcode = "audit"
	preset.effects = [color]
	var label := SpireRichTextLabel.new()
	label.bbcode_enabled = true
	label.text = "[audit]AB[/audit]"
	label.effect_presets = [preset]
	label.size = Vector2(400, 150)
	add_child(label)
	for _frame in 6: await get_tree().process_frame
	check(label.get_parsed_text() == "AB", "runtime tag adapter recognized")
	check(label.get_char_color(0).is_equal_approx(Color.RED), "preset executes in actual glyph callback")
	check(not label.has_method("render_sample_for_test"), "customer library excludes observation helpers")
	var characters := label.offset_chars(0, 1, Vector2(10, 0), 1.0, Spire.EASE_LINEAR)
	characters.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)
	characters.custom_step(0.5)
	check(label.get_char_offset(0).is_equal_approx(Vector2(5, 0)), "first range character advances")
	check(label.get_char_offset(1).is_zero_approx(), "last range character respects start delay")
	characters.force_complete()
	check(label.get_char_offset(1).is_equal_approx(Vector2(10, 0)), "range forced endpoint")
	var code := label.color_char(0, Color.BLUE, 0.1)
	code.set_process_mode(Spire.PROCESS_MODE_MANUAL).force_complete()
	for _frame in 4: await get_tree().process_frame
	check(label.get_char_color(0).is_equal_approx(Color.BLUE), "code color runs after preset")

	var fade := SpireTextEffect.new()
	fade.kind = Spire.TEXT_EFFECT_KIND_FADE
	fade.from = 0.0
	fade.to = 1.0
	fade.duration = 0.25
	var text_blueprint := TweenBlueprint.new()
	text_blueprint.kind = Spire.BLUEPRINT_KIND_TEXT
	text_blueprint.text_effect = fade
	text_blueprint.property_start_enabled = true
	text_blueprint.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	label.add_child(text_blueprint)
	var text_sequence := text_blueprint.play()
	text_sequence.custom_step(0.125)
	check(is_equal_approx(label.get_char_alpha(0), 0.5), "native text Blueprint midpoint")
	check(is_zero_approx(label.get_char_alpha(1)), "native text Blueprint stagger")
	text_sequence.force_complete()
	check(is_equal_approx(label.get_char_alpha(1), 1.0), "native text Blueprint endpoint")
	node.queue_free()
	label.queue_free()
	for _frame in 4: await get_tree().process_frame
	print("RELEASE_FEATURE_SMOKE_RESULT checks=", checks, " failed=", failed)
	get_tree().quit(0 if failed == 0 else 1)
