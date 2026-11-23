@tool
extends EditorPlugin

var failures: Array[String] = []


func _enter_tree() -> void:
	_run.call_deferred()


func check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		push_error("SCENE_TEMPLATE_EDITOR_ASSERTION: " + message)


func _property(name: StringName) -> EditorProperty:
	for control: EditorProperty in EditorInterface.get_inspector().find_children("*", "EditorProperty", true, false):
		if control.get_edited_property() == name: return control
	return null


func _frames(count: int = 4) -> void:
	for _frame in count: await get_tree().process_frame


func _reveal(control: Control) -> void:
	# Expand real Inspector sections through their mouse-input path.
	for _attempt in 5:
		if control.is_visible_in_tree(): return
		var section := control.get_parent()
		while section != null and (section.get_class() != "EditorInspectorSection" or not section.is_visible_in_tree()):
			section = section.get_parent()
		if section == null: break
		for pressed in [true, false]:
			var click := InputEventMouseButton.new()
			click.position = section.get_global_rect().position + Vector2(20, 12)
			click.button_index = MOUSE_BUTTON_LEFT
			click.pressed = pressed
			Input.parse_input_event(click)
		await _frames()
	check(control.is_visible_in_tree(), "Template control is visible in the real Inspector")


func _run() -> void:
	await _frames(8)
	EditorInterface.open_scene_from_path("res://examples/effect_templates/gallery.tscn")
	EditorInterface.set_main_screen_editor("2D")
	await _frames(12)
	var scene := EditorInterface.get_edited_scene_root()
	check(scene != null, "Gallery opens in the real editor")
	if scene == null:
		get_tree().quit(1)
		return
	var cards := scene.get_node("Scroll/Grid").get_children()
	check(cards.size() == 33, "Editor gallery contains all 33 scenes")
	for card in cards:
		var target := card.get_node("Content/TargetSlot/Target")
		var effect: TweenBlueprint = target.get_node("Effect")
		var pose: Array = [target.position, target.scale, target.modulate]
		EditorInterface.edit_node(effect)
		await _frames(2)
		check(effect.get_template_warnings().is_empty(), "Editor configuration is valid: " + str(effect.scene_file_path))
		check(_property(&"duration") != null and _property(&"target") != null, "Template root exposes timing and target")
		check(_property(&"style") == null, "Implementation style is stored without an Inspector control")
		check(pose == [target.position, target.scale, target.modulate], "@tool and Inspector do not animate targets")
	# Edit through a real native EditorProperty, then exercise editor Undo/Redo.
	var effect: TweenBlueprint = cards[2].get_node("Content/TargetSlot/Target/Effect")
	EditorInterface.edit_node(effect)
	await _frames()
	check(_property(&"offset") != null and _property(&"scatter_seed") == null, "Slide shows relevant controls")
	var field := _property(&"offset")
	if field != null:
		await _reveal(field)
		field.emit_changed(&"offset", Vector2(21, 34))
		await _frames()
		check(effect.offset == Vector2(21, 34), "Inspector updates scene instance parameter")
		var history := get_undo_redo().get_history_undo_redo(get_undo_redo().get_object_history_id(effect))
		check(history.has_undo(), "Inspector edit is undoable")
		if history.has_undo():
			history.undo()
			await _frames()
			check(effect.offset == Vector2(0, 24), "Undo restores the shipped parameter")
			history.redo()
			await _frames()
			check(effect.offset == Vector2(21, 34), "Redo restores the artist parameter")
	var duration_field := _property(&"duration")
	if duration_field != null: await _reveal(duration_field)
	var ease_field := _property(&"ease")
	check(ease_field != null, "Native easing selector is present")
	for info in effect.get_property_list():
		if info.name != &"ease": continue
		var labels: String = info.hint_string.to_lower().replace("_", "").replace(" ", "")
		for variant in ["inpop", "outpop", "inoutpop", "inbounce", "outbounce", "inoutbounce"]:
			check(labels.contains(variant), "Inspector easing selector includes " + variant)
	if ease_field != null:
		await _reveal(ease_field)
		ease_field.emit_changed(&"ease", Spire.EASE_OUT_POP)
		await _frames()
		check(effect.ease == Spire.EASE_OUT_POP, "Inspector selects native OutPop")
	# Save, close, and reopen a scene containing ordinary PackedScene instances.
	var save_path := "res://authored_templates.tscn"
	EditorInterface.save_scene_as(save_path)
	await _frames()
	var packed := ResourceLoader.load(save_path, "PackedScene", ResourceLoader.CACHE_MODE_IGNORE) as PackedScene
	check(packed != null, "Editor saves authored template instances")
	if packed != null:
		var reloaded := packed.instantiate()
		var saved: TweenBlueprint = reloaded.get_node("Scroll/Grid/Card02/Content/TargetSlot/Target/Effect")
		check(saved.offset == Vector2(21, 34), "Instance override survives scene save/reload")
		check(saved.ease == Spire.EASE_OUT_POP, "Native easing enum survives scene save/reload")
		check(saved.get_script() != null, "Saved template retains its GDScript")
		reloaded.free()
	var image := EditorInterface.get_base_control().get_viewport().get_texture().get_image()
	check(image != null and image.save_png("user://template-inspector.png") == OK, "Real rendered Inspector screenshot is saved")
	print("SCENE_TEMPLATE_EDITOR_RESULT total=33 failed=", failures.size())
	get_tree().quit(0 if failures.is_empty() else 1)
