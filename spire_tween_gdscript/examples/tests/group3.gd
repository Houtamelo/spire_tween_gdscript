extends "res://examples/tests/test_base.gd"

func _manual(t):
	assert_true(Spire.is_registered(t))
	return t.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)

func _near(a: float, b: float) -> void:
	assert_true(abs(a-b) < 0.0002, "%s != %s" % [a, b])

func _v(a, b) -> void:
	_near(a.distance_to(b), 0)

func _q(a: Quaternion, b: Quaternion) -> void:
	assert_true(abs(a.dot(b)) > 0.99999, "%s != %s" % [a, b])

func test_group3_global_easing() -> bool:
	var n := Node2D.new()
	add_child(n)
	var original := SpireGlobalSettings.get_default_ease()
	SpireGlobalSettings.set_default_ease(Spire.EASE_OUT_QUAD)
	var t = DoNode2D.shake(n, 1, Vector2(3, 4), 4, 0, false).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	t.custom_step(0.125)
	var first := n.position.length()
	SpireGlobalSettings.set_default_ease(Spire.EASE_IN_QUAD)
	t.custom_step(0.125)
	var changed := n.position.length()
	t.force_complete()
	var explicit = _manual(DoNode2D.shake(n, 1, Vector2(3, 4), 4, 0, false))
	explicit.custom_step(0.125)
	var overridden := n.position.length()
	explicit.force_complete()
	SpireGlobalSettings.set_default_ease(Spire.EASE_OUT_QUAD)
	var p = DoNode2D.punch_position(n, Vector2(8, 0), 1, 4).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	p.custom_step(0.05)
	var punch_eased := n.position.x
	SpireGlobalSettings.set_default_ease(Spire.EASE_LINEAR)
	p.custom_step(0.05)
	var punch_changed := n.position.x
	p.force_complete()
	SpireGlobalSettings.set_default_ease(original)
	_near(first, 4.6875)
	_near(changed, 1.25)
	_near(overridden, 2.5)
	_near(punch_eased, 7.8)
	_near(punch_changed, 8)
	n.free()
	return true

func test_group3_punch_2d() -> bool:
	var n := Node2D.new()
	add_child(n)
	n.position = Vector2(2, 3)
	var t = _manual(DoNode2D.punch_position(n, Vector2(8, 0), 1, 4, 0.5))
	t.custom_step(0.1)
	_v(n.position, Vector2(10, 3))
	t.custom_step(0.2)
	_v(n.position, Vector2(-1, 3))
	t.custom_step(0.3)
	_v(n.position, Vector2(6, 3))
	t.force_complete()
	_v(n.position, Vector2(2, 3))
	var s = _manual(DoNode2D.punch_scale(n, Vector2.ONE, 1, 4))
	var r = _manual(DoNode2D.punch_rotation(n, 0.3, 1, 4))
	s.custom_step(0.1)
	r.custom_step(0.1)
	_v(n.scale, Vector2(2, 2))
	_near(n.rotation, 0.3)
	s.force_complete()
	r.force_complete()
	n.free()
	return true

func test_group3_punch_control() -> bool:
	var n := Control.new()
	add_child(n)
	var effects = [_manual(DoControl.punch_position(n, Vector2.ONE, 1, 4)), _manual(DoControl.punch_scale(n, Vector2.ONE, 1, 4)), _manual(DoControl.punch_rotation(n, 0.2, 1, 4))]
	for t in effects:
		t.custom_step(0.1)
	_v(n.position, Vector2.ONE)
	_v(n.scale, Vector2(2, 2))
	_near(n.rotation, 0.2)
	for t in effects:
		t.force_complete()
	_v(n.position, Vector2.ZERO)
	_v(n.scale, Vector2.ONE)
	_near(n.rotation, 0)
	n.free()
	return true

