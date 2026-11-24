extends "res://examples/tests/text_effect_presets.gd"

class DecoratedOffsetEffect extends SpireTextEffect:
	func _init() -> void:
		kind = Spire.TEXT_EFFECT_KIND_OFFSET
		from = Vector2.ZERO
		to = Vector2.ZERO
	@export var unrelated_metadata: float = INF

func _sample(n: SpireRichTextLabel, time: float, index: int = 0, outline: bool = false) -> Dictionary:
	var adapter: Object = n.call("preset_adapter_for_test", 0)
	assert_true(adapter != null, "Preset adapter must exist")
	return adapter.call("sample_for_test", time, index, outline) if adapter != null else {}

func _near(actual: float, expected: float) -> void:
	assert_true(absf(actual - expected) < 0.0003, "%s != %s" % [actual, expected])

func test_character_ranges_real_rendering() -> bool:
	# Glyph samples exist only in integration-test libraries. Keep this check in
	# the rendered suite so production addons can load the ordinary test catalog.
	var n := _label("AB", [])
	await _draw_frames()
	var before_first: Dictionary = n.render_sample_for_test(0)
	var before_last: Dictionary = n.render_sample_for_test(1)
	assert_true(not before_first.is_empty() and not before_last.is_empty())
	var delay := Curve.new()
	delay.add_point(Vector2.ZERO, 0.0, 0.5)
	delay.add_point(Vector2(1, 0.5), 0.5, 0.0)
	var tween := n.offset_chars(0, 1, Vector2(10, 0), 1, delay)
	tween.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)
	tween.custom_step(0.25)
	await _draw_frames()
	var first: Dictionary = n.render_sample_for_test(0)
	var last: Dictionary = n.render_sample_for_test(1)
	_near((first.transform.origin - before_first.transform.origin).distance_to(Vector2(2.5, 0)), 0)
	_near((last.transform.origin - before_last.transform.origin).length(), 0)
	n.free()
	return true

func test_text_presets_independent_timing() -> bool:
	var offset := Effects.make(Spire.TEXT_EFFECT_KIND_OFFSET)
	offset.to = Vector2(10, 0)
	offset.duration = 1.0
	var scale := Effects.make(Spire.TEXT_EFFECT_KIND_SCALE)
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
	var entry := Effects.make(Spire.TEXT_EFFECT_KIND_OFFSET)
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
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	color.from = Color.RED
	color.to = Color.BLUE
	var fade := Effects.make(Spire.TEXT_EFFECT_KIND_FADE)
	fade.from = 0.2
	fade.to = 0.8
	var n := _label("[test]A[/test]", [_preset("test", [color, fade])])
	assert_true(_sample(n, 0.5).color.is_equal_approx(Color(0.5, 0, 0.5, 0.5)))
	assert_true(_sample(n, 0.5, 0, true).color.is_equal_approx(Color(1, 1, 1, 0.5)))
	var rotation := Effects.make(Spire.TEXT_EFFECT_KIND_ROTATION)
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
	var zero := Effects.make(Spire.TEXT_EFFECT_KIND_SCALE)
	zero.from = Vector2.ZERO
	zero.to = Vector2.ZERO
	n.effect_presets = [_preset("test", [zero, rotation])]
	await _draw_frames()
	assert_true(_sample(n, 0.25).transform.is_finite(), "A collapsed incoming transform must remain finite")
	n.free()
	return true

func test_text_presets_punch_and_shake() -> bool:
	for kind in ENTRY_KINDS.slice(5):
		var entry: SpireTextEffect = Effects.make(kind)
		entry.set("vibrato", 4)
		var is_rotation: bool = Effects.is_rotation(kind)
		var strength := Vector3(0.2, 0.3, 0.4) if is_rotation else Vector3(0.25, 0.5, 0)
		entry.set("punch" if Effects.is_punch(kind) else "strength", strength)
		var n := _label("[test]A[/test]", [_preset("test", [entry])])
		var first: Transform2D = _sample(n, 0.1).transform
		var repeated: Transform2D = _sample(n, 0.1).transform
		assert_true(first.is_equal_approx(repeated), str(kind) + " sampling must be stable")
		assert_true(first.is_equal_approx(_sample(n, 0.1, 0, true).transform), str(kind) + " outline and fill must agree")
		assert_true(not first.is_equal_approx(Transform2D.IDENTITY), str(kind) + " should deform glyphs")
		assert_true(_sample(n, 0).transform.is_equal_approx(Transform2D.IDENTITY))
		assert_true(_sample(n, 1).transform.is_equal_approx(Transform2D.IDENTITY))
		if kind == Spire.TEXT_EFFECT_KIND_PUNCH_OFFSET:
			_near(first.origin.x, 0.25)
			_near(first.origin.y, 0.5)
		if kind == Spire.TEXT_EFFECT_KIND_PUNCH_SCALE:
			_near(first.x.x, 1.25)
			_near(first.y.y, 1.5)
		entry.set("loop_mode", Spire.LOOP_MODE_YOYO)
		await _draw_frames()
		assert_true(_sample(n, 0.3).transform.is_equal_approx(_sample(n, 1.7).transform))
		n.free()
	return true

