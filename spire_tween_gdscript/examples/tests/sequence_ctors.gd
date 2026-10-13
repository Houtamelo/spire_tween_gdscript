extends "res://examples/tests/test_base.gd"

class FactoryTarget extends Node:
	func make_child() -> RefCounted:
		return Spire.sequence().append_interval(0.2)

var failures: Array[String] = []
var roots: Array[SpireSequence] = []

func check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		push_error(message)

func near(actual: float, expected: float) -> bool:
	return abs(actual - expected) < 0.000001

func sequence() -> SpireSequence:
	var seq := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
	roots.append(seq)
	return seq

func add_ctor(seq: SpireSequence, operation: int, factory: Callable, offset: float = 0.0) -> SpireSequence:
	match operation:
		0: return seq.append_ctor(factory)
		1: return seq.join_ctor(factory)
		2: return seq.insert_ctor(offset, factory)
	return seq

func _exit_tree() -> void:
	for seq in roots:
		seq.unregister()

func test_bridge_signatures_and_chaining() -> bool:
	var expected := {"append_ctor": [TYPE_CALLABLE], "join_ctor": [TYPE_CALLABLE], "insert_ctor": [TYPE_FLOAT, TYPE_CALLABLE]}
	for method in ClassDB.class_get_method_list(&"SpireSequence", true):
		if not expected.has(method.name):
			continue
		var types: Array = []
		for arg in method.args:
			types.append(arg.type)
		check(types == expected[method.name], "%s must expose the expected argument types" % method.name)
		var return_info: Dictionary = method["return"]
		check(return_info.type == TYPE_OBJECT and return_info.class_name == &"SpireSequence", "Constructor methods must return SpireSequence")
		expected.erase(method.name)
	check(expected.is_empty(), "All three constructor methods must be registered")
	var seq := sequence()
	check(seq.append_ctor(func(): return Spire.sequence()) == seq, "append_ctor must return self")
	check(seq.join_ctor(func(): return Spire.sequence()) == seq, "join_ctor must return self")
	check(seq.insert_ctor(1.0, func(): return Spire.sequence()) == seq, "insert_ctor must return self")
	return failures.is_empty()

func test_append_is_lazy_and_reads_activation_state() -> bool:
	var sprite := Node2D.new()
	add_child(sprite)
	sprite.position = Vector2(10, 0)
	var count := [0]
	var children: Array[SpirePropertyVec2] = []
	var seq := sequence().append_interval(1.0)
	seq.append_ctor(func():
		count[0] += 1
		var child := DoNode2D.position(sprite, sprite.position + Vector2(10, 0), 1.0)
		children.append(child)
		return child)
	check(count[0] == 0, "Scheduling must not construct a child")
	seq.custom_step(0.5)
	check(count[0] == 0, "A future block must remain lazy")
	sprite.position = Vector2(20, 0)
	seq.custom_step(0.6)
	check(count[0] == 1 and children.size() == 1, "The block must construct once on activation")
	check(near(sprite.position.x, 21.0), "The child must read current state and receive only block excess")
	if not children.is_empty():
		check(near(children[0].get_total_elapsed_time(), 0.1), "The child must receive 0.1 seconds")
		check(not children[0].is_registered(), "The manager must release the newly constructed child")
	seq.force_complete()
	return failures.is_empty()

func test_join_constructs_in_parallel_and_waits() -> bool:
	var count := [0]
	var children: Array[SpireDelayedCall] = []
	var seq := sequence().append_interval(1.0)
	seq.join_ctor(func():
		count[0] += 1
		var child := Spire.do_delayed_call(func(): pass, 0.5)
		children.append(child)
		return child)
	seq.custom_step(0.25)
	check(count[0] == 1, "A joined factory must activate with the block")
	if not children.is_empty():
		check(near(children[0].get_total_elapsed_time(), 0.25), "Parallel child must receive the same step")
	seq.custom_step(0.3)
	check(seq.is_playing(), "The longer sibling must keep the parent playing")
	seq.custom_step(0.5)
	check(seq.is_stopped() and count[0] == 1, "The block must complete without reconstructing")
	return failures.is_empty()

func test_join_on_empty_sequence() -> bool:
	var count := [0]
	var seq := sequence().join_ctor(func():
		count[0] += 1
		return Spire.sequence().append_interval(0.2))
	seq.custom_step(0.1)
	check(count[0] == 1 and seq.is_playing(), "Joining an empty sequence must create its first block")
	seq.custom_step(0.2)
	check(seq.is_stopped(), "The first constructed block must complete")
	return failures.is_empty()

