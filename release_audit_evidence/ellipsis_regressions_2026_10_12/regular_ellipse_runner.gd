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
	var total := 0
	var failed := 0
	for scene: PackedScene in QuietRunner.TESTS:
		if scene.resource_path != "res://examples/tests/ellipsis_regressions.tscn": continue
		for method in runner.get_prefab_test_names(scene):
			await runner.run_test(scene, method)
			total += 1
			if runner.cached_results[scene.resource_path][method] != true: failed += 1
			for _frame in 4: await process_frame
	runner.free()
	print("REGULAR_ELLIPSE_RESULT total=", total, " failed=", failed)
	quit(0 if total == 11 and failed == 0 else 1)
