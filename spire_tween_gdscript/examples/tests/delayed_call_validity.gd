extends Node

const Waits := preload("res://examples/tests/test_waits.gd")

func run_test(method: Callable) -> bool:
	print("---------------------------------")
	print("Test Started: " + method.get_method())
	var result: bool = await method.call()
	print("Test Finished: " + method.get_method())
	return result

func check(condition: bool, message: String) -> bool:
	if not condition:
		push_error(message)
	return condition

func cleanup(tween: SpireDelayedCall) -> void:
	tween.unregister()

func test_freed_node_during_delay() -> bool:
	var target := Node.new()
	add_child(target)
	var tween := Spire.do_delayed_call(Callable(target, &"queue_free"), 100.0)
	var events := [0, 0]
	tween.finished.connect(func(): events[0] += 1)
	tween.loop_finished.connect(func(): events[1] += 1)
	await Waits.manager_turns(get_tree())
	target.queue_free()
	await Waits.manager_turns(get_tree())
	var ok := check(not is_instance_valid(target), "Node must really be freed")
	ok = check(not tween.get_callable().is_valid(), "Callable must be invalid") and ok
	ok = check(tween.is_stopped() and not tween.is_registered(), "Freed target must cancel before its long delay elapses") and ok
	ok = check(events == [0, 0], "Cancellation must not emit completion signals") and ok
	cleanup(tween)
	return ok

func test_freed_refcounted_during_delay() -> bool:
	var target := RefCounted.new()
	var weak: WeakRef = weakref(target)
	var tween := Spire.do_delayed_call(Callable(target, &"get_instance_id"), 100.0)
	target = null
	await Waits.manager_turns(get_tree())
	var ok := check(weak.get_ref() == null, "RefCounted target must really be freed")
	ok = check(tween.is_stopped() and not tween.is_registered(), "Freed RefCounted target must cancel") and ok
	cleanup(tween)
	return ok

func test_initially_invalid_callables() -> bool:
	var target := Node.new()
	add_child(target)
	var calls: Array[Callable] = [Callable(), Callable(target, &"missing_delayed_method")]
	var ok := true
	for call in calls:
		var tween := Spire.do_delayed_call(call, 100.0)
		await Waits.manager_turns(get_tree())
		ok = check(tween.is_stopped() and not tween.is_registered(), "Initially invalid callable must cancel") and ok
		cleanup(tween)
	return ok

func test_invalid_force_complete_playing_and_paused() -> bool:
	var ok := true
	for mode in [Spire.PROCESS_MODE_IDLE, Spire.PROCESS_MODE_PHYSICS, Spire.PROCESS_MODE_MANUAL]:
		for paused in [false, true]:
			var tween := Spire.do_delayed_call(Callable(), 100.0).set_process_mode(mode)
			var events := [0, 0]
			var weak: WeakRef = weakref(tween)
			tween.finished.connect(func(): events[0] += 1)
			tween.loop_finished.connect(func(): events[1] += 1)
			if paused:
				tween.pause()
			tween.force_complete()
			ok = check(tween.is_stopped() and events == [0, 0], "Invalid forced completion must stop silently") and ok
			await Waits.manager_turns(get_tree())
			ok = check(tween.is_registered(), "A retained stopped call must preserve registration") and ok
			tween.force_complete()
			ok = check(events == [0, 0], "Repeated forced stopping must remain silent") and ok
			tween = null
			await Waits.manager_turns(get_tree())
			ok = check(weak.get_ref() == null, "Manager must release the stopped call when its owner drops it") and ok
	await Waits.manager_turns(get_tree())
	return ok

func test_valid_anonymous_call_respects_delay() -> bool:
	var calls := [0]
	var events := [0, 0]
	var tween := Spire.do_delayed_call(func(): calls[0] += 1, 0.2).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	tween.finished.connect(func(): events[0] += 1)
	tween.loop_finished.connect(func(): events[1] += 1)
	tween.custom_step(0.1)
	var ok := check(calls[0] == 0 and tween.is_playing(), "Valid call must respect its delay")
	tween.set_process_mode(Spire.PROCESS_MODE_IDLE)
	var completed := await Waits.until(get_tree(), func(): return tween.is_stopped())
	ok = check(completed and calls[0] == 1, "Valid anonymous call must execute once before the timeout") and ok
	ok = check(events == [1, 1], "Normal completion signals must be preserved") and ok
	cleanup(tween)
	return ok

func test_valid_refcounted_method() -> bool:
	var target := RefCounted.new()
	var call := Callable(target, &"set_meta").bind(&"delayed_result", 7)
	var tween := Spire.do_delayed_call(call, 0.02)
	var completed := await Waits.until(get_tree(), func(): return tween.is_stopped())
	var ok := check(completed and target.get_meta(&"delayed_result", 0) == 7, "Valid RefCounted callable with bound arguments must execute")
	cleanup(tween)
	return ok

func test_valid_force_complete_and_replay() -> bool:
	var calls := [0]
	var finished := [0]
	var tween := Spire.do_delayed_call(func(): calls[0] += 1, 100.0)
	tween.finished.connect(func(): finished[0] += 1)
	tween.force_complete()
	var ok := check(calls[0] == 1 and finished[0] == 1 and tween.is_stopped(), "Valid forced completion must execute and finish")
	tween.play()
	tween.pause()
	tween.force_complete()
	ok = check(calls[0] == 2 and finished[0] == 2 and tween.is_stopped(), "Valid paused replay must still force-complete") and ok
	cleanup(tween)
	await Waits.manager_turns(get_tree())
	return ok

func test_callback_invalidates_its_looping_target() -> bool:
	var target := Node.new()
	add_child(target)
	var events := [0, 0]
	var tween := Spire.do_delayed_call(Callable(target, &"queue_free"), 0.02).set_loops(-1)
	tween.finished.connect(func(): events[0] += 1)
	tween.loop_finished.connect(func(): events[1] += 1)
	var completed := await Waits.until(get_tree(), func(): return tween.is_stopped())
	var ok := check(completed and not is_instance_valid(target), "Loop callback must free its target")
	ok = check(tween.is_stopped() and not tween.is_registered(), "Next loop must cancel after the target dies") and ok
	ok = check(events == [0, 1], "Only the successful loop may emit its loop signal") and ok
	cleanup(tween)
	return ok

func test_invalid_sequence_child_does_not_block_following_call() -> bool:
	var calls := [0]
	var child_finished := [0]
	var child := Spire.do_delayed_call(Callable(), 100.0)
	child.finished.connect(func(): child_finished[0] += 1)
	var parent := Spire.sequence().append(child).append_call(func(): calls[0] += 1)
	var completed := await Waits.until(get_tree(), func(): return parent.is_stopped())
	var ok := check(completed and child.is_stopped() and not child.is_registered(), "Invalid sequence child must stop")
	ok = check(calls[0] == 1 and parent.is_stopped(), "Sequence must proceed past the cancelled child") and ok
	ok = check(child_finished[0] == 0 and not parent.remove(child), "Cancelled child must be removed without completion") and ok
	cleanup(child)
	parent.unregister()
	return ok

func test_wait_timeout_does_not_complete_a_tween() -> bool:
	var calls := [0]
	var tween := Spire.do_delayed_call(func(): calls[0] += 1, 100.0).pause()
	var completed := await Waits.until(get_tree(), func(): return tween.is_stopped(), 0.03)
	var ok := check(not completed and tween.is_paused() and calls[0] == 0, "An expired test wait must report failure without completing the paused tween")
	cleanup(tween)
	return ok
