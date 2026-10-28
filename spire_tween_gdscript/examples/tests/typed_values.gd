extends "res://examples/tests/test_base.gd"

var vector4_value := Vector4.ZERO
var quaternion_value := Quaternion.IDENTITY
var rect2_value := Rect2()
var variant_value: Variant

func _set_variant(value: Variant) -> void:
	variant_value = value

func _set_vector4(value: Vector4) -> void:
	vector4_value = value

func _set_quaternion(value: Quaternion) -> void:
	quaternion_value = value

func _set_rect2(value: Rect2) -> void:
	rect2_value = value

func _manual(t):
	assert_true(Spire.is_registered(t))
	return t.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)

func _near(a: float, b: float) -> void:
	assert_true(abs(a - b) < 0.00002, "%s != %s" % [a, b])

func _v4(a: Vector4, b: Vector4) -> void:
	_near(a.distance_to(b), 0)

func _rect(a: Rect2, b: Rect2) -> void:
	_near(a.position.distance_to(b.position), 0)
	_near(a.size.distance_to(b.size), 0)

func _q(a: Quaternion, b: Quaternion) -> void:
	_near(a.length(), 1)
	_near(abs(a.dot(b.normalized())), 1)

func _rotation(angle: float) -> Quaternion:
	return Quaternion(Vector3.UP, angle)

func test_typed_vec4_property() -> bool:
	var source := VisualShaderNodeVec4Constant.new()
	source.constant = Quaternion(1, 2, 3, 4)
	var t: SpirePropertyVec4 = _manual(DoVisualShaderNodeVec4Constant.constant(source, Vector4(5, 6, 7, 8), 1))
	assert_true(t.custom_step(0.5))
	var v := source.constant
	_v4(Vector4(v.x, v.y, v.z, v.w), Vector4(3, 4, 5, 6))
	assert_true(not t.custom_step(0.75))
	var w: SpirePropertyFloat = _manual(DoVisualShaderNodeVec4Constant.constant_w(source, 12, 1))
	w.force_complete()
	v = source.constant
	_v4(Vector4(v.x, v.y, v.z, v.w), Vector4(5, 6, 7, 12))
	# Generic dispatch must find the same optimized adapter, including conversion
	# from Godot's Quaternion storage for this non-rotation shader constant.
	var generic: SpirePropertyVec4 = _manual(Spire.do_property_vec4(source, "constant", Vector4.ONE, 1))
	generic.force_complete()
	v = source.constant
	_v4(Vector4(v.x, v.y, v.z, v.w), Vector4.ONE)
	return true

func test_typed_rect2_property() -> bool:
	var n := Sprite2D.new()
	add_child(n)
	var t: SpirePropertyRect2 = _manual(DoSprite2D.region_rect(n, Rect2(4, 6, 8, 10), 1))
	n.region_rect = Rect2(2, 4, 6, 8)
	t.custom_step(0.5)
	_rect(n.region_rect, Rect2(3, 5, 7, 9))
	t.force_complete()
	_rect(n.region_rect, Rect2(4, 6, 8, 10))
	var p: SpirePropertyVec2 = _manual(DoSprite2D.region_rect_position(n, Vector2(-2, -4), 1))
	p.force_complete()
	_rect(n.region_rect, Rect2(-2, -4, 8, 10))
	var s: SpirePropertyVec2 = _manual(DoSprite2D.region_rect_size(n, Vector2(12, 14), 1))
	s.force_complete()
	_rect(n.region_rect, Rect2(-2, -4, 12, 14))
	var generic: SpirePropertyRect2 = _manual(Spire.do_property_rect2(n, "region_rect", Rect2(1, 2, 3, 4), 1))
	generic.force_complete()
	_rect(n.region_rect, Rect2(1, 2, 3, 4))
	n.free()
	return true

func test_typed_quaternion_property() -> bool:
	var n := Node3D.new()
	add_child(n)
	n.scale = Vector3(2, 3, 4)
	var t: SpirePropertyQuaternion = _manual(DoNode3D.quaternion(n, _rotation(2), 1))
	t.custom_step(0.5)
	_q(n.quaternion, _rotation(1))
	_near(n.scale.distance_to(Vector3(2, 3, 4)), 0)
	t.force_complete()
	_q(n.quaternion, _rotation(2))
	var generic: SpirePropertyQuaternion = _manual(Spire.do_property_quaternion(n, "quaternion", _rotation(0.5), 1))
	generic.force_complete()
	_q(n.quaternion, _rotation(0.5))
	n.free()
	return true

