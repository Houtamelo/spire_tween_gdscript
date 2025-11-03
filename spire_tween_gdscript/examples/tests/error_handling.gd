extends "res://examples/tests/test_base.gd"

@onready var sprite: Sprite2D = $BouncyBall/CollisionShape2D/Sprite2D


func test_invalid_enum() -> bool:
	var initial_scale := sprite.scale
	var final_scale := Vector2.ZERO
	@warning_ignore("int_as_enum_without_match")
	# `from()` is Needed for loop to restart properly
	var tween := DoNode2D.scale(sprite, final_scale, 5.0)\
		.from(initial_scale)\
		.set_loops(2, -1 as Spire.LoopMode) # Should print error but work fine otherwise
	
	tween.call(&"set_loops", 2, -1)
	
	await wait_loop_finished(tween, 5)
	 # assert_eq doesn't work because Godot does not allow scale = ZERO
	assert(sprite.scale.distance_to(final_scale) < 0.001)
	
	await get_tree().process_frame
	await get_tree().process_frame
	assert(tween.get_animation_position() < 0.02)
	
	await wait_finished(tween, 10)
	assert(sprite.scale.distance_to(final_scale) < 0.001)
	assert_eq(tween.get_loops_finished(), 2)
	assert_eq(tween.get_loops_left(), 0)
	return true


func test_invalid_tween() -> bool:
	var ref_counted := RefCounted.new()
	# These should print errors but that's it
	assert(!Spire.is_registered(ref_counted))
	Spire.register(ref_counted)
	Spire.unregister(ref_counted)
	
	assert(!Spire.is_registered(null))
	Spire.register(null)
	Spire.unregister(null)
	return true


func test_free_while_playing() -> bool:
	var tween := DoCanvasItem.color_g(sprite, 0, 3.0)
	await wait_seconds(2.0)
	sprite.queue_free()
	debug("Freed!")
	
	await next_frame()
	await next_frame()
	assert(!tween.is_registered())
	assert(tween.is_stopped())
	return true


func test_invalid_sequence_adding() -> bool:
	var final_scale := Vector2.ONE * 3
	var seq := Spire.sequence()
	seq.append(seq)
	var tween := DoNode2D.scale(sprite, final_scale, 3.0)
	seq.append(tween)
	seq.append(tween)
	seq.append_interval(1.0)
	seq.join(tween)
	
	await wait_finished(tween, 3, "Child tween")
	assert_eq(sprite.scale, final_scale)
	return true
