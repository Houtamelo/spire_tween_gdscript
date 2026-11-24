extends TestBase

func _label(text: String, bbcode: bool = false) -> SpireRichTextLabel:
	var n := SpireRichTextLabel.new()
	n.bbcode_enabled = bbcode
	n.text = text
	n.size = Vector2(500, 140)
	n.add_theme_color_override("default_color", Color.WHITE)
	n.add_theme_font_size_override("normal_font_size", 36)
	add_child(n)
	return n

func _manual(t: SpireCharTween) -> SpireCharTween:
	return t.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)

func _near(a: float, b: float) -> void:
	assert_true(abs(a - b) < 0.0002, str(a) + " != " + str(b))

func _v2(a: Vector2, b: Vector2) -> void:
	_near(a.distance_to(b), 0)

func _v3(a: Vector3, b: Vector3) -> void:
	_near(a.distance_to(b), 0)

func _overshoot_ease(p: float) -> float:
	return p + 3 * p * (1 - p)

func _delay_curve(from: float, to: float) -> Curve:
	var curve := Curve.new()
	curve.min_value = minf(0, minf(from, to))
	curve.max_value = maxf(1, maxf(from, to))
	curve.add_point(Vector2(0, from), 0, to - from)
	curve.add_point(Vector2(1, to), to - from, 0)
	return curve

func test_character_ranges_basic_curves() -> bool:
	var n := _label("ABC")
	var t := _manual(n.offset_chars(0, -1, Vector2(10, 0), 1, Spire.EASE_LINEAR))
	_near(t.get_duration(), 2)
	t.custom_step(0.5)
	_v2(n.get_char_offset(0), Vector2(5, 0))
	_v2(n.get_char_offset(1), Vector2.ZERO)
	t.stop()
	n.reset()
	var curved := _manual(n.offset_chars(0, -1, Vector2(10, 0), 1, Spire.EASE_OUT_QUAD))
	curved.custom_step(1)
	_v2(n.get_char_offset(1), Vector2(2.5, 0))
	_v2(n.get_char_offset(2), Vector2.ZERO)
	curved.stop()
	n.reset()
	n.text = "ABCD"
	n.refresh()
	var invalid := _manual(n.offset_chars(0, -1, Vector2.ONE, 1, Spire.EASE_PUNCH))
	assert_true(not invalid.custom_step(0.25))
	_v2(n.get_char_offset(0), Vector2.ZERO)
	n.free()
	return true

func test_character_ranges_duration_and_snapshot() -> bool:
	var n := _label("ABC")
	var curve := _delay_curve(0, 1)
	var t := _manual(n.offset_chars(0, -1, Vector2(10, 0), 1, curve))
	_near(t.get_character_duration(), 1)
	_near(t.get_duration(), 2)
	var events: Array[String] = []
	t.loop_finished.connect(func(): events.append("loop"))
	t.stopped.connect(func(): events.append("stop"))
	t.finished.connect(func(): events.append("finish"))
	t.custom_step(0.25)
	_v2(n.get_char_offset(0), Vector2(2.5, 0))
	_v2(n.get_char_offset(1), Vector2.ZERO)
	curve.clear_points()
	curve.add_point(Vector2.ZERO)
	t.custom_step(1.0)
	_v2(n.get_char_offset(0), Vector2(10, 0))
	_v2(n.get_char_offset(1), Vector2(7.5, 0))
	_v2(n.get_char_offset(2), Vector2(2.5, 0))
	assert_eq(t.get_loops_finished(), 0)
	assert_eq(events, [])
	assert_true(not t.custom_step(0.75))
	assert_eq(events, ["loop", "stop", "finish"])
	n.reset()
	var next := _manual(n.offset_chars(0, -1, Vector2(10, 0), 1, curve))
	_near(next.get_duration(), 1)
	n.free()
	return true

func test_character_ranges_singleton_reverse_and_middle_delays() -> bool:
	var n := _label("ABCDE")
	var reverse := _manual(n.offset_chars(1, 3, Vector2(10, 0), 1, _delay_curve(1, 0)))
	reverse.custom_step(0.25)
	_v2(n.get_char_offset(0), Vector2.ZERO)
	_v2(n.get_char_offset(1), Vector2.ZERO)
	_v2(n.get_char_offset(3), Vector2(2.5, 0))
	reverse.stop()
	n.reset()
	var middle := _delay_curve(0, 0)
	middle.add_point(Vector2(0.5, 1))
	var wave := _manual(n.offset_chars(1, 3, Vector2(10, 0), 2, middle))
	_near(wave.get_duration(), 4)
	wave.custom_step(1)
	_v2(n.get_char_offset(1), Vector2(5, 0))
	_v2(n.get_char_offset(2), Vector2.ZERO)
	_v2(n.get_char_offset(3), Vector2(5, 0))
	wave.stop()
	n.reset()
	var single := _manual(n.offset_chars(2, 2, Vector2(10, 0), 0.5, _delay_curve(2, 0)))
	_near(single.get_duration(), 1.5)
	single.custom_step(0.75)
	_v2(n.get_char_offset(2), Vector2.ZERO)
	single.custom_step(0.5)
	_v2(n.get_char_offset(2), Vector2(5, 0))
	single.custom_step(0.25)
	assert_true(single.is_stopped())
	n.free()
	return true

