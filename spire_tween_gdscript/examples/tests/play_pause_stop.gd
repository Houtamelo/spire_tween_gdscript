extends "res://examples/tests/test_base.gd"

@onready var ball: Node2D = $Ball


func test_play_works() -> bool:
	var final_scale := Vector2(30, 30)
	var tween := DoNode2D.scale(ball, final_scale, 2).from(Vector2(0, 0))
	
	await wait_loop_finished(tween, 2)
	await wait_finished(tween, 2)
	
	assert_eq(ball.scale, final_scale)
	assert(tween.is_stopped())
	return true


func test_pause_play_works() -> bool:
	var initial_scale := ball.scale
	var final_scale := Vector2(30, 30)
	var tween := DoNode2D.scale(ball, final_scale, 2).from(Vector2(0, 0))
	tween.pause()
	
	# Assert that a paused tween does not influence game state.
	await wait_seconds(3)
	
	assert(tween.is_paused())
	assert_eq(tween.get_total_elapsed_time(), 0)
	assert_eq(tween.get_loops_finished(), 0)
	assert_eq(ball.scale, initial_scale)
	
	# Assert that a paused tween can be properly resumed.
	tween.play()
	# `loop_finished` should be emitted before `finished`
	await wait_loop_finished(tween, 5)
	await wait_finished(tween, 5)
	assert_eq(ball.scale, final_scale)
	assert(tween.is_stopped())
	
	# Assert that a stopped tween will never emit any signals.
	tween.loop_finished.connect(func(): printerr("Unexpected emission of signal `loop_finished`."))
	tween.finished.connect(func(): printerr("Unexpected emission of signal `finished`."))
	return true


func test_play_stop_play_works() -> bool:
	var final_scale := Vector2(30, 30)
	var tween := DoNode2D.scale(ball, final_scale, 2).from(Vector2(0, 0))
	
	await wait_loop_finished(tween, 2)
	await wait_finished(tween, 2)
	
	assert_eq(ball.scale, final_scale)
	assert(tween.is_stopped())
	tween.stop()
	
	# Assert that a stopped tween does not influence game state.
	await wait_seconds(3)
	
	# Assert that playing again is fine
	tween.play()
	
	await wait_loop_finished(tween, 7)
	await wait_finished(tween, 7)
	
	assert_eq(ball.scale, final_scale)
	assert(tween.is_stopped())
	return true
