extends TestBase

const Effects := preload("res://examples/tests/text_effect_factory.gd")

const Waits := preload("res://examples/tests/test_waits.gd")

const ENTRY_KINDS := [
	Spire.TEXT_EFFECT_KIND_COLOR, Spire.TEXT_EFFECT_KIND_FADE, Spire.TEXT_EFFECT_KIND_OFFSET,
	Spire.TEXT_EFFECT_KIND_ROTATION, Spire.TEXT_EFFECT_KIND_SCALE, Spire.TEXT_EFFECT_KIND_PUNCH_OFFSET,
	Spire.TEXT_EFFECT_KIND_PUNCH_ROTATION, Spire.TEXT_EFFECT_KIND_PUNCH_SCALE,
	Spire.TEXT_EFFECT_KIND_SHAKE_OFFSET, Spire.TEXT_EFFECT_KIND_SHAKE_ROTATION, Spire.TEXT_EFFECT_KIND_SHAKE_SCALE,
]

func _preset(tag: String, entries: Array[SpireTextEffect]) -> SpireRichTextEffectPreset:
	var preset := SpireRichTextEffectPreset.new()
	preset.bbcode = tag
	preset.effects = entries
	return preset

func _label(content: String, presets: Array[SpireRichTextEffectPreset], bbcode: bool = true) -> SpireRichTextLabel:
	var label := SpireRichTextLabel.new()
	label.bbcode_enabled = bbcode
	label.text = content
	label.effect_presets = presets
	label.size = Vector2(500, 140)
	label.add_theme_color_override("default_color", Color.WHITE)
	label.add_theme_font_size_override("normal_font_size", 36)
	add_child(label)
	return label

func _draw_frames() -> void:
	for _frame in 4: await next_frame()

func test_text_presets_typed_array() -> bool:
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	color.from = Color.RED
	color.to = Color.RED
	var preset := _preset("glow", [color])
	var n := _label("[glow]A[/glow]", [null, preset, null])
	var presets: Array[SpireRichTextEffectPreset] = n.effect_presets
	assert_true(presets.is_typed())
	assert_eq(presets.get_typed_builtin(), TYPE_OBJECT)
	assert_eq(presets.get_typed_class_name(), &"SpireRichTextEffectPreset")
	await _draw_frames()
	assert_eq(n.get_char_color(0), Color.RED)
	presets.resize(4)
	assert_true(presets[0] == null and presets[2] == null and presets[3] == null)
	presets[1] = null
	await _draw_frames()
	assert_eq(n.get_parsed_text(), "[glow]A[/glow]")
	presets[3] = preset
	await _draw_frames()
	assert_eq(n.get_parsed_text(), "A")
	assert_eq(n.get_char_color(0), Color.RED)
	n.free()
	return true

func test_text_presets_registration() -> bool:
	var red := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	red.from = Color.RED
	red.to = Color.RED
	var green := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	green.from = Color.GREEN
	green.to = Color.GREEN
	var n := _label("[red]A[/red][green]B[/green][red]C[/red]", [_preset("red", [red]), _preset("green", [green])])
	await _draw_frames()
	assert_eq(n.text, "[red]A[/red][green]B[/green][red]C[/red]")
	assert_eq(n.get_parsed_text(), "ABC")
	assert_eq(n.get_character_count(), 3)
	assert_eq(n.custom_effects.size(), 2)
	assert_eq(n.get_char_color(0), Color.RED)
	assert_eq(n.get_char_color(1), Color.GREEN)
	assert_eq(n.get_char_color(2), Color.RED)
	n.free()
	return true

