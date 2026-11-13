extends "res://examples/tests/text_effect_presets.gd"

class DecoratedOffsetEffect extends SpireTextOffsetEffect:
	@export var unrelated_metadata: float = INF

func _sample(n: SpireRichTextLabel, time: float, index: int = 0, outline: bool = false) -> Dictionary:
	var adapter: Object = n.call("preset_adapter_for_test", 0)
	assert_true(adapter != null, "Preset adapter must exist")
	return adapter.call("sample_for_test", time, index, outline) if adapter != null else {}

func _near(actual: float, expected: float) -> void:
	assert_true(absf(actual - expected) < 0.0003, "%s != %s" % [actual, expected])

func test_text_presets_independent_timing() -> bool:
	var offset := SpireTextOffsetEffect.new()
	offset.to = Vector2(10, 0)
	offset.duration = 1.0
	var scale := SpireTextScaleEffect.new()
	scale.to = Vector2(3, 3)
	scale.duration = 2.0
	var n := _label("[test]A[/test]", [_preset("test", [offset, scale])])
	var transform: Transform2D = _sample(n, 0.5).transform
	_near(transform.origin.x, 5)
	_near(transform.x.x, 1.5)
	scale.ease = Spire.EASE_IN_QUAD
	await _draw_frames()
	transform = _sample(n, 0.5).transform
	_near(transform.x.x, 1.125)
	n.free()
	return true

func test_text_presets_loop_boundaries() -> bool:
	var entry := SpireTextOffsetEffect.new()
	entry.from = Vector2(1, 0)
	entry.to = Vector2(3, 0)
	var n := _label("[test]A[/test]", [_preset("test", [entry])])
	_near(_sample(n, 1.0).transform.origin.x, 1)
	entry.loop_mode = Spire.LOOP_MODE_YOYO
	await _draw_frames()
	_near(_sample(n, 1.0).transform.origin.x, 3)
	_near(_sample(n, 1.25).transform.origin.x, 2.5)
	_near(_sample(n, 2.0).transform.origin.x, 1)
	entry.loop_mode = Spire.LOOP_MODE_INCREMENTAL
	await _draw_frames()
	_near(_sample(n, 1.5).transform.origin.x, 4)
	_near(_sample(n, 100.5).transform.origin.x, 202)
	entry.character_phase_offset = 0.25
	await _draw_frames()
	_near(_sample(n, 0, 2).transform.origin.x, 2)
	entry.character_phase_offset = -0.25
	entry.loop_mode = Spire.LOOP_MODE_RESTART
	await _draw_frames()
	_near(_sample(n, 0, 1).transform.origin.x, 2.5)
	n.free()
	return true

func test_text_presets_all_basic_channels() -> bool:
	var color := SpireTextColorEffect.new()
	color.from = Color.RED
	color.to = Color.BLUE
	var fade := SpireTextFadeEffect.new()
	fade.from = 0.2
	fade.to = 0.8
	var n := _label("[test]A[/test]", [_preset("test", [color, fade])])
	assert_true(_sample(n, 0.5).color.is_equal_approx(Color(0.5, 0, 0.5, 0.5)))
	assert_true(_sample(n, 0.5, 0, true).color.is_equal_approx(Color(1, 1, 1, 0.5)))
	var rotation := SpireTextRotationEffect.new()
	rotation.to = Vector3(0, 0, TAU)
	rotation.rotate_mode = Spire.ROTATE_MODE_FAST_BEYOND_360
	n.effect_presets = [_preset("test", [rotation])]
	await _draw_frames()
	var transform: Transform2D = _sample(n, 0.25).transform
	_near(transform.x.x, 0)
	_near(transform.x.y, 1)
	rotation.pivot = Vector2(10, 0)
	await _draw_frames()
	transform = _sample(n, 0.25).transform
	_near(transform.origin.x, 10)
	_near(transform.origin.y, -10)
	var zero := SpireTextScaleEffect.new()
	zero.from = Vector2.ZERO
	zero.to = Vector2.ZERO
	n.effect_presets = [_preset("test", [zero, rotation])]
	await _draw_frames()
	assert_true(_sample(n, 0.25).transform.is_finite(), "A collapsed incoming transform must remain finite")
	n.free()
	return true

