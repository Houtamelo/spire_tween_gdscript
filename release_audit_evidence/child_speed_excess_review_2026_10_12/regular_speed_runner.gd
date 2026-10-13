extends SceneTree

class QuietRunner extends "res://examples/tests/test_runner.gd":
	func _ready() -> void:
		pass

func _initialize() -> void:
	call_deferred("run")

func run() -> void:
	var runner := QuietRunner.new()
	runner.use_cache = false
	root.add_child(runner)
	var count := 0
	var failed := 0
	for scene: PackedScene in QuietRunner.TESTS:
		if scene.resource_path != "res://examples/tests/sequence_speed.tscn":
			continue
		for name in runner.get_prefab_test_names(scene):
			await runner.run_test(scene, name)
			count += 1
			if runner.cached_results[scene.resource_path][name] != true:
				failed += 1
			for _frame in 4:
				await process_frame
	runner.free()
	print("REGULAR_EXCESS_RESULT total=", count, " failed=", failed)
	quit(0 if count == 9 and failed == 0 else 1)
