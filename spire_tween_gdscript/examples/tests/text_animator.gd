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

func _manual(t: SpireMethodFloat) -> SpireMethodFloat:
	return t.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)

func _near(a: float, b: float) -> void:
	assert_true(abs(a - b) < 0.0002, str(a) + " != " + str(b))

func _v2(a: Vector2, b: Vector2) -> void:
	_near(a.distance_to(b), 0)

func _v3(a: Vector3, b: Vector3) -> void:
	_near(a.distance_to(b), 0)

func _overshoot_ease(p: float) -> float:
	return p + 3 * p * (1 - p)

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
	for tween: SpireMethodFloat in tweens:
		_manual(tween)
		tween.custom_step(0.1)
	n.text = "A"
	n.refresh()
	for tween: SpireMethodFloat in tweens:
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
