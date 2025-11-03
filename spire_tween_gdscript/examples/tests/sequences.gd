extends "res://examples/tests/test_base.gd"

@onready var shape: CollisionShape2D = $BouncyBall/CollisionShape2D


func _reset(): shape.scale = Vector2.ONE


class MessageQueue extends RefCounted:
	var msgs: Array[String] = []
	var _debug: Callable
	
	func _init(debug: Callable): _debug = debug
	
	func push(msg: String): 
		msgs.push_back(msg)
		_debug.call(msg)


func test_proper_ordering() -> bool:
	var seq := Spire.sequence()
	
	var queue := MessageQueue.new(debug)
	
	seq.append_call(queue.push.bind("First call"))
	seq.append_interval(1.0)
	seq.append_call(queue.push.bind("Second call"))
	seq.join_call(queue.push.bind("Same block as second call"))
	seq.append(Spire.do_delayed_call(queue.push.bind("Third call with delay"), 2.0))
	seq.join_interval(5.0)
	
	await wait_loop_finished(seq, 6)
	await wait_finished(seq, 6)
	
	assert_eq(queue.msgs, [
		"First call",
		"Second call",
		"Same block as second call",
		"Third call with delay",
	])
	return true


func test_stopped_child_does_not_halt_sequence() -> bool:
	var seq := Spire.sequence()
	var stopped_child := DoNode2D.scale(shape, Vector2.ZERO, 1.0)
	seq.append(stopped_child)
	stopped_child.stop()
	var good_child := DoNode2D.scale(shape, Vector2(3, 3), 2.0)
	seq.join(good_child)
	seq.append_interval(1.0)
	var dead_child := DoNode2D.position(shape, Vector2(-500, 30), 0.5)
	seq.append(dead_child)
	seq.append_call(debug.bind("Last block"))
	
	await get_tree().process_frame
	
	assert(stopped_child.is_stopped())
	
	await wait_finished(good_child, 2, "Good child")
	assert_eq(shape.scale, Vector2(3, 3))
	
	shape.queue_free()
	
	await wait_loop_finished(seq, 3)
	await wait_finished(seq, 3)
	return true


func test_append_many() -> bool:
	var shrink_scale := Vector2.ONE
	var grown_scale := Vector2.ONE * 3
	var first := DoNode2D.scale(shape, grown_scale, 1.0)
	var second := DoNode2D.scale(shape, shrink_scale, 3.0)
	var third := DoNode2D.scale(shape, grown_scale, 2.0)
	var fourth := DoNode2D.scale(shape, shrink_scale, 1.0)
	
	var seq := Spire.sequence()
	seq.append_many([
		first,
		second,
		debug.bind("Waiting 5 seconds..."),
		Spire.do_delayed_call(debug.bind("5 seconds waited!"), 5.0),
		third,
		fourth,
		debug.bind("Waiting 2 seconds..."),
		2.0,
		debug.bind("2 seconds waited!")
	])
	
	await wait_finished(first, 1, "First")
	assert_eq(shape.scale, grown_scale)
	
	await wait_finished(second, 4, "Second")
	assert_eq(shape.scale, shrink_scale)
	
	await wait_finished(third, 11, "Third")
	assert_eq(shape.scale, grown_scale)
	
	await wait_finished(fourth, 12, "Fourth")
	assert_eq(shape.scale, shrink_scale)
	
	await wait_finished(seq, 14, "Sequence")
	return true


func test_remove() -> bool:
	var shrink_scale := Vector2.ONE
	var grown_scale := Vector2.ONE * 3
	var first := DoNode2D.scale(shape, grown_scale, 1.0)
	var second := DoNode2D.scale(shape, shrink_scale * 5, 3.0)
	var third := DoNode2D.scale(shape, shrink_scale, 2.0)
	
	var seq := Spire.sequence()
	seq.append(first)
	seq.append(second)
	seq.append(third)
	
	assert(seq.remove(second))
	
	await wait_finished(first, 1, "First")
	assert_eq(shape.scale, grown_scale)
	
	await wait_finished(third, 3, "Third")
	assert_eq(shape.scale, shrink_scale)
	
	await wait_finished(seq, 3, "Sequence")
	return true


func test_remove_midway() -> bool:
	var shrink_scale := Vector2.ONE
	var grown_scale := Vector2.ONE * 3
	var first := DoNode2D.scale(shape, grown_scale, 1.0)
	var second := DoNode2D.scale(shape, grown_scale * 3, 3.0)
	var third := DoNode2D.scale(shape, shrink_scale, 2.0)
	
	var seq := Spire.sequence()
	seq.append(first)
	seq.append(second)
	seq.append(third)
	
	await wait_finished(first, 1, "First")
	assert_eq(shape.scale, grown_scale)
	
	await wait_seconds(1.5)
	assert(seq.remove(second))
	
	await wait_finished(third, 4.5, "Third")
	assert_eq(shape.scale, shrink_scale)
	
	await wait_finished(seq, 4.5, "Sequence")
	return true


func test_remove_call() -> bool:
	var shrink_scale := Vector2.ONE
	var grown_scale := Vector2.ONE * 3
	var first := DoNode2D.scale(shape, grown_scale, 1.0)
	var third := DoNode2D.scale(shape, shrink_scale, 2.0)
	var callable := DoNode2D.scale.bind(shape, Vector2.ONE * 10, 3.0)
	
	var seq := Spire.sequence()
	seq.append(first)
	seq.append_call(callable)
	seq.append(third)
	
	assert(seq.remove_call(callable))
	
	await wait_finished(first, 1, "First")
	assert_eq(shape.scale, grown_scale)
	
	await wait_finished(third, 3, "Third")
	assert_eq(shape.scale, shrink_scale)
	
	await wait_finished(seq, 3, "Sequence")
	return true


func test_default_children_ease() -> bool:
	var initial_scale := shape.scale
	var final_scale := Vector2.ONE * 3
	var first := DoNode2D.scale(shape, final_scale, 3.0)
	var second := DoNode2D.scale(shape, initial_scale, 2.0)
	
	var seq := Spire.sequence().set_default_children_ease(Spire.EASE_IN_EXPO)
	seq.append(first)
	seq.append(second)
	
	assert_eq(first.get_ease(), Spire.EASE_IN_EXPO)
	assert_eq(second.get_ease(), Spire.EASE_IN_EXPO)
	
	await wait_seconds(2.0)
	var progress := first.get_animation_position() / first.get_duration()
	var weight := sample_in_expo(progress)
	var expected_val: Vector2 = lerp(initial_scale, final_scale, weight)
	assert_le(shape.scale.distance_to(expected_val), 0.001)
	
	await wait_finished(first, 3.0)
	progress = first.get_animation_position() / first.get_duration()
	assert_eq(progress, 1)
	assert_eq(shape.scale, final_scale)
	
	await wait_seconds(1.0)
	progress = second.get_animation_position() / second.get_duration()
	weight = sample_in_expo(progress)
	expected_val = lerp(final_scale, initial_scale, weight)
	assert_le(shape.scale.distance_to(expected_val), 0.001)
	
	await wait_finished(seq, 5.0)
	assert_eq(shape.scale, initial_scale)
	return true

# 2. * ((f64::powf(C, B * x - B) / D) - J)
func sample_in_expo(x: float) -> float: 
	const B := 10.0
	const C := 2.0
	const D := 1.99804687
	const J := 0.0004887581
	return 2 * ((pow(C, B * x - B) / D) - J)