func test_character_ranges_local_easing_and_implicit_starts() -> bool:
	var n := _label("AB")
	var t := _manual(n.offset_chars(0, 1, Vector2(10, 0), 1, _delay_curve(0, 0.5)))
	t.set_ease(Spire.EASE_OUT_QUAD)
	t.custom_step(0.25)
	_v2(n.get_char_offset(0), Vector2(4.375, 0))
	n.set_char_offset(1, Vector2(2, 0))
	t.custom_step(0.5)
	_v2(n.get_char_offset(0), Vector2(9.375, 0))
	_v2(n.get_char_offset(1), Vector2(5.5, 0))
	n.free()
	return true

func test_character_ranges_continuous_loops_and_collective_events() -> bool:
	var n := _label("AB")
	var t := _manual(n.offset_chars(0, 1, Vector2(10, 0), 1, _delay_curve(0, 0.5)))
	t.set_loops(2)
	_near(t.get_duration(), 2.5)
	var loops: Array[int] = []
	t.loop_finished.connect(func(): loops.append(loops.size() + 1))
	t.custom_step(0.75)
	t.custom_step(0.25)
	t.custom_step(0.25)
	_v2(n.get_char_offset(0), Vector2(2.5, 0))
	_v2(n.get_char_offset(1), Vector2(7.5, 0))
	assert_eq(loops, [])
	t.custom_step(0.25)
	assert_eq(loops, [1])
	_v2(n.get_char_offset(0), Vector2(5, 0))
	_v2(n.get_char_offset(1), Vector2(10, 0))
	t.custom_step(0.5)
	_v2(n.get_char_offset(0), Vector2(10, 0))
	_v2(n.get_char_offset(1), Vector2(5, 0))
	t.custom_step(0.5)
	assert_eq(loops, [1, 2])
	assert_true(t.is_stopped())
	n.free()
	return true

func test_character_ranges_yoyo_incremental_and_large_steps() -> bool:
	var n := _label("AB")
	for mode in [Spire.LOOP_MODE_YOYO, Spire.LOOP_MODE_INCREMENTAL]:
		n.reset()
		var t := _manual(n.offset_chars(0, 1, Vector2(10, 0), 1, _delay_curve(0, 0.5)))
		t.set_loops(2, mode)
		for _tick in 3: t.custom_step(0.5)
		_v2(n.get_char_offset(0), Vector2(5 if mode == Spire.LOOP_MODE_YOYO else 15, 0))
		_v2(n.get_char_offset(1), Vector2(10, 0))
		for _tick in 2: t.custom_step(0.5)
		_v2(n.get_char_offset(1), Vector2(0 if mode == Spire.LOOP_MODE_YOYO else 20, 0))
		assert_true(t.is_stopped())
	n.reset()
	var infinite := _manual(n.offset_chars(0, 1, Vector2(10, 0), 1, _delay_curve(0, 0.5)))
	infinite.set_loops(-1)
	assert_true(is_inf(infinite.get_duration()))
	infinite.custom_step(8)
	assert_eq(infinite.get_loops_finished(), 1)
	infinite.custom_step(0)
	assert_eq(infinite.get_loops_finished(), 2)
	assert_true(infinite.is_playing())
	n.free()
	return true

func test_character_ranges_lifecycle_and_scaled_sequence_excess() -> bool:
	var n := _label("AB")
	var t := _manual(n.offset_chars(0, 1, Vector2(10, 0), 1, _delay_curve(0, 0.5)))
	t.custom_step(0.25)
	t.pause()
	t.custom_step(10)
	_v2(n.get_char_offset(0), Vector2(2.5, 0))
	t.play()
	t.custom_step(0.25)
	t.stop()
	_v2(n.get_char_offset(0), Vector2(5, 0))
	t.play()
	t.custom_step(0.25)
	_v2(n.get_char_offset(0), Vector2(2.5, 0))
	t.force_complete()
	_v2(n.get_char_offset(1), Vector2(10, 0))
	n.reset()
	var sprite := Node2D.new()
	add_child(sprite)
	var sequence := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
	sequence.set_default_children_ease(Spire.EASE_LINEAR)
	var range := n.offset_chars(0, 1, Vector2(10, 0), 1, _delay_curve(0, 0.5))
	range.set_delay(0.25).set_speed_scale(2)
	sequence.append(range)
	sequence.append(Spire.do_property_float(sprite, ^"position:x", 8, 1))
	sequence.custom_step(1)
	_near(sprite.position.x, 1)
	assert_true(range.is_stopped() and sequence.is_playing())
	sequence.force_complete()
	_near(sprite.position.x, 8)
	n.free()
	sprite.free()
	return true

func test_character_ranges_force_completion_and_extended_loops() -> bool:
	var n := _label("AB")
	var t := _manual(n.offset_chars(0, 1, Vector2(10, 0), 1, Spire.EASE_LINEAR))
	t.custom_step(1.25)
	n.set_char_offset(0, Vector2(20, 0))
	t.force_complete()
	_v2(n.get_char_offset(0), Vector2(20, 0))
	_v2(n.get_char_offset(1), Vector2(10, 0))
	n.reset()
	var infinite := _manual(n.offset_chars(0, 1, Vector2(10, 0), 1, Spire.EASE_LINEAR))
	infinite.set_loops(-1, Spire.LOOP_MODE_YOYO)
	infinite.custom_step(1.25)
	infinite.force_complete()
	_v2(n.get_char_offset(0), Vector2.ZERO)
	_v2(n.get_char_offset(1), Vector2(10, 0))
	n.reset()
	var extended := _manual(n.offset_chars(0, 1, Vector2(10, 0), 1, Spire.EASE_LINEAR))
	extended.custom_step(1.25)
	extended.set_loops(2)
	extended.custom_step(0.25)
	_v2(n.get_char_offset(0), Vector2(5, 0))
	extended.custom_step(0.5)
	assert_eq(extended.get_loops_finished(), 1)
	extended.custom_step(1)
	assert_eq(extended.get_loops_finished(), 2)
	assert_true(extended.is_stopped())
	n.free()
	return true