func test_typed_methods_and_builders() -> bool:
	var a: SpireMethodVec4 = _manual(Spire.do_call_vec4(_set_vector4, Vector4.ZERO, Vector4.ONE * 2, 1))
	var b: SpireMethodQuaternion = _manual(Spire.do_call_quaternion(_set_quaternion, Quaternion.IDENTITY, _rotation(2), 1))
	var c: SpireMethodRect2 = _manual(Spire.do_call_rect2(_set_rect2, Rect2(), Rect2(2, 4, 6, 8), 1))
	for t in [a, b, c]:
		assert_true(t.custom_step(0.5))
	_v4(vector4_value, Vector4.ONE)
	_q(quaternion_value, _rotation(1))
	_rect(rect2_value, Rect2(1, 2, 3, 4))
	for t in [a, b, c]:
		t.force_complete()
	_v4(vector4_value, Vector4.ONE * 2)
	_q(quaternion_value, _rotation(2))
	_rect(rect2_value, Rect2(2, 4, 6, 8))
	# Each new wrapper's public build entry point must also be registered.
	var builders = [
		SpireMethodVec4.build(_set_vector4, Vector4.ZERO, Vector4.ONE, 1),
		SpireMethodQuaternion.build(_set_quaternion, Quaternion.IDENTITY, _rotation(1), 1),
		SpireMethodRect2.build(_set_rect2, Rect2(), Rect2(1, 2, 3, 4), 1),
		SpirePropertyVec4.build(self, "vector4_value", Vector4.ONE * 3, 1),
		SpirePropertyQuaternion.build(self, "quaternion_value", _rotation(3), 1),
		SpirePropertyRect2.build(self, "rect2_value", Rect2(3, 6, 9, 12), 1),
	]
	for t in builders:
		_manual(t).force_complete()
	_v4(vector4_value, Vector4.ONE * 3)
	_q(quaternion_value, _rotation(3))
	_rect(rect2_value, Rect2(3, 6, 9, 12))
	return true

func test_typed_relative() -> bool:
	vector4_value = Vector4.ONE * 10
	rect2_value = Rect2(10, 20, 30, 40)
	var initial := Quaternion(Vector3.RIGHT, 0.7)
	quaternion_value = initial
	var a = _manual(Spire.do_property_vec4(self, "vector4_value", Vector4(2, 4, 6, 8), 1)).as_relative()
	var b = _manual(Spire.do_property_rect2(self, "rect2_value", Rect2(2, 4, -6, -8), 1)).as_relative()
	var c = _manual(Spire.do_property_quaternion(self, "quaternion_value", _rotation(1), 1)).as_relative()
	for t in [a, b, c]:
		t.custom_step(0.5)
	_v4(vector4_value, Vector4(11, 12, 13, 14))
	_rect(rect2_value, Rect2(11, 22, 27, 36))
	_q(quaternion_value, initial * _rotation(0.5))
	var external := Quaternion(Vector3.FORWARD, 0.3)
	quaternion_value = external * quaternion_value
	vector4_value += Vector4.ONE * 100
	rect2_value.position += Vector2.ONE * 100
	for t in [a, b, c]:
		t.force_complete()
	_v4(vector4_value, Vector4(112, 114, 116, 118))
	_rect(rect2_value, Rect2(112, 124, 24, 32))
	_q(quaternion_value, external * initial * _rotation(1))
	return true

func test_typed_speed() -> bool:
	var a = _manual(Spire.do_property_vec4(self, "vector4_value", Vector4(0, 0, 3, 4), 2)).as_speed_based()
	var b = _manual(Spire.do_property_rect2(self, "rect2_value", Rect2(0, 0, 3, 4), 2)).as_speed_based()
	var c = _manual(Spire.do_property_quaternion(self, "quaternion_value", _rotation(2), 0.8)).as_speed_based()
	for t in [a, b, c]:
		_near(t.get_duration(), 2.5)
		assert_true(t.custom_step(0.5))
	_v4(vector4_value, Vector4(0, 0, 0.6, 0.8))
	_rect(rect2_value, Rect2(0, 0, 0.6, 0.8))
	_q(quaternion_value, _rotation(0.4))
	for t in [a, b, c]:
		assert_true(not t.custom_step(3))
	_v4(vector4_value, Vector4(0, 0, 3, 4))
	_rect(rect2_value, Rect2(0, 0, 3, 4))
	_q(quaternion_value, _rotation(2))
	var same = _manual(Spire.do_property_quaternion(self, "quaternion_value", -_rotation(2), 1)).as_speed_based()
	assert_true(not same.custom_step(0.75))
	return true