func test_group3_punch_3d() -> bool:
	var n := Node3D.new()
	add_child(n)
	var effects = [_manual(DoNode3D.punch_position(n, Vector3(2, 3, 4), 1, 4)), _manual(DoNode3D.punch_scale(n, Vector3.ONE, 1, 4)), _manual(DoNode3D.punch_rotation(n, Vector3(0.2, 0.3, 0.4), 1, 4))]
	for t in effects:
		t.custom_step(0.1)
	_v(n.position, Vector3(2, 3, 4))
	_v(n.scale, Vector3(2, 2, 2))
	_q(n.quaternion, Quaternion.from_euler(Vector3(0.2, 0.3, 0.4)))
	for t in effects:
		t.force_complete()
	_q(n.quaternion, Quaternion.IDENTITY)
	var t = _manual(DoNode3D.blendable_punch_rotation(n, Vector3(0.3, 0, 0), 1, 4))
	t.custom_step(0.1)
	_q(n.quaternion, Quaternion.from_euler(Vector3(0.3, 0, 0)))
	t.force_complete()
	n.free()
	return true

func test_group3_shake_properties() -> bool:
	for n in [Node2D.new(), Control.new()]:
		add_child(n)
		var s = _manual(DoNode2D.shake_scale(n, 1, Vector2(3, 0), 4, 0, false)) if n is Node2D else _manual(DoControl.shake_scale(n, 1, Vector2(3, 0), 4, 0, false))
		var r = _manual(DoNode2D.shake_rotation(n, 1, 0.3, 4, 0, false)) if n is Node2D else _manual(DoControl.shake_rotation(n, 1, 0.3, 4, 0, false))
		s.custom_step(0.25)
		r.custom_step(0.25)
		_near((n.scale - Vector2.ONE).length(), 3)
		_near(abs(n.rotation), 0.3)
		s.force_complete()
		r.force_complete()
		_v(n.scale, Vector2.ONE)
		_near(n.rotation, 0)
		n.free()
	return true

func test_group3_shake_3d() -> bool:
	var n := Node3D.new()
	add_child(n)
	var p = _manual(DoNode3D.shake(n, 1, Vector3(0, 0, 3), 4, 90, false))
	var s = _manual(DoNode3D.shake_scale(n, 1, Vector3(0.2, 0, 0), 4, 90, false))
	var r = _manual(DoNode3D.shake_rotation(n, 1, Vector3(0, 0.3, 0), 4, 90, false))
	for t in [p, s, r]:
		t.custom_step(0.25)
	_near(n.position.length(), 3)
	_near(n.position.x, 0)
	_near(n.position.y, 0)
	_near((n.scale - Vector3.ONE).length(), 0.2)
	_near(n.quaternion.get_angle(), 0.3)
	for t in [p, s, r]:
		t.force_complete()
	_v(n.position, Vector3.ZERO)
	_v(n.scale, Vector3.ONE)
	_q(n.quaternion, Quaternion.IDENTITY)
	n.free()
	return true

func test_group3_camera() -> bool:
	var n := Camera3D.new()
	add_child(n)
	n.rotation = Vector3(0, 0.7, 0)
	var q := n.quaternion
	var p = _manual(DoCamera3D.shake(n, 1, Vector3(3, 4, 9), 4, 0, false))
	p.custom_step(0.25)
	_near(n.position.length(), 5)
	_near((q.inverse() * n.position).z, 0)
	p.force_complete()
	var r = _manual(DoCamera3D.shake_rotation(n, 1, Vector3(0.3, 0, 0), 4, 90, false))
	r.custom_step(0.25)
	assert_true(abs(n.quaternion.dot(q)) < 0.9999)
	r.force_complete()
	_q(n.quaternion, q)
	n.free()
	return true

