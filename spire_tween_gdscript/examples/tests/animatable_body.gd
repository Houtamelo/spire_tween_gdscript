extends "res://examples/tests/test_base.gd"


func physics_frames(count: int) -> void:
	for _frame in count:
		await get_tree().physics_frame


func finish_physics_tween(tween) -> void:
	for _frame in 150:
		if tween.is_stopped():
			break
		await physics_frames(1)
	assert_true(tween.is_stopped(), "Physics-clock tween did not finish")
	# The final setter runs before AnimatableBody consumes its synchronized pose.
	await physics_frames(3)


func make_world() -> SubViewport:
	var viewport := SubViewport.new()
	viewport.world_2d = World2D.new()
	viewport.own_world_3d = true
	add_child(viewport)
	return viewport


func shape_2d(body: Node2D, size: Vector2, center := Vector2.ZERO) -> void:
	var shape := RectangleShape2D.new()
	shape.size = size
	var collider := CollisionShape2D.new()
	collider.shape = shape
	collider.position = center
	body.add_child(collider)


func shape_3d(body: Node3D, size: Vector3, center := Vector3.ZERO) -> void:
	var shape := BoxShape3D.new()
	shape.size = size
	var collider := CollisionShape3D.new()
	collider.shape = shape
	collider.position = center
	body.add_child(collider)


func probe_2d(viewport: SubViewport, at: Vector2, collides: bool) -> RigidBody2D:
	var probe := RigidBody2D.new()
	probe.position = at
	probe.gravity_scale = 0
	probe.lock_rotation = true
	probe.collision_layer = 2 if collides else 0
	probe.collision_mask = 1 if collides else 0
	probe.contact_monitor = true
	probe.max_contacts_reported = 16
	shape_2d(probe, Vector2.ONE * 16)
	viewport.add_child(probe)
	return probe


func probe_3d(viewport: SubViewport, at: Vector3, collides: bool) -> RigidBody3D:
	var probe := RigidBody3D.new()
	probe.position = at
	probe.gravity_scale = 0
	probe.lock_rotation = true
	probe.collision_layer = 2 if collides else 0
	probe.collision_mask = 1 if collides else 0
	probe.contact_monitor = true
	probe.max_contacts_reported = 16
	shape_3d(probe, Vector3.ONE * 0.5)
	viewport.add_child(probe)
	return probe


func test_animatable_body_2d_translation() -> bool:
	var viewport := make_world()
	var platform := AnimatableBody2D.new()
	platform.collision_layer = 1
	platform.collision_mask = 2
	shape_2d(platform, Vector2(20, 80))
	viewport.add_child(platform)
	assert_true(platform.sync_to_physics)
	var start := Vector2(45, 0)
	var probe := probe_2d(viewport, start, true)
	var control := probe_2d(viewport, start, false)
	var contacts := [0]
	probe.body_entered.connect(func(body):
		if body == platform:
			contacts[0] += 1)
	await physics_frames(3)
	var tween := DoNode2D.position(platform, Vector2(100, 0), 1.0)\
		.set_ease(Spire.EASE_LINEAR).set_process_mode(Spire.PROCESS_MODE_PHYSICS)
	await physics_frames(12)
	tween.pause()
	await physics_frames(2)
	var paused := platform.position
	await physics_frames(6)
	assert_eq(platform.position, paused)
	assert_true(paused.x > 0 and paused.x < 100)
	tween.play()
	await finish_physics_tween(tween)
	assert_true(platform.position.distance_to(Vector2(100, 0)) < 0.01)
	assert_true(contacts[0] > 0, "Translation must generate real physics contacts")
	assert_true(probe.position.x > 110, "Collider must push the dynamic body")
	assert_true(control.position.distance_to(start) < 0.01, "Masked control must stay still")
	print("ANIMATABLE_2D translation contacts=", contacts[0], " probe=", probe.position)
	viewport.queue_free()
	return true


func test_animatable_body_3d_translation() -> bool:
	var viewport := make_world()
	var parent := Node3D.new()
	parent.position = Vector3(10, 2, 0)
	viewport.add_child(parent)
	var platform := AnimatableBody3D.new()
	platform.collision_layer = 1
	platform.collision_mask = 2
	shape_3d(platform, Vector3(1, 4, 4))
	parent.add_child(platform)
	assert_true(platform.sync_to_physics)
	var origin := platform.global_position
	var start := origin + Vector3(3, 0, 0)
	var probe := probe_3d(viewport, start, true)
	var control := probe_3d(viewport, start, false)
	var contacts := [0]
	probe.body_entered.connect(func(body):
		if body == platform:
			contacts[0] += 1)
	await physics_frames(3)
	var target := origin + Vector3(6, 0, 0)
	var tween := DoNode3D.global_position(platform, target, 1.0)\
		.set_ease(Spire.EASE_LINEAR).set_process_mode(Spire.PROCESS_MODE_PHYSICS)
	await finish_physics_tween(tween)
	assert_true(platform.global_position.distance_to(target) < 0.01)
	assert_true(contacts[0] > 0, "Translation must generate real physics contacts")
	assert_true(probe.position.x > target.x + 0.6, "Collider must push the dynamic body")
	assert_true(control.position.distance_to(start) < 0.01, "Masked control must stay still")
	print("ANIMATABLE_3D translation contacts=", contacts[0], " probe=", probe.position)
	viewport.queue_free()
	return true