func test_typed_sequence_and_lifetimes() -> bool:
	var children = [
		_manual(Spire.do_property_vec4(self, "vector4_value", Vector4(0, 0, 0, 1), 1)),
		_manual(Spire.do_property_rect2(self, "rect2_value", Rect2(0, 0, 0, 1), 1)),
		_manual(Spire.do_property_quaternion(self, "quaternion_value", _rotation(1), 1)),
	]
	for i in children.size():
		var child = children[i].as_speed_based().set_speed_scale(2)
		var weak: WeakRef = weakref(child)
		var seen := {"value": 0.0}
		var seq = Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
		seq.append(child)
		seq.append(_manual(Spire.do_call_float(func(value): seen.value = value, 0, 1, 1)))
		seq.custom_step(0.75)
		_near(seen.value, 0.25)
		seq.force_complete()
		Spire.unregister(seq)
		children[i] = null
		child = null
		seq = null
		assert_true(weak.get_ref() == null)
	return true

func test_typed_loops() -> bool:
	for mode in [Spire.LOOP_MODE_YOYO, Spire.LOOP_MODE_INCREMENTAL, Spire.LOOP_MODE_RESTART]:
		vector4_value = Vector4.ZERO
		quaternion_value = Quaternion.IDENTITY
		rect2_value = Rect2()
		var a = _manual(Spire.do_property_vec4(self, "vector4_value", Vector4.ONE * 2, 1)).from(Vector4.ZERO).set_loops(2, mode)
		var b = _manual(Spire.do_property_rect2(self, "rect2_value", Rect2(2, 4, 6, 8), 1)).from(Rect2()).set_loops(2, mode)
		var c = _manual(Spire.do_property_quaternion(self, "quaternion_value", _rotation(1), 1)).from(Quaternion.IDENTITY).set_loops(2, mode)
		for t in [a, b, c]:
			t.custom_step(1)
			t.custom_step(0.5)
		var factor := 1.5 if mode == Spire.LOOP_MODE_INCREMENTAL else 0.5
		_v4(vector4_value, Vector4.ONE * 2 * factor)
		_rect(rect2_value, Rect2(Vector2(2, 4) * factor, Vector2(6, 8) * factor))
		_q(quaternion_value, _rotation(factor))
		for t in [a, b, c]:
			assert_true(not t.custom_step(0.5))
			t.play()
			t.custom_step(0.5)
			t.stop()
		_v4(vector4_value, Vector4.ONE)
		_rect(rect2_value, Rect2(1, 2, 3, 4))
		_q(quaternion_value, _rotation(0.5))
	return true

func test_typed_dynamic_targets() -> bool:
	var target := {"vector": Vector4.ZERO, "rect": Rect2(), "rotation": Quaternion.IDENTITY}
	var a = _manual(Spire.do_property_vec4(self, "vector4_value", Vector4.ZERO, 1)).set_dynamic_target(func(): return target.vector)
	var b = _manual(Spire.do_property_rect2(self, "rect2_value", Rect2(), 1)).set_dynamic_target(func(): return target.rect)
	var c = _manual(Spire.do_property_quaternion(self, "quaternion_value", Quaternion.IDENTITY, 1)).set_dynamic_target(func(): return target.rotation)
	target.vector = Vector4.ONE
	target.rect = Rect2(1, 2, 3, 4)
	target.rotation = _rotation(1)
	for t in [a, b, c]:
		t.force_complete()
	_v4(vector4_value, Vector4.ONE)
	_rect(rect2_value, Rect2(1, 2, 3, 4))
	_q(quaternion_value, _rotation(1))
	return true

func test_typed_cancellation() -> bool:
	var victim = get_script().new()
	add_child(victim)
	var counts := {"stopped": 0, "finished": 0}
	var tweens = [
		_manual(Spire.do_property_vec4(victim, "vector4_value", Vector4.ONE, 1)),
		_manual(Spire.do_property_quaternion(victim, "quaternion_value", _rotation(1), 1)),
		_manual(Spire.do_property_rect2(victim, "rect2_value", Rect2(1, 2, 3, 4), 1)),
		_manual(Spire.do_call_vec4(victim._set_vector4, Vector4.ZERO, Vector4.ONE, 1)),
		_manual(Spire.do_call_quaternion(victim._set_quaternion, Quaternion.IDENTITY, _rotation(1), 1)),
		_manual(Spire.do_call_rect2(victim._set_rect2, Rect2(), Rect2(1, 2, 3, 4), 1)),
	]
	for t in tweens:
		t.stopped.connect(func(): counts.stopped += 1)
		t.finished.connect(func(): counts.finished += 1)
	victim.free()
	for t in tweens:
		assert_true(not t.custom_step(0.5))
		assert_true(t.is_stopped())
		t.force_complete()
	assert_eq(counts.stopped, 6)
	assert_eq(counts.finished, 0)
	# Null owners fail through the existing optional-return bridge.
	assert_true(Spire.do_property_vec4(null, "value", Vector4.ONE, 1) == null)
	assert_true(Spire.do_property_quaternion(null, "value", Quaternion.IDENTITY, 1) == null)
	assert_true(Spire.do_property_rect2(null, "value", Rect2(), 1) == null)
	return true