func test_character_ranges_small_positive_durations() -> bool:
	var samples: Array[float] = []
	var method := Spire.do_call_float(func(value: float): samples.append(value), 0.0, 10.0, 1.0e-10)
	method.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)
	method.custom_step(0.5e-10)
	_near(samples.back(), 5)
	method.custom_step(0.5e-10)
	assert_true(method.is_stopped())
	var n := _label("AB")
	var t := _manual(n.offset_chars(0, 1, Vector2(10, 0), 1.0e-10, _delay_curve(0, 0)))
	t.custom_step(0.5e-10)
	for index in 2: _v2(n.get_char_offset(index), Vector2(5, 0))
	t.custom_step(0.5e-10)
	assert_true(t.is_stopped())
	n.free()
	return true

func test_character_ranges_callable_delay_curve() -> bool:
	var n := _label("ABC")
	var samples: Array[float] = []
	var delays := func(x: float) -> float:
		samples.append(x)
		return x * 0.5
	var t := _manual(n.offset_chars(0, -1, Vector2(10, 0), 1, delays))
	assert_eq(samples, [0.0, 0.5, 1.0])
	_near(t.get_duration(), 1.5)
	t.custom_step(0.5)
	_v2(n.get_char_offset(0), Vector2(5, 0))
	_v2(n.get_char_offset(1), Vector2(2.5, 0))
	_v2(n.get_char_offset(2), Vector2.ZERO)
	t.custom_step(1.0)
	assert_eq(samples.size(), 3)
	assert_true(t.is_stopped())
	n.free()
	for factor in [-0.1, NAN, INF]:
		n = _label("AB")
		var invalid := func(_x: float) -> float: return factor
		t = _manual(n.offset_chars(0, -1, Vector2.ONE, 1, invalid))
		t.custom_step(0.1)
		assert_true(t.is_stopped())
		_v2(n.get_char_offset(0), Vector2.ZERO)
		n.free()
	return true

func test_character_ranges_invalid_samples_and_ranges() -> bool:
	var n := _label("ABC")
	var middle := _delay_curve(0, 0)
	middle.min_value = -1
	middle.add_point(Vector2(0.5, -0.5))
	for curve in [_delay_curve(-0.5, -0.5), middle]:
		var t := _manual(n.offset_chars(0, 2, Vector2.ONE, 1, curve))
		var finished: Array[bool] = []
		t.finished.connect(func(): finished.append(true))
		assert_true(not t.custom_step(0.25))
		assert_eq(finished, [])
		_v2(n.get_char_offset(0), Vector2.ZERO)
	var overflow := _delay_curve(2, 2)
	var huge := _manual(n.offset_chars(0, 2, Vector2.ONE, 1.0e308, overflow))
	assert_true(not huge.custom_step(0.25))
	for bounds in [Vector2i(2, 1), Vector2i(-2, 1), Vector2i(0, 9)]:
		var t := _manual(n.offset_chars(bounds.x, bounds.y, Vector2.ONE, 1, _delay_curve(0, 0)))
		assert_true(not t.custom_step(0.25))
		_v2(n.get_char_offset(0), Vector2.ZERO)
	n.free()
	return true

func test_character_ranges_missing_indices_and_target_lifetime() -> bool:
	var n := _label("ABC")
	var t := _manual(n.offset_chars(0, 2, Vector2(10, 0), 1, _delay_curve(0, 1)))
	t.custom_step(0.25)
	n.text = "A"
	n.refresh()
	t.custom_step(1)
	n.text = "XYZ"
	n.refresh()
	t.custom_step(0.25)
	_v2(n.get_char_offset(2), Vector2(5, 0))
	t.custom_step(0.5)
	assert_true(t.is_stopped())
	var freed := _manual(n.offset_chars(0, 2, Vector2.ONE, 1, _delay_curve(0, 0)))
	var events: Array[String] = []
	freed.stopped.connect(func(): events.append("stop"))
	freed.finished.connect(func(): events.append("finish"))
	n.free()
	assert_true(not freed.custom_step(0.25))
	assert_eq(events, ["stop"])
	return true