func test_text_presets_rendered_order() -> bool:
	var offset := Effects.make(Spire.TEXT_EFFECT_KIND_OFFSET)
	offset.from = Vector2(10, 20)
	offset.to = offset.from
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
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

func test_scroll_changes_preserve_character_rendering() -> bool:
	for with_preset in [false, true]:
		var offset := Effects.make(Spire.TEXT_EFFECT_KIND_OFFSET)
		offset.from = Vector2(10, 20)
		offset.to = offset.from
		var presets: Array[SpireRichTextEffectPreset] = []
		if with_preset:
			presets.append(_preset("test", [offset]))
		var n := _label("[test]A[/test]" if with_preset else "A", presets)
		await _draw_frames()
		var original: Dictionary = n.render_sample_for_test(0)
		assert_true(not original.is_empty(), "The original glyph must render")
		for scrolling in [false, true]:
			n.scroll_active = scrolling
			var delta := Vector2(3, 7) if scrolling else Vector2(11, 13)
			n.set_char_offset(0, delta)
			await _draw_frames()
			assert_true(n.get_char_offset(0) == delta, "Rebuilding the stack must preserve character state")
			var sample: Dictionary = n.render_sample_for_test(0)
			assert_true(not sample.is_empty(), "Changing scroll_active must retain real glyph callbacks")
			if not original.is_empty() and not sample.is_empty():
				assert_true(sample.transform.origin.is_equal_approx(original.transform.origin + delta), "Scrolling changes must preserve presets and render new character offsets")
		n.free()
	return true

func test_text_presets_pause() -> bool:
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
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
		[Effects.make(Spire.TEXT_EFFECT_KIND_OFFSET), "character_phase_offset", INF],
		[Effects.make(Spire.TEXT_EFFECT_KIND_PUNCH_OFFSET), "elasticity", -1.0],
		[Effects.make(Spire.TEXT_EFFECT_KIND_PUNCH_ROTATION), "vibrato", 0],
		[Effects.make(Spire.TEXT_EFFECT_KIND_SHAKE_SCALE), "randomness", 181.0],
		[Effects.make(Spire.TEXT_EFFECT_KIND_SHAKE_OFFSET), "strength", Vector3(-1, 1, 0)],
		[Effects.make(Spire.TEXT_EFFECT_KIND_SHAKE_ROTATION), "strength", Vector3(1, -1, 1)],
	]
	for data in cases:
		var entry: Resource = data[0]
		entry.set(data[1], data[2])
		var n := _label("[test]A[/test]", [_preset("test", [entry])])
		assert_true(_sample(n, 0.15).transform.is_equal_approx(Transform2D.IDENTITY))
		n.free()
	return true

func test_text_presets_typed_compilation_and_recovery() -> bool:
	for kind in ENTRY_KINDS:
		var entry: SpireTextEffect = Effects.make(kind)
		var rotation: bool = Effects.is_rotation(kind)
		if Effects.is_punch(kind) or Effects.is_shake(kind):
			entry.set("vibrato", 4)
			entry.set("punch" if Effects.is_punch(kind) else "strength", Vector3(0.2, 0.3, 0.4) if rotation else Vector3(0.25, 0.5, 0))
		elif kind == Spire.TEXT_EFFECT_KIND_COLOR:
			entry.set("to", Color.RED)
		elif kind != Spire.TEXT_EFFECT_KIND_FADE:
			entry.set("to", Vector3(0.2, 0.3, 0.4) if rotation else Vector2(2, 3))
		var n := _label("[test]A[/test]", [_preset("test", [entry])])
		var valid := _sample(n, 0.1)
		assert_true(not valid.transform.is_equal_approx(Transform2D.IDENTITY) or not valid.color.is_equal_approx(Color.WHITE), str(kind) + " must compile")
		for property in entry.get_property_list():
			var field: String = property.name
			if property.usage & PROPERTY_USAGE_EDITOR == 0: continue
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
			if field == "strength": invalid_values.append(Vector3(-1, 0, 0) if rotation else Vector3(0, -1, 0))
			for invalid in invalid_values:
				entry.set(field, invalid)
				n.refresh()
				var skipped := _sample(n, 0.1)
				assert_true(skipped.transform.is_equal_approx(Transform2D.IDENTITY) and skipped.color == Color.WHITE, "%s.%s must reject %s" % [kind, field, invalid])
				entry.set(field, original)
				n.refresh()
				var recovered := _sample(n, 0.1)
				# Shake paths receive a fresh seed when recompiled; their exact values can change.
				assert_true(not recovered.transform.is_equal_approx(Transform2D.IDENTITY) or not recovered.color.is_equal_approx(Color.WHITE), str(kind) + " must recover after fixing " + field)
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
	var invalid := Effects.make(Spire.TEXT_EFFECT_KIND_FADE)
	invalid.duration = 0.0
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	color.from = Color.GREEN
	color.to = Color.GREEN
	var offset := Effects.make(Spire.TEXT_EFFECT_KIND_OFFSET)
	offset.from = Vector2(4, 7)
	offset.to = offset.from
	var n := _label("[test]A[/test]", [_preset("test", [invalid, color, null, offset])])
	await _draw_frames()
	assert_eq(n.get_parsed_text(), "A")
	assert_eq(n.get_char_color(0), Color.GREEN)
	var sample := _sample(n, 0.25)
	assert_eq(sample.color, Color.GREEN)
	assert_true(sample.transform.origin == Vector2(4, 7), "Invalid entries must not prevent later valid effects from being compiled")
	n.free()
	return true

