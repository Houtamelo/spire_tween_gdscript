extends Node

const TEST_SCENE := preload("res://examples/tests/delayed_call_validity.tscn")

func _ready() -> void:
	var probe := TEST_SCENE.instantiate()
	var names: Array[String] = []
	for method in probe.get_method_list():
		if method.name.begins_with("test_"):
			names.append(method.name)
	probe.free()
	var failed := 0 if not names.is_empty() else 1
	for method in names:
		var test := TEST_SCENE.instantiate()
		add_child(test)
		print("DELAY_VALIDITY_START ", method)
		var result: Variant = await test.call(method)
		if result is bool and result:
			print("DELAY_VALIDITY_PASS ", method)
		else:
			failed += 1
			print("DELAY_VALIDITY_FAIL ", method)
		test.queue_free()
		for _frame in 4:
			await get_tree().process_frame
	print("DELAY_VALIDITY_RESULT total=", names.size(), " failed=", failed)
	get_tree().quit(0 if failed == 0 else 1)
