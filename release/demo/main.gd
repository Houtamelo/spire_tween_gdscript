extends Control

const State := preload("res://template_state.gd")


func _ready() -> void:
	for argument in OS.get_cmdline_user_args():
		if argument == "--spire-demo-test":
			_test.call_deferred()
		elif argument.begins_with("--spire-demo-screenshot="):
			_screenshot.call_deferred(argument.trim_prefix("--spire-demo-screenshot="))
		elif argument.begins_with("--spire-demo-notices="):
			var file := FileAccess.open(argument.trim_prefix("--spire-demo-notices="), FileAccess.WRITE)
			file.store_string(Engine.get_license_text() + "\n\n" + JSON.stringify(Engine.get_copyright_info(), "\t")
				+ "\n\n" + JSON.stringify(Engine.get_license_info(), "\t"))


func _screenshot(path: String) -> void:
	await get_tree().process_frame
	await RenderingServer.frame_post_draw
	var image := get_viewport().get_texture().get_image()
	var error := image.save_png(path)
	print("SPIRE_DEMO_SCREENSHOT result=", error)
	get_tree().quit(0 if error == OK else 1)


func _test() -> void:
	await get_tree().process_frame
	var cards := $Gallery/Scroll/Grid.get_children()
	var failures := 0
	if cards.size() != 33:
		push_error("SPIRE_DEMO_FAILED expected 33 template cards")
		failures += 1
	for card in cards:
		var effect := card.get_node("Content/TargetSlot/Target/Effect") as TweenBlueprint
		if effect == null or effect.get_script() == null or not effect.get_template_warnings().is_empty():
			push_error("SPIRE_DEMO_FAILED invalid template: " + str(card.name))
			failures += 1
			continue
		effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
		effect.duration = 0.3
		var target := effect.get_resolved_target()
		var original := State.snapshot(target)
		var spawned: Array[SpireSequence] = [null]
		effect.tween_spawned.connect(func(sequence: SpireSequence) -> void: spawned[0] = sequence, CONNECT_ONE_SHOT)
		card.get_node("Content/Buttons/Play").pressed.emit()
		var sequence := spawned[0]
		if sequence == null:
			push_error("SPIRE_DEMO_FAILED Play did not spawn: " + str(card.name))
			failures += 1
			continue
		var changed := false
		for step in [0.07, 0.06, 0.08]:
			sequence.custom_step(step)
			changed = changed or not State.same(original, State.snapshot(target))
		card.get_node("Content/Buttons/Restore").pressed.emit()
		if not changed or not State.same(original, State.snapshot(target)):
			push_error("SPIRE_DEMO_FAILED animation/restoration: " + str(card.name))
			failures += 1
	print("SPIRE_DEMO_RESULT total=", cards.size(), " failed=", failures)
	get_tree().quit(0 if failures == 0 else 1)
