extends "res://examples/tests/test_base.gd"

var failures: Array[String] = []
var completions: int = 0
var method_value: float = 0.0


func check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		push_error(message)


func flush_frames(count: int = 8) -> void:
	# Keep these cases usable in an uncapped editor as well as --fixed-fps runs.
	await get_tree().create_timer(0.05).timeout
	for _i in count:
		await get_tree().process_frame


func completed() -> void:
	completions += 1


func set_method_value(value: float) -> void:
	method_value = value


func test_completed_sequences_are_collected() -> bool:
	var warmup := Spire.sequence().append_interval(0.01)
	warmup.force_complete()
	warmup = null
	await flush_frames()
	var before := Performance.get_monitor(Performance.OBJECT_COUNT)
	var ids: Array[int] = []
	for _i in 1000:
		var tween := Spire.sequence().append_interval(0.01)
		ids.append(tween.get_instance_id())
		tween.force_complete()
		tween = null
	await flush_frames()
	var retained := Performance.get_monitor(Performance.OBJECT_COUNT) - before
	var live_wrappers := 0
	for id in ids:
		if is_instance_id_valid(id):
			live_wrappers += 1
	print("OWNERSHIP_BULK live_wrappers=", live_wrappers, " object_count_delta=", retained)
	check(live_wrappers == 0, "Completed sequences retained %s wrappers" % live_wrappers)
	return failures.is_empty()


func test_unretained_handle_delivers_finished() -> bool:
	var tween := Spire.sequence().append_interval(0.02)
	var weak: WeakRef = weakref(tween)
	tween.finished.connect(completed)
	tween = null
	await flush_frames(20)
	check(completions == 1, "Unretained handle must deliver finished exactly once")
	check(weak.get_ref() == null, "Unretained completed handle must be collected")
	return failures.is_empty()


func test_unregister_then_release_godot_handle() -> bool:
	var tween := Spire.sequence().append_interval(10.0)
	var weak: WeakRef = weakref(tween)
	tween.unregister()
	await flush_frames()
	check(weak.get_ref() != null, "Caller-held unregistered handle must remain alive")
	check(not tween.is_registered(), "Unregistered handle must leave the manager")
	tween = null
	await flush_frames()
	check(weak.get_ref() == null, "Last Godot reference must release an unregistered tween")
	return failures.is_empty()


func test_retained_completed_handle_restarts() -> bool:
	var tween := Spire.sequence().append_interval(0.02)
	var weak: WeakRef = weakref(tween)
	tween.finished.connect(completed)
	await flush_frames(20)
	check(completions == 1, "First playback must finish")
	check(tween.is_registered(), "Caller-held completed tween must stay registered")
	tween.play()
	await flush_frames(20)
	check(completions == 2, "Completed handle must restart with play alone")
	tween = null
	await flush_frames()
	check(weak.get_ref() == null, "Dropping a retained completed handle must collect it")
	return failures.is_empty()


func test_sequence_children_preserve_signals_across_loops() -> bool:
	var sequence := Spire.sequence().set_loops(2)
	var first := Spire.sequence().append_interval(0.02)
	var second := Spire.sequence().append_interval(0.02)
	var first_weak: WeakRef = weakref(first)
	var second_weak: WeakRef = weakref(second)
	var sequence_weak: WeakRef = weakref(sequence)
	first.finished.connect(completed)
	second.finished.connect(completed)
	sequence.append(first).append(second)
	first = null
	second = null
	sequence = null
	await flush_frames(40)
	check(completions == 4, "Both unretained child handles must finish in both loops")
	check(first_weak.get_ref() == null, "First child must be collected with its sequence")
	check(second_weak.get_ref() == null, "Second child must be collected with its sequence")
	check(sequence_weak.get_ref() == null, "Unretained parent sequence must be collected")
	return failures.is_empty()


func test_bound_node_cancellation_collects_handle() -> bool:
	var target := Node.new()
	add_child(target)
	var tween := Spire.sequence().append_interval(10.0).bind_node(target)
	var weak: WeakRef = weakref(tween)
	tween.finished.connect(completed)
	tween = null
	target.queue_free()
	await flush_frames()
	check(completions == 0, "Node destruction must cancel without emitting finished")
	check(weak.get_ref() == null, "Cancelled bound tween must be collected")
	return failures.is_empty()


