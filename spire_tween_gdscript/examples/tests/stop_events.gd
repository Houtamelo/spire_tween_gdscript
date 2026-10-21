extends "res://examples/tests/test_base.gd"

const KINDS := ["sequence", "property", "property_variant", "method", "method_variant", "delayed"]

func make_tween(kind: String, node: Node2D = null) -> RefCounted:
	var tween: RefCounted
	match kind:
		"sequence": tween = Spire.sequence().append_interval(1.0)
		"property", "property_variant":
			if node == null:
				node = Node2D.new()
				add_child(node)
			if kind == "property":
				tween = DoNode2D.position(node, Vector2.ONE, 1.0)
			else:
				tween = Spire.do_property(node, ^"position", Vector2.ONE, 1.0)
		"method": tween = Spire.do_call_float(func(_value): pass, 0.0, 1.0, 1.0)
		"method_variant": tween = Spire.do_call(func(_value): pass, 0.0, 1.0, 1.0)
		"delayed": tween = Spire.do_delayed_call(func(): pass, 1.0)
	tween.set_process_mode(Spire.PROCESS_MODE_MANUAL)
	return tween

func events_for(tween: RefCounted) -> Array:
	var events := []
	var weak: WeakRef = weakref(tween)
	tween.stopped.connect(func():
		assert_true(weak.get_ref().is_stopped(), "Stopped listeners must observe stopped state")
		events.append("stopped"))
	tween.loop_finished.connect(func(): events.append("loop"))
	tween.finished.connect(func(): events.append("finished"))
	return events

func test_stop_transitions_and_pause_after_stop() -> bool:
	for kind in KINDS:
		var tween := make_tween(kind)
		var events := events_for(tween)
		tween.custom_step(0.25)
		tween.pause().pause()
		assert_eq(events, [])
		tween.stop().stop().pause().set_state(Spire.STATE_PAUSED)
		tween.force_complete()
		assert_true(tween.is_stopped())
		assert_eq(events, ["stopped"])
		tween.play()
		assert_eq(tween.get_total_elapsed_time(), 0.0)
		assert_true(tween.custom_step(0.25), "Replay must not complete using its old clock")
		assert_eq(events, ["stopped"])
		tween.set_state(Spire.STATE_STOPPED)
		assert_eq(events, ["stopped", "stopped"])
		tween.unregister()
	return true

func test_stop_completion_order_and_loops() -> bool:
	for kind in KINDS:
		var tween := make_tween(kind)
		tween.set_loops(2)
		var events := events_for(tween)
		assert_true(tween.custom_step(1.25))
		assert_eq(events, ["loop"])
		# Step beyond the endpoint: sequence intervals require positive excess.
		assert_true(not tween.custom_step(1.25))
		assert_eq(events, ["loop", "loop", "stopped", "finished"])
		tween.custom_step(10.0)
		tween.stop()
		assert_eq(events.size(), 4)
		tween.unregister()
	return true

func test_stop_forced_completion_and_replay() -> bool:
	for kind in KINDS:
		for paused in [false, true]:
			var tween := make_tween(kind)
			tween.set_loops(-1)
			var events := events_for(tween)
			if paused:
				tween.pause()
			tween.force_complete()
			tween.force_complete()
			assert_eq(events, ["stopped", "finished"])
			tween.play()
			tween.force_complete()
			assert_eq(events, ["stopped", "finished", "stopped", "finished"])
			tween.unregister()
	return true

func test_stop_invalid_callable_cancellation() -> bool:
	for kind in ["method", "method_variant", "delayed"]:
		for forced in [false, true]:
			for paused in [false, true]:
				var tween: RefCounted
				match kind:
					"method": tween = Spire.do_call_float(Callable(), 0.0, 1.0, 1.0)
					"method_variant": tween = Spire.do_call(Callable(), 0.0, 1.0, 1.0)
					"delayed": tween = Spire.do_delayed_call(Callable(), 1.0)
				tween.set_process_mode(Spire.PROCESS_MODE_MANUAL)
				var events := events_for(tween)
				if paused:
					tween.pause()
				if forced:
					tween.force_complete()
				else:
					if paused:
						tween.play()
					assert_true(not tween.custom_step(0.0))
				tween.stop()
				tween.force_complete()
				assert_eq(events, ["stopped"])
				tween.unregister()
	return true

