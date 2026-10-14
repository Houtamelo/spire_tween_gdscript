extends Node

func run_test(method: Callable) -> bool:
	return await method.call()

func frames(count: int = 8) -> void:
	for _frame in count:
		await get_tree().process_frame

func check(condition: bool, message: String) -> bool:
	if not condition:
		push_error(message)
	return condition

func events(tween: RefCounted) -> Array[int]:
	var counts: Array[int] = [0, 0]
	tween.finished.connect(func(): counts[0] += 1)
	tween.loop_finished.connect(func(): counts[1] += 1)
	return counts

func cancelled(tween: RefCounted, counts: Array[int]) -> bool:
	var ok := check(tween.is_stopped() and not tween.is_registered(), "Invalid method must stop and unregister")
	ok = check(counts == [0, 0], "Cancellation must not emit finished or loop_finished") and ok
	tween.unregister()
	return ok

func test_initially_invalid_callables_across_clocks() -> bool:
	var target := Node.new()
	add_child(target)
	var ok := true
	for mode in [Spire.PROCESS_MODE_IDLE, Spire.PROCESS_MODE_PHYSICS]:
		for call in [Callable(), Callable(target, &"missing_method_validity_target")]:
			var tween := Spire.do_call_float(call, 0.0, 2.0, 100.0).set_process_mode(mode)
			var counts := events(tween)
			await frames()
			ok = cancelled(tween, counts) and ok
	return ok

func test_freed_node_target_during_processing() -> bool:
	var ok := true
	for mode in [Spire.PROCESS_MODE_IDLE, Spire.PROCESS_MODE_PHYSICS]:
		var target := Node2D.new()
		add_child(target)
		var tween := Spire.do_call_float(target.set_rotation, 0.0, 2.0, 100.0).set_process_mode(mode)
		var counts := events(tween)
		target.queue_free()
		await frames()
		ok = check(not is_instance_valid(target), "Method target must really be freed") and ok
		ok = check(not tween.get_callable().is_valid(), "Method callable must become invalid") and ok
		ok = cancelled(tween, counts) and ok
	return ok

func test_invalid_manual_step_keeps_owner_control() -> bool:
	var ok := true
	for kind in ["typed", "variant", "delayed"]:
		var tween: RefCounted
		match kind:
			"typed": tween = Spire.do_call_float(Callable(), 0.0, 2.0, 100.0)
			"variant": tween = Spire.do_call(Callable(), 0.0, 2.0, 100.0)
			"delayed": tween = Spire.do_delayed_call(Callable(), 100.0)
		tween.set_process_mode(Spire.PROCESS_MODE_MANUAL)
		var counts := events(tween)
		ok = check(not tween.custom_step(0.0), "Invalid custom_step must report no further lifetime") and ok
		ok = check(tween.is_stopped() and counts == [0, 0], "Manual invalidation must stop silently") and ok
		await frames()
		ok = check(tween.is_registered(), "Manual stepping must leave registration policy to its caller") and ok
		ok = check(not tween.custom_step(1.0), "Repeated invalid step must stay stopped") and ok
		tween.force_complete()
		ok = check(counts == [0, 0], "Repeated cancellation must stay silent") and ok
		var weak: WeakRef = weakref(tween)
		if kind == "typed":
			# A manual caller may explicitly consume the false result by unregistering.
			tween.unregister()
			ok = check(not tween.is_registered() and weak.get_ref() != null, "Unregistration must preserve the caller's handle") and ok
		tween = null
		await frames()
		ok = check(weak.get_ref() == null, "Dropping the last external handle must let the manager release it") and ok
	await frames()
	return ok