func test_typed_quaternion_edges() -> bool:
	for values in [
		[Quaternion.IDENTITY, -_rotation(1), _rotation(0.5)],
		[_rotation(0.3), -_rotation(0.3), _rotation(0.3)],
		[Quaternion.IDENTITY * 3, _rotation(1) * 2, _rotation(0.5)],
		[Quaternion.IDENTITY, _rotation(0.001), _rotation(0.0005)],
		[Quaternion.IDENTITY, Quaternion(0, 1, 0, 0), _rotation(PI / 2)],
		[Quaternion(0, 0, 0, 0), _rotation(1), _rotation(0.5)],
		[Quaternion(NAN, 0, 0, 1), _rotation(1), _rotation(0.5)],
	]:
		var t = _manual(Spire.do_call_quaternion(_set_quaternion, values[0], values[1], 1))
		t.custom_step(0.5)
		_q(quaternion_value, values[2])
		t.stop()
	var overshoot = _manual(Spire.do_call_quaternion(_set_quaternion, Quaternion.IDENTITY, _rotation(1), 1)).set_ease_func(func(_p): return 1.5)
	overshoot.custom_step(0.5)
	_q(quaternion_value, _rotation(1.5))
	overshoot.set_ease_func(func(_p): return -0.5)
	overshoot.custom_step(0)
	_q(quaternion_value, _rotation(-0.5))
	overshoot.stop()
	quaternion_value = Quaternion.IDENTITY
	var small = _manual(Spire.do_property_quaternion(self, "quaternion_value", _rotation(0.001), 0.0002)).as_speed_based()
	small.custom_step(1)
	_near(quaternion_value.y, sin(0.0001))
	small.stop()
	var forced = _manual(Spire.do_property_quaternion(self, "quaternion_value", _rotation(0.7) * 5, 1))
	forced.force_complete()
	_q(quaternion_value, _rotation(0.7))
	return true

func test_typed_quaternion_completion() -> bool:
	var initial := Quaternion(Vector3.RIGHT, 0.4)
	for mode in range(3):
		for paused in [false, true]:
			for pair in [
				[_rotation(0.7) * 5, _rotation(0.7)],
				[Quaternion(0, 0, 0, 0), Quaternion.IDENTITY],
				[Quaternion(NAN, 0, 0, 1), Quaternion.IDENTITY],
			]:
				quaternion_value = initial
				var t = _manual(Spire.do_property_quaternion(self, "quaternion_value", pair[0], 1))
				if mode == 1:
					t.as_speed_based()
				elif mode == 2:
					t.as_relative()
				var counts := {"stopped": 0, "finished": 0}
				t.stopped.connect(func(): counts.stopped += 1)
				t.finished.connect(func(): counts.finished += 1)
				if paused:
					t.pause()
				t.force_complete()
				_q(quaternion_value, initial * pair[1] if mode == 2 else pair[1])
				assert_eq(counts.stopped, 1)
				assert_eq(counts.finished, 1)

	for target in [_rotation(0.7) * 5, -_rotation(0.7) * 3]:
		quaternion_value = _rotation(0.7)
		var t = _manual(Spire.do_property_quaternion(self, "quaternion_value", target, 1)).as_speed_based()
		assert_true(not t.custom_step(0.25))
		_q(quaternion_value, _rotation(0.7))

	var raw := _rotation(0.4) * 3
	quaternion_value = raw
	var stopped = _manual(Spire.do_property_quaternion(self, "quaternion_value", _rotation(0.7), 1))
	stopped.stop()
	stopped.force_complete()
	assert_eq(quaternion_value, raw)
	var method = _manual(Spire.do_call_quaternion(_set_quaternion, Quaternion.IDENTITY, _rotation(0.7) * 5, 1))
	method.force_complete()
	_q(quaternion_value, _rotation(0.7))
	var calls := {"count": 0}
	var lerp_override: Callable = func(_from, _to, _weight):
		calls.count += 1
		return 999.0
	var custom = _manual(Spire.do_property_custom(self, "variant_value", 12.0, 1,
		Callable(), lerp_override, Callable(), Callable()))
	custom.force_complete()
	assert_eq(calls.count, 0)
	_near(variant_value, 12.0)
	return true