func test_text_presets_nested_clusters() -> bool:
	var offset := Effects.make(Spire.TEXT_EFFECT_KIND_OFFSET)
	offset.from = Vector2(5, 0)
	offset.to = offset.from
	var scale := Effects.make(Spire.TEXT_EFFECT_KIND_SCALE)
	scale.from = Vector2(2, 2)
	scale.to = scale.from
	var combined_offset := Effects.make(Spire.TEXT_EFFECT_KIND_OFFSET)
	combined_offset.from = Vector2(15, 0)
	combined_offset.to = combined_offset.from
	var combined_scale := Effects.make(Spire.TEXT_EFFECT_KIND_SCALE)
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

func test_text_effect_endpoint_type_repair() -> bool:
	var entry := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	entry.from = Color.RED
	entry.to = Color.BLUE
	entry.kind = Spire.TEXT_EFFECT_KIND_OFFSET
	assert_eq(entry.from, Color.RED)
	assert_eq(entry.to, Color.BLUE)
	assert_eq(_effect_property(entry, "from").type, TYPE_NIL)
	var n := _label("[test]A[/test]", [_preset("test", [entry])])
	assert_eq(_sample(n, 0.5).transform, Transform2D.IDENTITY)
	assert_eq(entry.property_get_revert("from"), Vector2.ZERO)
	entry.from = entry.property_get_revert("from")
	entry.to = Vector2(4, 6)
	assert_eq(_effect_property(entry, "from").type, TYPE_VECTOR2)
	n.refresh()
	assert_eq(_sample(n, 0.5).transform.origin, Vector2(2, 3))
	entry.from = null
	assert_eq(_effect_property(entry, "from").type, TYPE_VECTOR2)
	n.refresh()
	assert_eq(_sample(n, 0.5).transform, Transform2D.IDENTITY)
	entry.from = Vector2.ZERO
	n.refresh()
	assert_eq(_sample(n, 0.5).transform.origin, Vector2(2, 3))
	entry.kind = Spire.TEXT_EFFECT_KIND_COLOR
	entry.from = Color.RED
	entry.to = Color.BLUE
	n.refresh()
	assert_true(_sample(n, 0.5).color.is_equal_approx(Color(0.5, 0, 0.5)))
	n.free()
	return true

func test_text_effect_ignores_inactive_fields_and_z() -> bool:
	for kind in [Spire.TEXT_EFFECT_KIND_PUNCH_OFFSET, Spire.TEXT_EFFECT_KIND_PUNCH_SCALE, Spire.TEXT_EFFECT_KIND_SHAKE_OFFSET, Spire.TEXT_EFFECT_KIND_SHAKE_SCALE]:
		var entry := Effects.make(kind)
		entry.from = "inactive"
		entry.to = Color(NAN, 0, 0)
		entry.pivot = Vector2(INF, NAN)
		entry.vibrato = 4
		if Effects.is_punch(kind):
			entry.punch = Vector3(0.25, 0.5, NAN)
			entry.strength = Vector3(INF, INF, INF)
			entry.randomness = NAN
		else:
			entry.strength = Vector3(0.25, 0.5, NAN)
			entry.punch = Vector3(INF, INF, INF)
			entry.elasticity = NAN
		# Scale uses pivot; only offset can ignore it.
		if kind in [Spire.TEXT_EFFECT_KIND_PUNCH_SCALE, Spire.TEXT_EFFECT_KIND_SHAKE_SCALE]: entry.pivot = Vector2.ZERO
		var n := _label("[test]A[/test]", [_preset("test", [entry])])
		var transform: Transform2D = _sample(n, 0.1).transform
		assert_true(transform.is_finite() and not transform.is_equal_approx(Transform2D.IDENTITY), "Unused fields/Z must not reject offset or scale")
		n.free()
	for kind in [Spire.TEXT_EFFECT_KIND_PUNCH_ROTATION, Spire.TEXT_EFFECT_KIND_SHAKE_ROTATION]:
		var entry := Effects.make(kind)
		entry.punch = Vector3(0.25, 0.5, NAN)
		entry.strength = entry.punch
		var n := _label("[test]A[/test]", [_preset("test", [entry])])
		assert_eq(_sample(n, 0.1).transform, Transform2D.IDENTITY)
		n.free()
	return true