func test_text_presets_punch_and_shake() -> bool:
	for type in ENTRY_TYPES.slice(5):
		var entry: Resource = ClassDB.instantiate(type)
		entry.set("vibrato", 4)
		var is_rotation: bool = "Rotation" in type
		var strength: Variant = Vector3(0.2, 0.3, 0.4) if is_rotation else Vector2(0.25, 0.5)
		entry.set("punch" if "Punch" in type else "strength", strength)
		var n := _label("[test]A[/test]", [_preset("test", [entry])])
		var first: Transform2D = _sample(n, 0.1).transform
		var repeated: Transform2D = _sample(n, 0.1).transform
		assert_true(first.is_equal_approx(repeated), type + " sampling must be stable")
		assert_true(first.is_equal_approx(_sample(n, 0.1, 0, true).transform), type + " outline and fill must agree")
		assert_true(not first.is_equal_approx(Transform2D.IDENTITY), type + " should deform glyphs")
		assert_true(_sample(n, 0).transform.is_equal_approx(Transform2D.IDENTITY))
		assert_true(_sample(n, 1).transform.is_equal_approx(Transform2D.IDENTITY))
		if type == "SpireTextPunchOffsetEffect":
			_near(first.origin.x, 0.25)
			_near(first.origin.y, 0.5)
		if type == "SpireTextPunchScaleEffect":
			_near(first.x.x, 1.25)
			_near(first.y.y, 1.5)
		entry.set("loop_mode", Spire.LOOP_MODE_YOYO)
		await _draw_frames()
		assert_true(_sample(n, 0.3).transform.is_equal_approx(_sample(n, 1.7).transform))
		n.free()
	return true

func test_text_presets_rendered_order() -> bool:
	var offset := SpireTextOffsetEffect.new()
	offset.from = Vector2(10, 20)
	offset.to = offset.from
	var color := SpireTextColorEffect.new()
	color.from = Color.RED
	color.to = Color.RED
	var n := _label("[test]A[/test]", [_preset("test", [offset, color])])
	var plain := _label("A", [])
	n.set_char_offset(0, Vector2(3, 4))
	n.set_char_color(0, Color.BLUE)
	n.set_char_alpha(0, 0.3)
	for _frame in 6:
		await _draw_frames()
		var actual: Dictionary = n.call("render_sample_for_test", 0)
		var baseline: Dictionary = plain.call("render_sample_for_test", 0)
		assert_true(not actual.is_empty() and not baseline.is_empty(), "Real glyph draw callbacks must run")
		if not actual.is_empty() and not baseline.is_empty():
			var delta: Vector2 = actual.transform.origin - baseline.transform.origin
			_near(delta.x, 13)
			_near(delta.y, 24)
			assert_true(actual.color.is_equal_approx(Color(0, 0, 1, 0.3)))
		assert_eq(n.get_char_offset(0), Vector2(3, 4))
	n.reset()
	await _draw_frames()
	assert_eq(n.call("render_sample_for_test", 0).color, Color.RED)
	n.free()
	plain.free()
	return true

func test_text_observation_does_not_refresh() -> bool:
	var n := _label("A", [])
	await _draw_frames()
	var before: Dictionary = n.call("render_sample_for_test", 0)
	assert_true(not before.is_empty())
	n.text = "BC"
	n.append_text("SENTINEL")
	assert_eq(n.call("render_sample_for_test", 0), before)
	assert_eq(n.call("preset_adapter_for_test", 0), null)
	assert_eq(n.get_character_count(), 1)
	assert_true(n.get_parsed_text() == "BCSENTINEL", "Observations must not trigger content rebuilding")
	n.refresh()
	assert_eq(n.get_character_count(), 2)
	assert_true(n.call("render_sample_for_test", 0).is_empty())
	await _draw_frames()
	assert_true(not n.call("render_sample_for_test", 0).is_empty())
	n.free()
	return true

