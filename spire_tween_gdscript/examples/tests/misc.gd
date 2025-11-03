extends "res://examples/tests/test_base.gd"

@onready var shape: CollisionShape2D = $BouncyBall/CollisionShape2D


func test_force_complete() -> bool:
	var final_value := Vector2.ONE * 2.5
	var tween := DoNode2D.scale(shape, final_value, 5.0)
	tween.force_complete()
	assert(tween.is_stopped())
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