func test_stop_manager_completion_and_cancellation() -> bool:
	for mode in [Spire.PROCESS_MODE_IDLE, Spire.PROCESS_MODE_PHYSICS]:
		var complete := Spire.do_delayed_call(func(): pass, 0.0).set_process_mode(mode)
		var completed := events_for(complete)
		var invalid := Spire.do_call_float(Callable(), 0.0, 1.0, 100.0).set_process_mode(mode)
		var cancelled := events_for(invalid)
		for _frame in 8:
			await get_tree().process_frame
		assert_eq(completed, ["loop", "stopped", "finished"])
		assert_eq(cancelled, ["stopped"])
		assert_true(not invalid.is_registered())
		complete.unregister()
	return true

func test_stop_dead_bindings_in_all_states() -> bool:
	for kind in KINDS:
		for state in [Spire.STATE_PLAYING, Spire.STATE_PAUSED, Spire.STATE_STOPPED]:
			var target := Node2D.new()
			add_child(target)
			var tween := make_tween(kind, target)
			tween.bind_node(target)
			var events := events_for(tween)
			tween.set_state(state)
			target.free()
			assert_true(not tween.custom_step(0.0))
			assert_true(tween.is_stopped())
			assert_eq(events, ["stopped"])
			tween.unregister()
	return true

func test_stop_failed_property_force_complete() -> bool:
	for paused in [false, true]:
		var target := Node2D.new()
		add_child(target)
		var tween := make_tween("property", target)
		var events := events_for(tween)
		if paused:
			tween.pause()
		target.free()
		tween.force_complete()
		assert_eq(events, ["stopped"])
		tween.unregister()
	return true

func test_stop_signal_connection_flags() -> bool:
	var tween := make_tween("sequence")
	var counts := [0, 0, 0, 0]
	var flags := [0, CONNECT_ONE_SHOT, CONNECT_DEFERRED, CONNECT_DEFERRED | CONNECT_ONE_SHOT]
	for idx in flags.size():
		tween.stopped.connect(func(): counts[idx] += 1, flags[idx])
	tween.stop().stop().play().stop()
	assert_eq(counts, [2, 1, 0, 0])
	await get_tree().process_frame
	assert_eq(counts, [2, 1, 2, 1])
	tween.unregister()
	return true

func test_stop_nested_sequence_cancellation_and_replay() -> bool:
	var child := make_tween("delayed")
	var child_events := events_for(child)
	var inserted := make_tween("method")
	var inserted_events := events_for(inserted)
	var nested := Spire.sequence().append(child).insert(0.0, inserted)
	var nested_events := events_for(nested)
	var counters := [0, 0]
	var parent := Spire.sequence().append(nested)
	parent.set_process_mode(Spire.PROCESS_MODE_MANUAL)
	parent.append_call(func(): counters[0] += 1)
	parent.append_ctor(func():
		counters[1] += 1
		return Spire.sequence())
	var parent_events := events_for(parent)
	parent.stop().stop()
	for events in [child_events, inserted_events, nested_events, parent_events]:
		assert_eq(events, ["stopped"])
	assert_eq(counters, [0, 0])
	parent.play().custom_step(1.25)
	assert_true(parent.is_stopped())
	assert_eq(counters, [1, 1])
	assert_eq(parent_events, ["stopped", "loop", "stopped", "finished"])
	parent.play().force_complete()
	assert_eq(counters, [2, 1])
	parent.unregister()
	return true

func test_stop_sequence_adoption_rearms_children() -> bool:
	for kind in KINDS:
		for child_state in [Spire.STATE_PAUSED, Spire.STATE_STOPPED]:
			for parent_state in [Spire.STATE_PLAYING, Spire.STATE_PAUSED, Spire.STATE_STOPPED]:
				var child := make_tween(kind)
				child.set_state(child_state)
				var events := events_for(child)
				var parent := Spire.sequence()
				parent.set_state(parent_state)
				parent.append(child)
				if parent_state == Spire.STATE_STOPPED:
					parent.play()
				assert_true(child.is_playing())
				parent.force_complete()
				assert_eq(events.slice(-2), ["stopped", "finished"])
				parent.unregister()
	return true

func test_stop_unregister_and_drop_do_not_emit() -> bool:
	var tween := make_tween("sequence")
	var events := events_for(tween)
	var weak: WeakRef = weakref(tween)
	tween.unregister()
	assert_true(tween.is_playing())
	tween = null
	assert_true(weak.get_ref() == null)
	assert_eq(events, [])
	return true
