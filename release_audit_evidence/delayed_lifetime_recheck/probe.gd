extends SceneTree

var failures := 0

func _initialize() -> void:
	call_deferred(&"_probe")

func frames(count: int) -> void:
	for _frame in count:
		await process_frame

func check(condition: bool, message: String) -> void:
	if not condition:
		failures += 1
		push_error(message)

func _probe() -> void:
	for explicitly_bound in [false, true]:
		var target := Node.new()
		root.add_child(target)
		var call := Callable(target, "queue_free")
		var tween := Spire.do_delayed_call(call, 100.0)
		if explicitly_bound:
			tween.bind_node(target)
		var wrapper: WeakRef = weakref(tween)
		await frames(4)
		check(call.is_valid() and tween.is_registered() and tween.is_playing(), "Initial target/tween must be valid and playing")
		target.queue_free()
		await frames(8)
		var target_valid := is_instance_valid(target)
		print("DELAY_LIFETIME bound=", explicitly_bound,
			" target_valid=", target_valid, " callable_valid=", call.is_valid(),
			" registered=", tween.is_registered(), " stopped=", tween.is_stopped())
		check(not target_valid and not call.is_valid(), "Target must really be freed and callable invalid")
		if explicitly_bound:
			check(not tween.is_registered() and tween.is_stopped(), "Explicit binding must cancel the tween despite its caller-held wrapper")
		else:
			check(tween.is_registered() and tween.is_playing(), "Current unbound delayed-call behavior must reproduce the audit finding")
		tween.unregister()
		tween = null
		await frames(8)
		print("DELAY_WRAPPER_RELEASED bound=", explicitly_bound, " released=", wrapper.get_ref() == null)
		check(wrapper.get_ref() == null, "The old wrapper ownership cycle must not keep the cleaned-up tween alive")
	print("DELAY_LIFETIME_PROBE_RESULT failed=", failures)
	quit(0 if failures == 0 else 1)