func test_text_presets_runtime_registration() -> bool:
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	color.from = Color.RED
	color.to = Color.RED
	var preset := _preset("glow", [color])
	var n := _label("[glow]A[/glow]", [preset])
	await _draw_frames()
	assert_eq(n.custom_effects.size(), 1)
	var adapter: Resource = n.custom_effects[0]
	for _refresh in 3:
		n.refresh()
		assert_eq(n.custom_effects, [adapter])
	await _draw_frames()
	assert_eq(n.custom_effects, [adapter])
	# Native text assignment can resolve tags before the addon's next refresh.
	n.text = "[glow]BC[/glow]"
	assert_eq(n.get_parsed_text(), "BC")
	await _draw_frames()
	assert_eq(n.custom_effects, [adapter])
	assert_eq(n.get_char_color(1), Color.RED)
	# Repair duplicate or missing managed registrations without replacing adapters.
	n.custom_effects.append(adapter)
	n.custom_effects.append(adapter)
	await _draw_frames()
	assert_eq(n.custom_effects, [adapter])
	n.custom_effects.clear()
	await _draw_frames()
	assert_eq(n.custom_effects, [adapter])
	var foreign := preload("res://examples/tests/text_effect_presets_foreign.gd").new()
	n.custom_effects.append(foreign)
	await _draw_frames()
	assert_eq(n.custom_effects.size(), 2)
	assert_eq(n.custom_effects[0], foreign)
	assert_eq(n.custom_effects[1].get_class(), "SpireRichTextEffect")
	n.effect_presets.clear()
	await _draw_frames()
	assert_eq(n.custom_effects, [foreign])
	assert_eq(n.get_parsed_text(), "[glow]BC[/glow]")
	n.text = "[foreign_glow]A[/foreign_glow]"
	await _draw_frames()
	assert_true(n.get_char_color(0).is_equal_approx(Color(0, 1, 0, 0.75)))
	n.free()
	return true

func test_text_presets_example_scene() -> bool:
	var packed: PackedScene = load("res://examples/text_effect_presets/preset_demo.tscn")
	var example := packed.instantiate()
	add_child(example)
	await _draw_frames()
	var label: SpireRichTextLabel = example.get_node("Text")
	assert_true(not label.get_parsed_text().contains("[enchanted]"))
	var blueprint: TweenBlueprint = label.get_node("SharedEffect")
	assert_eq(blueprint.text_effect, label.effect_presets[0].effects[0])
	blueprint.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	var spawned: Array[SpireSequence] = []
	blueprint.tween_spawned.connect(func(sequence: SpireSequence): spawned.append(sequence))
	example.get_node("Buttons/Blueprint").pressed.emit()
	assert_eq(spawned.size(), 1)
	if not spawned.is_empty():
		spawned[0].custom_step(0.2)
		assert_true(not label.get_char_offset(0).is_zero_approx(), "The shared resource must drive Blueprint playback")
		example.get_node("Buttons/Reset").pressed.emit()
		assert_true(spawned[0].is_stopped() and label.get_char_offset(0).is_zero_approx(), "Reset must stop the example Blueprint before clearing its effects")
	example.get_node("Buttons/Punch").pressed.emit()
	await get_tree().create_timer(0.7).timeout
	await _draw_frames()
	assert_true(label.get_char_offset(0).is_equal_approx(Vector2.ZERO))
	example.get_node("Buttons/Fade").pressed.emit()
	assert_true(await Waits.until(get_tree(), func(): return is_zero_approx(label.get_char_alpha(0))))
	assert_true(is_zero_approx(label.get_char_alpha(0)))
	example.get_node("Buttons/Reset").pressed.emit()
	await _draw_frames()
	assert_true(label.get_char_alpha(0) > 0.9)
	example.get_node("Buttons/Fade").pressed.emit()
	assert_true(await Waits.until(get_tree(), func(): return label.get_char_alpha(0) < 0.9))
	example.get_node("Buttons/Reset").pressed.emit()
	await get_tree().create_timer(0.7).timeout
	await _draw_frames()
	assert_true(label.get_char_alpha(0) > 0.9, "Reset must stop this example's active code tweens before clearing values")
	example.free()
	return true

func test_text_presets_resource_notifications() -> bool:
	var entry := Effects.make(Spire.TEXT_EFFECT_KIND_OFFSET)
	var observed: Array[float] = []
	var callback := func(): observed.append(entry.duration)
	entry.changed.connect(callback)
	entry.duration = 0.75
	entry.duration = 1.25
	assert_eq(observed, [0.75, 1.25])
	entry.changed.disconnect(callback)
	return true

