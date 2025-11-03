extends "res://examples/tests/test_base.gd"

@onready var shape: CollisionShape2D = $BouncyBall/CollisionShape2D


func test_delay_is_respected() -> bool:
	var initial_value := shape.scale
	var final_value := Vector2.ONE * 3
	var tween := DoNode2D.scale(shape, final_value, 1.0).set_delay(5.0)
	# Due to how delta time works, it is very unlikely that a frame will end at exactly 5.0 seconds,
	# So by the time this timer is finished there would likely be a small delta already processed
	# by the tween. So we await for 4.9s instead, which makes sure the delay hasn't passed yet.
	await wait_seconds(4.9)
	assert_eq(shape.scale, initial_value)
	assert_eq(tween.get_animation_position(), 0)
	
	await wait_finished(tween, 6)
	assert_eq(shape.scale, final_value)
	return true


func test_extra_loops_dont_have_delay() -> bool:
	var initial_value := shape.scale
	var final_value := Vector2.ONE * 3
	
	var tween := DoNode2D.scale(shape, final_value, 3.0)\
		.set_loops(2, Spire.LOOP_MODE_YOYO)\
		.set_delay(4.0)
	
	await wait_seconds(3.9)
	assert_eq(shape.scale, initial_value)
	
	await wait_loop_finished(tween, 7)
	assert_eq(shape.scale, final_value)
	
	await wait_loop_finished(tween, 10)
	assert_eq(shape.scale, initial_value)
	assert_eq(tween.get_loops_finished(), 2)
	
	await wait_finished(tween, 10)
	assert_eq(tween.get_loops_finished(), 2)
	return true


func test_sequence_respects_delay() -> bool:
	var initial_value := shape.scale
	var final_value := Vector2.ONE * 4
	var seq := Spire.sequence().set_delay(3.0).set_loops(2)
	var first_tween := DoNode2D.scale(shape, final_value, 2.0)
	seq.append(first_tween)
	var second_tween := DoNode2D.scale(shape, initial_value, 2.0)
	seq.append(second_tween)
	
	await wait_seconds(2.9)
	assert_eq(shape.scale, initial_value)
	
	await wait_finished(first_tween, 5, "Grow")
	assert_eq(shape.scale, final_value)
	
	await wait_finished(second_tween, 7, "Shrink")
	assert_eq(shape.scale, initial_value)
	
	await wait_loop_finished(seq, 7, "Sequence")
	assert_eq(shape.scale, initial_value)
	
	assert(seq.get_total_elapsed_time() >= 7.0)
	assert(seq.get_total_elapsed_time() <= 7.1)
	
	await wait_finished(first_tween, 9, "Grow")
	assert_eq(shape.scale, final_value)
	
	await wait_finished(second_tween, 11, "Shrink")
	assert_eq(shape.scale, initial_value)
	
	await wait_finished(seq, 11, "Sequence")
	assert_eq(shape.scale, initial_value)
	return true


func test_delayed_call() -> bool:
	var final_val := Vector2.ONE * 3
	var callable := func(): shape.scale = final_val
	
	var tween := Spire.do_delayed_call(callable, 3)
	assert_eq(tween.get_callable(), callable)
	await wait_finished(tween, 3)
	assert_eq(shape.scale, final_val)
	
	return true