func test_abandoned_paused_handle_is_collected() -> bool:
	var tween := Spire.sequence().append_interval(10.0).pause()
	var weak: WeakRef = weakref(tween)
	tween.finished.connect(completed)
	tween = null
	await flush_frames()
	check(completions == 0, "Abandoned paused tween must not emit finished")
	check(weak.get_ref() == null, "Abandoned paused tween must be collected")
	return failures.is_empty()


func test_abandoned_manual_handle_is_collected() -> bool:
	var tween := Spire.sequence().append_interval(10.0).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	var weak: WeakRef = weakref(tween)
	tween = null
	await flush_frames()
	check(weak.get_ref() == null, "Abandoned manually processed tween must be collected")
	return failures.is_empty()


func test_property_handle_is_collected_after_playback() -> bool:
	var target := Node2D.new()
	add_child(target)
	var tween := DoNode2D.position(target, Vector2(10.0, 20.0), 0.02)
	var weak: WeakRef = weakref(tween)
	tween.finished.connect(completed)
	tween = null
	await flush_frames(20)
	check(target.position == Vector2(10.0, 20.0), "Unretained property tween must reach its target")
	check(completions == 1, "Unretained property handle must emit finished")
	check(weak.get_ref() == null, "Completed property handle must be collected")
	return failures.is_empty()


func test_delayed_call_handle_is_collected() -> bool:
	var tween := Spire.do_delayed_call(completed, 0.02)
	var weak: WeakRef = weakref(tween)
	tween = null
	await flush_frames(20)
	check(completions == 1, "Unretained delayed call must invoke its callable")
	check(weak.get_ref() == null, "Completed delayed call handle must be collected")
	return failures.is_empty()


func test_finished_callback_can_restart_unretained_tween() -> bool:
	var tween := Spire.sequence().append_interval(0.02)
	var weak: WeakRef = weakref(tween)
	tween.finished.connect(func():
		completions += 1
		if completions == 1:
			var live: SpireSequence = weak.get_ref()
			live.play()
	)
	tween = null
	await flush_frames(40)
	check(completions == 2, "Synchronous finished callback must be able to restart playback")
	check(weak.get_ref() == null, "Callback-restarted tween must be collected after final playback")
	return failures.is_empty()


func test_retained_paused_handle_can_resume() -> bool:
	var tween := Spire.sequence().append_interval(0.02).pause()
	var weak: WeakRef = weakref(tween)
	tween.finished.connect(completed)
	await flush_frames(20)
	check(tween.is_registered(), "Caller-held paused handle must stay registered")
	check(completions == 0, "Paused handle must not complete")
	tween.play()
	await flush_frames(20)
	check(completions == 1, "Retained paused handle must resume and complete")
	tween = null
	await flush_frames()
	check(weak.get_ref() == null, "Resumed handle must be collected after its caller releases it")
	return failures.is_empty()


func test_retained_manual_handle_can_advance() -> bool:
	var tween := Spire.sequence().append_interval(0.02).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	var weak: WeakRef = weakref(tween)
	tween.finished.connect(completed)
	await flush_frames()
	check(tween.is_registered(), "Caller-held manual handle must stay registered")
	check(completions == 0, "Manual handle must not advance automatically")
	tween.custom_step(0.1)
	check(completions == 1, "Retained manual handle must emit finished when advanced")
	tween = null
	await flush_frames()
	check(weak.get_ref() == null, "Completed manual handle must be collected after release")
	return failures.is_empty()


func test_stopped_handle_is_collected_after_release() -> bool:
	var tween := Spire.sequence().append_interval(10.0)
	var weak: WeakRef = weakref(tween)
	tween.finished.connect(completed)
	tween.stop()
	tween = null
	await flush_frames()
	check(completions == 0, "Stopping a tween must not emit finished")
	check(weak.get_ref() == null, "Stopped unretained handle must be collected")
	return failures.is_empty()


func test_method_handle_is_collected_after_playback() -> bool:
	var tween := Spire.do_call_float(set_method_value, 0.0, 1.0, 0.02)
	var weak: WeakRef = weakref(tween)
	tween.finished.connect(completed)
	tween = null
	await flush_frames(20)
	check(method_value == 1.0, "Unretained method tween must reach its final value")
	check(completions == 1, "Unretained method handle must emit finished")
	check(weak.get_ref() == null, "Completed method handle must be collected")
	return failures.is_empty()