func test_text_presets_foreign_effects() -> bool:
	var foreign := preload("res://examples/tests/text_effect_presets_foreign.gd").new()
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	color.from = Color.RED
	color.to = Color.RED
	var preset := _preset("foreign_glow", [color])
	var n := SpireRichTextLabel.new()
	n.custom_effects = [foreign]
	n.effect_presets = [preset]
	n.bbcode_enabled = true
	n.text = "[foreign_glow]A[/foreign_glow]"
	n.size = Vector2(500, 140)
	add_child(n)
	await _draw_frames()
	assert_eq(n.custom_effects, [foreign])
	assert_true(n.get_char_color(0).is_equal_approx(Color(0, 1, 0, 0.75)))
	foreign.bbcode = &"relocated"
	foreign.emit_changed()
	await _draw_frames()
	assert_eq(n.get_char_color(0), Color.RED)
	foreign.bbcode = null
	foreign.emit_changed()
	await _draw_frames()
	assert_eq(n.get_char_color(0), Color.RED)
	foreign.bbcode = "foreign_glow"
	foreign.emit_changed()
	preset.bbcode = "ours"
	n.text = "[foreign_glow]A[/foreign_glow][ours]B[/ours]"
	await _draw_frames()
	assert_eq(n.custom_effects.size(), 2)
	assert_eq(n.custom_effects[0], foreign)
	assert_eq(n.custom_effects[1].get_class(), "SpireRichTextEffect")
	assert_eq(n.get_character_count(), 2)
	assert_true(n.get_char_color(0).is_equal_approx(Color(0, 1, 0, 0.75)))
	assert_eq(n.get_char_color(1), Color.RED)
	n.free()
	return true

func test_text_presets_translation() -> bool:
	var locale := TranslationServer.get_locale()
	var translation := Translation.new()
	translation.set_locale("es")
	translation.add_message("SPIRE_PRESET_TRANSLATION", "[glow]猫AB[/glow]")
	TranslationServer.add_translation(translation)
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	color.from = Color.RED
	color.to = Color.RED
	var n := _label("SPIRE_PRESET_TRANSLATION", [_preset("glow", [color])])
	TranslationServer.set_locale("es")
	await _draw_frames()
	assert_eq(n.text, "SPIRE_PRESET_TRANSLATION")
	assert_eq(n.get_parsed_text(), "猫AB")
	assert_eq(n.get_character_count(), 3)
	assert_eq(n.get_char_color(0), Color.RED)
	TranslationServer.set_locale(locale)
	TranslationServer.remove_translation(translation)
	n.free()
	return true

func test_text_presets_code_last() -> bool:
	var red := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	red.from = Color.RED
	red.to = Color.RED
	var n := _label("[glow]A[/glow]", [_preset("glow", [red])])
	await _draw_frames()
	assert_eq(n.get_char_color(0), Color.RED)
	var tween := n.color_char(0, Color.BLUE, 1).set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)
	tween.custom_step(0.5)
	await _draw_frames()
	assert_true(n.get_char_color(0).is_equal_approx(Color(0.5, 0, 0.5, 1)))
	red.from = Color.GREEN
	red.to = Color.GREEN
	await _draw_frames()
	assert_true(n.get_char_color(0).is_equal_approx(Color(0.5, 0, 0.5, 1)), "Editing a preset must preserve code-controlled state")
	tween.force_complete()
	assert_eq(n.get_char_color(0), Color.BLUE)
	n.reset()
	await _draw_frames()
	assert_eq(n.get_char_color(0), Color.GREEN)
	n.free()
	return true

func test_text_presets_live_resource_changes() -> bool:
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	color.from = Color.RED
	color.to = Color.RED
	var preset := _preset("glow", [color])
	var a := _label("[glow]A[/glow]", [preset])
	var b := _label("[glow]B[/glow]", [preset])
	await _draw_frames()
	assert_eq(a.get_char_color(0), Color.RED)
	color.from = Color.BLUE
	color.to = Color.BLUE
	await _draw_frames()
	assert_eq(a.get_char_color(0), Color.BLUE)
	assert_eq(b.get_char_color(0), Color.BLUE)
	a.free()
	color.from = Color.GREEN
	color.to = Color.GREEN
	await _draw_frames()
	assert_eq(b.get_char_color(0), Color.GREEN)
	var fade := Effects.make(Spire.TEXT_EFFECT_KIND_FADE)
	fade.from = 0.25
	fade.to = 0.25
	preset.effects.append(fade)
	await _draw_frames()
	assert_true(b.get_char_color(0).is_equal_approx(Color(0, 1, 0, 0.25)))
	assert_eq(b.custom_effects.size(), 1)
	b.free()
	return true