func test_character_ranges_all_effects() -> bool:
	var n := _label("AB")
	var cases := [
		["offset_chars", [Vector2(8, 5), 1, _delay_curve(0, 0)], "get_char_offset", Vector2.ZERO, Vector2(8, 5)],
		["scale_chars", [Vector2(2, 3), 1, _delay_curve(0, 0)], "get_char_scale", Vector2.ONE, Vector2(2, 3)],
		["color_chars", [Color(0.2, 0.3, 0.4), 1, _delay_curve(0, 0)], "get_char_color", Color.WHITE, Color(0.2, 0.3, 0.4)],
		["fade_chars", [0.2, 1, _delay_curve(0, 0)], "get_char_alpha", 1.0, 0.2],
		["rotate_chars", [Vector3(0.2, 0.3, 0.4), 1, _delay_curve(0, 0)], "get_char_rotation", Vector3.ZERO, Vector3(0.2, 0.3, 0.4)],
		["punch_chars_offset", [Vector2(8, 5), 1, _delay_curve(0, 0)], "get_char_offset", Vector2.ZERO, Vector2.ZERO],
		["punch_chars_scale", [Vector2(0.5, 0.4), 1, _delay_curve(0, 0)], "get_char_scale", Vector2.ONE, Vector2.ONE],
		["punch_chars_rotation", [Vector3(0.2, 0.3, 0.4), 1, _delay_curve(0, 0)], "get_char_rotation", Vector3.ZERO, Vector3.ZERO],
		["shake_chars_offset", [1, Vector2(8, 5), _delay_curve(0, 0)], "get_char_offset", Vector2.ZERO, Vector2.ZERO],
		["shake_chars_scale", [1, Vector2(0.5, 0.4), _delay_curve(0, 0)], "get_char_scale", Vector2.ONE, Vector2.ONE],
		["shake_chars_rotation", [1, Vector3(0.2, 0.3, 0.4), _delay_curve(0, 0)], "get_char_rotation", Vector3.ZERO, Vector3.ZERO],
	]
	for item in cases:
		n.reset()
		var args: Array = [0, 1]
		args.append_array(item[1])
		var t := _manual(n.callv(item[0], args))
		t.custom_step(0.173)
		for index in 2: assert_true(n.call(item[2], index) != item[3], item[0] + " must animate every selected character")
		t.force_complete()
		for index in 2:
			if item[4] is Vector2: _v2(n.call(item[2], index), item[4])
			elif item[4] is Vector3: _v3(n.call(item[2], index), item[4])
			elif item[4] is float: _near(n.call(item[2], index), item[4])
			else: assert_true(n.call(item[2], index).is_equal_approx(item[4]))
	n.free()
	return true

func test_character_ranges_impulse_blending_and_reset() -> bool:
	var n := _label("AB")
	for blendable in [true, false]:
		n.reset()
		for index in 2: n.set_char_offset(index, Vector2(10, 0))
		var t := _manual(n.punch_chars_offset(0, 1, Vector2(4, 0), 1, _delay_curve(0, 0), 4, 1, blendable))
		t.custom_step(0.1)
		for index in 2: _v2(n.get_char_offset(index), Vector2(14, 0))
		n.set_char_offset(0, Vector2(19, 0))
		t.custom_step(0.2)
		_v2(n.get_char_offset(0), Vector2(12 if blendable else 7, 0))
		_v2(n.get_char_offset(1), Vector2(7, 0))
		t.force_complete()
		_v2(n.get_char_offset(0), Vector2(15 if blendable else 10, 0))
		_v2(n.get_char_offset(1), Vector2(10, 0))
		n.reset()
		var reset := _manual(n.punch_chars_offset(0, 1, Vector2(4, 0), 1, _delay_curve(0, 0), 4, 1, blendable))
		reset.custom_step(0.1)
		n.refresh()
		reset.custom_step(0.2)
		for index in 2: _v2(n.get_char_offset(index), Vector2(-3, 0))
		n.reset()
		reset.force_complete()
		for index in 2: _v2(n.get_char_offset(index), Vector2.ZERO)
	n.free()
	return true

func test_characters_translation() -> bool:
	var locale := TranslationServer.get_locale()
	TranslationServer.set_locale("en")
	var translation := Translation.new()
	translation.set_locale("es")
	translation.add_message("SPIRE_CHAR_TRANSLATION", "[color=red]猫AB[/color]")
	TranslationServer.add_translation(translation)
	var n := _label("SPIRE_CHAR_TRANSLATION", true)
	var a := n
	TranslationServer.set_locale("es")
	for _i in 4:
		await next_frame()
	assert_eq(a.get_character_count(), 3)
	assert_eq(n.text, "SPIRE_CHAR_TRANSLATION")
	assert_eq(n.get_parsed_text(), "猫AB")
	assert_eq(a.get_char_color(0), Color.RED)
	TranslationServer.set_locale(locale)
	TranslationServer.remove_translation(translation)
	n.free()
	return true

func test_characters_rotation() -> bool:
	var n := _label("A")
	var a := n
	var angle := TAU * 2 + 0.8
	for mode in [Spire.ROTATE_MODE_FAST, Spire.ROTATE_MODE_FAST_BEYOND_360]:
		a.reset()
		var t := _manual(a.rotate_char(0, Vector3(0, 0, angle), 1, mode))
		t.custom_step(0.5)
		_near(a.get_char_rotation(0).z, 0.4 if mode == Spire.ROTATE_MODE_FAST else angle / 2)
	var from := Vector3(0.2, 0.3, 0.4)
	var by := Vector3(0.3, -0.4, 0)
	for mode in [Spire.ROTATE_MODE_WORLD_AXIS_ADD, Spire.ROTATE_MODE_LOCAL_AXIS_ADD]:
		a.set_char_rotation(0, from)
		var t := _manual(a.rotate_char(0, by, 1, mode))
		t.force_complete()
		var expected := Quaternion.from_euler(by) * Quaternion.from_euler(from) if mode == Spire.ROTATE_MODE_WORLD_AXIS_ADD else Quaternion.from_euler(from) * Quaternion.from_euler(by)
		assert_true(abs(Quaternion.from_euler(a.get_char_rotation(0)).dot(expected)) > 0.99999)
	n.free()
	return true

