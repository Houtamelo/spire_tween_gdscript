extends "res://examples/tests/test_base.gd"

const TOLERANCE := 0.00002
const SAMPLES := [0.1, 0.25, 0.5, 0.75, 1.0]
const KINDS := ["ellipse2", "circle2", "ellipse3", "circle3"]
var failures: Array[String] = []
var cases := 0
var tweens: Array[SpireMethodFloat] = []
var sequences: Array[SpireSequence] = []

func check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		push_error(message)

func near(actual: float, expected: float) -> bool:
	return abs(actual - expected) <= TOLERANCE

func finite3(point: Vector3) -> bool:
	return is_finite(point.x) and is_finite(point.y) and is_finite(point.z)

func track(tween: SpireMethodFloat) -> SpireMethodFloat:
	tweens.append(tween)
	return tween.set_ease(Spire.EASE_LINEAR)

func _exit_tree() -> void:
	for tween in tweens:
		tween.stop()
		tween.unregister()
	for seq in sequences:
		seq.stop()
		seq.unregister()

func new_receiver(kind: String, inside_tree: bool = true) -> Node:
	var node: Node = Node2D.new() if kind.ends_with("2") else Node3D.new()
	if inside_tree:
		add_child(node)
	return node

func template(kind: String, node: Node, duration: float) -> SpireMethodFloat:
	match kind:
		"ellipse2": return track(DoNode2D.ellipsis(node as Node2D, Vector2.ZERO, 0.0, TAU, Vector2(2, 1), Vector2(3, 2), duration))
		"circle2": return track(DoNode2D.circle(node as Node2D, Vector2.ZERO, 0.0, TAU, 2.0, duration))
		"ellipse3": return track(DoNode3D.ellipsis(node as Node3D, Vector3.ZERO, 0.0, TAU, Vector3(2, 1, 0), Vector3(3, 2, 0), Vector3.UP, duration))
		"circle3": return track(DoNode3D.circle(node as Node3D, Vector3.ZERO, 0.0, TAU, 2.0, Vector3.UP, duration))
	return null

func verify_circle(node: Node3D, center: Vector3, axis: Vector3, radius: float, label: String) -> void:
	var normal := Vector3.UP if axis.is_zero_approx() else axis.normalized()
	var offset := node.global_position - center
	check(finite3(node.global_position), label + ": position must be finite")
	check(near(offset.length(), radius), label + ": distance from center must equal radius")
	check(near(offset.dot(normal), 0.0), label + ": circle must stay in its plane")

func test_circle3_stays_planar_with_constant_radius() -> bool:
	var axes := [Vector3.UP, Vector3.LEFT * 3, Vector3.FORWARD * 4, Vector3(2, -3, 5), Vector3(5, 2, -1), Vector3.ZERO]
	var spans := [TAU, -TAU, TAU * 2]
	for axis in axes:
		for center in [Vector3.ZERO, Vector3(3, -5, 7)]:
			for radius in [0.0, 2.5]:
				for span in spans:
					cases += 1
					var node := Node3D.new()
					add_child(node)
					var tween := track(DoNode3D.circle(node, center, 0.35, 0.35 + span, radius, axis, 1.0)).set_process_mode(Spire.PROCESS_MODE_MANUAL)
					var previous := 0.0
					for sample in SAMPLES:
						tween.custom_step(sample - previous)
						previous = sample
						verify_circle(node, center, axis, radius, "circle axis=%s center=%s radius=%s span=%s progress=%s" % [axis, center, radius, span, sample])
					check(tween.is_stopped(), "A completed circle must stop")
					tween.unregister()
					node.free()
	return failures.is_empty()

