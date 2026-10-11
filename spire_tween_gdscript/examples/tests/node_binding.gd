extends "res://examples/tests/test_base.gd"

var failures: Array[String] = []
var completions: int = 0


func check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		push_error(message)


func flush_frames() -> void:
	await get_tree().create_timer(0.05).timeout
	for _i in 4:
		await get_tree().process_frame


func target_node() -> Node:
	var target := Node.new()
	add_child(target)
	return target


func completed() -> void:
	completions += 1


func test_clear_removes_all_manager_bindings() -> bool:
	var first := target_node()
	var second := target_node()
	var tween := Spire.sequence().append_interval(10.0).bind_node(first).bind_node(second)
	tween.clear_bound_nodes()
	tween.clear_bound_nodes()
	check(tween.get_bound_nodes().is_empty(), "Clear must remove both explicit bindings")
	first.queue_free()
	second.queue_free()
	await flush_frames()
	check(tween.is_registered() and tween.is_playing(), "Cleared nodes must not cancel the tween")
	tween.stop()
	return failures.is_empty()


func test_clear_preserves_other_tween_binding() -> bool:
	var target := target_node()
	var cleared := Spire.sequence().append_interval(10.0).bind_node(target)
	var bound := Spire.sequence().append_interval(10.0).bind_node(target)
	cleared.clear_bound_nodes()
	target.queue_free()
	await flush_frames()
	check(cleared.is_registered() and cleared.is_playing(), "Clear must affect only its own tween")
	check(not bound.is_registered() and bound.is_stopped(), "Other tween must still be cancelled")
	cleared.stop()
	return failures.is_empty()


func test_duplicate_bind_and_unbind_are_idempotent() -> bool:
	var target := target_node()
	var tween := Spire.sequence().append_interval(10.0).bind_node(target)
	tween.bind_node(target).bind_node(target)
	check(tween.get_bound_nodes().size() == 1, "Duplicate binds must create one explicit binding")
	tween.unbind_node(target)
	tween.unbind_node(target)
	check(tween.get_bound_nodes().is_empty(), "Repeated unbind must leave no explicit binding")
	target.queue_free()
	await flush_frames()
	check(tween.is_registered() and tween.is_playing(), "Unbound node must not cancel the tween")
	tween.stop()
	return failures.is_empty()


func test_unbind_absent_node_preserves_other_tween() -> bool:
	var target := target_node()
	var unrelated := Spire.sequence().append_interval(10.0)
	var bound := Spire.sequence().append_interval(10.0).bind_node(target)
	unrelated.unbind_node(target)
	target.queue_free()
	await flush_frames()
	check(unrelated.is_registered() and unrelated.is_playing(), "Absent unbind must leave its tween unchanged")
	check(not bound.is_registered() and bound.is_stopped(), "Absent unbind must preserve another tween's binding")
	unrelated.stop()
	return failures.is_empty()


func test_unbind_one_of_two_nodes_preserves_remaining_binding() -> bool:
	var first := target_node()
	var second := target_node()
	var tween := Spire.sequence().append_interval(10.0).bind_node(first).bind_node(second)
	tween.unbind_node(first)
	first.queue_free()
	await flush_frames()
	check(tween.is_registered() and tween.is_playing(), "Removing one binding must allow playback")
	check(tween.get_bound_nodes() == [second], "Remaining binding must be reported")
	second.queue_free()
	await flush_frames()
	check(not tween.is_registered() and tween.is_stopped(), "Remaining bound node must cancel the tween")
	return failures.is_empty()


func test_explicit_binding_survives_unregister_and_register() -> bool:
	var target := target_node()
	var tween := Spire.sequence().append_interval(10.0).bind_node(target)
	tween.unregister()
	await flush_frames()
	check(not tween.is_registered(), "Unregister must remove the root")
	tween.register()
	tween.register()
	check(tween.is_registered(), "Explicitly bound tween must re-register")
	check(tween.get_bound_nodes() == [target], "Re-registration must preserve explicit binding")
	target.queue_free()
	await flush_frames()
	check(not tween.is_registered() and tween.is_stopped(), "Re-registered binding must still cancel")
	return failures.is_empty()