func test_text_presets_label_lifetime() -> bool:
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	color.from = Color.RED
	color.to = Color.RED
	var preset := _preset("glow", [color])
	var foreign := preload("res://examples/tests/text_effect_presets_foreign.gd").new()
	var watched: Array[Resource] = [preset, color, foreign]
	var connections: Array[int] = []
	for resource in watched:
		connections.append(resource.get_signal_connection_list("changed").size())
	var a := _label("[glow]A[/glow]", [preset])
	var b := _label("[glow]B[/glow]", [preset])
	a.custom_effects = [foreign]
	b.custom_effects = [foreign]
	await _draw_frames()
	for index in watched.size():
		assert_eq(watched[index].get_signal_connection_list("changed").size(), connections[index] + 2)
	# A retained tween must not retain the deleted label's preset subscriptions
	# or affect another label sharing resources.
	var tween := a.offset_char(0, Vector2(8, 0), 1).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	tween.custom_step(0.25)
	a.free()
	for index in watched.size():
		assert_eq(watched[index].get_signal_connection_list("changed").size(), connections[index] + 1)
	color.from = Color.BLUE
	color.to = Color.BLUE
	await _draw_frames()
	assert_eq(b.get_char_color(0), Color.BLUE)
	assert_eq(b.get_char_offset(0), Vector2.ZERO)
	b.free()
	for index in watched.size():
		assert_eq(watched[index].get_signal_connection_list("changed").size(), connections[index])
		watched[index].emit_changed()
	assert_true(not tween.custom_step(1))
	tween.force_complete()
	assert_eq(tween.get_state(), Spire.STATE_STOPPED)
	return true

func test_text_presets_disconnected_watch_cleanup() -> bool:
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	var preset := _preset("glow", [color, color])
	var n := _label("[glow]A[/glow]", [preset])
	await _draw_frames()
	var connections := color.get_signal_connection_list("changed")
	assert_eq(connections.size(), 1)
	assert_eq(preset.get_signal_connection_list("changed").size(), 1)
	# External code can disconnect a watch before the registry cleans it up.
	color.changed.disconnect(connections[0].callable)
	n.effect_presets = []
	n.refresh()
	assert_eq(color.get_signal_connection_list("changed").size(), 0)
	assert_eq(preset.get_signal_connection_list("changed").size(), 0)
	color.emit_changed()
	preset.emit_changed()
	n.free()
	return true

func test_text_presets_nested_and_disabled() -> bool:
	var red := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	red.from = Color.RED
	red.to = Color.RED
	var blue := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	blue.from = Color.BLUE
	blue.to = Color.BLUE
	var n := _label("[red][blue]A[/blue][/red]", [_preset("red", [red]), _preset("blue", [blue])])
	await _draw_frames()
	assert_eq(n.get_char_color(0), Color.RED)
	n.bbcode_enabled = false
	await _draw_frames()
	assert_eq(n.get_parsed_text(), n.text)
	assert_eq(n.get_character_count(), n.text.length())
	n.bbcode_enabled = true
	await _draw_frames()
	assert_eq(n.get_parsed_text(), "A")
	n.free()
	return true

func test_text_presets_serialization() -> bool:
	var entries: Array[SpireTextEffect] = []
	for kind in ENTRY_KINDS:
		var entry: SpireTextEffect = Effects.make(kind)
		entry.set("duration", 1.75)
		entry.set("ease", Spire.EASE_OUT_CUBIC)
		entry.set("loop_mode", Spire.LOOP_MODE_YOYO)
		entry.set("character_phase_offset", -0.1)
		entries.append(entry)
	var preset := _preset("all_effects", entries)
	assert_eq(ResourceSaver.save(preset, "user://text_effect_preset.tres"), OK)
	var loaded := ResourceLoader.load("user://text_effect_preset.tres", "", ResourceLoader.CACHE_MODE_IGNORE) as SpireRichTextEffectPreset
	assert_true(loaded != null)
	assert_eq(loaded.effects.size(), ENTRY_KINDS.size())
	for index in ENTRY_KINDS.size():
		var entry: Resource = loaded.effects[index]
		assert_eq(entry.kind, ENTRY_KINDS[index])
		assert_eq(entry.get("duration"), 1.75)
		assert_eq(entry.get("ease"), Spire.EASE_OUT_CUBIC)
		assert_eq(entry.get("loop_mode"), Spire.LOOP_MODE_YOYO)
		assert_eq(entry.get("character_phase_offset"), -0.1)
	var n := _label("[all_effects]A[/all_effects]", [null, loaded, null])
	var packed := PackedScene.new()
	assert_eq(packed.pack(n), OK)
	var restored := packed.instantiate() as SpireRichTextLabel
	add_child(restored)
	await _draw_frames()
	assert_eq(restored.effect_presets.size(), 3)
	assert_eq(restored.effect_presets.get_typed_class_name(), &"SpireRichTextEffectPreset")
	assert_true(restored.effect_presets[0] == null and restored.effect_presets[2] == null)
	assert_true(restored.effect_presets[1] is SpireRichTextEffectPreset)
	assert_eq(restored.custom_effects.size(), 1)
	assert_true(restored.custom_effects[0] != n.custom_effects[0], "Instantiated labels must build their own adapters")
	assert_eq(restored.get_parsed_text(), "A")
	restored.free()
	n.free()
	return true

