extends SceneTree

const TEST_SCENE := preload("res://examples/tests/sequence_speed.tscn")
var completed := false

func _initialize() -> void:
	call_deferred("run_tests")

func run_tests() -> void:
	create_timer(30.0).timeout.connect(func():
		if not completed:
			push_error("Sequence speed tests timed out")
			quit(1))
	var probe := TEST_SCENE.instantiate()
	var names: Array[String] = []
	for method in probe.get_method_list():
		if method.name.begins_with("test_"):
			names.append(method.name)
	probe.free()
	var failed := 0 if not names.is_empty() else 1
	var cases := 0
	for method_name in names:
		var test := TEST_SCENE.instantiate()
		root.add_child(test)
		print("EXCESS_START ", method_name)
		var result: Variant = await test.call(method_name)
		cases += int(test.get("cases"))
		if result is bool and result:
			print("EXCESS_PASS ", method_name)
		else:
			failed += 1
			print("EXCESS_FAIL ", method_name)
		test.queue_free()
		for _i in 4:
			await process_frame
	completed = true
	print("EXCESS_RESULT total=", names.size(), " cases=", cases, " failed=", failed)
	quit(0 if failed == 0 else 1)
