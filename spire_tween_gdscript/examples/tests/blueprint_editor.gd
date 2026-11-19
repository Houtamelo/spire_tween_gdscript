@tool
extends EditorPlugin

const Effects := preload("res://examples/tests/text_effect_factory.gd")

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

func edited_property(name: StringName) -> EditorProperty:
	for property: EditorProperty in EditorInterface.get_inspector().find_children("*", "EditorProperty", true, false):
		if property.get_edited_property() == name: return property
	return null

func wait_for_edited_property(name: StringName, previous_id: int = 0) -> EditorProperty:
	var deadline := Time.get_ticks_usec() + 2_000_000
	while true:
		var property := edited_property(name)
		if property != null and property.get_instance_id() != previous_id: return property
		if Time.get_ticks_usec() >= deadline: return null
		await get_tree().process_frame
	return null

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

func wait_for_scene_warning(node: Node, field: String, present: bool) -> bool:
	# The Scene dock batches warning updates independently of process frames.
	var deadline := Time.get_ticks_usec() + 2_000_000
	while scene_warnings(node).contains(field) != present:
		if Time.get_ticks_usec() >= deadline: return false
		await get_tree().process_frame
	return true

func reveal_property(control: Control) -> void:
	# Expand collapsed Inspector sections through their actual mouse input path.
	for _attempt in 4:
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
		for _frame in 4: await get_tree().process_frame
	check(control.is_visible_in_tree(), "The curve editor must be visible in the Inspector")

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
	child.property_to_animate = &"position"
	container.add_child(child)
	check(child.get_resolved_target() == scene, "Inspector fallback must ignore a container's configured target")
	child.target = other
	check(child.get_resolved_target() == other, "The Inspector must honor the child's configured target")
	check(property_info(child, &"property_end_value").type == TYPE_VECTOR2, "Independent child targets must keep typed fields")
	container.free()
	other.free()
	check(property_info(effect, &"property_end_value").type == TYPE_VECTOR2, "The Inspector must expose the selected property's concrete type")
	var metadata := TweenBlueprint.new()
	metadata.kind = Spire.BLUEPRINT_KIND_PROPERTY
	metadata.target = target
	metadata.property_to_animate = &"process_mode"
	scene.add_child(metadata)
	var mode_info: Dictionary = ClassDB.class_get_property_list(&"Node").filter(func(info): return info.name == "process_mode")[0]
	check(property_info(metadata, &"property_end_value").class_name == mode_info.class_name, "Typed metadata must retain Godot's class_name field")
	metadata.property_to_animate = &" position:x "
	check(property_info(metadata, &"property_end_value").type == TYPE_FLOAT, "Indexed properties must expose the component's type")
	check(property_info(metadata, &"property_end_value").class_name == &"", "A scalar component must not carry the target node's class name")
	check(property_info(metadata, &"property_end_value").hint == PROPERTY_HINT_NONE, "Indexed properties must clear the root property's hints")
	metadata.target = custom
	check(property_info(metadata, &"property_to_animate").hint_string.is_empty(), "Unsupported Blueprint targets must not offer property suggestions")
	var validation_target := PROPERTY_TARGET.new()
	scene.add_child(validation_target)
	metadata.target = validation_target
	var suggestions: PackedStringArray = property_info(metadata, &"property_to_animate").hint_string.split(",")
	check("dynamic_scalar" in suggestions and "seed" in suggestions, "Suggestions must include writable dynamic properties and script variables with excluded built-in names")
	for name in ["readonly_scalar", "unsupported_toggle", "editor_description", "discrete_value", "file_value", "shape"]:
		check(name not in suggestions, "Suggestions must exclude read-only, unsupported and semantically unsuitable properties: " + name)
	for path in [&"dynamic_scalar", &"shape:distance"]:
		metadata.property_to_animate = path
		check(property_info(metadata, &"property_end_value").type == TYPE_FLOAT, "Valid dynamic and indexed Resource properties must have typed endpoint fields")
	metadata.free()
	validation_target.free()
	check(property_info(effect, &"property_start_value").usage & PROPERTY_USAGE_EDITOR == 0, "Disabled explicit start must be hidden")
	effect.property_start_enabled = true
	check(property_info(effect, &"property_start_value").type == TYPE_VECTOR2, "Explicit start must use the property type")
	check(property_info(effect, &"property_start_value").usage & PROPERTY_USAGE_EDITOR != 0, "Enabled explicit start must be visible")
	var custom_kind := custom.kind
	custom.kind = Spire.BLUEPRINT_KIND_PROPERTY
	for name in [&"property_to_animate", &"property_speed_or_duration", &"property_start_enabled", &"property_start_value", &"property_end_value", &"property_lerp_mode", &"property_ease_source", &"property_ease_enum", &"property_ease_curve"]:
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
	characters.kind = Spire.BLUEPRINT_KIND_TEXT
	text.add_child(characters)
	characters.owner = scene
	for _frame in 3: await get_tree().process_frame
	check(characters.target == null and characters.get_resolved_target() == text, "Characters must support ancestor target fallback")
	check(scene_warnings(characters).contains("text_effect"), "An unconfigured character step must warn about its missing resource")
	var rotation := Effects.make(Spire.TEXT_EFFECT_KIND_ROTATION)
	rotation.to = Vector3(0, 0, TAU)
	rotation.pivot = Vector2(12, -14)
	rotation.rotate_mode = Spire.ROTATE_MODE_FAST_BEYOND_360
	rotation.loop_mode = Spire.LOOP_MODE_YOYO
	EditorInterface.edit_node(characters)
	for _frame in 3: await get_tree().process_frame
	var assigned := false
	for property: EditorProperty in EditorInterface.get_inspector().find_children("*", "EditorProperty", true, false):
		if property.get_edited_property() == &"text_effect":
			property.emit_changed(&"text_effect", rotation)
			assigned = true
			break
	for _frame in 8: await get_tree().process_frame
	check(assigned and characters.text_effect == rotation, "The actual Inspector must assign an existing effect resource")
	var resource_info := property_info(characters, &"text_effect")
	check(resource_info.type == TYPE_OBJECT and resource_info.hint == PROPERTY_HINT_RESOURCE_TYPE, "Character effects must use the Inspector resource picker")
	check(resource_info.hint_string == "SpireTextEffect", "The resource picker must offer the unified effect type")
	check(await wait_for_scene_warning(characters, "text_effect", false), "Assigning a valid resource must clear the warning")
	characters.text_loops = 2
	characters.text_start_index = 99
	var invalid_delay := Curve.new()
	invalid_delay.min_value = -1.0
	invalid_delay.add_point(Vector2(0, -0.1))
	invalid_delay.add_point(Vector2(1, -0.1))
	characters.text_delay_ease_source = Spire.EASE_CURVE
	characters.text_delay_ease_curve = invalid_delay
	characters.text_loops = 0
	characters.update_configuration_warnings()
	var range_error_visible: bool = await wait_for_scene_warning(characters, "text_start_index", true)
	check(range_error_visible and not scene_warnings(characters).contains("text_delay_ease_curve") and not scene_warnings(characters).contains("text_loops"), "Character validation must report its first error")
	characters.text_start_index = 0
	characters.update_configuration_warnings()
	var stagger_error_visible: bool = await wait_for_scene_warning(characters, "text_delay_ease_curve", true)
	check(stagger_error_visible and not scene_warnings(characters).contains("text_loops"), "Fixing the range must expose the next character error")
	characters.text_delay_ease_source = Spire.EASE_ENUM
	characters.update_configuration_warnings()
	check(await wait_for_scene_warning(characters, "text_loops", true), "Fixing stagger must expose the invalid loop count")
	check(characters.text_delay_ease_curve == invalid_delay and not scene_warnings(characters).contains("text_delay_ease_curve"), "Inactive invalid curves must be preserved without rejecting Enum mode")
	characters.text_delay_ease_curve = null
	characters.text_loops = 2
	characters.update_configuration_warnings()
	check(await wait_for_scene_warning(characters, "text_loops", false), "A valid character configuration must clear the error")
	text.text = "ABCD"
	characters.text_delay_ease_enum = Spire.EASE_PUNCH
	check(await wait_for_scene_warning(characters, "text_delay_ease_enum", true), "Invalid built-in delay samples must identify the exported Ease field")
	characters.text_delay_ease_enum = Spire.EASE_LINEAR
	check(await wait_for_scene_warning(characters, "text_delay_ease_enum", false), "Repairing the built-in delay Ease must clear the warning")
	text.text = "ABC"
	for name in [&"text_effect", &"text_start_index", &"text_end_index", &"text_delay_ease_source", &"text_delay_ease_enum", &"text_loops", &"property_start_enabled", &"property_ease_source", &"property_ease_enum"]:
		check(property_info(characters, name).usage & PROPERTY_USAGE_EDITOR != 0, "Relevant character fields must be visible: " + name)
		check(property_info(characters, name).usage & PROPERTY_USAGE_STORAGE != 0, "Character fields must remain serializable: " + name)
	for name in [&"property_to_animate", &"property_lerp_mode", &"property_end_value", &"property_start_value", &"property_speed_or_duration", &"text_delay_ease_curve", &"property_ease_curve", &"text_blendable"]:
		check(property_info(characters, name).usage & PROPERTY_USAGE_EDITOR == 0, "Irrelevant character fields must be hidden: " + name)
		check(property_info(characters, name).usage & PROPERTY_USAGE_STORAGE != 0, "Hidden settings must retain serialization: " + name)
	for name in [&"character_stagger", &"character_animation", &"character_pivot", &"character_rotate_mode", &"character_loop_mode", &"character_vibrato", &"character_elasticity", &"character_randomness", &"character_fade_out", &"character_randomness_mode"]:
		check(property_info(characters, name).is_empty(), "Duplicated character settings must be removed: " + name)
	EditorInterface.edit_node(characters)
	for _frame in 4: await get_tree().process_frame
	var mode := edited_property(&"text_delay_ease_source")
	check(mode != null and characters.text_delay_ease_source == Spire.EASE_ENUM, "Character delays must expose a native EaseSource property")
	if mode != null: await reveal_property(mode)
	if DisplayServer.get_name() != "headless":
		# The 4.4 editor can stop drawing while idle; force a fresh capture.
		RenderingServer.force_draw()
		EditorInterface.get_base_control().get_viewport().get_texture().get_image().save_png("res://character_curve_basic.png")
	if mode != null:
		mode.emit_changed(&"text_delay_ease_source", Spire.EASE_CURVE)
		check(await wait_for_edited_property(&"text_delay_ease_curve") != null, "Curve mode must rebuild the native Inspector with a resource editor")
		check(characters.text_delay_ease_source == Spire.EASE_CURVE, "The native Inspector must change the selected delay source")
		check(property_info(characters, &"text_delay_ease_curve").usage & PROPERTY_USAGE_EDITOR != 0, "Curve mode must expose the native resource picker")
		check(property_info(characters, &"text_delay_ease_enum").usage & PROPERTY_USAGE_EDITOR == 0, "Curve mode must hide the inactive Ease")
		var curve_undo := EditorInterface.get_editor_undo_redo()
		var curve_history := curve_undo.get_history_undo_redo(curve_undo.get_object_history_id(characters))
		curve_history.undo()
		check(await wait_for_edited_property(&"text_delay_ease_enum") != null, "Undo must restore the Ease control")
		check(characters.text_delay_ease_source == Spire.EASE_ENUM, "Undo must restore the original delay source")
		curve_history.redo()
		check(await wait_for_edited_property(&"text_delay_ease_curve") != null, "Redo must restore the Curve editor")
		check(characters.text_delay_ease_source == Spire.EASE_CURVE, "Redo must restore Curve mode")
		mode = edited_property(&"text_delay_ease_source")
		mode.emit_changed(&"text_delay_ease_source", Spire.EASE_ENUM)
		await wait_for_edited_property(&"text_delay_ease_enum")
		var basic := edited_property(&"text_delay_ease_enum")
		check(basic != null, "Basic mode must expose an Ease dropdown")
		if basic != null:
			basic.emit_changed(&"text_delay_ease_enum", Spire.EASE_OUT_QUAD)
			for _frame in 4: await get_tree().process_frame
			check(characters.text_delay_ease_enum == Spire.EASE_OUT_QUAD, "The Ease dropdown must update the stored Ease")
		mode = edited_property(&"text_delay_ease_source")
		mode.emit_changed(&"text_delay_ease_source", Spire.EASE_CURVE)
		await wait_for_edited_property(&"text_delay_ease_curve")
	var start_curve := Curve.new()
	start_curve.min_value = -1
	start_curve.add_point(Vector2.ZERO, 0, 1)
	start_curve.add_point(Vector2.ONE, 1, 0)
	var curve_assigned := false
	var delay_property := edited_property(&"text_delay_ease_curve")
	var delay_property_id := delay_property.get_instance_id() if delay_property != null else 0
	if delay_property != null:
		for picker: EditorResourcePicker in delay_property.find_children("*", "EditorResourcePicker", true, false):
			picker.set_edited_resource(start_curve)
			picker.resource_changed.emit(start_curve)
			curve_assigned = true
			break
	check(curve_assigned and characters.text_delay_ease_curve == start_curve, "The actual Inspector must assign a typed start-delay Curve")
	check(characters.text_delay_ease_enum == Spire.EASE_OUT_QUAD, "Switching sources must preserve the inactive Ease")
	# The setter schedules a full Inspector rebuild. Check the replacement
	# resource picker, rather than the control that emitted the edit.
	var refreshed := await wait_for_edited_property(&"text_delay_ease_curve", delay_property_id)
	check(refreshed != null, "Curve assignment must rebuild the Inspector")
	var picker_updated := false
	if refreshed != null:
		for picker: EditorResourcePicker in refreshed.find_children("*", "EditorResourcePicker", true, false):
			picker_updated = picker.get_edited_resource() == start_curve
			break
	check(picker_updated, "The rebuilt Inspector must display the assigned Curve")
	if DisplayServer.get_name() != "headless":
		await reveal_property(refreshed)
		await get_tree().process_frame
		RenderingServer.force_draw()
		EditorInterface.get_base_control().get_viewport().get_texture().get_image().save_png("res://character_curve_custom.png")
	start_curve.set_point_value(1, -0.2)
	check(await wait_for_scene_warning(characters, "text_delay_ease_curve", true), "Curve edits must refresh configuration warnings through Resource.changed")
	start_curve.set_point_value(1, 1.0)
	check(await wait_for_scene_warning(characters, "text_delay_ease_curve", false), "Repairing the curve must clear its configuration warning")
	var impulse := Effects.make(Spire.TEXT_EFFECT_KIND_PUNCH_SCALE)
	characters.text_effect = impulse
	characters.property_start_enabled = true
	for _frame in 3: await get_tree().process_frame
	check(characters.property_start_value == Vector2.ONE, "Impulse starts must initialize to the neutral channel value")
	check(property_info(characters, &"property_start_value").type == TYPE_VECTOR2 and property_info(characters, &"property_start_value").usage & PROPERTY_USAGE_EDITOR != 0, "Impulse starts must use the typed editor")
	check(property_info(characters, &"text_blendable").usage & PROPERTY_USAGE_EDITOR != 0, "Impulse resources must expose blending")
	characters.text_effect = rotation
	EditorInterface.inspect_object(rotation)
	for _frame in 3: await get_tree().process_frame
	var edited := false
	for property: EditorProperty in EditorInterface.get_inspector().find_children("*", "EditorProperty", true, false):
		if property.get_edited_property() == &"duration":
			property.emit_changed(&"duration", 2.0)
			edited = true
			break
	for _frame in 8: await get_tree().process_frame
	check(edited and rotation.duration == 2.0, "The Inspector must edit the shared resource's timing")
	var resource_undo := EditorInterface.get_editor_undo_redo()
	var resource_history := resource_undo.get_history_undo_redo(resource_undo.get_object_history_id(rotation))
	resource_history.undo()
	check(rotation.duration == 1.0, "Undo must restore resource timing")
	resource_history.redo()
	check(rotation.duration == 2.0, "Redo must restore edited resource timing")
	rotation.duration = 0.0
	for _frame in 8: await get_tree().process_frame
	check(await wait_for_scene_warning(characters, "duration", true), "Nested resource edits must update the Scene dock warnings")
	rotation.duration = 2.0
	for _frame in 8: await get_tree().process_frame
	check(await wait_for_scene_warning(characters, "duration", false), "Repairing the resource must clear its warnings")
	EditorInterface.edit_node(characters)
	for _frame in 3: await get_tree().process_frame
	check(text.get_child_count() == 1 and text.text == "ABC", "Editor inspection must not add helper nodes or mutate text")
	for kind in [Spire.BLUEPRINT_KIND_CHAIN_BLOCK, Spire.BLUEPRINT_KIND_PARALLEL_BLOCK]:
		effect.kind = kind
		check(property_info(effect, &"property_end_value").usage & PROPERTY_USAGE_EDITOR == 0, "Container fields must be hidden")
		check(property_info(effect, &"property_end_value").usage & PROPERTY_USAGE_STORAGE != 0, "Hidden fields must remain serializable")
	effect.kind = Spire.BLUEPRINT_KIND_PROPERTY
	var undo := EditorInterface.get_editor_undo_redo()
	undo.create_action("Blueprint endpoint test")
	undo.add_do_property(effect, &"property_end_value", Vector2(8, 9))
	undo.add_undo_property(effect, &"property_end_value", effect.property_end_value)
	undo.commit_action()
	check(effect.property_end_value == Vector2(8, 9), "The editor must apply a property edit")
	var history := undo.get_history_undo_redo(undo.get_object_history_id(effect))
	history.undo()
	check(effect.property_end_value == Vector2(6, 7), "Undo must restore the authored value")
	history.redo()
	check(effect.property_end_value == Vector2(8, 9), "Redo must restore the edited value")
	if DisplayServer.get_name() == "headless":
		# The dummy renderer cannot generate the editor's save thumbnail.
		var saved := PackedScene.new()
		check(saved.pack(scene) == OK and ResourceSaver.save(saved, "res://blueprint_editor_fixture.tscn") == OK, "The edited scene must serialize")
	else:
		check(EditorInterface.save_scene() == OK, "The edited scene must save through the editor")
	var packed: PackedScene = ResourceLoader.load("res://blueprint_editor_fixture.tscn", "PackedScene", ResourceLoader.CACHE_MODE_IGNORE)
	var restored := packed.instantiate()
	var restored_effect: TweenBlueprint = restored.get_node("Blueprint")
	check(restored_effect.property_start_enabled and restored_effect.property_start_value == Vector2(1, 2), "Save/reload must preserve explicit start configuration")
	check(restored_effect.property_end_value == Vector2(8, 9), "Save/reload must preserve the endpoint")
	check(restored_effect.target == restored, "Save/reload must preserve relative node references")
	var restored_characters: TweenBlueprint = restored.get_node("CharacterText/Characters")
	check(restored_characters.text_effect is SpireTextEffect and restored_characters.text_effect.kind == Spire.TEXT_EFFECT_KIND_ROTATION and restored_characters.text_effect.to.is_equal_approx(Vector3(0, 0, TAU)) and restored_characters.text_effect.pivot == Vector2(12, -14), "Editor save/reload must preserve the effect resource and its values")
	check(restored_characters.text_loops == 2 and restored_characters.text_effect.loop_mode == Spire.LOOP_MODE_YOYO and restored_characters.text_effect.duration == 2.0, "Editor save/reload must preserve resource timing and loop settings")
	check(restored_characters.text_delay_ease_source == Spire.EASE_CURVE and restored_characters.text_delay_ease_enum == Spire.EASE_OUT_QUAD and restored_characters.text_delay_ease_curve is Curve and is_equal_approx(restored_characters.text_delay_ease_curve.sample(1), 1), "Editor save/reload must preserve the start-delay Curve")
	restored.free()
	var original: Variant = effect.property_end_value
	effect.property_to_animate = &"rotation"
	check(effect.property_end_value == original, "Changing to an incompatible property must preserve its authored payload")
	effect.property_to_animate = &"position"
	effect.target = null
	check(effect.get_resolved_target() == target, "The editor must reflect ancestor fallback")
	check(property_info(effect, &"property_end_value").type == TYPE_VECTOR2, "Fallback targets must keep typed editors")
	for _frame in 5: await get_tree().process_frame
	if DisplayServer.get_name() != "headless":
		EditorInterface.get_base_control().get_viewport().get_texture().get_image().save_png("res://blueprint_inspector.png")
	print("BLUEPRINT_EDITOR_RESULT failed=", failures.size())
	get_tree().quit(0 if failures.is_empty() else 1)