func test_text_presets_invalid_configuration() -> bool:
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	color.duration = 0.0
	var n := _label("[glow]A[/glow]", [_preset("glow", [null, color]), _preset("glow", []), _preset("color", []), _preset("bad tag", [])])
	await _draw_frames()
	assert_eq(n.custom_effects.size(), 1)
	assert_eq(n.get_char_color(0), Color.WHITE)
	color.duration = 1
	color.from = Color.RED
	color.to = Color.RED
	await _draw_frames()
	assert_eq(n.get_char_color(0), Color.RED)
	color.to = Color(NAN, 0, 0)
	await _draw_frames()
	assert_eq(n.get_char_color(0), Color.WHITE)
	n.free()
	return true

func test_text_presets_replacement_and_duplication() -> bool:
	var color := Effects.make(Spire.TEXT_EFFECT_KIND_COLOR)
	color.from = Color.RED
	color.to = Color.RED
	var preset := _preset("glow", [color])
	var n := _label("[glow]A[/glow]", [preset])
	await _draw_frames()
	var copy := n.duplicate() as SpireRichTextLabel
	add_child(copy)
	await _draw_frames()
	assert_eq(copy.custom_effects.size(), 1)
	assert_true(copy.custom_effects[0] != n.custom_effects[0], "Duplicated labels must build their own adapters")
	n.effect_presets = []
	await _draw_frames()
	assert_eq(n.custom_effects.size(), 0)
	assert_eq(copy.get_parsed_text(), "A")
	assert_eq(copy.get_char_color(0), Color.RED)
	preset.bbcode = "renamed"
	copy.text = "[renamed]B[/renamed]"
	await _draw_frames()
	assert_eq(copy.get_parsed_text(), "B")
	assert_eq(copy.custom_effects.size(), 1)
	copy.free()
	n.free()
	return true

func _effect_property(effect: SpireTextEffect, name: StringName) -> Dictionary:
	for info in effect.get_property_list():
		if info.name == name: return info
	return {}

func test_text_effect_metadata() -> bool:
	assert_true(ClassDB.class_get_enum_list(&"Spire").has("TextEffectKind"))
	assert_eq(ClassDB.class_get_enum_constants(&"Spire", &"TextEffectKind").size(), 11)
	var optional := ["from", "to", "punch", "strength", "vibrato", "elasticity", "randomness", "randomness_mode", "fade_out", "pivot", "rotate_mode"]
	var cases := [
		[Spire.TEXT_EFFECT_KIND_COLOR, TYPE_COLOR, ["from", "to"]],
		[Spire.TEXT_EFFECT_KIND_FADE, TYPE_FLOAT, ["from", "to"]],
		[Spire.TEXT_EFFECT_KIND_OFFSET, TYPE_VECTOR2, ["from", "to"]],
		[Spire.TEXT_EFFECT_KIND_ROTATION, TYPE_VECTOR3, ["from", "to", "pivot", "rotate_mode"]],
		[Spire.TEXT_EFFECT_KIND_SCALE, TYPE_VECTOR2, ["from", "to", "pivot"]],
		[Spire.TEXT_EFFECT_KIND_PUNCH_OFFSET, TYPE_NIL, ["punch", "vibrato", "elasticity"]],
		[Spire.TEXT_EFFECT_KIND_PUNCH_ROTATION, TYPE_NIL, ["punch", "vibrato", "elasticity", "pivot"]],
		[Spire.TEXT_EFFECT_KIND_PUNCH_SCALE, TYPE_NIL, ["punch", "vibrato", "elasticity", "pivot"]],
		[Spire.TEXT_EFFECT_KIND_SHAKE_OFFSET, TYPE_NIL, ["strength", "vibrato", "randomness", "randomness_mode", "fade_out"]],
		[Spire.TEXT_EFFECT_KIND_SHAKE_ROTATION, TYPE_NIL, ["strength", "vibrato", "randomness", "randomness_mode", "fade_out", "pivot"]],
		[Spire.TEXT_EFFECT_KIND_SHAKE_SCALE, TYPE_NIL, ["strength", "vibrato", "randomness", "randomness_mode", "fade_out", "pivot"]],
	]
	for item in cases:
		var entry := Effects.make(item[0])
		for name in optional:
			var info := _effect_property(entry, name)
			assert_true(not info.is_empty())
			assert_true(info.usage & PROPERTY_USAGE_STORAGE != 0, "Hidden fields must remain stored: " + name)
			assert_eq(info.usage & PROPERTY_USAGE_EDITOR != 0, name in item[2])
		if item[1] != TYPE_NIL:
			for name in ["from", "to"]:
				var info := _effect_property(entry, name)
				assert_eq(info.type, item[1])
				assert_true(info.usage & PROPERTY_USAGE_NIL_IS_VARIANT == 0)
		assert_true(_effect_property(entry, "kind").usage & PROPERTY_USAGE_UPDATE_ALL_IF_MODIFIED != 0)
		assert_eq(_effect_property(entry, "punch").type, TYPE_VECTOR3)
		assert_eq(_effect_property(entry, "strength").type, TYPE_VECTOR3)
	return true