func test_typed_shader_vec4() -> bool:
	var shader := Shader.new()
	shader.code = "shader_type canvas_item; uniform vec4 typed_value; void fragment() { COLOR = typed_value; }"
	var material := ShaderMaterial.new()
	material.shader = shader
	material.set_shader_parameter("typed_value", Vector4.ZERO)
	var t: SpirePropertyVec4 = _manual(DoShaderMaterial.parameter_vec4(material, "typed_value", Vector4(2, 4, 6, 8), 1))
	t.custom_step(0.5)
	_v4(material.get_shader_parameter("typed_value"), Vector4(1, 2, 3, 4))
	t.force_complete()
	_v4(material.get_shader_parameter("typed_value"), Vector4(2, 4, 6, 8))
	assert_true(DoShaderMaterial.parameter_vec4(material, "missing", Vector4.ONE, 1) == null)
	material.set_shader_parameter("typed_value", 1.0)
	assert_true(DoShaderMaterial.parameter_vec4(material, "typed_value", Vector4.ONE, 1) == null)
	material.set_shader_parameter("typed_value", Vector4.ONE)
	var invalidated = _manual(DoShaderMaterial.parameter_vec4(material, "typed_value", Vector4.ZERO, 1))
	material.shader = Shader.new()
	assert_true(not invalidated.custom_step(0.5))
	return true

func test_typed_variant_dispatch() -> bool:
	for speed in [false, true]:
		vector4_value = Vector4.ZERO
		quaternion_value = Quaternion.IDENTITY
		rect2_value = Rect2()
		var a = _manual(Spire.do_property(self, "vector4_value", Vector4.ONE * 2, 1))
		var b = _manual(Spire.do_property(self, "rect2_value", Rect2(2, 4, 6, 8), 1))
		var c = _manual(Spire.do_property(self, "quaternion_value", _rotation(1), 1))
		for t in [a, b, c]:
			if speed:
				t.as_speed_based()
			var delta: float = 0.5 * t.get_duration() if speed else 0.5
			t.custom_step(delta)
		_v4(vector4_value, Vector4.ONE)
		_rect(rect2_value, Rect2(1, 2, 3, 4))
		_q(quaternion_value, _rotation(0.5))
		for t in [a, b, c]:
			t.force_complete()
		_v4(vector4_value, Vector4.ONE * 2)
		_rect(rect2_value, Rect2(2, 4, 6, 8))
		_q(quaternion_value, _rotation(1))
	return true

func test_variant_common_types() -> bool:
	for values in [
		[0, 3, 2],
		[0.0, 3.0, 1.5],
		[1, 4.0, 2.5],
		[1.0, 4, 2.5],
		[Vector2(2, 4), Vector2(6, 10), Vector2(4, 7)],
		[Vector2i(0, 1), Vector2i(3, 4), Vector2i(2, 3)],
		[Vector3(1, 2, 3), Vector3(3, 6, 9), Vector3(2, 4, 6)],
		[Vector3i.ZERO, Vector3i(1, 3, 5), Vector3i(1, 2, 3)],
		[Color(0, 0.25, 0.5, 0.75), Color(1, 0.75, 0, 0.25), Color(0.5, 0.5, 0.25, 0.5)],
		["abcd", "WXYZ", "WXcd"],
		[&"abcd", &"WXYZ", "WXcd"],
		[NodePath("abcd"), NodePath("WXYZ"), "WXcd"],
		["a😊界d", "Ωx😁z", "Ωx界d"],
	]:
		for property in [false, true]:
			variant_value = values[0]
			var tween = _manual(Spire.do_property(self, "variant_value", values[1], 1)) if property else _manual(Spire.do_call(_set_variant, values[0], values[1], 1))
			tween.custom_step(0.5)
			assert_eq(typeof(variant_value), typeof(values[2]))
			assert_eq(variant_value, values[2])
			tween.stop()
	return true

func test_variant_conversion_and_override() -> bool:
	print("EXPECTED_VARIANT_CONVERSION_BEGIN")
	for start in [1, Vector2.ONE, "text"]:
		var tween = _manual(Spire.do_call(_set_variant, start, {}, 1))
		tween.custom_step(0.5)
		assert_eq(variant_value, start)
		tween.stop()
	print("EXPECTED_VARIANT_CONVERSION_END")
	var custom = _manual(Spire.do_call_custom(_set_variant, 0, 3, 1, func(_from, _to, _weight): return 777.0))
	custom.custom_step(0.5)
	_near(variant_value, 777.0)
	custom.stop()
	return true
