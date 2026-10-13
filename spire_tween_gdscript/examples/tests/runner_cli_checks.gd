extends SceneTree

class QuietRunner extends "res://examples/tests/test_runner.gd":
	func _ready() -> void:
		pass

const ELLIPSIS := preload("res://examples/tests/ellipsis.tscn")
const SEQUENCE_SPEED := preload("res://examples/tests/sequence_speed.tscn")
const ELLIPSIS_REGRESSIONS := preload("res://examples/tests/ellipsis_regressions.tscn")
var total := 0
var failed := 0

func _initialize() -> void:
	call_deferred("run_checks")

func check(condition: bool, name: String) -> void:
	total += 1
	if condition:
		print("RUNNER_CLI_PASS ", name)
	else:
		failed += 1
		print("RUNNER_CLI_FAIL ", name)

func write_cache(contents: String) -> void:
	var file := FileAccess.open(QuietRunner.CACHE_PATH, FileAccess.WRITE)
	file.store_string(contents)
	file.close()

func run_checks() -> void:
	var cache_existed := FileAccess.file_exists(QuietRunner.CACHE_PATH)
	var original_cache := FileAccess.get_file_as_bytes(QuietRunner.CACHE_PATH) if cache_existed else PackedByteArray()
	var runner := QuietRunner.new()
	root.add_child(runner)
	runner.configure_from_command_line()
	var args := OS.get_cmdline_user_args()
	check(runner.run_mode == (QuietRunner.RunMode.All if args.has("--all-tests") else QuietRunner.RunMode.FailedOnly), "command_line_run_mode")
	check(runner.use_cache == not args.has("--no-cache"), "command_line_cache_mode")

	DirAccess.remove_absolute(QuietRunner.CACHE_PATH)
	runner.load_cache_from_disk()
	check(runner.cached_results.is_empty(), "missing_cache_is_allowed")

	var sentinel := '{"sentinel": true}'
	write_cache(sentinel)
	runner.load_cache_from_disk()
	check(runner.cached_results.has("sentinel") == runner.use_cache, "cache_read_or_bypass")
	runner.cached_results = {"saved": true}
	runner.save_cache_to_disk()
	var contents := FileAccess.get_file_as_string(QuietRunner.CACHE_PATH)
	check(JSON.parse_string(contents).has("saved") if runner.use_cache else contents == sentinel, "cache_write_or_bypass")

	runner.cached_results = {}
	write_cache("{ invalid json")
	runner.load_cache_from_disk()
	check(runner.cached_results.is_empty(), "invalid_cache_is_ignored")

	var included := false
	var speed_included := false
	var ellipsis_regressions_included := false
	for scene: PackedScene in QuietRunner.TESTS:
		if scene.resource_path == ELLIPSIS.resource_path:
			included = true
		if scene.resource_path == SEQUENCE_SPEED.resource_path:
			speed_included = true
		if scene.resource_path == ELLIPSIS_REGRESSIONS.resource_path:
			ellipsis_regressions_included = true
	check(included, "ellipsis_scene_is_in_catalog")
	check(speed_included, "sequence_speed_scene_is_in_catalog")
	check(ellipsis_regressions_included, "ellipsis_regressions_scene_is_in_catalog")
	var all_suites_runnable := true
	for suite: PackedScene in QuietRunner.TESTS:
		var instance := suite.instantiate()
		all_suites_runnable = instance.has_method("run_test") and all_suites_runnable
		instance.free()
	check(all_suites_runnable, "every_catalog_suite_has_the_runner_interface")
	var names: Array = runner.get_prefab_test_names(ELLIPSIS)
	names.sort()
	check(names == ["test_circle", "test_circle_inverted", "test_ellipsis", "test_ellipsis_varying", "test_ellipsis_varying_inverted"], "all_five_ellipsis_tests_are_discovered")
	check(runner.get_prefab_test_names(SEQUENCE_SPEED).size() == 9, "all_nine_sequence_speed_tests_are_discovered")
	check(runner.get_prefab_test_names(ELLIPSIS_REGRESSIONS).size() == 11, "all_eleven_ellipsis_regressions_are_discovered")
	await process_frame
	runner.get_prefab_hash(ELLIPSIS)
	var count_before := Performance.get_monitor(Performance.OBJECT_COUNT)
	for _iteration in 8:
		runner.get_prefab_hash(ELLIPSIS)
	check(Performance.get_monitor(Performance.OBJECT_COUNT) == count_before, "hashing_frees_temporary_nodes")

	var parent := Node.new()
	root.add_child(parent)
	var scene := ELLIPSIS.instantiate()
	parent.add_child(scene)
	check(scene.process_priority == -10 and scene.process_physics_priority == -10, "ellipsis_initializes_test_base")
	await process_frame
	parent.free()
	runner.free()

	if cache_existed:
		var file := FileAccess.open(QuietRunner.CACHE_PATH, FileAccess.WRITE)
		file.store_buffer(original_cache)
		file.close()
	else:
		DirAccess.remove_absolute(QuietRunner.CACHE_PATH)
	print("RUNNER_CLI_RESULT total=", total, " failed=", failed)
	quit(0 if failed == 0 else 1)