func test_text_effect_preserves_hidden_settings() -> bool:
	var entry := Effects.make(Spire.TEXT_EFFECT_KIND_SHAKE_OFFSET)
	entry.from = Color.CORNFLOWER_BLUE
	entry.to = Color.MEDIUM_ORCHID
	entry.punch = Vector3(2, 3, 4)
	entry.pivot = Vector2(7, 8)
	entry.rotate_mode = Spire.ROTATE_MODE_FAST_BEYOND_360
	entry.elasticity = 0.25
	entry.kind = Spire.TEXT_EFFECT_KIND_COLOR
	assert_eq(entry.from, Color.CORNFLOWER_BLUE)
	entry.kind = Spire.TEXT_EFFECT_KIND_SHAKE_OFFSET
	var preset := _preset("stored", [entry, null])
	assert_true(preset.effects.is_typed())
	assert_eq(preset.effects.get_typed_class_name(), &"SpireTextEffect")
	assert_eq(ResourceSaver.save(preset, "user://hidden_effect_fields.tres"), OK)
	var restored: SpireRichTextEffectPreset = ResourceLoader.load("user://hidden_effect_fields.tres", "", ResourceLoader.CACHE_MODE_IGNORE)
	assert_eq(restored.effects.get_typed_class_name(), &"SpireTextEffect")
	assert_eq(restored.effects.size(), 2)
	assert_eq(restored.effects[1], null)
	for loaded: SpireTextEffect in [restored.effects[0], entry.duplicate(true)]:
		assert_eq(loaded.kind, Spire.TEXT_EFFECT_KIND_SHAKE_OFFSET)
		assert_true(loaded.from.is_equal_approx(Color.CORNFLOWER_BLUE))
		assert_true(loaded.to.is_equal_approx(Color.MEDIUM_ORCHID))
		assert_eq(loaded.punch, Vector3(2, 3, 4))
		assert_eq(loaded.pivot, Vector2(7, 8))
		assert_eq(loaded.rotate_mode, Spire.ROTATE_MODE_FAST_BEYOND_360)
		assert_eq(loaded.elasticity, 0.25)
	return true

func test_text_effect_setters_emit_changed() -> bool:
	var entry := SpireTextEffect.new()
	var changes := [0]
	var metadata_changes := [0]
	entry.changed.connect(func(): changes[0] += 1)
	entry.property_list_changed.connect(func(): metadata_changes[0] += 1)
	var values := {
		"kind": Spire.TEXT_EFFECT_KIND_ROTATION, "duration": 2.0,
		"ease": Spire.EASE_OUT_QUAD, "loop_mode": Spire.LOOP_MODE_YOYO,
		"character_phase_offset": 0.1, "from": Vector3.ZERO, "to": Vector3.ONE,
		"punch": Vector3.ONE, "strength": Vector3(2, 3, 4), "vibrato": 5,
		"elasticity": 0.5, "randomness": 45.0,
		"randomness_mode": Spire.SHAKE_RANDOMNESS_MODE_HARMONIC,
		"fade_out": false, "pivot": Vector2.ONE,
		"rotate_mode": Spire.ROTATE_MODE_FAST_BEYOND_360,
	}
	for name in values:
		entry.set(name, values[name])
		assert_eq(entry.get(name), values[name])
	assert_eq(changes[0], values.size())
	assert_eq(metadata_changes[0], 3)
	return true
