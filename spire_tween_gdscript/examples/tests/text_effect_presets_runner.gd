extends SceneTree

const TEST_SCRIPTS = [preload("res://examples/tests/text_effect_presets_render.gd"), preload("res://examples/tests/text_animator.gd")]
var completed := false

func _initialize() -> void:
	call_deferred("run_type_rejections" if "--typed-array-rejections" in OS.get_cmdline_user_args() else "run_tests")

func run_type_rejections() -> void:
	var label := SpireRichTextLabel.new()
	var preset := SpireRichTextEffectPreset.new()
	label.effect_presets = [preset]
	# Erasing the static type exercises Godot's runtime element checks.
	var presets: Array = label.effect_presets
	print("TEXT_PRESET_TYPE_REJECTION_BEGIN")
	presets.append(Resource.new())
	var insert_result := presets.insert(0, Resource.new())
	print("TEXT_PRESET_TYPE_REJECTION_END")
	var effect := SpireTextEffect.new()
	preset.effects = [effect]
	var entries: Array = preset.effects
	print("TEXT_PRESET_TYPE_REJECTION_BEGIN")
	entries.append(Resource.new())
	var effect_insert := entries.insert(0, Resource.new())
	print("TEXT_PRESET_TYPE_REJECTION_END")
	var valid: bool = effect_insert != OK and entries.size() == 1 and entries[0] == effect and insert_result != OK and presets.size() == 1 and presets[0] == preset and label.effect_presets[0] == preset
	label.free()
	print("TEXT_PRESET_TYPE_REJECTION_RESULT failed=", 0 if valid else 1)
	quit(0 if valid else 1)

func run_tests() -> void:
	create_timer(60.0).timeout.connect(func():
		if not completed:
			push_error("Text preset tests timed out")
			quit(1))
	var failed := 0
	var total := 0
	for script in TEST_SCRIPTS:
		var probe: Node = script.new()
		var names: Array[String] = []
		for method in probe.get_method_list():
			if method.name.begins_with("test_"): names.append(method.name)
		probe.free()
		names.sort()
		for method in names:
			var test: Node = script.new()
			root.add_child(test)
			print("TEXT_PRESET_START ", method)
			var result: bool = await test.run_test(Callable(test, method))
			if not result: failed += 1
			total += 1
			print("TEXT_PRESET_PASS " if result else "TEXT_PRESET_FAIL ", method)
			test.queue_free()
			for _frame in 4: await process_frame
	completed = true
	print("TEXT_PRESET_RESULT total=", total, " failed=", failed)
	quit(0 if failed == 0 and total > 0 else 1)