func test_characters_effect_refresh() -> bool:
	var n := _label("A")
	var a := n
	for blendable in [true, false]:
		a.reset()
		var t := _manual(a.punch_char_offset(0, Vector2(4, 0), 1, 4, 1, blendable))
		t.custom_step(0.1)
		_v2(a.get_char_offset(0), Vector2(4, 0))
		n.text = "B" if blendable else "C"
		n.refresh()
		_v2(a.get_char_offset(0), Vector2.ZERO)
		t.custom_step(0.2)
		_v2(a.get_char_offset(0), Vector2(-3, 0))
		a.reset()
		t.force_complete()
		_v2(a.get_char_offset(0), Vector2.ZERO)
	n.free()
	return true

func test_characters_automatic() -> bool:
	var n := _label("A")
	var a := n
	var t := a.offset_char(0, Vector2(8, 0), 0.05)
	var events := [0, 0]
	t.stopped.connect(func(): events[0] += 1)
	t.finished.connect(func(): events[1] += 1)
	for _i in 60:
		await next_frame()
		if t.get_state() == Spire.STATE_STOPPED:
			break
	assert_eq(events, [1, 1])
	_v2(a.get_char_offset(0), Vector2(8, 0))
	n.text = "[color=red]B[/color]"
	n.bbcode_enabled = true
	for _i in 4:
		await next_frame()
	# The red baseline can only be learned by the reattached real draw effect.
	assert_eq(a.get_char_color(0), Color.RED)
	_v2(a.get_char_offset(0), Vector2.ZERO)
	n.free()
	return true

func test_characters_label() -> bool:
	var n := _label("[b]AB[/b]")
	var a := n
	var count := n.get_child_count()
	assert_eq(count, 0)
	assert_true(n is RichTextLabel)
	assert_true(ClassDB.is_parent_class("SpireRichTextLabel", "RichTextLabel"))
	assert_true(not ClassDB.class_exists("SpireTextAnimator"))
	assert_true(not n.has_method("attach"))
	assert_eq(n.process_mode, Node.PROCESS_MODE_INHERIT)
	var plain := RichTextLabel.new()
	assert_eq(n.get_child_count(true), plain.get_child_count(true))
	plain.free()
	assert_eq(n.text, "[b]AB[/b]")
	assert_true(not n.bbcode_enabled)
	assert_eq(a.get_character_count(), 9)
	n.bbcode_enabled = true
	n.refresh()
	assert_eq(a.get_character_count(), 2)
	assert_eq(n.get_parsed_text(), "AB")
	var copy := n.duplicate() as SpireRichTextLabel
	assert_true(copy != null, "Character state must not block Node.duplicate")
	if copy != null:
		add_child(copy)
		var other := copy
		a.set_char_offset(1, Vector2.ONE)
		other.set_char_offset(1, Vector2(4, 0))
		_v2(a.get_char_offset(1), Vector2.ONE)
		_v2(other.get_char_offset(1), Vector2(4, 0))
		copy.free()
	assert_true(a.is_valid_char(1))
	assert_true(not a.is_valid_char(-1))
	assert_true(not a.is_valid_char(2))
	var orphan := SpireRichTextLabel.new()
	assert_eq(orphan.get_character_count(), 0)
	var invalid := orphan.offset_char(0, Vector2.ONE, 1)
	assert_eq(invalid.get_state(), Spire.STATE_PLAYING)
	invalid.force_complete()
	orphan.text = "A"
	orphan.refresh()
	assert_eq(orphan.get_character_count(), 1)
	orphan.set_char_offset(0, Vector2(3, 4))
	_v2(orphan.get_char_offset(0), Vector2(3, 4))
	orphan.process_mode = Node.PROCESS_MODE_WHEN_PAUSED
	add_child(orphan)
	assert_eq(orphan.process_mode, Node.PROCESS_MODE_WHEN_PAUSED)
	orphan.process_mode = Node.PROCESS_MODE_INHERIT
	var packed := PackedScene.new()
	assert_eq(packed.pack(orphan), OK)
	var restored := packed.instantiate() as SpireRichTextLabel
	assert_true(restored != null)
	if restored != null:
		assert_eq(restored.text, "A")
		assert_eq(restored.process_mode, Node.PROCESS_MODE_INHERIT)
		restored.refresh()
		assert_eq(restored.get_character_count(), 1)
		_v2(restored.get_char_offset(0), Vector2.ZERO)
		restored.free()
	orphan.free()
	var content := DoRichTextLabel.text(n, "Hello", 1.0)
	content.set_process_mode(Spire.PROCESS_MODE_MANUAL)
	content.force_complete()
	assert_eq(n.text, "Hello")
	n.refresh()
	assert_eq(n.get_character_count(), 5)
	n.free()
	return true

func test_characters_process_mode_and_pause() -> bool:
	var n := _label("A")
	var tween := _manual(n.offset_char(0, Vector2(8, 0), 1))
	tween.custom_step(0.25)
	_v2(n.get_char_offset(0), Vector2(2, 0))
	n.process_mode = Node.PROCESS_MODE_DISABLED
	tween.custom_step(0.25)
	_v2(n.get_char_offset(0), Vector2(2, 0))
	_near(tween.get_total_elapsed_time(), 0.25)
	n.process_mode = Node.PROCESS_MODE_INHERIT
	var was_paused := get_tree().paused
	get_tree().paused = true
	tween.custom_step(0.25)
	_v2(n.get_char_offset(0), Vector2(2, 0))
	# Detect automatic refresh through the actual RichTextLabel parsing stack.
	n.text = "B"
	n.append_text("SENTINEL")
	for _frame in 3: await get_tree().process_frame
	assert_eq(n.get_parsed_text(), "BSENTINEL")
	get_tree().paused = false
	for _frame in 3: await get_tree().process_frame
	assert_eq(n.get_parsed_text(), "B")
	tween.custom_step(0.25)
	_v2(n.get_char_offset(0), Vector2(4, 0))
	get_tree().paused = true
	n.process_mode = Node.PROCESS_MODE_ALWAYS
	tween.custom_step(0.25)
	_v2(n.get_char_offset(0), Vector2(6, 0))
	assert_eq(n.process_mode, Node.PROCESS_MODE_ALWAYS)
	get_tree().paused = was_paused
	n.free()
	return true