func test_circle3_full_turns_return_to_start() -> bool:
	for axis in [Vector3.UP, Vector3(2, -3, 5), Vector3(5, 2, -1), Vector3.ZERO]:
		for span in [TAU, -TAU, TAU * 2]:
			cases += 1
			var node := Node3D.new()
			add_child(node)
			var center := Vector3(3, -5, 7)
			var start := track(DoNode3D.ellipsis(node, center, 0.35, 0.35, Vector3(2, 2, 0), Vector3(2, 2, 0), axis, 1.0)).set_process_mode(Spire.PROCESS_MODE_MANUAL)
			start.custom_step(1.0)
			var start_point := node.global_position
			check(finite3(start_point), "Equal-angle ellipse must define a finite circle start")
			var circle := track(DoNode3D.circle(node, center, 0.35, 0.35 + span, 2.0, axis, 1.0)).set_process_mode(Spire.PROCESS_MODE_MANUAL)
			circle.custom_step(1.0)
			check(node.global_position.distance_to(start_point) <= TOLERANCE, "Full revolutions must return to their starting point")
			start.unregister()
			circle.unregister()
			node.free()
	return failures.is_empty()

func test_circle3_uses_global_center_under_transformed_parent() -> bool:
	cases += 1
	var parent := Node3D.new()
	add_child(parent)
	parent.position = Vector3(8, -4, 3)
	parent.rotation = Vector3(0.3, -0.5, 0.7)
	parent.scale = Vector3(3, 0.5, 2)
	var node := Node3D.new()
	parent.add_child(node)
	var center := Vector3(-3, 2, 5)
	var axis := Vector3(2, -3, 5)
	var circle := track(DoNode3D.circle(node, center, 0.0, PI * 1.5, 2.5, axis, 1.0)).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	var previous := 0.0
	for sample in SAMPLES:
		circle.custom_step(sample - previous)
		previous = sample
		verify_circle(node, center, axis, 2.5, "transformed-parent circle progress=%s" % sample)
	circle.unregister()
	parent.free()
	return failures.is_empty()

func test_equal_angle_ellipse2_interpolates_radius() -> bool:
	for angle in [0.0, PI / 2, 0.75]:
		cases += 1
		var node := Node2D.new()
		add_child(node)
		var center := Vector2(2, 3)
		var tween := track(DoNode2D.ellipsis(node, center, angle, angle, Vector2.ONE, Vector2.ONE * 3, 1.0)).set_process_mode(Spire.PROCESS_MODE_MANUAL)
		var previous := 0.0
		for sample in SAMPLES:
			tween.custom_step(sample - previous)
			previous = sample
			var expected: Vector2 = center + Vector2(cos(angle), sin(angle)) * (1.0 + 2.0 * sample)
			check(is_finite(node.global_position.x) and is_finite(node.global_position.y), "Equal-angle 2D ellipse must remain finite")
			check(node.global_position.distance_to(expected) <= TOLERANCE, "Equal-angle 2D ellipse must interpolate radius without rotation")
		check(tween.is_stopped(), "Equal-angle 2D ellipse must complete normally")
		tween.unregister()
		node.free()
	return failures.is_empty()

func test_equal_angle_ellipse3_interpolates_radius() -> bool:
	for axis in [Vector3.UP, Vector3(2, -3, 5)]:
		cases += 1
		var node := Node3D.new()
		add_child(node)
		var center := Vector3(2, 3, -4)
		var tween := track(DoNode3D.ellipsis(node, center, 0.75, 0.75, Vector3(1, 1, 0), Vector3(3, 3, 0), axis, 1.0)).set_process_mode(Spire.PROCESS_MODE_MANUAL)
		var direction := Vector3.ZERO
		var previous := 0.0
		for sample in SAMPLES:
			tween.custom_step(sample - previous)
			previous = sample
			verify_circle(node, center, axis, 1.0 + 2.0 * sample, "equal-angle 3D ellipse progress=%s" % sample)
			var next_direction := (node.global_position - center).normalized()
			if direction != Vector3.ZERO:
				check(next_direction.distance_to(direction) <= TOLERANCE, "Equal-angle 3D ellipse must preserve direction")
			direction = next_direction
		check(tween.is_stopped(), "Equal-angle 3D ellipse must complete normally")
		tween.unregister()
		node.free()
	return failures.is_empty()