func test_text_presets_pause() -> bool:
	var color := SpireTextColorEffect.new()
	color.from = Color.RED
	color.to = Color.BLUE
	color.duration = 10
	var n := _label("[test]A[/test]", [_preset("test", [color])])
	await _draw_frames()
	get_tree().paused = true
	await _draw_frames()
	var paused := n.get_char_color(0)
	await _draw_frames()
	assert_eq(n.get_char_color(0), paused)
	n.process_mode = Node.PROCESS_MODE_ALWAYS
	await _draw_frames()
	assert_true(not n.get_char_color(0).is_equal_approx(paused))
	get_tree().paused = false
	n.free()
	return true

func test_text_presets_invalid_parameters() -> bool:
	var cases: Array = [
		[SpireTextOffsetEffect.new(), "character_phase_offset", INF],
		[SpireTextPunchOffsetEffect.new(), "elasticity", -1.0],
		[SpireTextPunchRotationEffect.new(), "vibrato", 0],
		[SpireTextShakeScaleEffect.new(), "randomness", 181.0],
		[SpireTextShakeOffsetEffect.new(), "strength", Vector2(-1, 1)],
		[SpireTextShakeRotationEffect.new(), "strength", Vector3(1, -1, 1)],
	]
	for data in cases:
		var entry: Resource = data[0]
		entry.set(data[1], data[2])
		var n := _label("[test]A[/test]", [_preset("test", [entry])])
		assert_true(_sample(n, 0.15).transform.is_equal_approx(Transform2D.IDENTITY))
		n.free()
	return true

func test_text_presets_typed_compilation_and_recovery() -> bool:
	for type in ENTRY_TYPES:
		var entry: Resource = ClassDB.instantiate(type)
		var rotation: bool = "Rotation" in type
		if "Punch" in type or "Shake" in type:
			entry.set("vibrato", 4)
			entry.set("punch" if "Punch" in type else "strength", Vector3(0.2, 0.3, 0.4) if rotation else Vector2(0.25, 0.5))
		elif type == "SpireTextColorEffect":
			entry.set("to", Color.RED)
		elif type != "SpireTextFadeEffect":
			entry.set("to", Vector3(0.2, 0.3, 0.4) if rotation else Vector2(2, 3))
		var n := _label("[test]A[/test]", [_preset("test", [entry])])
		var valid := _sample(n, 0.1)
		assert_true(not valid.transform.is_equal_approx(Transform2D.IDENTITY) or not valid.color.is_equal_approx(Color.WHITE), type + " must compile")
		for property in entry.get_property_list():
			var field: String = property.name
			if field not in ["duration", "character_phase_offset", "from", "to", "pivot", "punch", "strength", "elasticity", "randomness", "vibrato"]:
				continue
			var original: Variant = entry.get(field)
			var invalid_values: Array = []
			match typeof(original):
				TYPE_FLOAT: invalid_values = [NAN, INF]
				TYPE_VECTOR2: invalid_values = [Vector2(NAN, 1)]
				TYPE_VECTOR3: invalid_values = [Vector3(0, INF, 0)]
				TYPE_COLOR: invalid_values = [Color(0, 0, NAN)]
			if field == "duration": invalid_values.append_array([0.0, -1.0])
			if field == "vibrato": invalid_values = [0, 65537]
			if field == "elasticity": invalid_values.append_array([-0.1, 1.1])
			if field == "randomness": invalid_values.append_array([-1.0, 181.0])
			if field == "strength": invalid_values.append(Vector3(-1, 0, 0) if rotation else Vector2(0, -1))
			for invalid in invalid_values:
				entry.set(field, invalid)
				n.refresh()
				var skipped := _sample(n, 0.1)
				assert_true(skipped.transform.is_equal_approx(Transform2D.IDENTITY) and skipped.color == Color.WHITE, "%s.%s must reject %s" % [type, field, invalid])
				entry.set(field, original)
				n.refresh()
				var recovered := _sample(n, 0.1)
				# Shake paths receive a fresh seed when recompiled; their exact values can change.
				assert_true(not recovered.transform.is_equal_approx(Transform2D.IDENTITY) or not recovered.color.is_equal_approx(Color.WHITE), type + " must recover after fixing " + field)
		n.free()
	return true

