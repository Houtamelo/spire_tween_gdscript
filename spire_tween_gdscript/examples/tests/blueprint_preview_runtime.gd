extends SceneTree

func _initialize() -> void:
	_run.call_deferred()

func _run() -> void:
	var node := Node2D.new()
	root.add_child(node)
	var blueprint := TweenBlueprint.new()
	node.add_child(blueprint)
	blueprint.kind = Spire.BLUEPRINT_KIND_PROPERTY
	blueprint.property_to_animate = &"position"
	blueprint.property_end_value = Vector2(100, 50)
	blueprint.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	var spawned: Array[SpireSequence] = []
	blueprint.tween_spawned.connect(func(sequence: SpireSequence) -> void: spawned.append(sequence))
	blueprint.play_animation_preview()
	var failed := 0
	if not spawned.is_empty() or node.position != Vector2.ZERO: failed += 1
	var run := blueprint.play()
	run.custom_step(0.2)
	var pose := node.position
	blueprint.stop_animation_preview()
	if not run.is_playing() or node.position != pose: failed += 1
	blueprint.stop()
	node.free()
	print("BLUEPRINT_PREVIEW_RUNTIME_RESULT checks=2 failed=", failed)
	quit(0 if failed == 0 else 1)