func test_invalid_force_complete_playing_and_paused() -> bool:
	var ok := true
	for mode in [Spire.PROCESS_MODE_IDLE, Spire.PROCESS_MODE_PHYSICS, Spire.PROCESS_MODE_MANUAL]:
		for paused in [false, true]:
			# A non-Node target isolates callable validation from node-deletion tracking.
			var target := Object.new()
			var tween := Spire.do_call_float(Callable(target, &"get_instance_id"), 0.0, 2.0, 100.0)
			tween.set_process_mode(mode).set_delay(100.0).set_loops(-1)
			var counts := events(tween)
			var weak: WeakRef = weakref(tween)
			if paused:
				tween.pause()
			target.free()
			tween.force_complete()
			ok = check(tween.is_stopped() and counts == [0, 0], "Invalid forced completion must stop silently") and ok
			await frames()
			ok = check(tween.is_registered(), "A retained stopped handle must preserve registration") and ok
			tween.force_complete()
			ok = check(counts == [0, 0], "Repeated invalid force_complete must stay silent") and ok
			tween = null
			await frames()
			ok = check(weak.get_ref() == null, "Manager must release the stopped tween when its owner drops it") and ok
	await frames()
	return ok

func test_invalid_custom_method_skips_lerper() -> bool:
	var ok := true
	for forced in [false, true]:
		var lerp_calls := [0]
		var lerper := func(from: Variant, to: Variant, weight: float) -> Variant:
			lerp_calls[0] += 1
			return lerp(from, to, weight)
		var tween := Spire.do_call_custom(Callable(), 0.0, 2.0, 100.0, lerper)
		tween.set_process_mode(Spire.PROCESS_MODE_MANUAL)
		var counts := events(tween)
		if forced:
			tween.force_complete()
		else:
			tween.custom_step(0.25)
		ok = check(tween.is_stopped() and tween.is_registered() and counts == [0, 0], "Custom method validation must stop silently without choosing registration policy") and ok
		ok = check(lerp_calls[0] == 0, "An invalid target must skip its custom lerper") and ok
		tween.unregister()
	await frames()
	return ok

func test_valid_completion_force_complete_and_replay() -> bool:
	var ok := true
	for kind in ["typed", "variant", "custom"]:
		var samples: Array[float] = []
		var sample := func(value: float): samples.append(value)
		var tween: RefCounted
		match kind:
			"typed": tween = Spire.do_call_float(sample, 0.0, 2.0, 0.5)
			"variant": tween = Spire.do_call(sample, 0.0, 2.0, 0.5)
			"custom":
				var lerper := func(from: Variant, to: Variant, weight: float) -> Variant: return lerp(from, to, weight)
				tween = Spire.do_call_custom(sample, 0.0, 2.0, 0.5, lerper)
		tween.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_delay(0.2).set_ease(Spire.EASE_LINEAR)
		var counts := events(tween)
		tween.custom_step(0.1)
		ok = check(samples.is_empty(), "A valid method must respect its delay") and ok
		ok = check(tween.custom_step(0.35) and samples.size() == 1, "A valid method must interpolate") and ok
		ok = check(not tween.custom_step(0.3), "A valid completed method must report completion") and ok
		ok = check(tween.is_stopped() and samples.back() == 2.0 and counts == [1, 1], "Natural completion must preserve the final value and both events") and ok
		for paused in [false, true]:
			tween.play().set_loops(-1)
			if paused:
				tween.pause()
			tween.force_complete()
		ok = check(tween.is_stopped() and samples.size() == 4 and is_equal_approx(samples[0], 1.0) and samples.slice(1) == [2.0, 2.0, 2.0] and counts == [3, 1], "Valid forced completion and paused replay must emit finished without inventing loop events") and ok
		tween.unregister()
	await frames()
	return ok

func test_invalid_sequence_child_does_not_block() -> bool:
	var child := Spire.do_call_float(Callable(), 0.0, 2.0, 100.0)
	var child_events := events(child)
	var calls := [0]
	var parent := Spire.sequence().append(child).append_call(func(): calls[0] += 1)
	parent.set_process_mode(Spire.PROCESS_MODE_MANUAL)
	var parent_events := events(parent)
	parent.custom_step(1.0)
	parent.unregister()
	var ok := cancelled(child, child_events)
	ok = check(calls[0] == 1 and parent.is_stopped() and parent_events == [1, 1], "Sequence must complete after cancelling its invalid child") and ok
	ok = check(not parent.remove(child), "Invalid child must leave the sequence") and ok
	await frames()
	return ok