func test_insert_waits_for_offset_plus_delay() -> bool:
	var count := [0]
	var children: Array[SpireDelayedCall] = []
	var seq := sequence().set_delay(0.5)
	seq.insert_ctor(1.0, func():
		count[0] += 1
		var child := Spire.do_delayed_call(func(): pass, 1.0)
		children.append(child)
		return child)
	seq.custom_step(0.75)
	check(count[0] == 0 and seq.is_playing(), "The empty parent must wait for its future constructor")
	seq.custom_step(1.0)
	check(count[0] == 1 and seq.is_playing(), "Delay plus offset must determine activation")
	if not children.is_empty():
		check(near(children[0].get_total_elapsed_time(), 0.25), "The first inserted step must be clipped to the offset")
	seq.force_complete()
	return failures.is_empty()

func test_insert_uses_per_update_delta_and_outlives_queue() -> bool:
	var children: Array[SpireDelayedCall] = []
	var seq := sequence().append_interval(0.1)
	seq.insert_ctor(0.25, func():
		var child := Spire.do_delayed_call(func(): pass, 1.0)
		children.append(child)
		return child)
	seq.custom_step(0.3)
	check(seq.is_playing() and children.size() == 1, "An inserted child must outlive the shorter queue")
	if not children.is_empty():
		check(near(children[0].get_total_elapsed_time(), 0.05), "Initial insertion delta must be 0.05")
	seq.custom_step(0.1)
	if not children.is_empty():
		check(near(children[0].get_total_elapsed_time(), 0.15), "Later updates must use delta rather than cumulative time")
	seq.force_complete()
	return failures.is_empty()

func test_factories_run_once_across_loops_and_replay() -> bool:
	for operation in 3:
		var counts := [0, 0]
		var seq := sequence().set_loops(2)
		add_ctor(seq, operation, func():
			counts[0] += 1
			return Spire.do_delayed_call(func(): counts[1] += 1, 0.2), 0.1)
		seq.custom_step(0.31)
		seq.custom_step(0.31)
		check(seq.is_stopped() and counts == [1, 2], "Each constructor must create one child for both loops")
		seq.play()
		seq.custom_step(0.31)
		seq.custom_step(0.31)
		check(seq.is_stopped() and counts == [1, 4], "Replay must reuse the same child for all operations")
	return failures.is_empty()

func test_force_complete_constructs_all_pending_children() -> bool:
	var counts := [0, 0]
	var factory := func():
		counts[0] += 1
		return Spire.do_delayed_call(func(): counts[1] += 1, 100.0)
	var seq := sequence().append_interval(100.0)
	seq.append_ctor(factory).join_ctor(factory).insert_ctor(100.0, factory)
	check(counts == [0, 0], "All forced children must initially remain pending")
	seq.force_complete()
	check(seq.is_stopped() and counts == [3, 3], "Force completion must construct and finish all pending slots")
	seq.force_complete()
	check(counts == [3, 3], "Repeated force completion must not reconstruct or reinvoke completed children")
	return failures.is_empty()

func test_invalid_results_are_skipped_for_all_operations() -> bool:
	for operation in 3:
		for bad_result in [null, 17, RefCounted.new()]:
			var count := [0]
			var seq := sequence()
			add_ctor(seq, operation, func(): return bad_result)
			seq.append_call(func(): count[0] += 1)
			seq.custom_step(0.1)
			check(seq.is_stopped() and count[0] == 1, "A bad factory result must not stall following items")
	return failures.is_empty()

func test_invalid_callables_and_freed_factory_targets() -> bool:
	for operation in 3:
		var target := FactoryTarget.new()
		add_child(target)
		var freed_callable := Callable(target, "make_child")
		target.free()
		for factory in [Callable(), Callable(self, "missing_constructor"), freed_callable]:
			var count := [0]
			var seq := sequence()
			add_ctor(seq, operation, factory)
			seq.append_call(func(): count[0] += 1)
			seq.custom_step(0.1)
			check(seq.is_stopped() and count[0] == 1, "Invalid factories must be removed without stalling")
	return failures.is_empty()

func test_forced_invalid_results_do_not_block_valid_factories() -> bool:
	for operation in 3:
		var count := [0]
		var seq := sequence()
		add_ctor(seq, operation, func(): return null, 10.0)
		add_ctor(seq, operation, func(): return Spire.do_delayed_call(func(): count[0] += 1, 20.0), 11.0)
		seq.force_complete()
		check(seq.is_stopped() and count[0] == 1, "Forced traversal must continue from invalid to valid factories")
	return failures.is_empty()

func test_duplicate_results_are_rejected() -> bool:
	for operation in 3:
		var child := Spire.sequence().append_interval(0.1)
		var seq := sequence().append(child)
		add_ctor(seq, operation, func(): return child)
		seq.custom_step(0.2)
		check(seq.remove(child), "The original child must remain present")
		check(not seq.remove(child), "A factory must not add the same child twice")
	return failures.is_empty()