func test_group3_effect_lifecycle() -> bool:
	var n := Node2D.new()
	add_child(n)
	var t = _manual(DoNode2D.punch_position(n, Vector2(8, 0), 1, 4))
	n.position = Vector2(2, 0)
	t.custom_step(0.1)
	n.position += Vector2(0, 5)
	t.stop()
	_v(n.position, Vector2(10, 5))
	t.force_complete()
	_v(n.position, Vector2(10, 5)) # force_complete is a no-op while stopped.
	t.play()
	t.custom_step(0.1)
	_v(n.position, Vector2(10, 5))
	t.force_complete()
	var y = _manual(DoNode2D.punch_position(n, Vector2(8, 0), 1, 4)).set_loops(2, Spire.LOOP_MODE_YOYO)
	y.custom_step(0.1)
	var outbound := n.position
	y.custom_step(0.9)
	y.custom_step(0.9)
	_v(n.position, outbound)
	y.custom_step(0.1)
	_v(n.position, Vector2(2, 5))
	n.free()
	return true

func test_group3_quaternions() -> bool:
	var parent := Node3D.new()
	var n := Node3D.new()
	add_child(parent)
	parent.add_child(n)
	parent.rotation.y = 0.5
	n.scale = Vector3(2, 3, 4)
	var t = _manual(DoNode3D.rotate_quaternion(n, Quaternion.from_euler(Vector3(0, 1, 0)), 1))
	t.custom_step(0.5)
	_q(n.quaternion, Quaternion.from_euler(Vector3(0, 0.5, 0)))
	t.force_complete()
	_v(n.scale, Vector3(2, 3, 4))
	var g = _manual(DoNode3D.global_rotate_quaternion(n, Quaternion.IDENTITY, 1))
	g.force_complete()
	_q(n.global_basis.orthonormalized().get_rotation_quaternion(), Quaternion.IDENTITY)
	_v(n.scale, Vector3(2, 3, 4))
	var target := Quaternion.from_euler(Vector3(0.3, 0.4, 0.2))
	var nonuniform = _manual(DoNode3D.global_rotate_quaternion(n, target, 1))
	nonuniform.force_complete()
	_q(n.global_basis.orthonormalized().get_rotation_quaternion(), target)
	_v(n.scale, Vector3(2, 3, 4))
	parent.free()
	return true

func test_group3_rotation_modes() -> bool:
	var n := Node3D.new()
	add_child(n)
	for pair in [[Spire.ROTATE_MODE_FAST, PI], [Spire.ROTATE_MODE_FAST_BEYOND_360, 0.0]]:
		n.rotation = Vector3(0, deg_to_rad(170), 0)
		var t = _manual(DoNode3D.rotate(n, Vector3(0, deg_to_rad(-170), 0), 1, pair[0]))
		t.custom_step(0.5)
		_q(n.quaternion, Quaternion.from_euler(Vector3(0, pair[1], 0)))
	var start := Quaternion.from_euler(Vector3(0.6, 0.7, 0))
	var by := Vector3(0, 0, 0.8)
	for mode in [Spire.ROTATE_MODE_WORLD_AXIS_ADD, Spire.ROTATE_MODE_LOCAL_AXIS_ADD]:
		n.quaternion = start
		var t = _manual(DoNode3D.rotate(n, by, 1, mode))
		t.force_complete()
		_q(n.quaternion, Quaternion.from_euler(by) * start if mode == Spire.ROTATE_MODE_WORLD_AXIS_ADD else start * Quaternion.from_euler(by))
	var g = _manual(DoNode3D.global_rotate(n, Vector3.ZERO, 1))
	g.force_complete()
	_q(n.quaternion, Quaternion.IDENTITY)
	n.free()
	return true

func test_group3_blendable_rotation() -> bool:
	var n := Node3D.new()
	add_child(n)
	var start := Quaternion.from_euler(Vector3(0.3, 0.4, 0))
	var by := Vector3(0, 0, 0.8)
	n.quaternion = start
	var t = _manual(DoNode3D.blendable_rotation(n, by, 1))
	t.custom_step(0.5)
	_q(n.quaternion, start * Quaternion.from_euler(by * 0.5))
	t.force_complete()
	_q(n.quaternion, start * Quaternion.from_euler(by))
	n.quaternion = start
	var g = _manual(DoNode3D.blendable_global_rotation(n, by, 1))
	g.force_complete()
	_q(n.quaternion, Quaternion.from_euler(by) * start)
	n.free()
	return true