func test_completed_bound_tween_registers_and_replays() -> bool:
	var target := target_node()
	var tween := Spire.sequence().append_interval(0.01).bind_node(target)
	tween.finished.connect(completed)
	await flush_frames()
	check(completions == 1 and tween.is_stopped(), "First bound playback must complete")
	tween.register()
	tween.play()
	await flush_frames()
	check(completions == 2, "Registration after completion must preserve replay and signals")
	target.queue_free()
	await flush_frames()
	check(not tween.is_registered(), "Completed replay must retain its binding")
	return failures.is_empty()


func test_bound_children_append_join_and_insert() -> bool:
	for placement in ["append", "join", "insert"]:
		var target := target_node()
		var child := Spire.sequence().append_interval(0.01).bind_node(target)
		var parent := Spire.sequence()
		if placement == "insert":
			# Keep the queue active while the inserted child runs.
			parent.append_interval(0.03)
			parent.insert(0.0, child)
		else:
			parent.call(placement, child)
		check(not child.is_registered(), "Bound child must stop ticking as a root after " + placement)
		child.finished.connect(completed)
		await flush_frames()
		check(child.is_stopped(), "Bound child must finish inside parent after " + placement)
		parent.unregister()
		parent.register()
		parent.play()
		await flush_frames()
		check(parent.is_registered(), "Parent must re-register with bound child after " + placement)
		child.clear_bound_nodes()
		target.queue_free()
		await flush_frames()
		check(parent.is_registered(), "Cleared child binding must not cancel parent after " + placement)
	check(completions == 6, "Each bound child must complete on initial playback and replay")
	return failures.is_empty()


func test_nested_bound_child_cancels_on_node_free() -> bool:
	var target := target_node()
	var child := Spire.sequence().append_interval(10.0).bind_node(target)
	var middle := Spire.sequence().append(child)
	var parent := Spire.sequence().append(middle)
	parent.unregister()
	parent.register()
	target.queue_free()
	await flush_frames()
	check(child.is_stopped() and not child.is_registered(), "Nested explicit binding must cancel its child")
	return failures.is_empty()


func test_clear_after_bound_node_freed() -> bool:
	var target := target_node()
	var tween := Spire.sequence().append_interval(10.0).bind_node(target)
	target.queue_free()
	await flush_frames()
	check(tween.is_stopped() and not tween.is_registered(), "Node destruction must cancel first")
	tween.clear_bound_nodes()
	tween.register()
	tween.play()
	await flush_frames()
	check(tween.get_bound_nodes().is_empty(), "Clear must also accept a previously freed node")
	check(tween.is_registered() and tween.is_playing(), "Clearing dead bindings must permit replay")
	tween.stop()
	return failures.is_empty()


func test_clear_preserves_automatic_property_owner() -> bool:
	var target := Node2D.new()
	add_child(target)
	var explicit := target_node()
	var tween := DoNode2D.position(target, Vector2.ONE, 10.0).bind_node(explicit)
	tween.bind_node(target)
	tween.unbind_node(target)
	tween.clear_bound_nodes()
	explicit.queue_free()
	await flush_frames()
	check(tween.is_registered() and tween.is_playing(), "Clearing explicit bindings must allow property playback")
	target.queue_free()
	await flush_frames()
	check(not tween.is_registered(), "Property owner lifetime must remain automatic after clearing")
	return failures.is_empty()


func test_clear_preserves_automatic_method_owner() -> bool:
	var target := Node2D.new()
	add_child(target)
	var explicit := target_node()
	var tween := Spire.do_call_vec2(target.set_position, Vector2.ZERO, Vector2.ONE, 10.0).bind_node(explicit)
	tween.clear_bound_nodes()
	explicit.queue_free()
	await flush_frames()
	check(tween.is_registered() and tween.is_playing(), "Clearing explicit bindings must allow method playback")
	target.queue_free()
	await flush_frames()
	check(not tween.is_registered(), "Method owner lifetime must remain automatic after clearing")
	return failures.is_empty()


func test_clear_bound_pause_mode_resumes_processing() -> bool:
	var target := target_node()
	target.process_mode = Node.PROCESS_MODE_DISABLED
	var tween := Spire.sequence().append_interval(0.01).bind_node(target)
	tween.finished.connect(completed)
	await flush_frames()
	check(completions == 0 and tween.is_playing(), "Disabled bound node must prevent processing")
	tween.clear_bound_nodes()
	await flush_frames()
	check(completions == 1, "Clearing disabled binding must restore processing")
	return failures.is_empty()
