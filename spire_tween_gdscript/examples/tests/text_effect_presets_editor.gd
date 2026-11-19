@tool
extends EditorPlugin

const Effects := preload("res://examples/tests/text_effect_factory.gd")

var failures: Array[String] = []

func _enter_tree() -> void:
	_run.call_deferred()

func check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		push_error(message)

func frames() -> void:
	for _frame in 8: await get_tree().process_frame

func edited_property(name: StringName) -> EditorProperty:
	for property: EditorProperty in EditorInterface.get_inspector().find_children("*", "EditorProperty", true, false):
		if property.get_edited_property() == name: return property
	return null

func effect_property(effect: SpireTextEffect, name: StringName) -> Dictionary:
	for info in effect.get_property_list():
		if info.name == name: return info
	return {}

func _run() -> void:
	await frames()
	EditorInterface.open_scene_from_path("res://text_effect_presets_editor_fixture.tscn")
	EditorInterface.set_main_screen_editor("2D")
	await frames()
	var scene := EditorInterface.get_edited_scene_root()
	var label: SpireRichTextLabel = scene.get_node("Text")
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	color.from = Color.CORNFLOWER_BLUE
	color.to = Color.MEDIUM_ORCHID
	color.duration = 2
	color.loop_mode = Spire.LOOP_MODE_YOYO
	var offset := Effects.make(Spire.TEXT_EFFECT_KIND_OFFSET)
	offset.from = Vector2(0, -10)
	offset.to = Vector2(0, 10)
	offset.duration = 0.6
	offset.ease = Spire.EASE_IN_OUT_SINE
	offset.loop_mode = Spire.LOOP_MODE_YOYO
	offset.character_phase_offset = 0.06
	var preset := SpireRichTextEffectPreset.new()
	preset.bbcode = "enchanted"
	preset.effects = [offset, color]
	label.effect_presets = [preset]
	await frames()
	check(label.get_parsed_text() == "Characters in motion", "Preset tags must render in the editor")
	check(label.custom_effects.is_empty(), "Editor preview must retain the authored custom_effects list")
	var sample: Dictionary = label.call("render_sample_for_test", 0)
	check(not sample.is_empty(), "Editor preview must use real glyph rendering")
	await frames()
	var next_sample: Dictionary = label.call("render_sample_for_test", 0)
	check(sample != next_sample, "Editor preview must animate")
	EditorInterface.inspect_object(label)
	await frames()
	var array_edited := false
	for property: EditorProperty in EditorInterface.get_inspector().find_children("*", "EditorProperty", true, false):
		if property.get_edited_property() == &"effect_presets":
			var presets: Array[SpireRichTextEffectPreset] = [preset, null]
			property.emit_changed(&"effect_presets", presets)
			array_edited = true
			break
	await frames()
	check(array_edited and label.effect_presets.size() == 2, "The Inspector must edit the typed preset array")
	check(label.effect_presets.get_typed_class_name() == &"SpireRichTextEffectPreset", "The Inspector must preserve the preset element type")
	check(label.effect_presets[1] == null, "The Inspector must allow empty preset slots")
	EditorInterface.get_base_control().get_viewport().get_texture().get_image().save_png("res://text_effect_presets_array_inspector.png")
	EditorInterface.inspect_object(preset)
	await frames()
	var entries_property := edited_property(&"effects")
	check(entries_property != null, "The Inspector must expose a typed effects array")
	if entries_property != null:
		var entries: Array[SpireTextEffect] = [offset, color, null]
		entries_property.emit_changed(&"effects", entries)
		await frames()
		check(preset.effects.size() == 3 and preset.effects[2] == null, "The effects array editor must preserve empty slots")
		check(preset.effects.get_typed_class_name() == &"SpireTextEffect", "Effect entries must retain their native element type")
	var schema := SpireTextEffect.new()
	schema.from = Color.RED
	schema.to = Color.BLUE
	schema.punch = Vector3(5, 6, 7)
	EditorInterface.inspect_object(schema)
	await frames()
	check(effect_property(schema, &"from").type == TYPE_COLOR, "Color endpoints must use color metadata")
	var kind_property := edited_property(&"kind")
	check(kind_property != null, "The resource must expose a native kind dropdown")
	if kind_property != null:
		kind_property.emit_changed(&"kind", Spire.TEXT_EFFECT_KIND_OFFSET)
		await frames()
		check(schema.kind == Spire.TEXT_EFFECT_KIND_OFFSET and schema.from == Color.RED and schema.to == Color.BLUE, "Kind edits must preserve incompatible endpoints")
		check(effect_property(schema, &"from").type == TYPE_NIL, "An incompatible payload must retain its Variant editor")
		check(edited_property(&"from") != null and edited_property(&"punch") == null, "Kind changes must rebuild actual Inspector controls")
		var schema_undo := EditorInterface.get_editor_undo_redo()
		var schema_history := schema_undo.get_history_undo_redo(schema_undo.get_object_history_id(schema))
		schema_history.undo()
		await frames()
		check(schema.kind == Spire.TEXT_EFFECT_KIND_COLOR and schema.from == Color.RED and effect_property(schema, &"from").type == TYPE_COLOR, "Undo must restore the kind and endpoint metadata")
		schema_history.redo()
		await frames()
		check(schema.kind == Spire.TEXT_EFFECT_KIND_OFFSET and schema.from == Color.RED, "Redo must restore the kind without replacing values")
		var from_property := edited_property(&"from")
		var to_property := edited_property(&"to")
		check(from_property != null and to_property != null, "Incompatible endpoints must remain editable")
		if from_property != null: from_property.emit_changed(&"from", schema.property_get_revert(&"from"))
		if to_property != null: to_property.emit_changed(&"to", Vector2(5, 6))
		await frames()
		check(schema.from == Vector2.ZERO and schema.to == Vector2(5, 6), "Inspector edits must repair endpoint types")
		check(effect_property(schema, &"from").type == TYPE_VECTOR2 and effect_property(schema, &"to").type == TYPE_VECTOR2, "Repair must restore typed endpoint editors")
	check(schema.punch == Vector3(5, 6, 7), "Hiding a field must preserve its value")
	check(ResourceSaver.save(schema, "res://unified_effect.tres") == OK, "The unified resource must save")
	var reloaded: SpireTextEffect = ResourceLoader.load("res://unified_effect.tres", "", ResourceLoader.CACHE_MODE_IGNORE)
	check(reloaded.kind == Spire.TEXT_EFFECT_KIND_OFFSET and reloaded.from == Vector2.ZERO and reloaded.to == Vector2(5, 6) and reloaded.punch == Vector3(5, 6, 7), "Resource reload must preserve active and hidden configuration")
	for kind in [Spire.TEXT_EFFECT_KIND_COLOR, Spire.TEXT_EFFECT_KIND_FADE, Spire.TEXT_EFFECT_KIND_OFFSET, Spire.TEXT_EFFECT_KIND_ROTATION, Spire.TEXT_EFFECT_KIND_SCALE, Spire.TEXT_EFFECT_KIND_PUNCH_OFFSET, Spire.TEXT_EFFECT_KIND_PUNCH_ROTATION, Spire.TEXT_EFFECT_KIND_PUNCH_SCALE, Spire.TEXT_EFFECT_KIND_SHAKE_OFFSET, Spire.TEXT_EFFECT_KIND_SHAKE_ROTATION, Spire.TEXT_EFFECT_KIND_SHAKE_SCALE]:
		var entry := Effects.make(kind)
		EditorInterface.inspect_object(entry)
		await frames()
		var fields: Array[StringName] = []
		for property: EditorProperty in EditorInterface.get_inspector().find_children("*", "EditorProperty", true, false):
			fields.append(property.get_edited_property())
		for field in [&"kind", &"duration", &"ease", &"loop_mode", &"character_phase_offset"]:
			check(field in fields, str(kind) + " must expose " + field + " through the real Inspector")
	EditorInterface.inspect_object(offset)
	await frames()
	var edited := false
	for property: EditorProperty in EditorInterface.get_inspector().find_children("*", "EditorProperty", true, false):
		if property.get_edited_property() == &"duration":
			property.emit_changed(&"duration", 1.25)
			edited = true
			break
	await frames()
	check(edited and offset.duration == 1.25, "An Inspector property edit must update the effect resource")
	var undo := EditorInterface.get_editor_undo_redo()
	var history := undo.get_history_undo_redo(undo.get_object_history_id(offset))
	history.undo()
	await frames()
	check(is_equal_approx(offset.duration, 0.6), "Undo must restore the Resource field")
	history.redo()
	await frames()
	check(offset.duration == 1.25, "Redo must restore the edited Resource field")
	check(ResourceSaver.save(preset, "res://looping_text_preset.tres") == OK, "The preset must save as an external Resource")
	EditorInterface.mark_scene_as_unsaved()
	check(EditorInterface.save_scene() == OK, "The editor must save the authored scene")
	await frames()
	var saved := FileAccess.get_file_as_string("res://text_effect_presets_editor_fixture.tscn")
	check(not saved.contains('type="SpireRichTextEffect"') and not saved.contains('type="GDScript"'), "Saved scenes must exclude transient adapters and embedded scripts")
	var packed := ResourceLoader.load("res://text_effect_presets_editor_fixture.tscn", "PackedScene", ResourceLoader.CACHE_MODE_IGNORE) as PackedScene
	var restored := packed.instantiate()
	var restored_label: SpireRichTextLabel = restored.get_node("Text")
	check(restored_label.effect_presets.size() == 2, "Scene reload must preserve the assigned preset and empty slot")
	check(restored_label.effect_presets.get_typed_class_name() == &"SpireRichTextEffectPreset", "Scene reload must preserve the preset element type")
	check(restored_label.effect_presets[1] == null, "Scene reload must preserve empty preset slots")
	var restored_preset: SpireRichTextEffectPreset = restored_label.effect_presets[0]
	check(restored_preset.effects.get_typed_class_name() == &"SpireTextEffect" and restored_preset.effects.size() == 3 and restored_preset.effects[2] == null, "Scene reload must preserve the typed effect array and empty entry")
	check(restored_preset.effects[0].duration == 1.25 and restored_preset.effects[1].duration == 2.0, "Scene reload must retain independent timing")
	restored.free()
	EditorInterface.inspect_object(offset)
	await frames()
	EditorInterface.get_base_control().get_viewport().get_texture().get_image().save_png("res://text_effect_presets_inspector.png")
	print("TEXT_PRESET_EDITOR_RESULT failed=", failures.size())
	get_tree().quit(0 if failures.is_empty() else 1)
