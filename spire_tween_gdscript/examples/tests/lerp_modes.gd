extends "res://examples/tests/test_base.gd"

@onready var ball: Sprite2D = $Sprite2D


func test_relative() -> bool:
	var initial_pos := ball.global_position
	var translation := Vector2(0, -500)
	
	var tween := DoNode2D.global_position(ball, translation, 2)\
		.as_relative()
	
	await wait_seconds(1)
	assert(tween.is_relative())
	var progress := tween.get_animation_position() / tween.get_duration()
	var expected_pos := initial_pos + translation * progress
	assert_le(ball.global_position.distance_to(expected_pos), 0.1)
	
	var warp_pos := Vector2(600, 900)
	ball.global_position = warp_pos
	
	await wait_finished(tween, 2)
	expected_pos = warp_pos + translation * (1 - progress)
	assert_le(ball.global_position.distance_to(expected_pos), 0.1)
	return true


func test_two_relatives() -> bool:
	var initial_pos := ball.global_position
	
	var first_trans := Vector2(0, -500)
	var first := DoNode2D.global_position(ball, first_trans, 2)\
		.as_relative()
	
	var second_trans := Vector2(200, -100)
	var second := DoNode2D.global_position(ball, second_trans, 3)\
		.as_relative()
	
	assert(first.is_relative())
	
	await wait_finished(first, 2)
	
	assert(first.is_relative())
	assert(second.is_relative())
	
	var second_progress := second.get_animation_position() / second.get_duration()
	var expected_pos := initial_pos + first_trans + second_trans * second_progress
	assert_le(ball.global_position.distance_to(expected_pos), 0.1)
	
	await wait_finished(second, 3)
	expected_pos = initial_pos + first_trans + second_trans
	assert_le(ball.global_position.distance_to(expected_pos), 0.1)
	
	return true


func test_speed_based() -> bool:
	var initial_pos := ball.global_position
	var final_pos := Vector2(256, 256)
	var speed := 200.0
	
	var target_vfx := ball.duplicate()
	target_vfx.modulate = Color.RED
	add_child(target_vfx)
	target_vfx.global_position = final_pos
	
	var tween := DoNode2D.move(ball, final_pos, speed).as_speed_based()
	
	assert(tween.is_speed_based())
	
	var distance := initial_pos.distance_to(final_pos)
	var expected_time := distance / speed
	
	await wait_finished(tween, expected_time)
	assert_eq(ball.global_position, final_pos)
	
	return true


func test_speed_based_plus_relative() -> bool:
	var final_pos := Vector2(256, 256)
	var speed := 200.0
	
	var target_vfx := ball.duplicate()
	target_vfx.modulate = Color.RED
	add_child(target_vfx)
	target_vfx.global_position = final_pos
	
	var speed_tween := DoNode2D.move(ball, final_pos, speed).as_speed_based()
	
	var trans := Vector2(600, 200)
	var relative_tween := DoNode2D.move(ball, trans, 3).as_relative()
	assert(relative_tween.is_relative())
	
	await wait_finished(relative_tween, 3, "Relative")
	assert(speed_tween.is_playing())
	assert(speed_tween.is_speed_based())
	
	var distance := ball.global_position.distance_to(final_pos)
	var expected_time := distance / speed
	await speed_tween.finished
	assert_le(abs(expected_time + 3 - timer), TIME_TOLERANCE * 2)
	assert_eq(ball.global_position, final_pos)
	
	return true