func test_animatable_body_2d_rotation_sequence() -> bool:
	var viewport := make_world()
	var door := AnimatableBody2D.new()
	door.collision_layer = 1
	door.collision_mask = 2
	shape_2d(door, Vector2(100, 10), Vector2(50, 0))
	viewport.add_child(door)
	var start := Vector2(45, 45)
	var probe := probe_2d(viewport, start, true)
	var control := probe_2d(viewport, start, false)
	var contacts := [0]
	probe.body_entered.connect(func(body):
		if body == door:
			contacts[0] += 1)
	await physics_frames(3)
	var tween := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_PHYSICS)
	tween.set_default_children_ease(Spire.EASE_LINEAR)
	tween.append_interval(0.1)
	tween.append(DoNode2D.rotation(door, PI / 2, 1.0))
	tween.append_interval(0.1)
	await finish_physics_tween(tween)
	assert_true(absf(door.rotation - PI / 2) < 0.001)
	assert_true(contacts[0] > 0, "Rotating shape must generate real physics contacts")
	assert_true(probe.position.distance_to(start) > 10, "Door must push the dynamic body")
	assert_true(control.position.distance_to(start) < 0.01, "Masked control must stay still")
	print("ANIMATABLE_2D rotation contacts=", contacts[0], " probe=", probe.position)
	viewport.queue_free()
	return true


func test_animatable_body_3d_rotation_sequence() -> bool:
	var viewport := make_world()
	var door := AnimatableBody3D.new()
	door.collision_layer = 1
	door.collision_mask = 2
	shape_3d(door, Vector3(4, 0.4, 0.4), Vector3(2, 0, 0))
	viewport.add_child(door)
	var start := Vector3(1.4, 1.4, 0)
	var probe := probe_3d(viewport, start, true)
	var control := probe_3d(viewport, start, false)
	var contacts := [0]
	probe.body_entered.connect(func(body):
		if body == door:
			contacts[0] += 1)
	await physics_frames(3)
	var target := Vector3(0, 0, PI / 2)
	var tween := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_PHYSICS)
	tween.set_default_children_ease(Spire.EASE_LINEAR)
	tween.append_interval(0.1)
	tween.append(DoNode3D.rotation(door, target, 1.0))
	tween.append_interval(0.1)
	await finish_physics_tween(tween)
	assert_true(door.rotation.distance_to(target) < 0.001)
	assert_true(contacts[0] > 0, "Rotating shape must generate real physics contacts")
	assert_true(probe.position.distance_to(start) > 0.5, "Door must push the dynamic body")
	assert_true(control.position.distance_to(start) < 0.01, "Masked control must stay still")
	print("ANIMATABLE_3D rotation contacts=", contacts[0], " probe=", probe.position)
	viewport.queue_free()
	return true


func test_animatable_body_stop_and_free() -> bool:
	var viewport := make_world()
	var body2 := AnimatableBody2D.new()
	var body3 := AnimatableBody3D.new()
	viewport.add_child(body2)
	viewport.add_child(body3)
	var t2 := DoNode2D.position(body2, Vector2(100, 0), 1.0).set_process_mode(Spire.PROCESS_MODE_PHYSICS)
	var t3 := DoNode3D.position(body3, Vector3(6, 0, 0), 1.0).set_process_mode(Spire.PROCESS_MODE_PHYSICS)
	var finishes := [0]
	t2.finished.connect(func(): finishes[0] += 1)
	t3.finished.connect(func(): finishes[0] += 1)
	await physics_frames(12)
	t2.stop()
	t3.stop()
	await physics_frames(2)
	var stopped2 := body2.position
	var stopped3 := body3.position
	await physics_frames(6)
	assert_eq(body2.position, stopped2)
	assert_eq(body3.position, stopped3)
	assert_true(stopped2.x > 0 and stopped2.x < 100)
	assert_true(stopped3.x > 0 and stopped3.x < 6)
	assert_eq(finishes[0], 0)
	t2.play()
	t3.play()
	body2.queue_free()
	body3.queue_free()
	await physics_frames(3)
	assert_true(t2.is_stopped() and t3.is_stopped(), "Freed targets must cancel their tweens")
	assert_eq(finishes[0], 0)
	viewport.queue_free()
	return true
