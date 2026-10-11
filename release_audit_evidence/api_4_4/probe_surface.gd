extends SceneTree

func _initialize() -> void:
	call_deferred(&"_probe")

func _probe() -> void:
	var sequence = Spire.sequence()
	if not sequence.has_method(&"close_current_block"):
		push_error("Current source API is missing close_current_block")
		sequence.unregister()
		sequence = null
		quit(1)
		return
	sequence.unregister()
	sequence = null
	for _frame in 4:
		await process_frame
	print("API44_SURFACE_RESULT close_current_block=true")
	quit(0)
