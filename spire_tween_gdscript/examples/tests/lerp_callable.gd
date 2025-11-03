extends "res://examples/tests/test_base.gd"


@onready var ball: Sprite2D = $Sprite2D

var call_count := 0


func test_lerp_call() -> bool:
	var initial_pos := Vector2.ZERO
	var final_pos := Vector2(1920, 1080)
	var tween := Spire.do_call_vector2(custom_call, initial_pos, final_pos, 5)
	assert_eq(tween.get_start_value(), initial_pos)
	assert_eq(tween.get_final_value(), final_pos)
	assert_eq(tween.get_duration(), 5)
	assert_eq(tween.get_callable(), custom_call)
	
	await wait_finished(tween, 5)
	assert_le(abs(_frames_since_start() - call_count), 1)
	assert_eq(ball.global_position, final_pos)
	return true


func test_lerp_call_with_ease() -> bool:
	var initial_pos := Vector2.ZERO
	var final_pos := Vector2(1920, 1080)
	var eased_tween := Spire.do_call_vector2(custom_call, initial_pos, final_pos, 5)\
		.set_ease(Spire.EASE_IN_OUT_CUBIC)
	
	var second_ball := ball.duplicate()
	add_child(second_ball)
	
	var linear_tween := Spire.do_call_vector2(second_ball.set_global_position, initial_pos, final_pos, 5)
	
	while eased_tween.is_playing():
		await next_frame()
		
		# they meet in the middle
		if abs(timer - 2.5) > 0.2 and timer > 0.2 and timer < 4.8: 
			assert_ne(ball.global_position, second_ball.global_position)
	
	if linear_tween.is_playing():
		await wait_finished(linear_tween, 5)
	
	assert_le(abs(_frames_since_start() - call_count), 1)
	assert_eq(ball.global_position, final_pos)
	assert_eq(second_ball.global_position, final_pos)
	return true


func test_lerp_call_with_loop():
	var initial_pos := Vector2.ZERO
	var final_pos := Vector2(1920, 1080)
	var tween := Spire.do_call_vector2(custom_call, initial_pos, final_pos, 2)\
		.set_loops(2, Spire.LOOP_MODE_YOYO)
	
	await wait_loop_finished(tween, 2)
	assert_le(abs(_frames_since_start() - call_count), 1)
	assert_eq(ball.global_position, final_pos)
	
	await wait_loop_finished(tween, 4)
	await wait_finished(tween, 4)
	assert_le(abs(_frames_since_start() - call_count), 1)
	assert_eq(ball.global_position, initial_pos)
	
	return true


func custom_call(val: Vector2):
	ball.global_position = val
	call_count += 1
