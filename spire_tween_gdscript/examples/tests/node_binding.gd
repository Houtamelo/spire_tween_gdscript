extends "res://examples/tests/test_base.gd"

var failures: Array[String] = []
var completions: int = 0
var sequence_binding_roots: Array[SpireSequence] = []
var sequence_binding_children: Array[RefCounted] = []


func _exit_tree() -> void:
	for child in sequence_binding_children:
		child.call("stop")
		child.call("unregister")
	for root in sequence_binding_roots:
		root.stop()
		root.unregister()


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


func sequence_binding_fixture(kind: String, placement: String, nested: bool = false) -> Dictionary:
	var binding := target_node()
	var calls := [0]
	var child: Variant
	match kind:
		"method": child = Spire.do_call_float(func(_value): calls[0] += 1, 0.0, 1.0, 1.0)
		"delayed": child = Spire.do_delayed_call(func(): calls[0] += 1, 1.0)
		"property":
			var receiver := Node2D.new()
			add_child(receiver)
			child = DoNode2D.position(receiver, Vector2.ONE, 1.0)
		"sequence": child = Spire.sequence().append_interval(1.0)
	child.bind_node(binding)
	sequence_binding_children.append(child)
	var host := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
	sequence_binding_roots.append(host)
	var factory := func(): return child
	match placement:
		"append": host.append(child)
		"join": host.append_interval(0.1).join(child)
		"insert": host.insert(0.0, child)
		"append_ctor": host.append_ctor(factory)
		"join_ctor": host.append_interval(0.1).join_ctor(factory)
		"insert_ctor": host.insert_ctor(0.0, factory)
	var continued := [0]
	host.append_call(func(): continued[0] += 1)
	var root := host
	if nested:
		root = Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL).append(host)
		sequence_binding_roots.append(root)
	return {"binding": binding, "child": child, "host": host, "root": root, "calls": calls, "continued": continued}


func test_dead_bindings_remove_playing_paused_and_stopped_sequence_children() -> bool:
	for kind in ["method", "delayed", "property", "sequence"]:
		for placement in ["append", "join", "insert"]:
			for state in ["playing", "paused", "stopped"]:
				for nested in [false, true]:
					var fixture := sequence_binding_fixture(kind, placement, nested)
					var child: Variant = fixture.child
					match state:
						"playing": child.play()
						"paused": child.pause()
						"stopped": child.stop()
					fixture.binding.free()
					fixture.root.custom_step(0.2)
					var label := "%s/%s/%s/nested=%s" % [kind, placement, state, nested]
					check(child.is_stopped() and not child.is_registered(), "Dead binding must cancel " + label)
					check(not fixture.host.remove(child), "Dead child must be removed even when stopped: " + label)
					check(fixture.continued[0] == 1 and fixture.root.is_stopped(), "Parent must continue without spending the canceled child's time: " + label)
					check(fixture.calls[0] == 0, "Canceled child must not invoke its callable: " + label)
	return failures.is_empty()


func test_stopped_children_with_unprocessable_live_bindings_do_not_block() -> bool:
	for kind in ["method", "delayed", "property", "sequence"]:
		for placement in ["append", "join", "insert"]:
			for detached in [false, true]:
				var fixture := sequence_binding_fixture(kind, placement)
				fixture.child.stop()
				if detached: remove_child(fixture.binding)
				else: fixture.binding.process_mode = Node.PROCESS_MODE_DISABLED
				fixture.root.custom_step(0.2)
				check(fixture.continued[0] == 1 and fixture.root.is_stopped(), "A stopped child with a live blocked binding must not stall " + kind + "/" + placement)
				check(fixture.host.remove(fixture.child), "A live stopped child must remain available for replay")
				if detached: fixture.binding.free()
	return failures.is_empty()