func test_self_and_ancestor_results_are_rejected() -> bool:
	for operation in 3:
		var seq := sequence()
		add_ctor(seq, operation, func(): return seq)
		# Keep a faulty acceptance from recursively ticking itself before cleanup.
		seq.stop()
		seq.custom_step(0.1)
		check(not seq.remove(seq), "A constructor must not adopt its own sequence")
		var nested := sequence()
		var ancestor := sequence().append(nested)
		add_ctor(nested, operation, func(): return ancestor)
		ancestor.stop()
		nested.custom_step(0.1)
		check(not nested.remove(ancestor), "A constructor must not adopt an ancestor")
	return failures.is_empty()

func test_current_default_ease_and_explicit_override() -> bool:
	var sprite := Node2D.new()
	add_child(sprite)
	var children: Array[SpirePropertyVec2] = []
	var seq := sequence()
	seq.append_ctor(func():
		var child := DoNode2D.position(sprite, Vector2(10, 0), 1.0)
		children.append(child)
		return child)
	seq.join_ctor(func():
		var child := DoNode2D.scale(sprite, Vector2(2, 2), 1.0).set_ease(Spire.EASE_LINEAR)
		children.append(child)
		return child)
	seq.set_default_children_ease(Spire.EASE_OUT_QUAD)
	seq.custom_step(0.25)
	check(near(sprite.position.x, 4.375), "A factory scheduled earlier must use the activation-time default ease")
	check(near(sprite.scale.x, 1.25), "Explicit child easing must survive adoption")
	for child in children:
		check(not child.is_registered(), "Constructed properties must leave global registration")
	seq.force_complete()
	return failures.is_empty()

func test_constructed_child_preserves_binding_and_cancels() -> bool:
	var target := Node.new()
	add_child(target)
	var children: Array[SpireSequence] = []
	var count := [0]
	var seq := sequence().append_ctor(func():
		var child := Spire.sequence().append_interval(100.0).bind_node(target)
		children.append(child)
		return child)
	seq.append_call(func(): count[0] += 1)
	seq.custom_step(0.1)
	check(children.size() == 1, "The bound child must be constructed")
	if not children.is_empty():
		check(children[0].get_bound_nodes() == [target] and not children[0].is_registered(), "Adoption must preserve explicit binding")
	target.queue_free()
	for _i in 4:
		await get_tree().process_frame
	if not children.is_empty():
		check(children[0].is_stopped(), "Freed binding must stop the constructed child")
	seq.custom_step(0.1)
	check(seq.is_stopped() and count[0] == 1, "Cancellation must allow the following block to run")
	return failures.is_empty()

func test_bound_arguments_and_child_finished_signals() -> bool:
	var children: Array[SpireSequence] = []
	var count := [0]
	var factory := func(duration: float):
		var child := Spire.sequence().append_interval(duration)
		child.finished.connect(func(): count[0] += 1)
		children.append(child)
		return child
	var seq := sequence().append_ctor(factory.bind(0.2))
	seq.custom_step(0.3)
	seq.play()
	seq.custom_step(0.3)
	check(children.size() == 1 and count[0] == 2, "Bound factory arguments and child signals must survive replay")
	if not children.is_empty():
		check(seq.remove(children[0]), "A constructed child must support ordinary remove")
	return failures.is_empty()

func test_many_callables_remain_callbacks() -> bool:
	for use_join in [false, true]:
		var children: Array[SpireDelayedCall] = []
		var count := [0]
		var callback := func():
			count[0] += 1
			var child := Spire.do_delayed_call(func(): pass, 100.0).set_process_mode(Spire.PROCESS_MODE_MANUAL)
			children.append(child)
			return child
		var seq := sequence()
		if use_join:
			seq.join_many([callback])
		else:
			seq.append_many([callback])
		seq.custom_step(0.1)
		check(seq.is_stopped() and count[0] == 1, "Bulk Callable arguments must remain callbacks")
		if not children.is_empty():
			check(children[0].is_registered(), "Ordinary callbacks must not adopt their returned tween")
			children[0].unregister()
	return failures.is_empty()

func test_pending_factory_with_weak_parent_can_be_released() -> bool:
	# Do not add this parent to `roots`: this test intentionally releases every owner.
	var seq := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
	var observer: WeakRef = weakref(seq)
	var count := [0]
	seq.append_ctor(func():
		count[0] += 1
		return observer.get_ref())
	seq.stop()
	seq.unregister()
	seq = null
	for _i in 4:
		await get_tree().process_frame
	check(observer.get_ref() == null and count[0] == 0, "A pending factory's weak parent capture must not keep the parent alive")
	return failures.is_empty()
