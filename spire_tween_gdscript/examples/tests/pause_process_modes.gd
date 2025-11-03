extends "res://examples/tests/test_base.gd"

enum Process {
	None,
	Idle,
	Physics,
}

@onready var shape : CollisionShape2D = $BouncyBall/CollisionShape2D
@onready var sprite: Sprite2D = $BouncyBall/CollisionShape2D/Sprite2D

var curr_process: Process

signal process_changed()


func reset_process(): 
	curr_process = Process.None
	process_changed.emit()


func _process(_delta: float) -> void:
	timer += _delta
	curr_process = Process.Idle
	process_changed.emit()
	
	var fn := func():
		assert_eq(curr_process, Process.Idle)
		reset_process()
	
	fn.call_deferred()


func _physics_process(_delta: float) -> void:
	curr_process = Process.Physics
	process_changed.emit()
	
	var fn := func():
		assert_eq(curr_process, Process.Physics)
		reset_process()
	
	fn.call_deferred()


func test_pause_mode_process() -> bool:
	var final_value := Vector2.ONE * 6
	var tween := DoNode2D.scale(shape, final_value, 3.0)\
		.set_pause_mode(Spire.PAUSE_MODE_PROCESS)
	
	get_tree().paused = true
	# Timer doesn't run while scene tree is paused, so we manually increase it here
	timer += 3
	
	await wait_finished(tween, 3)
	assert_eq(shape.scale, final_value)
	return true


func test_pause_mode_stop() -> bool:
	var initial_value := shape.scale
	var final_value := Vector2.ONE * 6
	var tween := DoNode2D.scale(shape, final_value, 3.0)\
		.set_pause_mode(Spire.PAUSE_MODE_STOP)
	
	var tree := get_tree()
	tree.paused = true
	await tree.create_timer(5.0, true, false, true).timeout
	assert_eq(shape.scale, initial_value)
	
	tree.paused = false
	await wait_finished(tween, 3)
	assert_eq(shape.scale, final_value)
	return true


func test_pause_mode_bound() -> bool:
	var initial_value := sprite.scale
	var final_value := Vector2.ONE * 4
	var tween := DoNode2D.scale(sprite, final_value, 3.0)\
		.set_pause_mode(Spire.PAUSE_MODE_BOUND)
	
	sprite.process_mode = Node.PROCESS_MODE_DISABLED
	
	var tree := get_tree()
	tree.paused = true
	
	debug("Pausing tree for 4.0 seconds...")
	await tree.create_timer(4.0).timeout
	# Timer does not update while tree is paused, so we do it manually here.
	timer += 4
	
	assert_eq(sprite.scale, initial_value)
	
	debug("Tree unpaused, waiting 2 seconds...")
	tree.paused = false
	await tree.create_timer(2.0).timeout
	assert_eq(sprite.scale, initial_value)
	
	debug("Setting sprite process mode to INHERIT")
	sprite.process_mode = Node.PROCESS_MODE_INHERIT
	assert_eq(tween.get_total_elapsed_time(), 0)
	assert(tween.is_playing())
	assert(sprite.can_process())
	assert(tween.is_registered())
	await wait_finished(tween, 9)
	assert_eq(sprite.scale, final_value)
	return true


func test_process_mode_idle() -> bool:
	var final_value := Vector2.ONE * 3
	var tween := DoNode2D.scale(sprite, final_value, 4.0)\
		.set_process_mode(Spire.PROCESS_MODE_IDLE)
	
	assert_eq(tween.get_process_mode(), Spire.PROCESS_MODE_IDLE)
	await ensure_only_processing_at(sprite, tween, Process.Idle, 4)
	
	assert_eq(sprite.scale, final_value)
	return true


func test_process_mode_physics() -> bool:
	var final_value := Vector2.ONE * 3
	var tween := DoNode2D.scale(sprite, final_value, 4.0)\
		.set_process_mode(Spire.PROCESS_MODE_PHYSICS)
	
	assert_eq(tween.get_process_mode(), Spire.PROCESS_MODE_PHYSICS)
	#await next_frame()
	await ensure_only_processing_at(sprite, tween, Process.Physics, 4)
	
	assert_eq(sprite.scale, final_value)
	return true


func test_pause_mode_bound_process_mode_physics() -> bool:
	var initial_value := sprite.scale
	var final_value := Vector2.ONE * 4
	var tween := DoNode2D.scale(sprite, final_value, 3.0)\
		.set_pause_mode(Spire.PAUSE_MODE_BOUND)\
		.set_process_mode(Spire.PROCESS_MODE_PHYSICS)
	
	sprite.process_mode = Node.PROCESS_MODE_DISABLED
	
	var tree := get_tree()
	tree.paused = true
	
	debug("Tree paused")
	await wait_seconds(4)
	# Timer doesn't update while tree is paused, so we do it manually here
	timer += 4
	assert_eq(sprite.scale, initial_value)
	
	debug("Tree unpaused")
	tree.paused = false
	await wait_seconds(2)
	assert_eq(sprite.scale, initial_value)
	
	debug("Setting sprite process mode to INHERIT")
	sprite.process_mode = Node.PROCESS_MODE_INHERIT
	assert_eq(tween.get_total_elapsed_time(), 0)
	assert(tween.is_playing())
	assert(sprite.can_process())
	assert(tween.is_registered())
	
	#await next_frame()
	await ensure_only_processing_at(sprite, tween, Process.Physics, 9)
	
	assert_eq(sprite.scale, final_value)
	return true


func ensure_only_processing_at(
	node: Node2D,
	tween: SpirePropertyVector2, 
	process: Process,
	expected_end_time: float
):
	while true:
		if tween.is_stopped(): break
		
		await process_changed
		
		if curr_process == Process.None: 
			continue
		elif curr_process == process:
			var frame_begin_val := node.scale
			var frame_begin_time := tween.get_animation_position()
			await process_changed
			assert_eq(curr_process, Process.None)
			var frame_end_val := node.scale
			var frame_end_time := tween.get_animation_position()
			if abs(frame_begin_time - frame_end_time) > 0.00001:
				assert_ne(frame_begin_val, frame_end_val)
		else:
			var frame_begin_val := node.scale
			await process_changed
			assert_eq(curr_process, Process.None)
			var frame_end_val := node.scale
			assert_eq(frame_begin_val, frame_end_val)
	
	_assert_within_tolerance(_time_since_start_seconds() - expected_end_time)