func test_group3_look_at() -> bool:
	var n := Node3D.new()
	var target := Node3D.new()
	add_child(n)
	add_child(target)
	var point := Vector3(4, 0, -4)
	var t = _manual(DoNode3D.look_at(n, point, 1))
	t.custom_step(0.5)
	n.position.x = 3
	t.force_complete()
	_v(-n.basis.z, point.normalized())
	n.quaternion = Quaternion.IDENTITY
	var d = _manual(DoNode3D.dynamic_look_at(n, point, 1, Vector3.UP, Spire.ROTATION_AXIS_Y))
	d.custom_step(0.5)
	n.position.x = 2
	d.force_complete()
	_v(-n.basis.z, (point - n.position).normalized())
	n.quaternion = Quaternion.from_euler(Vector3(0.2, 0.3, 0.4))
	var constrained = _manual(DoNode3D.look_at(n, Vector3(4, 3, -4), 1, Vector3.UP, Spire.ROTATION_AXIS_XY))
	constrained.custom_step(0.5)
	_near(n.quaternion.get_euler().z, 0.4)
	constrained.force_complete()
	_near(n.quaternion.get_euler().z, 0.4)
	target.position = Vector3(-3, 2, 5)
	var f = _manual(DoNode3D.look_at_target(n, target, 1, Vector3.UP, Spire.ROTATION_AXIS_ALL, true))
	f.force_complete()
	_v(n.basis.z, (target.position - n.position).normalized())
	var cancelled = _manual(DoNode3D.look_at_target(n, target, 1))
	target.free()
	cancelled.custom_step(0.1)
	assert_true(cancelled.is_stopped())
	var before := n.quaternion
	var singular = _manual(DoNode3D.look_at(n, n.position, 1, Vector3.ZERO))
	singular.force_complete()
	_q(n.quaternion, before)
	n.free()
	return true

func test_group3_normalized_ui() -> bool:
	var n := Control.new()
	add_child(n)
	n.size = Vector2(100, 80)
	var t = _manual(DoControl.pivot(n, Vector2.ONE, 1))
	t.custom_step(0.5)
	_v(n.pivot_offset, Vector2(50, 40))
	n.size = Vector2(200, 160)
	t.force_complete()
	_v(n.pivot_offset, Vector2(200, 160))
	var x = _manual(DoControl.pivot_x(n, 0.25, 1))
	x.force_complete()
	var y = _manual(DoControl.pivot_y(n, 0.5, 1))
	y.force_complete()
	_v(n.pivot_offset, Vector2(50, 80))
	var s := ScrollContainer.new()
	add_child(s)
	var h := s.get_h_scroll_bar()
	var v := s.get_v_scroll_bar()
	h.min_value = 10
	h.max_value = 210
	h.page = 50
	h.value = 10
	v.min_value = 20
	v.max_value = 420
	v.page = 100
	v.value = 20
	var scroll = _manual(DoScrollContainer.normalized_position(s, Vector2(1, 0), 1))
	scroll.custom_step(0.5)
	_near(h.value, 85)
	_near(v.value, 170)
	h.max_value = 410
	scroll.force_complete()
	_near(h.value, 360)
	_near(v.value, 320)
	var hx = _manual(DoScrollContainer.horizontal_normalized_position(s, 0, 1))
	hx.force_complete()
	_near(v.value, 320)
	var vy = _manual(DoScrollContainer.vertical_normalized_position(s, 1, 1))
	vy.force_complete()
	_near(h.value, 10)
	_near(v.value, 20)
	var pixel_x = _manual(DoScrollContainer.scroll_horizontal(s, 50, 1))
	pixel_x.force_complete()
	var pixel_y = _manual(DoScrollContainer.scroll_vertical(s, 70, 1))
	pixel_y.force_complete()
	_near(h.value, 50)
	_near(v.value, 70)
	n.free()
	s.free()
	return true

