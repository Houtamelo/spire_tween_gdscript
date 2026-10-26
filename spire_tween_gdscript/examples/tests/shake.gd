extends "res://examples/tests/test_base.gd"


func _manual(tween: SpireMethodFloat) -> SpireMethodFloat:
	assert_true(tween != null)
	assert_true(Spire.is_registered(tween))
	tween.set_process_mode(Spire.PROCESS_MODE_MANUAL)
	return tween


func _near(a: Vector2, b: Vector2) -> void:
	assert_true(a.distance_to(b) < 0.0001, "%s != %s" % [a, b])


func _magnitude(value: Vector2, expected: float) -> void:
	assert_true(abs(value.length() - expected) < 0.0001)


func _shake(node, fade_out: bool = false, blendable: bool = true) -> SpireMethodFloat:
	if node is Control:
		return _manual(DoControl.shake(node, 1.0, Vector2(3, 4), 4, 0, fade_out, Spire.SHAKE_RANDOMNESS_MODE_HARMONIC, blendable))
	return _manual(DoNode2D.shake(node, 1.0, Vector2(3, 4), 4, 0, fade_out, Spire.SHAKE_RANDOMNESS_MODE_FULL, blendable))


func test_shake_defaults() -> bool:
	for node in [Node2D.new(), Control.new()]:
		add_child(node)
		var tween: SpireMethodFloat
		if node is Control:
			tween = _manual(DoControl.shake(node, 1.0, Vector2(3, 4)))
		else:
			tween = _manual(DoNode2D.shake(node, 1.0, Vector2(3, 4)))
		# Default vibrato=10 and fade_out=true: first segment ends at 1/55.
		tween.custom_step(1.0 / 110.0)
		_magnitude(node.position, 2.5)
		node.position += Vector2(7, 2)
		tween.force_complete()
		_near(node.position, Vector2(7, 2)) # blendable defaults to true.
		node.free()
	return true


func test_shake_interpolation() -> bool:
	for node in [Node2D.new(), Control.new()]:
		add_child(node)
		var tween := _shake(node)
		tween.custom_step(0.125)
		var half: Vector2 = node.position
		_magnitude(half, 2.5)
		tween.custom_step(0.125)
		_near(node.position, half * 2)
		tween.custom_step(0.125)
		_near(node.position, Vector2.ZERO)
		tween.custom_step(0.125)
		_near(node.position, -half * 2)
		tween.force_complete()
		_near(node.position, Vector2.ZERO)
		node.free()
	return true


func test_shake_fade() -> bool:
	var node := Node2D.new()
	add_child(node)
	var tween := _shake(node, true)
	tween.custom_step(0.05)
	var half := node.position
	_magnitude(half, 2.5)
	for step_and_factor in [[0.05, 2.0], [0.2, -1.5], [0.3, 1.0]]:
		tween.custom_step(step_and_factor[0])
		_near(node.position, half * step_and_factor[1])
	tween.custom_step(0.39999)
	tween.custom_step(0.00002)
	_near(node.position, Vector2.ZERO)
	assert_true(tween.is_stopped())
	node.free()
	return true


func test_shake_blending() -> bool:
	var node := Node2D.new()
	add_child(node)
	var movement := Vector2(7, -4)
	var a := _shake(node)
	var b := _shake(node)
	a.custom_step(0.125)
	var offset_a := node.position
	b.custom_step(0.125)
	var offset_b := node.position - offset_a
	node.position += movement
	a.custom_step(0.125)
	_near(node.position, movement + offset_a * 2 + offset_b)
	a.force_complete()
	_near(node.position, movement + offset_b)
	b.force_complete()
	_near(node.position, movement)
	var absolute := _shake(node, false, false)
	node.position = Vector2.ZERO
	absolute.custom_step(0.125)
	var half := node.position
	node.position = Vector2(100, 200)
	absolute.custom_step(0.125)
	_near(node.position, half * 2)
	absolute.force_complete()
	_near(node.position, Vector2.ZERO)
	node.free()
	return true


