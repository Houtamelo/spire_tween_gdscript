@tool
extends EditorPlugin

var failures: Array[String] = []

func _enter_tree() -> void:
	_run.call_deferred()

func check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		push_error(message)

func frames() -> void:
	for _frame in 8: await get_tree().process_frame

func _run() -> void:
	await frames()
	EditorInterface.open_scene_from_path("res://text_effect_presets_editor_fixture.tscn")
	EditorInterface.set_main_screen_editor("2D")
	await frames()
	var scene := EditorInterface.get_edited_scene_root()
	var label: SpireRichTextLabel = scene.get_node("Text")
	var color := SpireTextColorEffect.new()
	color.from = Color.CORNFLOWER_BLUE
	color.to = Color.MEDIUM_ORCHID
	color.duration = 2
	color.loop_mode = Spire.LOOP_MODE_YOYO
	var offset := SpireTextOffsetEffect.new()
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
	for type in ["SpireTextColorEffect", "SpireTextFadeEffect", "SpireTextOffsetEffect", "SpireTextRotationEffect", "SpireTextScaleEffect", "SpireTextPunchOffsetEffect", "SpireTextPunchRotationEffect", "SpireTextPunchScaleEffect", "SpireTextShakeOffsetEffect", "SpireTextShakeRotationEffect", "SpireTextShakeScaleEffect"]:
		var entry: Resource = ClassDB.instantiate(type)
		EditorInterface.inspect_object(entry)
		await frames()
		var fields: Array[StringName] = []
		for property: EditorProperty in EditorInterface.get_inspector().find_children("*", "EditorProperty", true, false):
			fields.append(property.get_edited_property())
		for field in [&"duration", &"ease", &"loop_mode", &"character_phase_offset"]:
			check(field in fields, type + " must expose " + field + " through the real Inspector")
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
	check(restored_preset.effects[0].duration == 1.25 and restored_preset.effects[1].duration == 2.0, "Scene reload must retain independent timing")
	restored.free()
	EditorInterface.inspect_object(offset)
	await frames()
	EditorInterface.get_base_control().get_viewport().get_texture().get_image().save_png("res://text_effect_presets_inspector.png")
	print("TEXT_PRESET_EDITOR_RESULT failed=", failures.size())
	get_tree().quit(0 if failures.is_empty() else 1)
