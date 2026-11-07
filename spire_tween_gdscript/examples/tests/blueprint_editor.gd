@tool
extends EditorPlugin

const PROPERTY_TARGET := preload("res://examples/tests/blueprint_property_target.gd")

var failures: Array[String] = []

func _enter_tree() -> void:
	_run.call_deferred()

func check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		push_error(message)

func property_info(effect: TweenBlueprint, name: StringName) -> Dictionary:
	for info in effect.get_property_list():
		if info.name == name: return info
	return {}

func scene_item(node: Node) -> TreeItem:
	for tree: Tree in EditorInterface.get_base_control().find_children("*", "Tree", true, false):
		var items: Array[TreeItem] = []
		if tree.get_root() != null: items.append(tree.get_root())
		while not items.is_empty():
			var item: TreeItem = items.pop_back()
			var metadata: Variant = item.get_metadata(0)
			if typeof(metadata) == TYPE_NODE_PATH and metadata == node.get_path():
				return item
			for child in item.get_children(): items.append(child)
	return null

func scene_warnings(node: Node) -> String:
	var item := scene_item(node)
	if item == null: return ""
	var text := ""
	for button in item.get_button_count(0):
		text += item.get_button_tooltip_text(0, button) + "\n"
	return text

func _run() -> void:
	for _frame in 8: await get_tree().process_frame
	EditorInterface.open_scene_from_path("res://blueprint_editor_fixture.tscn")
	EditorInterface.set_main_screen_editor("2D")
	for _frame in 8: await get_tree().process_frame
	var scene := EditorInterface.get_edited_scene_root()
	check(scene != null, "The editor must open the fixture scene")
	if not scene:
		get_tree().quit(1)
		return
	var target: Node2D = scene
	var effect: TweenBlueprint = scene.get_node("Blueprint")
	var custom: TweenBlueprint = scene.get_node("Custom")
	EditorInterface.edit_node(effect)
	for _frame in 8: await get_tree().process_frame
	check(target.position == Vector2(3, 4), "Editor ready/enter and Inspector selection must not play")
	check(custom.get("build_count") == 0, "Editor lifecycle must not invoke custom builders")
	check(property_info(custom, &"character_distance").usage & PROPERTY_USAGE_EDITOR != 0, "Character settings must not hide similarly named script exports")
	var incomplete := TweenBlueprint.new()
	scene.add_child(incomplete)
	EditorInterface.edit_node(incomplete)
	for _frame in 3: await get_tree().process_frame
	incomplete.kind = Spire.BLUEPRINT_KIND_PROPERTY
	EditorInterface.edit_node(incomplete)
	for _frame in 3: await get_tree().process_frame
	for _query in 3: incomplete.get_property_list()
	incomplete.free()
	EditorInterface.edit_node(effect)
	check(not effect.has_method("play_on"), "The editor API must not expose target overrides")
	check(property_info(effect, &"override_children_targets").is_empty(), "Target propagation must not appear in the Inspector")
	check(property_info(effect, &"trigger_signal").is_empty(), "The signal picker must not appear in the Inspector")
	check(not property_info(effect, &"trigger_condition").hint_string.contains("Signal"), "The trigger dropdown must not contain Signal")
	var other := Node2D.new()
	scene.add_child(other)
	var container := TweenBlueprint.new()
	container.target = other
	scene.add_child(container)
	var child := TweenBlueprint.new()
	child.kind = Spire.BLUEPRINT_KIND_PROPERTY
	child.animated_property = &"position"
	container.add_child(child)
	check(child.get_resolved_target() == scene, "Inspector fallback must ignore a container's configured target")
	child.target = other
	check(child.get_resolved_target() == other, "The Inspector must honor the child's configured target")
	check(property_info(child, &"end_value").type == TYPE_VECTOR2, "Independent child targets must keep typed fields")
	container.free()
	other.free()
	check(property_info(effect, &"end_value").type == TYPE_VECTOR2, "The Inspector must expose the selected property's concrete type")
	var metadata := TweenBlueprint.new()
	metadata.kind = Spire.BLUEPRINT_KIND_PROPERTY
	metadata.target = target
	metadata.animated_property = &"process_mode"
	scene.add_child(metadata)
	var mode_info: Dictionary = ClassDB.class_get_property_list(&"Node").filter(func(info): return info.name == "process_mode")[0]
	check(property_info(metadata, &"end_value").class_name == mode_info.class_name, "Typed metadata must retain Godot's class_name field")
	metadata.animated_property = &" position:x "
	check(property_info(metadata, &"end_value").type == TYPE_FLOAT, "Indexed properties must expose the component's type")
	check(property_info(metadata, &"end_value").class_name == &"", "A scalar component must not carry the target node's class name")
	check(property_info(metadata, &"end_value").hint == PROPERTY_HINT_NONE, "Indexed properties must clear the root property's hints")
	metadata.target = custom
	check(property_info(metadata, &"animated_property").hint_string.is_empty(), "Unsupported Blueprint targets must not offer property suggestions")
	var validation_target := PROPERTY_TARGET.new()
	scene.add_child(validation_target)
	metadata.target = validation_target
	var suggestions: PackedStringArray = property_info(metadata, &"animated_property").hint_string.split(",")
	check("dynamic_scalar" in suggestions and "seed" in suggestions, "Suggestions must include writable dynamic properties and script variables with excluded built-in names")
	for name in ["readonly_scalar", "unsupported_toggle", "editor_description", "discrete_value", "file_value", "shape"]:
		check(name not in suggestions, "Suggestions must exclude read-only, unsupported and semantically unsuitable properties: " + name)
	for path in [&"dynamic_scalar", &"shape:distance"]:
		metadata.animated_property = path
		check(property_info(metadata, &"end_value").type == TYPE_FLOAT, "Valid dynamic and indexed Resource properties must have typed endpoint fields")
	metadata.free()
	validation_target.free()
	check(property_info(effect, &"start_value").usage & PROPERTY_USAGE_EDITOR == 0, "Disabled explicit start must be hidden")
	effect.start_enabled = true
	check(property_info(effect, &"start_value").type == TYPE_VECTOR2, "Explicit start must use the property type")
	check(property_info(effect, &"start_value").usage & PROPERTY_USAGE_EDITOR != 0, "Enabled explicit start must be visible")
	var custom_kind := custom.kind
	custom.kind = Spire.BLUEPRINT_KIND_PROPERTY
	for name in [&"animated_property", &"speed_or_duration", &"start_enabled", &"start_value", &"end_value", &"lerp_mode", &"ease_basic", &"ease_curve"]:
		check(property_info(effect, name).usage & PROPERTY_USAGE_STORAGE != 0, "Visible fields must remain serializable: " + name)
		check(property_info(custom, name).usage & PROPERTY_USAGE_EDITOR == 0, "A script must hide built-in animation fields even when kind is Property: " + name)
		check(property_info(custom, name).usage & PROPERTY_USAGE_STORAGE != 0, "A script's hidden built-in fields must remain serializable: " + name)
	custom.kind = custom_kind
	var text := SpireRichTextLabel.new()
	text.name = "CharacterText"
	text.text = "ABC"
	scene.add_child(text)
	text.owner = scene
	for _frame in 8: await get_tree().process_frame
	check(scene_item(text) != null, "The Scene dock must display SpireRichTextLabel")
	check(not scene_warnings(text).contains("automatic character"), "An unscripted label must not warn about _process")
	for script_name in ["rich_text_label_process_override", "rich_text_label_process_inherited"]:
		text.set_script(load("res://examples/tests/" + script_name + ".gd"))
		for _frame in 8: await get_tree().process_frame
		check(scene_warnings(text).contains("automatic character") and scene_warnings(text).contains("_process"), "The Scene dock must warn for direct and inherited _process overrides: " + script_name)
	text.set_script(load("res://examples/tests/rich_text_label_no_process.gd"))
	for _frame in 8: await get_tree().process_frame
	check(not scene_warnings(text).contains("automatic character"), "A script without _process must clear the warning")
	text.set_script(load("res://examples/tests/rich_text_label_process_override.gd"))
	for _frame in 8: await get_tree().process_frame
	check(scene_warnings(text).contains("automatic character"), "Reattaching an overriding script must restore the warning")
	text.set_script(null)
	for _frame in 8: await get_tree().process_frame
	check(not scene_warnings(text).contains("automatic character"), "Removing the script must clear the warning")
	var plain_text := RichTextLabel.new()
	check(text.text == "ABC" and text.get_child_count(true) == plain_text.get_child_count(true), "Editor warning checks must not mutate text or add helper nodes")
	plain_text.free()
	var characters := TweenBlueprint.new()
	characters.name = "Characters"
	characters.kind = Spire.BLUEPRINT_KIND_CHARACTERS
	text.add_child(characters)
	characters.owner = scene
	for _frame in 3: await get_tree().process_frame
	check(characters.target == null and characters.get_resolved_target() == text, "Characters must support ancestor target fallback")
	check(property_info(characters, &"end_value").type == TYPE_VECTOR2, "Offset characters must use the Vector2 editor")
	check(characters.start_value == Vector2.ZERO and characters.end_value == Vector2.ZERO, "Character defaults must initialize without writing to the target")
	characters.character_animation = Spire.CHARACTER_ANIMATION_FADE
	for _frame in 3: await get_tree().process_frame
	check(property_info(characters, &"end_value").type == TYPE_FLOAT and characters.end_value == 1.0, "Unedited defaults must follow a new character channel's type")
	characters.end_value = 0.25
	characters.character_animation = Spire.CHARACTER_ANIMATION_ROTATION
	for _frame in 3: await get_tree().process_frame
	check(characters.end_value == 0.25, "Changing channels must preserve explicitly authored incompatible values")
	characters.end_value = Vector3(0, 0, TAU)
	characters.start_value = Vector3.ZERO
	characters.character_pivot = Vector2(12, -14)
	characters.character_rotate_mode = Spire.ROTATE_MODE_FAST_BEYOND_360
	characters.character_loops = 2
	characters.character_loop_mode = Spire.LOOP_MODE_YOYO
	for name in [&"character_animation", &"character_first", &"character_last", &"character_stagger", &"character_pivot", &"character_rotate_mode", &"character_loops", &"character_loop_mode", &"end_value", &"speed_or_duration"]:
		check(property_info(characters, name).usage & PROPERTY_USAGE_EDITOR != 0, "Relevant character fields must be visible: " + name)
		check(property_info(characters, name).usage & PROPERTY_USAGE_STORAGE != 0, "Character fields must remain serializable: " + name)
	for name in [&"animated_property", &"lerp_mode", &"character_vibrato", &"character_elasticity", &"character_randomness"]:
		check(property_info(characters, name).usage & PROPERTY_USAGE_EDITOR == 0, "Irrelevant character fields must be hidden: " + name)
		check(property_info(characters, name).usage & PROPERTY_USAGE_STORAGE != 0, "Hidden settings must retain serialization: " + name)
	characters.character_animation = Spire.CHARACTER_ANIMATION_SHAKE_ROTATION
	check(property_info(characters, &"character_randomness").usage & PROPERTY_USAGE_EDITOR != 0, "Shake must expose randomness controls")
	check(property_info(characters, &"character_elasticity").usage & PROPERTY_USAGE_EDITOR == 0, "Shake must hide punch controls")
	characters.character_animation = Spire.CHARACTER_ANIMATION_ROTATION
	EditorInterface.edit_node(characters)
	for _frame in 3: await get_tree().process_frame
	check(text.get_child_count() == 1 and text.text == "ABC", "Editor inspection must not add helper nodes or mutate text")
	for kind in [Spire.BLUEPRINT_KIND_CHAIN_BLOCK, Spire.BLUEPRINT_KIND_PARALLEL_BLOCK]:
		effect.kind = kind
		check(property_info(effect, &"end_value").usage & PROPERTY_USAGE_EDITOR == 0, "Container fields must be hidden")
		check(property_info(effect, &"end_value").usage & PROPERTY_USAGE_STORAGE != 0, "Hidden fields must remain serializable")
	effect.kind = Spire.BLUEPRINT_KIND_PROPERTY
	var undo := EditorInterface.get_editor_undo_redo()
	undo.create_action("Blueprint endpoint test")
	undo.add_do_property(effect, &"end_value", Vector2(8, 9))
	undo.add_undo_property(effect, &"end_value", effect.end_value)
	undo.commit_action()
	check(effect.end_value == Vector2(8, 9), "The editor must apply a property edit")
	var history := undo.get_history_undo_redo(undo.get_object_history_id(effect))
	history.undo()
	check(effect.end_value == Vector2(6, 7), "Undo must restore the authored value")
	history.redo()
	check(effect.end_value == Vector2(8, 9), "Redo must restore the edited value")
	if DisplayServer.get_name() == "headless":
		# The dummy renderer cannot generate the editor's save thumbnail.
		var saved := PackedScene.new()
		check(saved.pack(scene) == OK and ResourceSaver.save(saved, "res://blueprint_editor_fixture.tscn") == OK, "The edited scene must serialize")
	else:
		check(EditorInterface.save_scene() == OK, "The edited scene must save through the editor")
	var packed: PackedScene = ResourceLoader.load("res://blueprint_editor_fixture.tscn", "PackedScene", ResourceLoader.CACHE_MODE_IGNORE)
	var restored := packed.instantiate()
	var restored_effect: TweenBlueprint = restored.get_node("Blueprint")
	check(restored_effect.start_enabled and restored_effect.start_value == Vector2(1, 2), "Save/reload must preserve explicit start configuration")
	check(restored_effect.end_value == Vector2(8, 9), "Save/reload must preserve the endpoint")
	check(restored_effect.target == restored, "Save/reload must preserve relative node references")
	var restored_characters: TweenBlueprint = restored.get_node("CharacterText/Characters")
	check(restored_characters.end_value.is_equal_approx(Vector3(0, 0, TAU)) and restored_characters.character_pivot == Vector2(12, -14), "Editor save/reload must preserve character values and pivots")
	check(restored_characters.character_loops == 2 and restored_characters.character_loop_mode == Spire.LOOP_MODE_YOYO, "Editor save/reload must preserve character loop settings")
	restored.free()
	var original: Variant = effect.end_value
	effect.animated_property = &"rotation"
	check(effect.end_value == original, "Changing to an incompatible property must preserve its authored payload")
	effect.animated_property = &"position"
	effect.target = null
	check(effect.get_resolved_target() == target, "The editor must reflect ancestor fallback")
	check(property_info(effect, &"end_value").type == TYPE_VECTOR2, "Fallback targets must keep typed editors")
	for _frame in 5: await get_tree().process_frame
	if DisplayServer.get_name() != "headless":
		EditorInterface.get_base_control().get_viewport().get_texture().get_image().save_png("res://blueprint_inspector.png")
	print("BLUEPRINT_EDITOR_RESULT failed=", failures.size())
	get_tree().quit(0 if failures.is_empty() else 1)