func test_characters_script_process_override() -> bool:
	var n := _label("A")
	n.set_script(load("res://examples/tests/rich_text_label_process_override.gd"))
	n.text = "B"
	n.append_text("SENTINEL")
	for _frame in 3: await next_frame()
	assert_eq(n.get_parsed_text(), "BSENTINEL")
	n.set_script(load("res://examples/tests/rich_text_label_no_process.gd"))
	for _frame in 3: await next_frame()
	assert_eq(n.get_parsed_text(), "B")
	n.free()
	return true

func test_characters_channels() -> bool:
	var n := _label("ABC")
	var a := n
	a.set_char_offset(0, Vector2(2, 4))
	var offset := _manual(a.offset_char(0, Vector2(10, 12), 1))
	var scale := _manual(a.scale_char(1, Vector2(3, 5), 1))
	a.set_char_color(2, Color(1, 0, 0, 0.8))
	var color := _manual(a.color_char(2, Color(0, 0, 1, 0.4), 1))
	for t in [offset, scale, color]:
		t.custom_step(0.5)
	_v2(a.get_char_offset(0), Vector2(6, 8))
	_v2(a.get_char_scale(1), Vector2(2, 3))
	_near(a.get_char_color(2).r, 0.5)
	_near(a.get_char_alpha(2), 0.6)
	var fade := _manual(a.fade_char(2, 0, 1))
	fade.custom_step(0.5)
	_near(a.get_char_alpha(2), 0.3)
	_near(a.get_char_color(2).r, 0.5)
	for t in [offset, scale, color, fade]:
		t.force_complete()
	_v2(a.get_char_offset(0), Vector2(10, 12))
	_near(a.get_char_alpha(2), 0)
	a.set_char_alpha(2, 1)
	var overshoot := _manual(a.fade_char(2, 0, 1)).set_ease_func(_overshoot_ease)
	overshoot.custom_step(0.5)
	assert_true(a.get_char_alpha(2) < 0, "Valid easing overshoot must not be treated as invalid input")
	overshoot.force_complete()
	_near(a.get_char_alpha(2), 0)
	a.set_char_color(2, Color.WHITE)
	overshoot = _manual(a.color_char(2, Color(0, 0, 1, 0), 1)).set_ease_func(_overshoot_ease)
	overshoot.custom_step(0.5)
	assert_true(a.get_char_alpha(2) < 0)
	overshoot.force_complete()
	_near(a.get_char_alpha(2), 0)
	n.free()
	return true

func test_characters_refresh() -> bool:
	var n := _label("ABC")
	var a := n
	var t := _manual(a.offset_char(2, Vector2(8, 0), 1))
	t.custom_step(0.25)
	_v2(a.get_char_offset(2), Vector2(2, 0))
	n.text = "12345"
	n.refresh()
	_v2(a.get_char_offset(2), Vector2.ZERO)
	t.custom_step(0.25)
	_v2(a.get_char_offset(2), Vector2(4, 0))
	_near(t.get_total_elapsed_time(), 0.5)
	n.size = Vector2(100, 100)
	_v2(a.get_char_offset(2), Vector2(4, 0))
	var events := [0, 0, 0]
	t.stopped.connect(func(): events[0] += 1)
	t.finished.connect(func(): events[1] += 1)
	t.loop_finished.connect(func(): events[2] += 1)
	n.text = "A"
	n.refresh()
	assert_true(t.custom_step(0.25))
	assert_eq(events, [0, 0, 0])
	_near(t.get_total_elapsed_time(), 0.75)
	_v2(a.get_char_offset(0), Vector2.ZERO)
	n.text = "ABC"
	n.refresh()
	assert_true(not t.custom_step(0.25))
	assert_eq(events, [1, 1, 1])
	_v2(a.get_char_offset(2), Vector2(8, 0))
	assert_eq(t.get_state(), Spire.STATE_STOPPED)
	var invalid := _manual(a.offset_char(-1, Vector2.ONE, 1))
	assert_eq(invalid.get_state(), Spire.STATE_PLAYING)
	invalid.force_complete()
	assert_eq(invalid.get_state(), Spire.STATE_STOPPED)
	assert_eq(n.text, "ABC")
	n.free()
	return true

func test_characters_lifecycle() -> bool:
	var n := _label("AB")
	var a := n
	var t := _manual(a.offset_char(0, Vector2(8, 0), 1))
	t.custom_step(0.25)
	t.pause()
	t.custom_step(0.5)
	_v2(a.get_char_offset(0), Vector2(2, 0))
	t.play()
	t.custom_step(0.25)
	t.stop()
	t.force_complete()
	_v2(a.get_char_offset(0), Vector2(4, 0))
	t.play()
	t.custom_step(0.25)
	a.reset()
	_v2(a.get_char_offset(0), Vector2.ZERO)
	t.custom_step(0.25)
	_v2(a.get_char_offset(0), Vector2(4, 0))
	var events := [0, 0]
	t.stopped.connect(func(): events[0] += 1)
	t.finished.connect(func(): events[1] += 1)
	n.free()
	assert_true(not t.custom_step(1))
	assert_eq(events, [1, 0])
	n = _label("A")
	a = n
	t = _manual(a.fade_char(0, 0, 1))
	n.free()
	assert_true(not t.custom_step(1))
	assert_eq(t.get_state(), Spire.STATE_STOPPED)
	return true