func test_ellipse3_preserves_requested_helical_displacement() -> bool:
	cases += 1
	var node := Node3D.new()
	add_child(node)
	var center := Vector3(3, -5, 7)
	var axis := Vector3(2, -3, 5).normalized()
	var tween := track(DoNode3D.ellipsis(node, center, 0.0, TAU, Vector3(2, 2, 2), Vector3(2, 2, 4), axis, 1.0)).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	var previous := 0.0
	for sample in SAMPLES:
		tween.custom_step(sample - previous)
		previous = sample
		var offset := node.global_position - center
		var axial := offset.dot(axis)
		check(finite3(node.global_position), "Helical ellipse must remain finite")
		check(near(axial, (2.0 + 2.0 * sample) * sample), "Ellipse must preserve requested axial displacement")
		check(near((offset - axis * axial).length(), 2.0), "Helical ellipse must preserve in-plane radius")
	tween.unregister()
	node.free()
	return failures.is_empty()

func test_valid_templates_finish_once_normally_and_when_forced() -> bool:
	for kind in KINDS:
		for force in [false, true]:
			cases += 1
			var node := new_receiver(kind)
			var tween := template(kind, node, 1.0).set_process_mode(Spire.PROCESS_MODE_MANUAL)
			var finished := [0]
			tween.finished.connect(func(): finished[0] += 1)
			check(tween.get_bound_nodes() == [node], "Template must bind its receiver")
			check(near(tween.get_start_value(), 0.0) and near(tween.get_final_value(), 1.0), "Template method must interpolate normalized progress")
			if force: tween.force_complete()
			else: tween.custom_step(1.0)
			check(tween.is_stopped() and finished[0] == 1, "Valid template must stop and finish exactly once")
			tween.unregister()
			node.free()
	return failures.is_empty()

func test_immediate_receiver_deletion_cancels_registered_templates() -> bool:
	for kind in KINDS:
		for inside_tree in [false, true]:
			cases += 1
			var node := new_receiver(kind, inside_tree)
			var tween := template(kind, node, 100.0)
			var finished := [0]
			tween.finished.connect(func(): finished[0] += 1)
			node.free()
			for _frame in 4: await get_tree().process_frame
			check(tween.is_stopped() and not tween.is_registered(), "Immediate receiver deletion must stop and unregister " + kind)
			check(finished[0] == 0, "Receiver deletion must cancel without completion")
	return failures.is_empty()

func test_deferred_receiver_deletion_cancels_registered_templates() -> bool:
	for kind in KINDS:
		cases += 1
		var node := new_receiver(kind)
		var tween := template(kind, node, 100.0)
		var finished := [0]
		tween.finished.connect(func(): finished[0] += 1)
		node.queue_free()
		for _frame in 4: await get_tree().process_frame
		check(tween.is_stopped() and not tween.is_registered(), "Deferred receiver deletion must stop and unregister " + kind)
		check(finished[0] == 0, "Receiver deletion must cancel without completion")
	return failures.is_empty()

func deletion_in_sequence(deferred: bool) -> bool:
	for kind in KINDS:
		cases += 1
		var node := new_receiver(kind)
		var tween := template(kind, node, 100.0)
		var finished := [0]
		tween.finished.connect(func(): finished[0] += 1)
		var continued := [0]
		var seq := Spire.sequence().append(tween).append_call(func(): continued[0] += 1)
		sequences.append(seq)
		if deferred: node.queue_free()
		else: node.free()
		for _frame in 4: await get_tree().process_frame
		check(tween.is_stopped() and not tween.is_registered(), "Freed sequence child must stop: " + kind)
		check(seq.is_stopped() and continued[0] == 1, "Parent sequence must continue after receiver deletion: " + kind)
		check(finished[0] == 0, "Canceled sequence child must not emit finished")
		seq.unregister()
	return failures.is_empty()

func test_immediate_receiver_deletion_cancels_sequence_children() -> bool:
	return await deletion_in_sequence(false)

func test_deferred_receiver_deletion_cancels_sequence_children() -> bool:
	return await deletion_in_sequence(true)
