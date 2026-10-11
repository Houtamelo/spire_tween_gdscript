extends Node

const TEST_SCENE := preload("res://examples/tests/node_binding.tscn")


func _ready() -> void:
	var probe := TEST_SCENE.instantiate()
	var names: Array[String] = []
	for method in probe.get_method_list():
		if method.name.begins_with("test_"):
			names.append(method.name)
	probe.free()
	var failed := 0 if not names.is_empty() else 1
	for method_name in names:
		var test := TEST_SCENE.instantiate()
		add_child(test)
		print("BINDING_START ", method_name)
		var result: Variant = await test.call(method_name)
		if result is bool and result:
			print("BINDING_PASS ", method_name)
		else:
			failed += 1
			print("BINDING_FAIL ", method_name)
		test.queue_free()
		await get_tree().process_frame
	# Let deferred removals and final fixture destruction settle before shutdown.
	await get_tree().create_timer(0.05).timeout
	for _i in 4:
		await get_tree().process_frame
	print("BINDING_RESULT total=", names.size(), " failed=", failed)
	get_tree().quit(0 if failed == 0 else 1)
