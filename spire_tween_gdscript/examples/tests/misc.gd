extends "res://examples/tests/test_base.gd"

@onready var shape: CollisionShape2D = $BouncyBall/CollisionShape2D


func test_fade_alias() -> bool:
	var plain_node := Node2D.new()
	add_child(plain_node)
	var nodes: Array[Node2D] = [plain_node, shape]
	for node in nodes:
		var initial := Color(0.125, 0.25, 0.5, 1.0)
		node.modulate = initial
		var self_modulate := node.self_modulate
		var canonical := DoCanvasItem.modulate_a(node, 0.0, 1.0)
		var existing_alias := DoCanvasItem.color_a(node, 0.0, 1.0)
		assert_eq(existing_alias.get_property_path(), canonical.get_property_path())
		Spire.unregister(existing_alias)
		var tween: SpirePropertyFloat = DoCanvasItem.fade(node, 0.0, 1.0)
		assert_true(Spire.is_registered(tween))
		assert_eq(tween.get_property_path(), canonical.get_property_path())
		assert_eq(tween.get_property_path(), ^"modulate:a")
		Spire.unregister(canonical)
		tween.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)
		assert_true(tween.custom_step(0.5))
		assert_eq(node.modulate, Color(initial.r, initial.g, initial.b, 0.5))
		assert_true(not tween.custom_step(0.75))
		assert_true(tween.is_stopped())
		assert_eq(node.modulate, Color(initial.r, initial.g, initial.b, 0.0))
		assert_eq(node.self_modulate, self_modulate)
		Spire.unregister(tween)
	plain_node.free()
	return true


func test_fade_alias_freed_owner() -> bool:
	var node := Node2D.new()
	add_child(node)
	var tween := DoCanvasItem.fade(node, 0.0, 1.0).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	var finishes: Array[bool] = []
	tween.finished.connect(func(): finishes.append(true))
	node.free()
	assert_true(not tween.custom_step(0.5))
	assert_true(tween.is_stopped())
	assert_true(finishes.is_empty(), "Freed owners must cancel rather than complete the fade")
	Spire.unregister(tween)
	return true


func test_force_complete() -> bool:
	var final_value := Vector2.ONE * 2.5
	var tween := DoNode2D.scale(shape, final_value, 5.0)
	tween.force_complete()
	assert_true(tween.is_stopped())
	assert_eq(shape.scale, final_value)
	
	await wait_seconds(2.0)
	
	shape.scale = Vector2.ONE
	
	tween.play()
	await wait_finished(tween, 7)
	assert_eq(shape.scale, final_value)
	return true


var dyn_target: Vector2

func test_dyn_target() -> bool:
	dyn_target = Vector2.ONE * 5
	var initial_scale := shape.scale
	var speed := 0.5
	var tween := DoNode2D.scale(shape, Vector2.ZERO, speed)\
		.as_speed_based()\
		.set_dynamic_target(func(): return dyn_target)
	
	var wait_time := 3.0
	await wait_seconds(wait_time)
	var expected_scale := initial_scale.move_toward(dyn_target, speed * wait_time)
	assert_le(shape.scale.distance_to(expected_scale), 0.01)
	
	dyn_target = Vector2.ONE
	var expected_duration := shape.scale.distance_to(dyn_target) / speed
	await wait_finished(tween, wait_time + expected_duration)
	assert_eq(shape.scale, dyn_target)
	
	return true


func test_property_path() -> bool:
	var scale_tween := DoNode2D.scale_x(shape, 5, 1.0)
	assert_eq(scale_tween.get_property_path(), ^"scale:x")
	var pos_tween = DoNode2D.move(shape, Vector2(200, 500), 2.0)
	assert_eq(pos_tween.get_property_path(), ^"global_position")
	
	return true
