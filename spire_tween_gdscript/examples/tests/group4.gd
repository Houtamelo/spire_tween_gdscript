extends "res://examples/tests/test_base.gd"

var margins := Vector4i.ZERO
var number: int = 0
var text := ""

func _set_number(value: int) -> void:
	number = value

func _manual(t):
	assert_true(Spire.is_registered(t))
	return t.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)

func _near(a: float, b: float) -> void:
	assert_true(abs(a - b) < 0.002, "%s != %s" % [a, b])

func _v2(a: Vector2, b: Vector2) -> void:
	_near(a.distance_to(b), 0)

func _v3(a: Vector3, b: Vector3) -> void:
	_near(a.distance_to(b), 0)

func test_group4_text_buffers() -> bool:
	var opening := '[long_effect url="日本]語"]' + "[b]".repeat(12)
	var closing := "[/b]".repeat(12) + "[/long_effect]"
	var target := [""]
	var t = _manual(Spire.do_property_string(self, "text", "", 16, true).from("")
		.set_dynamic_target(func(): return target[0]))
	var elapsed := 0.0
	for length in [512, 7, 1024, 0, 96]:
		target[0] = opening + "猫".repeat(length) + closing
		t.custom_step(0.5)
		elapsed += 0.5
		var count := int(round(length * elapsed / 16.0))
		assert_eq(text, "" if count == 0 else opening + "猫".repeat(count) + closing)
	t.set_text_options()
	t.custom_step(0.5)
	elapsed += 0.5
	assert_eq(text, target[0].substr(0, int(round(target[0].length() * elapsed / 16.0))))
	t.set_text_options(true)
	t.custom_step(0.5)
	elapsed += 0.5
	assert_eq(text, opening + "猫".repeat(int(round(96 * elapsed / 16.0))) + closing)
	t.stop()
	text = opening + "α".repeat(512) + closing
	t = _manual(Spire.do_property_string(self, "text", opening + "Z".repeat(512) + closing, 512, true).as_speed_based())
	t.custom_step(0.5)
	assert_eq(text, opening + "Z".repeat(256) + "α".repeat(256) + closing)
	t.force_complete()
	assert_eq(text, opening + "Z".repeat(512) + closing)
	text = "α".repeat(512)
	t = _manual(Spire.do_property_string(self, "text", "Z".repeat(512), 1))
	t.custom_step(0.5)
	assert_eq(text, "Z".repeat(256) + "α".repeat(256))
	for tag in ["lb", "rb", "lrm", "rlm", "lre", "rle", "lro", "rlo", "pdf", "alm", "lri", "rli", "fsi", "pdi", "zwj", "zwnj", "wj", "shy"]:
		t = _manual(Spire.do_property_string(self, "text", "[" + tag + "]ABC", 1, true).from(""))
		t.custom_step(0.5)
		assert_eq(text, "[" + tag + "]A")
	t = _manual(Spire.do_property_string(self, "text", opening + "ABCD" + closing, 1,
		true, Spire.SCRAMBLE_MODE_CUSTOM, "猫".repeat(100)).from(""))
	t.custom_step(0.25)
	assert_eq(text, opening + "A猫猫猫" + closing)
	t.set_text_options(true, Spire.SCRAMBLE_MODE_CUSTOM, "[")
	t.custom_step(0.25)
	assert_eq(text, opening + "AB[lb][lb]" + closing)
	t.force_complete()
	assert_eq(text, opening + "ABCD" + closing)
	return true

func test_group4_text_plain() -> bool:
	var n := Label.new()
	add_child(n)
	n.text = "abcd"
	var t = _manual(DoLabel.text(n, "WXYZ", 1))
	n.text = "αβγδ"
	t.custom_step(0.5)
	assert_eq(n.text, "WXγδ")
	t.force_complete()
	assert_eq(n.text, "WXYZ")
	n.text = ""
	t = _manual(DoLabel.text(n, "é猫🦀Z", 1))
	t.custom_step(0.5)
	assert_eq(n.text, "é猫")
	n.free()
	return true