func test_bound_children_wait_without_advancing_and_resume_after_reentry() -> bool:
	for kind in ["method", "delayed", "property", "sequence"]:
		for placement in ["append", "join", "insert"]:
			for detached in [false, true]:
				var fixture := sequence_binding_fixture(kind, placement)
				if detached: remove_child(fixture.binding)
				else: fixture.binding.process_mode = Node.PROCESS_MODE_DISABLED
				fixture.root.custom_step(0.2)
				check(fixture.child.get_total_elapsed_time() == 0.0 and fixture.calls[0] == 0, "Blocked child must not advance or invoke callbacks")
				# Inserts run independently of the queue, so its callback may already run.
				var expected_continuation := 1 if placement == "insert" else 0
				check(fixture.continued[0] == expected_continuation and fixture.root.is_playing(), "Blocked child must keep the parent playing")
				if detached: add_child(fixture.binding)
				else: fixture.binding.process_mode = Node.PROCESS_MODE_INHERIT
				fixture.root.custom_step(1.2)
				check(fixture.child.is_stopped() and fixture.continued[0] == 1, "Re-entered/enabled binding must permit continuation: " + kind + "/" + placement)
	return failures.is_empty()


func test_process_pause_mode_allows_live_unprocessable_child_bindings() -> bool:
	for kind in ["method", "delayed", "property", "sequence"]:
		for placement in ["append", "join", "insert"]:
			for detached in [false, true]:
				var fixture := sequence_binding_fixture(kind, placement)
				fixture.child.set_pause_mode(Spire.PAUSE_MODE_PROCESS)
				if detached: remove_child(fixture.binding)
				else: fixture.binding.process_mode = Node.PROCESS_MODE_DISABLED
				fixture.root.custom_step(1.2)
				check(fixture.child.is_stopped() and fixture.continued[0] == 1, "Process pause mode must allow a live disabled/detached binding")
				if detached: fixture.binding.free()
	return failures.is_empty()


func test_manual_step_treats_stop_mode_sequence_children_as_unpaused() -> bool:
	var original_pause := get_tree().paused
	for kind in ["method", "delayed", "property", "sequence"]:
		for placement in ["append", "join", "insert"]:
			for tree_paused in [false, true]:
				get_tree().paused = false
				var fixture := sequence_binding_fixture(kind, placement)
				fixture.child.set_pause_mode(Spire.PAUSE_MODE_STOP)
				get_tree().paused = tree_paused
				fixture.root.custom_step(1.2)
				get_tree().paused = false
				check(fixture.child.is_stopped() and fixture.continued[0] == 1, "Manual stepping must treat Stop pause mode as unpaused: " + kind + "/" + placement)
	get_tree().paused = original_pause
	return failures.is_empty()


func test_lazy_children_with_dead_bindings_are_discarded() -> bool:
	for kind in ["method", "delayed", "property", "sequence"]:
		for placement in ["append_ctor", "join_ctor", "insert_ctor"]:
			for stopped in [false, true]:
				var fixture := sequence_binding_fixture(kind, placement)
				if stopped: fixture.child.stop()
				fixture.binding.free()
				fixture.root.custom_step(0.2)
				check(fixture.child.is_stopped() and not fixture.host.remove(fixture.child), "A constructed child with a dead binding must be discarded")
				check(fixture.continued[0] == 1 and fixture.calls[0] == 0, "Dead constructed child must not block or invoke its callable")
	return failures.is_empty()


func custom_step_fixture(kind: String) -> Dictionary:
	var binding := target_node()
	var calls := [0]
	var finished := [0]
	var tween: Variant
	match kind:
		"method": tween = Spire.do_call_float(func(_value): calls[0] += 1, 0.0, 1.0, 1.0)
		"delayed": tween = Spire.do_delayed_call(func(): calls[0] += 1, 1.0)
		"property":
			var receiver := Node2D.new()
			add_child(receiver)
			tween = DoNode2D.position(receiver, Vector2.ONE, 1.0)
		"sequence": tween = Spire.sequence().append_interval(1.0)
	tween.bind_node(binding).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	tween.finished.connect(func(): finished[0] += 1)
	sequence_binding_children.append(tween)
	return {"binding": binding, "tween": tween, "calls": calls, "finished": finished}


