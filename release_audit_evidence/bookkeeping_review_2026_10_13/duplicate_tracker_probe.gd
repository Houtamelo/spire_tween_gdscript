extends SceneTree
func _initialize() -> void:
	call_deferred("run_probe")
func run_probe() -> void:
	var source := Node.new()
	root.add_child(source)
	var original_id := source.get_instance_id()
	var tween := Spire.sequence().append_interval(10.0).bind_node(source).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	var duplicate := source.duplicate()
	print("DUPLICATE_TRACKER_SHARED ", source.get_meta("__tween_manager_lifetime_tracker") == duplicate.get_meta("__tween_manager_lifetime_tracker"))
	source.free()
	for _i in 12: await process_frame
	print("DUPLICATE_SOURCE_FREED original_alive=", is_instance_id_valid(original_id), " playing=", tween.is_playing(), " registered=", tween.is_registered())
	duplicate.free()
	for _i in 12: await process_frame
	print("DUPLICATE_BOTH_FREED playing=", tween.is_playing(), " registered=", tween.is_registered())
	quit(0)