func test_shake_lifecycle() -> bool:
	for blendable in [false, true]:
		var node := Node2D.new()
		add_child(node)
		var tween := _shake(node, false, blendable)
		tween.custom_step(0.125)
		var half := node.position
		tween.pause()
		tween.custom_step(0.4)
		_near(node.position, half)
		tween.play()
		tween.custom_step(0.125)
		_near(node.position, half * 2)
		var frozen := node.position
		tween.stop()
		tween.custom_step(0.5)
		tween.force_complete()
		_near(node.position, frozen)
		tween.play()
		tween.custom_step(0.125)
		_magnitude(node.position, 2.5)
		var events := []
		tween.stopped.connect(func():
			_near(node.position, Vector2.ZERO)
			events.append("stopped"))
		tween.finished.connect(func():
			_near(node.position, Vector2.ZERO)
			events.append("finished"))
		tween.force_complete()
		assert_true(events == ["stopped", "finished"])
		node.free()
	return true


func test_shake_loops() -> bool:
	for mode in [Spire.LOOP_MODE_RESTART, Spire.LOOP_MODE_INCREMENTAL, Spire.LOOP_MODE_YOYO]:
		var node := Node2D.new()
		add_child(node)
		var tween := _manual(DoNode2D.shake(node, 1.0, Vector2(3, 4), 4, 90, false))
		tween.set_loops(4, mode)
		tween.custom_step(0.125)
		var first := node.position
		tween.custom_step(0.25)
		var second := node.position
		tween.custom_step(0.625)
		_near(node.position, Vector2.ZERO)
		for loop_index in range(1, 4):
			if mode == Spire.LOOP_MODE_YOYO and loop_index % 2 == 1:
				tween.custom_step(0.625)
				_near(node.position, second)
				tween.custom_step(0.25)
				_near(node.position, first)
				tween.custom_step(0.125)
			else:
				tween.custom_step(0.125)
				_near(node.position, first)
				tween.custom_step(0.25)
				_near(node.position, second)
				tween.custom_step(0.625)
			_near(node.position, Vector2.ZERO)
		assert_true(tween.is_stopped())
		tween.play()
		tween.custom_step(0.125)
		var fresh_first := node.position
		tween.custom_step(0.25)
		assert_true(first != fresh_first or second != node.position, "replay must regenerate")
		tween.force_complete()
		_near(node.position, Vector2.ZERO)
		node.free()
	return true


func test_shake_phase_and_cleanup() -> bool:
	for blendable in [false, true]:
		for forced in [false, true]:
			for node in [Node2D.new(), Control.new()]:
				add_child(node)
				var origin := Vector2(10, -3)
				node.position = origin
				var tween := _shake(node, false, blendable)
				var weight := [0.125]
				tween.set_ease_func(func(_progress: float) -> float: return weight[0])
				tween.custom_step(0.01)
				var first: Vector2 = node.position - origin
				_magnitude(first, 2.5)
				for pair in [[1.125, 1.0], [-0.875, 1.0], [0.5, -2.0], [1.5, -2.0], [-0.5, -2.0], [0.75, 2.0], [-0.25, 2.0], [0.0, 0.0], [1.0, 0.0], [2.0, 0.0], [-1.0, 0.0]]:
					weight[0] = pair[0]
					tween.custom_step(0.01)
					_near(node.position, origin + first * pair[1])
				# A nonzero eased endpoint exposes the distinction between
				# stopped notification and the template's finished cleanup.
				weight[0] = 0.125
				var events := []
				tween.stopped.connect(func():
					_near(node.position, origin + first)
					events.append("stopped"))
				tween.finished.connect(func():
					_near(node.position, origin)
					events.append("finished"))
				if forced: tween.force_complete()
				else: tween.custom_step(1.0)
				assert_true(events == ["stopped", "finished"])
				_near(node.position, origin)
				node.free()
	return true