func test_group4_text_bbcode() -> bool:
	var n := RichTextLabel.new()
	add_child(n)
	n.bbcode_enabled = true
	var t = _manual(DoRichTextLabel.text(n, "[b]A[color=red]猫[/color]CD[/b]", 1, true))
	t.custom_step(0.5)
	assert_eq(n.text, "[b]A[color=red]猫[/color][/b]")
	assert_eq(n.get_parsed_text(), "A猫")
	n.text = "[i]wxyz[/i]"
	t = _manual(DoRichTextLabel.text(n, "[b]ABCD[/b]", 1, true))
	t.custom_step(0.5)
	assert_eq(n.text, "[b]AB[/b][i]yz[/i]")
	assert_eq(n.get_parsed_text(), "AByz")
	t.force_complete()
	assert_eq(n.text, "[b]ABCD[/b]")
	n.free()
	return true

func test_group4_text_scramble() -> bool:
	var n := Label.new()
	add_child(n)
	for row in [
		[Spire.SCRAMBLE_MODE_ALL, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"],
		[Spire.SCRAMBLE_MODE_UPPERCASE, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"],
		[Spire.SCRAMBLE_MODE_LOWERCASE, "abcdefghijklmnopqrstuvwxyz"],
		[Spire.SCRAMBLE_MODE_NUMERALS, "0123456789"],
		[Spire.SCRAMBLE_MODE_CUSTOM, "猫🦀"],
	]:
		n.text = ""
		var t = _manual(DoLabel.text(n, "abcd", 1, false, row[0], row[1]))
		t.custom_step(0.5)
		assert_true(n.text.begins_with("ab"))
		assert_eq(n.text.length(), 4)
		for c in n.text.substr(2):
			assert_true(row[1].contains(c))
		t.force_complete()
		assert_eq(n.text, "abcd")
	n.free()
	return true

func test_group4_text_markup_edges() -> bool:
	var n := RichTextLabel.new()
	add_child(n)
	n.bbcode_enabled = true
	var t = _manual(DoRichTextLabel.text(n, "[b]ABCD[/b]", 1, true, Spire.SCRAMBLE_MODE_CUSTOM, "["))
	t.custom_step(0.5)
	assert_eq(n.text, "[b]AB[lb][lb][/b]")
	assert_eq(n.get_parsed_text(), "AB[[")
	n.text = ""
	t = _manual(DoRichTextLabel.text(n, "[lb]A[rb]Z", 1, true))
	t.custom_step(0.5)
	assert_eq(n.get_parsed_text(), "[A")
	# Unclosed markup is ordinary text and never causes a tokenizer loop.
	text = ""
	var generic = _manual(Spire.do_property_string(self, "text", "[unfinished", 1, true))
	generic.force_complete()
	assert_eq(text, "[unfinished")
	text = ""
	generic = _manual(Spire.do_property_string(self, "text", "[br]ABC", 1, true))
	generic.custom_step(0.5)
	assert_eq(text, "[br]") # [br] is not a Godot BBCode tag.
	text = ""
	generic = _manual(Spire.do_property_string(self, "text", "[lrm]ABC", 1, true))
	generic.custom_step(0.5)
	assert_eq(text, "[lrm]A")
	n.free()
	return true

func test_group4_text_lifecycle() -> bool:
	var n := Label.new()
	add_child(n)
	n.text = "abcd"
	var t = _manual(DoLabel.text(n, "WXYZ", 1))
	t.set_loops(2, Spire.LOOP_MODE_YOYO)
	t.custom_step(1)
	assert_eq(n.text, "WXYZ")
	t.custom_step(0.5)
	assert_eq(n.text, "WXcd")
	t.custom_step(0.5)
	assert_eq(n.text, "abcd")
	t.play()
	t.custom_step(0.25)
	t.stop()
	var stopped := n.text
	t.force_complete()
	assert_eq(n.text, stopped)
	t = _manual(DoLabel.text(n, "end", 1))
	var events := [0, 0]
	t.finished.connect(func(): events[0] += 1)
	t.stopped.connect(func(): events[1] += 1)
	n.free()
	assert_true(not t.custom_step(0.5))
	assert_eq(events, [0, 1])
	return true

func test_group4_text_invalid() -> bool:
	var n := Label3D.new()
	add_child(n)
	var t = _manual(DoLabel3D.text(n, "abcd", 1, false, Spire.SCRAMBLE_MODE_CUSTOM, ""))
	_near(t.get_duration(), 1)
	t.custom_step(0.5)
	assert_eq(n.text.length(), 4)
	t.force_complete()
	assert_eq(n.text, "abcd")
	n.free()
	return true

func test_group4_path_linear() -> bool:
	var n := Node2D.new()
	add_child(n)
	var t = _manual(DoNode2D.path(n, PackedVector2Array([Vector2(10, 0), Vector2(10, 30)]), 1))
	t.custom_step(0.5)
	_v2(n.position, Vector2(10, 10))
	n.position = Vector2.ZERO
	t = _manual(DoNode2D.path(n, PackedVector2Array([Vector2(20, 0)]), 1))
	n.position = Vector2(10, 0)
	t.custom_step(0.5)
	_v2(n.position, Vector2(15, 0))
	t.force_complete()
	_v2(n.position, Vector2(20, 0))
	n.free()
	return true

func test_group4_text_relative() -> bool:
	var n := RichTextLabel.new()
	add_child(n)
	n.bbcode_enabled = true
	n.text = "[b]Hi[/b]"
	var t = _manual(DoRichTextLabel.text(n, "[i]猫Z[/i]", 1, true, Spire.SCRAMBLE_MODE_CUSTOM, "#").as_relative())
	t.set_loops(2, Spire.LOOP_MODE_YOYO)
	t.custom_step(0.5)
	assert_true(t.is_relative())
	assert_eq(n.text, "[b]Hi[/b][i]猫[/i]")
	t.custom_step(0.5)
	assert_eq(n.get_parsed_text(), "Hi猫Z")
	t.custom_step(0.5)
	assert_eq(n.text, "[b]Hi[/b][i]猫[/i]")
	t.custom_step(0.5)
	assert_eq(n.text, "[b]Hi[/b]")
	# Options may be set after choosing the mode; force_complete needs no prior step.
	t = _manual(DoRichTextLabel.text(n, "[i]猫Z[/i]", 1).as_relative().set_text_options(true, Spire.SCRAMBLE_MODE_CUSTOM, "#"))
	t.force_complete()
	assert_eq(n.text, "[b]Hi[/b][i]猫Z[/i]")
	n.text = "Hi"
	t = _manual(DoRichTextLabel.text(n, "猫Z", 1, false, Spire.SCRAMBLE_MODE_CUSTOM, "#").as_relative())
	t.custom_step(0.5)
	assert_eq(n.text, "Hi猫")
	t.force_complete()
	assert_eq(n.text, "Hi猫Z")
	n.free()
	return true

func test_group4_text_speed() -> bool:
	var n := RichTextLabel.new()
	add_child(n)
	n.bbcode_enabled = true
	n.text = "[i]a猫[/i]"
	var t = _manual(DoRichTextLabel.text(n, "[b]AXY[/b]", 2, true, Spire.SCRAMBLE_MODE_CUSTOM, "#").as_speed_based())
	t.custom_step(0.25)
	assert_true(t.is_speed_based())
	assert_eq(n.text, "[i]a猫[/i]")
	t.custom_step(0.25)
	assert_eq(n.text, "[b]A[/b][i]猫[/i]")
	t.custom_step(0.5)
	assert_eq(n.text, "[b]AX[/b]")
	assert_true(not t.custom_step(0.75))
	assert_eq(n.text, "[b]AXY[/b]")
	n.text = "[i]A[/i]"
	t = _manual(DoRichTextLabel.text(n, "[b]A[/b]", 2, true).as_speed_based())
	assert_true(not t.custom_step(0.1))
	assert_eq(n.text, "[b]A[/b]")
	n.text = "[i]AB猫[/i]"
	t = _manual(DoRichTextLabel.text(n, "[b]A[/b]", 2, true).as_speed_based())
	t.custom_step(0.5)
	assert_eq(n.text, "[b]A[/b][i]B[/i]")
	t.force_complete()
	assert_eq(n.text, "[b]A[/b]")

	n.text = "[i]ab[/i]"
	t = _manual(DoRichTextLabel.text(n, "[b]XY[/b]", 2, true).as_speed_based())
	t.set_loops(2, Spire.LOOP_MODE_YOYO)
	t.custom_step(1)
	assert_eq(n.text, "[b]XY[/b]")
	t.custom_step(0.5)
	assert_eq(n.text, "[i]a[/i][b]Y[/b]")
	t.custom_step(0.5)
	assert_eq(n.text, "[i]ab[/i]")
	n.text = ""
	t = _manual(DoRichTextLabel.text(n, "ab", 2).as_speed_based().set_text_options(false, Spire.SCRAMBLE_MODE_CUSTOM, "#"))
	t.custom_step(0.5)
	assert_eq(n.text, "a")
	t.custom_step(0.5)
	assert_eq(n.text, "ab")
	n.free()
	text = ""
	t = _manual(Spire.do_property_string(self, "text", "[img]res://image.png[/img]", 2, true).as_speed_based())
	assert_true(not t.custom_step(0.5))
	assert_eq(text, "[img]res://image.png[/img]")
	return true

func test_group4_text_options_api() -> bool:
	for class_name_to_check in ["DoLabel", "DoRichTextLabel", "DoLabel3D"]:
		assert_true(not ClassDB.class_has_method(class_name_to_check, "text_effect"))
	assert_true(not ClassDB.class_has_method("Spire", "do_text_effect"))
	var n := Label.new()
	add_child(n)
	n.text = "abcd"
	var t = _manual(DoLabel.text(n, "X", 1))
	assert_true(t is SpirePropertyString)
	t.custom_step(0.5)
	assert_eq(n.text, "Xb")
	t.stop()
	n.text = "abcd"
	t = _manual(DoLabel.text(n, "[b]WXYZ[/b]", 1, true))
	t.custom_step(0.25)
	assert_eq(n.text, "[b]W[/b]bcd")
	t.set_dynamic_target(func(): return "[i]1234[/i]")
	t.custom_step(0.25)
	assert_eq(n.text, "[i]12[/i]cd")
	t.set_dynamic_target(func(): return "[u]5678[/u]")
	t.custom_step(0.25)
	assert_eq(n.text, "[u]567[/u]d")
	t.force_complete()
	assert_eq(n.text, "[u]5678[/u]")
	n.text = ""
	t = _manual(DoLabel.text(n, "abcd", 1, false, Spire.SCRAMBLE_MODE_CUSTOM, "#"))
	t.custom_step(0.25)
	assert_eq(n.text, "a###")
	assert_eq(t.set_text_options(), t)
	t.custom_step(0.25)
	assert_eq(n.text, "ab")
	t = _manual(SpirePropertyString.build(n, "text", "[b]X[/b]", 1).set_text_options(true).from("[i]abcd[/i]"))
	t.custom_step(0.5)
	assert_eq(n.text, "[b]X[/b][i]b[/i]")
	t.force_complete()
	assert_eq(n.text, "[b]X[/b]")
	n.free()
	# Enabling BBCode parsing is explicit even for RichTextLabel.
	var rich := RichTextLabel.new()
	add_child(rich)
	t = _manual(DoRichTextLabel.text(rich, "[b]AB[/b]", 1))
	t.custom_step(0.5)
	assert_eq(rich.text, "[b]AB")
	rich.free()
	return true

func test_group4_path_curves() -> bool:
	var n := Node2D.new()
	add_child(n)
	var options := SpirePathOptions.new()
	options.path_type = Spire.PATH_TYPE_CUBIC_BEZIER
	options.resolution = 100
	var t = _manual(DoNode2D.path_with_options(n, PackedVector2Array([Vector2(10, 0), Vector2(0, 10), Vector2(10, 10)]), 1, options))
	# The constructor copies options rather than retaining a mutable configuration.
	options.lock_position = 7
	t.custom_step(0.5)
	_v2(n.position, Vector2(5, 7.5))
	n.position = Vector2.ZERO
	options.lock_position = 0
	options.path_type = Spire.PATH_TYPE_CATMULL_ROM
	t = _manual(DoNode2D.path_with_options(n, PackedVector2Array([Vector2(10, 10), Vector2(20, 0)]), 1, options))
	t.custom_step(0.5)
	_v2(n.position, Vector2(10, 10))
	n.free()
	return true

func test_group4_path_spaces() -> bool:
	var parent := Node2D.new()
	add_child(parent)
	parent.position = Vector2(10, 20)
	parent.rotation = 0.5
	parent.scale = Vector2(2, 3)
	var n := Node2D.new()
	parent.add_child(n)
	var t = _manual(DoNode2D.path(n, PackedVector2Array([Vector2(5, 8)]), 1))
	t.force_complete()
	_v2(n.position, Vector2(5, 8))
	_v2(n.global_position, parent.to_global(Vector2(5, 8)))
	t = _manual(DoNode2D.global_path(n, PackedVector2Array([Vector2(20, 40)]), 1))
	t.force_complete()
	_v2(n.global_position, Vector2(20, 40))
	parent.free()
	return true

func test_group4_path_3d() -> bool:
	var parent := Node3D.new()
	add_child(parent)
	parent.position = Vector3(10, 20, 30)
	parent.rotation = Vector3(0.2, 0.4, 0.3)
	var n := Node3D.new()
	parent.add_child(n)
	n.scale = Vector3(2, 3, 4)
	var t = _manual(DoNode3D.path(n, PackedVector3Array([Vector3(10, 0, 0)]), 1))
	t.force_complete()
	_v3(n.position, Vector3(10, 0, 0))
	_v3(n.global_position, parent.to_global(Vector3(10, 0, 0)))
	t = _manual(DoNode3D.global_path(n, PackedVector3Array([Vector3(1, 2, 3)]), 1))
	t.force_complete()
	_v3(n.global_position, Vector3(1, 2, 3))
	_v3(n.scale, Vector3(2, 3, 4))
	parent.free()
	return true

func test_group4_path_control() -> bool:
	var n := Control.new()
	add_child(n)
	var options := SpirePathOptions.new()
	options.orient = true
	var t = _manual(DoControl.path_with_options(n, PackedVector2Array([Vector2(0, 10)]), 1, options))
	t.custom_step(0.5)
	_v2(n.position, Vector2(0, 5))
	_near(n.rotation, PI / 2)
	t.force_complete()
	_v2(n.position, Vector2(0, 10))
	n.free()
	return true

func test_group4_path_closed_locks() -> bool:
	var n := Node2D.new()
	add_child(n)
	var options := SpirePathOptions.new()
	options.closed = true
	var t = _manual(DoNode2D.path_with_options(n, PackedVector2Array([Vector2(10, 0)]), 1, options))
	t.custom_step(0.5)
	_v2(n.position, Vector2(10, 0))
	t.custom_step(0.5)
	_v2(n.position, Vector2.ZERO)
	options.closed = false
	options.lock_position = 2 # Y; curve length must be computed after locking.
	t = _manual(DoNode2D.path_with_options(n, PackedVector2Array([Vector2(10, 30), Vector2(30, 30)]), 1, options))
	t.custom_step(0.5)
	_v2(n.position, Vector2(15, 0))
	n.free()
	return true

func test_group4_path_orientation() -> bool:
	var n := Node3D.new()
	add_child(n)
	n.scale = Vector3(2, 3, 4)
	var options := SpirePathOptions.new()
	options.orient = true
	var t = _manual(DoNode3D.path_with_options(n, PackedVector3Array([Vector3(10, 0, 0)]), 1, options))
	t.custom_step(0.5)
	_v3(-n.global_basis.orthonormalized().z, Vector3.RIGHT)
	_v3(n.scale, Vector3(2, 3, 4))
	t.force_complete()
	_v3(-n.global_basis.orthonormalized().z, Vector3.RIGHT)
	n.rotation = Vector3.ZERO
	options.use_look_at_position = true
	options.look_at_position = Vector3(15, 0, -10)
	t = _manual(DoNode3D.path_with_options(n, PackedVector3Array([Vector3(20, 0, 0)]), 1, options))
	t.custom_step(0.5)
	_v3(-n.global_basis.orthonormalized().z, Vector3.FORWARD)
	n.free()
	return true

func test_group4_path_loops() -> bool:
	var n := Node2D.new()
	add_child(n)
	for mode in [Spire.LOOP_MODE_RESTART, Spire.LOOP_MODE_YOYO, Spire.LOOP_MODE_INCREMENTAL]:
		n.position = Vector2.ZERO
		var t = _manual(DoNode2D.path(n, PackedVector2Array([Vector2(10, 0)]), 1))
		t.set_loops(2, mode)
		t.custom_step(1)
		t.custom_step(0.5)
		_v2(n.position, Vector2(15 if mode == Spire.LOOP_MODE_INCREMENTAL else 5, 0))
		t.stop()
		var stopped := n.position
		t.force_complete()
		_v2(n.position, stopped)
	n.free()
	return true

func test_group4_path_invalid_cancellation() -> bool:
	var n := Node3D.new()
	add_child(n)
	n.position = Vector3(2, 3, 4)
	var t = _manual(DoNode3D.path(n, PackedVector3Array(), 1))
	t.force_complete()
	_v3(n.position, Vector3(2, 3, 4))
	var options := SpirePathOptions.new()
	options.path_type = Spire.PATH_TYPE_CUBIC_BEZIER
	options.resolution = 0
	options.look_ahead = NAN
	options.up = Vector3.ZERO
	options.forward_direction = Vector3.ZERO
	options.lock_position = 99
	t = _manual(DoNode3D.path_with_options(n, PackedVector3Array([Vector3(NAN, 1, 0)]), NAN, options))
	_near(t.get_duration(), 1)
	t.force_complete()
	_v3(n.position, Vector3(2, 3, 4))
	var target := Node3D.new()
	add_child(target)
	options = SpirePathOptions.new()
	options.orient = true
	options.look_at_target = target
	t = _manual(DoNode3D.path_with_options(n, PackedVector3Array([Vector3.ONE]), 1, options))
	target.free()
	assert_true(not t.custom_step(0.5))
	n.free()
	return true

func test_group4_margins() -> bool:
	var n := MarginContainer.new()
	add_child(n)
	n.add_theme_constant_override("margin_left", 0)
	n.add_theme_constant_override("margin_right", 2)
	n.add_theme_constant_override("margin_top", 4)
	n.add_theme_constant_override("margin_bottom", 6)
	var t = _manual(DoMarginContainer.margins(n, Vector4i(4, 6, 8, 10), 1))
	t.custom_step(0.5)
	assert_eq(n.get_theme_constant("margin_left"), 2)
	assert_eq(n.get_theme_constant("margin_bottom"), 8)
	t.force_complete()
	assert_eq(n.get_theme_constant("margin_top"), 8)
	margins = Vector4i.ZERO
	t = _manual(Spire.do_property_margins(self, "margins", Vector4i(-2, 4, 6, 8), 1))
	t.force_complete()
	assert_eq(margins, Vector4i(-2, 4, 6, 8))
	var invalid = Spire.do_property_margins(self, "number", Vector4i.ONE, 1)
	assert_eq(invalid.get_state(), Spire.STATE_STOPPED)
	invalid.force_complete()
	assert_eq(number, 0)
	invalid.play()
	assert_true(not invalid.custom_step(0.5))
	n.free()
	return true

func test_group4_integer_api() -> bool:
	for method in ["do_property_uint", "do_property_ulong", "do_call_uint", "do_call_ulong"]:
		assert_true(not ClassDB.class_has_method("Spire", method), "Unexpected integer helper: " + method)
	number = -5000000000
	var t = _manual(Spire.do_property_int(self, "number", 5000000000, 1))
	t.custom_step(0.5)
	assert_eq(number, 0)
	t.force_complete()
	assert_eq(number, 5000000000)
	t = _manual(Spire.do_call_int(_set_number, 6, -6, 1))
	t.custom_step(0.75)
	assert_eq(number, -3)
	t.force_complete()
	assert_eq(number, -6)
	var invalid = _manual(Spire.do_call_int(Callable(), -6, 6, 1))
	assert_true(not invalid.custom_step(0.5))
	assert_eq(invalid.get_state(), Spire.STATE_STOPPED)
	var n := Label.new()
	add_child(n)
	t = _manual(Spire.do_call_int(n.set_visible_characters, 0, 10, 1))
	var events := [0, 0]
	t.finished.connect(func(): events[0] += 1)
	t.stopped.connect(func(): events[1] += 1)
	n.free()
	assert_true(not t.custom_step(0.5))
	assert_eq(events, [0, 1])
	return true

func test_group4_text_atomic_tags() -> bool:
	text = ""
	var t = _manual(Spire.do_property_string(self, "text", "[img]res://image.png[/img]XY", 1, true))
	t.custom_step(0.5)
	assert_eq(text, "[img]res://image.png[/img]X")
	t.force_complete()
	assert_eq(text, "[img]res://image.png[/img]XY")
	text = ""
	t = _manual(Spire.do_property_string(self, "text", '[url="a]b"]ABCD[/url]', 1, true))
	t.custom_step(0.5)
	assert_eq(text, '[url="a]b"]AB[/url]')
	return true

func test_group4_path_curve_edges() -> bool:
	var n := Node3D.new()
	add_child(n)
	var options := SpirePathOptions.new()
	options.path_type = Spire.PATH_TYPE_CATMULL_ROM
	options.resolution = 100
	var t = _manual(DoNode3D.path_with_options(n, PackedVector3Array([Vector3(10, 10, 0), Vector3(20, 0, 0)]), 1, options))
	t.custom_step(0.5)
	_v3(n.position, Vector3(10, 10, 0))
	t.force_complete()
	for kind in [Spire.PATH_TYPE_CATMULL_ROM, Spire.PATH_TYPE_CUBIC_BEZIER]:
		n.position = Vector3.ZERO
		options.closed = true
		options.path_type = kind
		var points := PackedVector3Array([Vector3(10, 0, 0)])
		if kind == Spire.PATH_TYPE_CUBIC_BEZIER:
			points = PackedVector3Array([Vector3(10, 0, 0), Vector3(3, 0, 0), Vector3(7, 0, 0)])
		t = _manual(DoNode3D.path_with_options(n, points, 1, options))
		t.custom_step(0.5)
		_v3(n.position, Vector3(10, 0, 0))
		t.force_complete()
		_v3(n.position, Vector3.ZERO)
	# A low-resolution curve whose endpoints coincide must still move.
	options.resolution = 1
	options.closed = false
	options.path_type = Spire.PATH_TYPE_CUBIC_BEZIER
	t = _manual(DoNode3D.path_with_options(n, PackedVector3Array([Vector3.ZERO, Vector3(0, 10, 0), Vector3(10, 10, 0)]), 1, options))
	t.custom_step(0.25)
	assert_true(n.position.length() > 1)
	t.force_complete()
	_v3(n.position, Vector3.ZERO)
	n.free()
	return true

func test_group4_path_modes() -> bool:
	var n := Node3D.new()
	add_child(n)
	var options := SpirePathOptions.new()
	options.orient = true
	for row in [
		[Spire.PATH_MODE_IGNORE, Vector3.RIGHT * 10, Vector3.ZERO],
		[Spire.PATH_MODE_SIDE_SCROLLER_2D, Vector3.UP * 10, Vector3(0, 0, PI / 2)],
		[Spire.PATH_MODE_TOP_DOWN_2D, Vector3.RIGHT * 10, Vector3(0, -PI / 2, 0)],
	]:
		n.position = Vector3.ZERO
		n.rotation = Vector3.ZERO
		options.mode = row[0]
		var t = _manual(DoNode3D.path_with_options(n, PackedVector3Array([row[1]]), 1, options))
		t.force_complete()
		_v3(n.rotation, row[2])
	n.position = Vector3.ZERO
	n.rotation = Vector3.ZERO
	options.mode = Spire.PATH_MODE_FULL_3D
	options.lock_rotation = 7
	var t = _manual(DoNode3D.path_with_options(n, PackedVector3Array([Vector3.RIGHT * 10]), 1, options))
	t.force_complete()
	_v3(n.rotation, Vector3.ZERO)
	n.position = Vector3.ZERO
	options.lock_rotation = 0
	options.forward_direction = Vector3.RIGHT
	t = _manual(DoNode3D.path_with_options(n, PackedVector3Array([Vector3.RIGHT * 10]), 1, options))
	t.force_complete()
	_v3(n.basis.x.normalized(), Vector3.RIGHT)
	n.position = Vector3.ZERO
	n.rotation = Vector3.ZERO
	options.forward_direction = Vector3.UP # Parallel model forward/up is degenerate.
	t = _manual(DoNode3D.path_with_options(n, PackedVector3Array([Vector3.RIGHT * 10]), 1, options))
	t.force_complete()
	_v3(n.rotation, Vector3.ZERO)
	n.free()
	return true

func test_group4_path_zero_scale_top_level() -> bool:
	var parent := Node2D.new()
	add_child(parent)
	parent.position = Vector2(30, 40)
	parent.rotation = 0.5
	var n := Node2D.new()
	parent.add_child(n)
	n.top_level = true
	n.scale = Vector2.ZERO
	var options := SpirePathOptions.new()
	options.orient = true
	var t = _manual(DoNode2D.path_with_options(n, PackedVector2Array([Vector2(0, 10)]), 1, options))
	t.force_complete()
	_v2(n.global_position, Vector2(0, 10))
	var control := Control.new()
	parent.add_child(control)
	control.scale = Vector2.ZERO
	t = _manual(DoControl.path_with_options(control, PackedVector2Array([Vector2(10, 0)]), 1, options))
	t.force_complete()
	_v2(control.position, Vector2(10, 0))
	parent.free()
	var parent3 := Node3D.new()
	add_child(parent3)
	parent3.position = Vector3(30, 40, 50)
	var n3 := Node3D.new()
	parent3.add_child(n3)
	n3.top_level = true
	n3.scale = Vector3.ZERO
	t = _manual(DoNode3D.path_with_options(n3, PackedVector3Array([Vector3.RIGHT * 10]), 1, options))
	t.force_complete()
	_v3(n3.global_position, Vector3.RIGHT * 10)
	parent3.free()
	return true

func test_group4_path_targets() -> bool:
	var n := Node2D.new()
	add_child(n)
	var target := Node2D.new()
	add_child(target)
	target.position = Vector2(5, 10)
	var options := SpirePathOptions.new()
	options.orient = true
	options.look_at_target = target
	var t = _manual(DoNode2D.path_with_options(n, PackedVector2Array([Vector2(10, 0)]), 1, options))
	t.custom_step(0.5)
	_near(n.global_rotation, PI / 2)
	target.position = Vector2(7.5, -10)
	t.custom_step(0.25)
	_near(n.global_rotation, -PI / 2)
	options.orient = false
	var ignored = _manual(DoNode2D.path_with_options(n, PackedVector2Array([Vector2(20, 0)]), 1, options))
	target.free()
	ignored.force_complete()
	_v2(n.position, Vector2(20, 0))
	n.free()
	return true

func test_group4_sequence_handoff() -> bool:
	var n := Node2D.new()
	add_child(n)
	var child = _manual(DoNode2D.path(n, PackedVector2Array([Vector2(10, 0)]), 1))
	child.set_speed_scale(2)
	var next = _manual(DoNode2D.position(n, Vector2(20, 0), 1))
	var sequence = Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
	sequence.append(child).append(next)
	sequence.custom_step(0.75)
	_v2(n.position, Vector2(12.5, 0))
	n.free()
	return true