func test_characters_loops_sequence() -> bool:
	var n := _label("A")
	var a := n
	for mode in [Spire.LOOP_MODE_RESTART, Spire.LOOP_MODE_YOYO, Spire.LOOP_MODE_INCREMENTAL]:
		a.reset()
		var t := _manual(a.offset_char(0, Vector2(8, 0), 1))
		t.set_loops(2, mode)
		t.custom_step(1)
		t.custom_step(0.25)
		var x: float = 2 if mode == Spire.LOOP_MODE_RESTART else (6 if mode == Spire.LOOP_MODE_YOYO else 10)
		_near(a.get_char_offset(0).x, x)
		t.stop()
	a.reset()
	var child := _manual(a.offset_char(0, Vector2(8, 0), 1))
	child.set_speed_scale(2)
	var sequence := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
	sequence.set_default_children_ease(Spire.EASE_LINEAR)
	sequence.append(child).append(a.offset_char(0, Vector2(16, 0), 1))
	sequence.custom_step(0.75)
	_near(a.get_char_offset(0).x, 10)
	n.free()
	return true

func test_characters_effects() -> bool:
	var n := _label("A")
	var a := n
	var effects := [
		_manual(a.punch_char_offset(0, Vector2(4, 0), 1, 4, 1)),
		_manual(a.punch_char_scale(0, Vector2(0, 2), 1, 4, 1)),
		_manual(a.punch_char_rotation(0, Vector3(0.3, 0, 0), 1, 4, 1)),
	]
	for t in effects:
		t.custom_step(0.1)
	_v2(a.get_char_offset(0), Vector2(4, 0))
	_v2(a.get_char_scale(0), Vector2(1, 3))
	_v3(a.get_char_rotation(0), Vector3(0.3, 0, 0))
	for t in effects:
		t.force_complete()
	_v2(a.get_char_offset(0), Vector2.ZERO)
	_v2(a.get_char_scale(0), Vector2.ONE)
	_v3(a.get_char_rotation(0), Vector3.ZERO)
	effects = [
		_manual(a.shake_char_offset(0, 1, Vector2(4, 0), 4, 0, false)),
		_manual(a.shake_char_scale(0, 1, Vector2(0, 2), 4, 0, false, Spire.SHAKE_RANDOMNESS_MODE_HARMONIC)),
		_manual(a.shake_char_rotation(0, 1, Vector3(0.3, 0, 0), 4, 0, false)),
	]
	for t in effects:
		t.custom_step(0.125)
	assert_true(a.get_char_offset(0).length() > 0.00001)
	assert_true((a.get_char_scale(0) - Vector2.ONE).length() > 0.00001)
	assert_true(a.get_char_rotation(0).length() > 0.00001)
	for t in effects:
		t.force_complete()
	_v2(a.get_char_offset(0), Vector2.ZERO)
	_v2(a.get_char_scale(0), Vector2.ONE)
	_v3(a.get_char_rotation(0), Vector3.ZERO)
	n.free()
	return true

func test_characters_invalid() -> bool:
	var n := _label("A")
	var a := n
	var t := _manual(a.offset_char(0, Vector2(NAN, 4), NAN))
	_near(t.get_duration(), 1)
	t.force_complete()
	_v2(a.get_char_offset(0), Vector2(0, 4))
	a.set_char_alpha(0, 3)
	_near(a.get_char_alpha(0), 1)
	a.set_char_offset(-1, Vector2.ONE)
	_v2(a.get_char_offset(-1), Vector2.ZERO)
	_v2(a.get_char_scale(2), Vector2.ONE)
	_v2(a.get_char_offset(0), Vector2(0, 4))
	a.set_char_rotation(0, Vector3(NAN, 0, 0))
	_v3(a.get_char_rotation(0), Vector3.ZERO)
	for randomness in [-5.0, 250.0]:
		t = _manual(a.shake_char_scale(0, 1, Vector2.ONE, 10, randomness))
		t.custom_step(0.5)
		assert_true(a.get_char_scale(0).is_finite())
		t.force_complete()
		_v2(a.get_char_scale(0), Vector2.ONE)
	t = _manual(a.shake_char_offset(0, 1, Vector2.ONE, 0, NAN, false))
	n.text = ""
	n.refresh()
	t.force_complete()
	assert_eq(t.get_state(), Spire.STATE_STOPPED)
	t = _manual(a.fade_char(-1, 0, 1))
	assert_eq(t.get_state(), Spire.STATE_PLAYING)
	t.force_complete()
	n.free()
	return true

func test_characters_color_input_normalization() -> bool:
	var n := _label("A")
	var cases := [
		[Color(2, -1, 0.5, -2), Color(2, -1, 0.5, 0)],
		[Color(0.25, 0.5, 0.75, 3), Color(0.25, 0.5, 0.75, 1)],
		[Color(NAN, INF, -INF, NAN), Color.WHITE],
	]
	for data in cases:
		n.set_char_color(0, data[0])
		assert_true(n.get_char_color(0).is_equal_approx(data[1]))
		n.set_char_color(0, Color.BLACK)
		var tween := _manual(n.color_char(0, data[0], 1))
		tween.force_complete()
		assert_true(n.get_char_color(0).is_equal_approx(data[1]))
	n.free()
	return true