func test_custom_step_reports_completion_and_stopped_bindings() -> bool:
	for kind in ["method", "delayed", "property", "sequence"]:
		var fixture := custom_step_fixture(kind)
		var tween: Variant = fixture.tween
		check(tween.custom_step(0.2), "Active tween must want to live: " + kind)
		check(not tween.custom_step(1.2) and tween.is_stopped(), "Completion must return false: " + kind)
		check(fixture.finished[0] == 1, "Successful completion must emit finished once: " + kind)
		check(not tween.custom_step(0.2), "Already-stopped tween must return false: " + kind)
		fixture.binding.process_mode = Node.PROCESS_MODE_DISABLED
		check(not tween.custom_step(0.2), "Stopped tween with a disabled live binding must return false: " + kind)
		remove_child(fixture.binding)
		check(not tween.custom_step(0.2), "Stopped tween with a detached live binding must return false: " + kind)
		add_child(fixture.binding)
		fixture.binding.process_mode = Node.PROCESS_MODE_INHERIT
		tween.play()
		check(tween.custom_step(0.2), "Retained completed tween must support replay: " + kind)
		check(not tween.custom_step(1.2) and fixture.finished[0] == 2, "Replay must complete normally: " + kind)
	return failures.is_empty()


func test_custom_step_keeps_live_paused_and_blocked_tweens() -> bool:
	for kind in ["method", "delayed", "property", "sequence"]:
		for paused in [false, true]:
			for binding_state in ["enabled", "disabled", "detached"]:
				var fixture := custom_step_fixture(kind)
				var tween: Variant = fixture.tween
				if paused: tween.pause()
				if binding_state == "disabled": fixture.binding.process_mode = Node.PROCESS_MODE_DISABLED
				elif binding_state == "detached": remove_child(fixture.binding)
				var label := "%s/paused=%s/%s" % [kind, paused, binding_state]
				check(tween.custom_step(0.2), "Live waiting tween must want to live: " + label)
				if paused or binding_state != "enabled":
					check(tween.get_total_elapsed_time() == 0.0 and fixture.calls[0] == 0, "Waiting tween must not advance or invoke its callable: " + label)
				check(fixture.finished[0] == 0, "Waiting tween must not emit finished: " + label)
				if binding_state == "disabled": fixture.binding.process_mode = Node.PROCESS_MODE_INHERIT
				elif binding_state == "detached": add_child(fixture.binding)
				if paused: tween.play()
				check(not tween.custom_step(1.2) and fixture.finished[0] == 1, "Resumed tween must complete: " + label)
	return failures.is_empty()


func test_custom_step_reports_dead_bindings_regardless_of_state() -> bool:
	for kind in ["method", "delayed", "property", "sequence"]:
		for state in ["playing", "paused", "stopped"]:
			var fixture := custom_step_fixture(kind)
			var tween: Variant = fixture.tween
			if state == "paused": tween.pause()
			elif state == "stopped": tween.stop()
			fixture.binding.free()
			var label: String = kind + "/" + state
			check(not tween.custom_step(0.2) and tween.is_stopped(), "Dead binding must return false and stop the tween: " + label)
			check(fixture.finished[0] == 0 and fixture.calls[0] == 0, "Binding cancellation must not invoke callbacks or emit finished: " + label)
	return failures.is_empty()


func test_custom_step_reports_invalid_callables() -> bool:
	for kind in ["method", "delayed"]:
		var tween: Variant
		if kind == "method": tween = Spire.do_call_float(Callable(), 0.0, 1.0, 1.0)
		else: tween = Spire.do_delayed_call(Callable(), 1.0)
		tween.set_process_mode(Spire.PROCESS_MODE_MANUAL)
		sequence_binding_children.append(tween)
		var finished := [0]
		tween.finished.connect(func(): finished[0] += 1)
		check(not tween.custom_step(0.2), "Invalid callable must report cancellation: " + kind)
		check(finished[0] == 0, "Invalid callable must not emit finished: " + kind)
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