func test_text_presets_extended_resource_metadata() -> bool:
	var entry := DecoratedOffsetEffect.new()
	entry.from = Vector2(2, 4)
	entry.to = Vector2(6, 8)
	var n := _label("[test]A[/test]", [_preset("test", [entry])])
	assert_eq(_sample(n, 0.5).transform.origin, Vector2(4, 6))
	entry.to = Vector2(NAN, 8)
	n.refresh()
	assert_eq(_sample(n, 0.5).transform, Transform2D.IDENTITY)
	entry.to = Vector2(10, 12)
	n.refresh()
	assert_eq(_sample(n, 0.5).transform.origin, Vector2(6, 8))
	n.free()
	return true

func test_text_presets_invalid_entries_preserve_valid_entries() -> bool:
	var invalid := SpireTextFadeEffect.new()
	invalid.duration = 0.0
	var color := SpireTextColorEffect.new()
	color.from = Color.GREEN
	color.to = Color.GREEN
	var offset := SpireTextOffsetEffect.new()
	offset.from = Vector2(4, 7)
	offset.to = offset.from
	var n := _label("[test]A[/test]", [_preset("test", [invalid, color, Resource.new(), offset])])
	await _draw_frames()
	assert_eq(n.get_parsed_text(), "A")
	assert_eq(n.get_char_color(0), Color.GREEN)
	var sample := _sample(n, 0.25)
	assert_eq(sample.color, Color.GREEN)
	assert_true(sample.transform.origin == Vector2(4, 7), "Invalid entries must not prevent later valid effects from being compiled")
	n.free()
	return true

func test_text_presets_nested_clusters() -> bool:
	var offset := SpireTextOffsetEffect.new()
	offset.from = Vector2(5, 0)
	offset.to = offset.from
	var scale := SpireTextScaleEffect.new()
	scale.from = Vector2(2, 2)
	scale.to = scale.from
	var combined_offset := SpireTextOffsetEffect.new()
	combined_offset.from = Vector2(15, 0)
	combined_offset.to = combined_offset.from
	var combined_scale := SpireTextScaleEffect.new()
	combined_scale.from = Vector2(4, 4)
	combined_scale.to = combined_scale.from
	var nested := _label("[nested][nested]x\u0301[/nested][/nested]", [_preset("nested", [offset, scale])])
	var combined := _label("[combined]x\u0301[/combined]", [_preset("combined", [combined_offset, combined_scale])])
	var code := _label("[nested]x\u0301[/nested]", [_preset("nested", [offset, scale])])
	code.set_char_offset(0, Vector2(5, 0))
	code.set_char_scale(0, Vector2(2, 2))
	await _draw_frames()
	assert_eq(nested.get_character_count(), 2)
	var actual: Dictionary = nested.call("render_sample_for_test", 0)
	var expected: Dictionary = combined.call("render_sample_for_test", 0)
	assert_true(not actual.is_empty() and not expected.is_empty())
	if not actual.is_empty() and not expected.is_empty():
		assert_true(actual.transform.is_equal_approx(expected.transform), "Nested combining cluster: actual=%s expected=%s" % [actual.transform, expected.transform])
		var code_sample: Dictionary = code.call("render_sample_for_test", 0)
		assert_true(code_sample.transform.is_equal_approx(expected.transform), "Code transforms must compose in the preset's coordinate system")
	nested.free()
	combined.free()
	code.free()
	return true
