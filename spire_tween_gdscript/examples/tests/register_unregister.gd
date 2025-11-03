extends "res://examples/tests/test_base.gd"

@onready var shape: CollisionShape2D = $BouncyBall/CollisionShape2D


func test_unregistered_does_not_affect_game() -> bool:
	var initial_scale := shape.scale
	var final_scale := Vector2(3, 3)
	var tween := DoNode2D.scale(shape, final_scale, 5)
	tween.unregister()
	
	await wait_seconds(2)
	
	assert_eq(shape.scale, initial_scale)
	assert_eq(tween.get_total_elapsed_time(), 0)
	assert(tween.is_playing()) # Unregistering does not pause a tween
	assert(!tween.is_registered())
	
	debug("Re-Registering")
	tween.register()
	assert(tween.is_registered())
	debug("Is registered")
	
	await wait_loop_finished(tween, 7)
	await wait_finished(tween, 7)
	
	assert_eq(shape.scale, final_scale)
	assert(tween.is_stopped())
	assert(tween.is_registered())
	return true


func test_unregistered_can_be_manually_stepped() -> bool:
	var final_scale := Vector2(3, 3)
	var tween := DoNode2D.scale(shape, final_scale, 5)
	tween.unregister()
	
	var tree := get_tree()
	var time := Time.get_ticks_msec()
	while tween.is_playing():
		await tree.process_frame
		var curr_time := Time.get_ticks_msec()
		var delta := (curr_time - time) / 1000.0
		time = curr_time
		tween.custom_step(delta)
	
	assert(tween.is_stopped())
	assert(!tween.is_registered())
	assert_eq(shape.scale, final_scale)
	return true