func test_group3_spiral() -> bool:
	var n := Node3D.new()
	add_child(n)
	var t = _manual(DoNode3D.spiral(n, 1, Vector3.BACK, Spire.SPIRAL_MODE_EXPAND, 1, TAU, 2))
	t.custom_step(0.25)
	_v(n.position, Vector3(0, 0.25, 0.5))
	t.force_complete()
	_v(n.position, Vector3(1, 0, 2))
	var start := n.position
	var c = _manual(DoNode3D.spiral(n, 1, Vector3.UP, Spire.SPIRAL_MODE_EXPAND_THEN_CONTRACT))
	c.custom_step(0.5)
	_near(n.position.distance_to(start), 1)
	c.force_complete()
	_v(n.position, start)
	n.free()
	return true

func test_group3_invalid_and_cancellation() -> bool:
	var warning_node := Node3D.new()
	add_child(warning_node)
	print("TEMPLATE_WARNINGS_INVALID_BEGIN")
	for randomness in [-12.5, 245.75]:
		var t = _manual(DoNode3D.shake(warning_node, 1, Vector3.ONE, 4, randomness))
		t.force_complete()
	for elasticity in [-0.25, 1.25]:
		var t = _manual(DoNode3D.punch_position(warning_node, Vector3.ONE, -2.5, 4, elasticity))
		t.force_complete()
	var invalid_spiral = _manual(DoNode3D.spiral(warning_node, 1, Vector3.ZERO, Spire.SPIRAL_MODE_EXPAND, 1, -3.25))
	invalid_spiral.force_complete()
	var invalid_rotation = _manual(DoNode3D.rotate_quaternion(warning_node, Quaternion(0, 0, 0, 0), 1))
	invalid_rotation.force_complete()
	var invalid_up = _manual(DoNode3D.look_at(warning_node, warning_node.position + Vector3.FORWARD, 1, Vector3.ZERO))
	invalid_up.force_complete()
	assert_true(warning_node.position.is_finite())
	print("TEMPLATE_WARNINGS_INVALID_END")
	print("TEMPLATE_WARNINGS_VALID_BEGIN")
	for randomness in [0.0, 180.0]:
		var t = _manual(DoNode3D.shake(warning_node, 1, Vector3.ONE, 4, randomness))
		t.force_complete()
	for elasticity in [0.0, 1.0]:
		var t = _manual(DoNode3D.punch_position(warning_node, Vector3.ONE, 1, 4, elasticity))
		t.force_complete()
	print("TEMPLATE_WARNINGS_VALID_END")
	warning_node.free()
	var bound := Node.new()
	add_child(bound)
	var bound_events := [0, 0, 0]
	var bound_tween = _manual(Spire.do_call_float(func(_value): bound_events[0] += 1, 0.0, 1.0, 1.0))
	bound_tween.bind_node(bound)
	bound_tween.stopped.connect(func(): bound_events[1] += 1)
	bound_tween.finished.connect(func(): bound_events[2] += 1)
	bound.free()
	bound_tween.force_complete()
	assert_true(bound_events == [0, 1, 0], "Forced completion must cancel before invoking accessors with a freed explicit binding: %s" % [bound_events])
	for forced in [false, true]:
		var n := Node3D.new()
		add_child(n)
		var tweens = [_manual(DoNode3D.punch_position(n, Vector3(NAN, 1, 0), -1, 0, 3)), _manual(DoNode3D.shake(n, 1, Vector3(-1, 2, 0), 0, NAN)), _manual(DoNode3D.rotate_quaternion(n, Quaternion(0, 0, 0, 0), 1)), _manual(DoNode3D.spiral(n, NAN, Vector3.ZERO, Spire.SPIRAL_MODE_EXPAND, NAN, -1, NAN, true))]
		var events := [0, 0]
		for t in tweens:
			t.stopped.connect(func(): events[0] += 1)
			t.finished.connect(func(): events[1] += 1)
		for t in tweens:
			t.custom_step(0.1)
		assert_true(n.position.is_finite())
		n.free()
		for t in tweens:
			if forced:
				t.force_complete()
			else:
				t.custom_step(0.1)
			assert_true(t.is_stopped())
		assert_eq(events, [4, 0])
	return true
