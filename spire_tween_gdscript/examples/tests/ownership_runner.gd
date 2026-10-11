extends Node

const TEST_SCENE := preload("res://examples/tests/ownership.tscn")


func _ready() -> void:
	var probe := TEST_SCENE.instantiate()
	var names: Array[String] = []
	for method in probe.get_method_list():
		if method.name.begins_with("test_"):
			names.append(method.name)
	probe.free()
	var failed := 0
	for method_name in names:
		var test := TEST_SCENE.instantiate()
		add_child(test)
		print("OWNERSHIP_START ", method_name)
		var result: Variant = await test.call(method_name)
		if result is bool and result:
			print("OWNERSHIP_PASS ", method_name)
		else:
			failed += 1
			print("OWNERSHIP_FAIL ", method_name)
		test.queue_free()
		await get_tree().process_frame
	print("OWNERSHIP_RESULT total=", names.size(), " failed=", failed)
	get_tree().quit(0 if failed == 0 else 1)
