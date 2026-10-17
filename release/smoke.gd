extends Node


func _ready() -> void:
	var target := Node2D.new()
	add_child(target)
	var calls := [0]
	var seq := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
	seq.append(DoNode2D.position(target, Vector2(10.0, 20.0), 0.1))
	seq.append_call(func(): calls[0] += 1)
	seq.custom_step(0.2)
	if not target.position.is_equal_approx(Vector2(10.0, 20.0)) or calls[0] != 1:
		push_error("SPIRE_RELEASE_SMOKE_FAILED")
		get_tree().quit(1)
		return
	# Also exercise cancellation through the actual installed bridge.
	var invalid := Spire.do_delayed_call(Callable(), 0.1)
	invalid.set_process_mode(Spire.PROCESS_MODE_MANUAL)
	if invalid.custom_step(0.2):
		push_error("SPIRE_RELEASE_SMOKE_FAILED invalid callable stayed alive")
		get_tree().quit(1)
		return
	seq = null
	invalid = null
	target.queue_free()
	for frame in range(4):
		await get_tree().process_frame
	print("SPIRE_RELEASE_SMOKE_OK")
	get_tree().quit(0)