func test_characters_missing_indices_keep_playing() -> bool:
	var n := _label("ABC")
	var tweens := [
		n.offset_char(2, Vector2(8, 0), 1), n.scale_char(2, Vector2(2, 2), 1),
		n.color_char(2, Color.RED, 1), n.fade_char(2, 0.25, 1),
		n.rotate_char(2, Vector3(0, 0, 1), 1),
		n.punch_char_offset(2, Vector2.ONE, 1), n.punch_char_rotation(2, Vector3.ONE, 1), n.punch_char_scale(2, Vector2.ONE, 1),
		n.shake_char_offset(2, 1, Vector2.ONE), n.shake_char_rotation(2, 1, Vector3.ONE), n.shake_char_scale(2, 1, Vector2.ONE),
	]
	for tween: SpireCharTween in tweens:
		_manual(tween)
		tween.custom_step(0.1)
	n.text = "A"
	n.refresh()
	for tween: SpireCharTween in tweens:
		var finished := [0]
		tween.finished.connect(func(): finished[0] += 1)
		assert_true(tween.custom_step(0.2))
		assert_eq(tween.get_state(), Spire.STATE_PLAYING)
		_near(tween.get_total_elapsed_time(), 0.3)
		tween.force_complete()
		assert_eq(finished, [1])
	_v2(n.get_char_offset(0), Vector2.ZERO)
	_v2(n.get_char_scale(0), Vector2.ONE)
	assert_eq(n.get_char_color(0), Color.WHITE)
	n.free()
	return true

func test_characters_explicit_refresh() -> bool:
	var n := SpireRichTextLabel.new()
	n.text = "A"
	assert_true(n.get_character_count() == 0, "Reading state must not initialize character data before ready")
	assert_true(not n.is_valid_char(0))
	_v2(n.get_char_offset(0), Vector2.ZERO)
	n.refresh()
	assert_eq(n.get_character_count(), 1)
	n.set_char_offset(0, Vector2(3, 4))
	n.text = "BC"
	n.append_text("SENTINEL")
	assert_true(n.get_character_count() == 1, "Getters must read the last refreshed state")
	_v2(n.get_char_offset(0), Vector2(3, 4))
	assert_true(not n.is_valid_char(1))
	n.set_char_offset(0, Vector2(7, 8))
	var tween := _manual(n.offset_char(0, Vector2(12, 13), 1))
	tween.force_complete()
	_v2(n.get_char_offset(0), Vector2(12, 13))
	assert_true(n.get_parsed_text() == "BCSENTINEL", "Setters and forced tween callbacks must not rebuild the stack")
	n.reset()
	assert_eq(n.get_character_count(), 1)
	assert_true(n.get_parsed_text() == "BCSENTINEL", "reset() must not refresh changed text")
	n.refresh()
	assert_eq(n.get_character_count(), 2)
	assert_eq(n.get_parsed_text(), "BC")
	_v2(n.get_char_offset(0), Vector2.ZERO)
	n.free()
	var automatic := _label("A")
	automatic.text = "BC"
	assert_eq(automatic.get_character_count(), 1)
	for _frame in 4: await next_frame()
	assert_true(automatic.get_character_count() == 2, "Automatic processing must still refresh text changes")
	automatic.free()
	return true

func test_characters_render() -> bool:
	var n := _label("[color=red]A[/color]B", true)
	var a := n
	await n.draw
	# Baseline is captured by the real RichTextEffect callback, not a synthetic call.
	assert_eq(a.get_char_color(0), Color.RED)
	assert_eq(a.get_char_color(1), Color.WHITE)
	var color := _manual(a.color_char(0, Color.BLUE, 1))
	color.custom_step(0.5)
	assert_eq(a.get_char_color(0), Color(0.5, 0, 0.5, 1))
	color.stop()
	a.set_char_pivot(0, Vector2(10, -6))
	_v2(a.get_char_pivot(0), Vector2(10, -6))
	a.set_char_rotation(0, Vector3(PI / 3, PI / 3, 0))
	a.set_char_scale(0, Vector2(2, 3))
	a.set_char_offset(0, Vector2(10, 20))
	a.set_char_color(0, Color.BLUE)
	a.set_char_alpha(0, 0.25)
	for _i in 3:
		await next_frame()
	assert_eq(a.get_char_color(0), Color(0, 0, 1, 0.25))
	a.reset()
	for _i in 3:
		await next_frame()
	assert_eq(a.get_char_color(0), Color.RED)
	_v2(a.get_char_offset(0), Vector2.ZERO)
	_v2(a.get_char_pivot(0), Vector2.ZERO)
	_v3(a.get_char_rotation(0), Vector3.ZERO)
	_v2(a.get_char_scale(0), Vector2.ONE)
	n.free()
	return true

func test_characters_unicode_render() -> bool:
	var n := _label("e\u0301אב")
	var a := n
	assert_eq(a.get_character_count(), 4)
	a.set_char_offset(1, Vector2(12, 3))
	for _i in 4:
		await next_frame()
	_v2(a.get_char_offset(0), Vector2(12, 3))
	_v2(a.get_char_offset(1), Vector2(12, 3))
	a.set_char_scale(0, Vector2(2, 2))
	_v2(a.get_char_scale(1), Vector2(2, 2))
	assert_true(a.is_valid_char(2))
	n.free()
	return true