func test_shake_owners_and_inputs() -> bool:
	for forced in [false, true]:
		for node in [Node2D.new(), Control.new()]:
			add_child(node)
			var tween := _shake(node)
			var events := [0, 0]
			tween.stopped.connect(func(): events[0] += 1)
			tween.finished.connect(func(): events[1] += 1)
			node.free()
			if forced: tween.force_complete()
			else: tween.custom_step(0.125)
			assert_true(tween.is_stopped())
			assert_true(events == [1, 0])
	for forced in [false, true]:
		for node in [Node2D.new(), Control.new()]:
			add_child(node)
			var tween := _shake(node)
			var events := []
			tween.stopped.connect(func(): node.free())
			tween.finished.connect(func(): events.append("finished"))
			if forced: tween.force_complete()
			else: tween.custom_step(1.0)
			assert_true(not is_instance_valid(node))
			assert_true(events == ["finished"])
	for node in [Node2D.new(), Control.new()]:
		add_child(node)
		# Each case includes the expected normalized strength. Invalid calls
		# must still produce registered, processable tweens in both bridges.
		for args in [
			[0.0, Vector2.ONE, 90, Vector2.ONE],
			[-1.0, Vector2.ONE, 90, Vector2.ONE],
			[NAN, Vector2.ONE, 90, Vector2.ONE],
			[INF, Vector2.ONE, 90, Vector2.ONE],
			[-INF, Vector2.ONE, 90, Vector2.ONE],
			[1.0, Vector2(-1, 4), 90, Vector2(0, 4)],
			[1.0, Vector2(3, -1), 90, Vector2(3, 0)],
			[1.0, Vector2(NAN, 4), 90, Vector2(0, 4)],
			[1.0, Vector2(3, INF), 90, Vector2(3, 0)],
			[1.0, Vector2(-INF, NAN), 90, Vector2.ZERO],
			[1.0, Vector2.ONE, -1, Vector2.ONE],
			[1.0, Vector2.ONE, 181, Vector2.ONE],
			[1.0, Vector2.ONE, NAN, Vector2.ONE],
			[1.0, Vector2.ONE, INF, Vector2.ONE],
			[1.0, Vector2.ONE, -INF, Vector2.ONE],
			[-1.0, Vector2(-1, 4), NAN, Vector2(0, 4)],
		]:
			var tween: SpireMethodFloat
			if node is Control:
				tween = _manual(DoControl.shake(node, args[0], args[1], 4, args[2], false))
			else:
				tween = _manual(DoNode2D.shake(node, args[0], args[1], 4, args[2], false))
			assert_eq(tween.get_duration(), 1.0)
			_near(node.position, Vector2.ZERO)
			tween.custom_step(0.125)
			var expected_strength: Vector2 = args[3]
			_magnitude(node.position, expected_strength.length() * 0.5)
			if expected_strength.x == 0: assert_eq(node.position.x, 0.0)
			if expected_strength.y == 0: assert_eq(node.position.y, 0.0)
			assert_true(not tween.is_stopped())
			tween.custom_step(0.875)
			_near(node.position, Vector2.ZERO)
			assert_true(tween.is_stopped())
		for vibrato in [-1, 0, 1, 131072]:
			var bounded: SpireMethodFloat
			if node is Control:
				bounded = _manual(DoControl.shake(node, 1.0, Vector2(3, 4), vibrato, 0, false))
			else:
				bounded = _manual(DoNode2D.shake(node, 1.0, Vector2(3, 4), vibrato, 0, false))
			bounded.custom_step(0.5 / 65536.0 if vibrato > 65536 else 0.25)
			_magnitude(node.position, 2.5)
			bounded.force_complete()
			_near(node.position, Vector2.ZERO)
		var zero: SpireMethodFloat
		if node is Control:
			zero = _manual(DoControl.shake(node, 1.0, Vector2.ZERO, -1, 0))
		else:
			zero = _manual(DoNode2D.shake(node, 1.0, Vector2.ZERO, -1, 0))
		zero.custom_step(0.5)
		zero.force_complete()
		_near(node.position, Vector2.ZERO)
		node.free()
	return true
