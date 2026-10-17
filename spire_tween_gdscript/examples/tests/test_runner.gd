@tool
extends Node

const CACHE_PATH := "res://examples/tests/results.json"
const CACHE_SCHEMA := 2

const TESTS: Array[PackedScene] = [
	preload("res://examples/tests/delays.tscn"),
	preload("res://examples/tests/error_handling.tscn"),
	preload("res://examples/tests/misc.tscn"),
	preload("res://examples/tests/pause_process_modes.tscn"),
	preload("res://examples/tests/play_pause_stop.tscn"),
	preload("res://examples/tests/register_unregister.tscn"),
	preload("res://examples/tests/sequences.tscn"),
	preload("res://examples/tests/sequence_ctors.tscn"),
	preload("res://examples/tests/sequence_speed.tscn"),
	preload("res://examples/tests/lerp_modes.tscn"),
	preload("res://examples/tests/lerp_callable.tscn"),
	preload("res://examples/tests/spiral.tscn"),
	preload("res://examples/tests/ellipsis.tscn"),
	preload("res://examples/tests/ellipsis_regressions.tscn"),
	preload("res://examples/tests/ownership.tscn"),
	preload("res://examples/tests/node_binding.tscn"),
	preload("res://examples/tests/sequence_cycles.tscn"),
	preload("res://examples/tests/delayed_call_validity.tscn"),
	preload("res://examples/tests/method_validity.tscn"),
]


enum RunMode {
	FailedOnly,
	All,
	Single,
}

@export var run_mode: RunMode = RunMode.FailedOnly
@export var single_test_to_run: String = ""

var cached_results: Dictionary = {}
var use_cache: bool = true
var test_scenes: Array[PackedScene] = TESTS.duplicate()
var test_timeout: float = 60.0
var suite_timeout: float = 900.0
var passed: int = 0
var failed: int = 0
var skipped: int = 0
var aborted: bool = false
var configuration_valid: bool = true
var _suite_started: int = 0
var _finished: bool = false
var _cache_identity: Dictionary = {}


func _ready():
	if Engine.is_editor_hint(): return
	configure_from_command_line()
	if not configuration_valid:
		get_tree().quit(2)
		return
	_suite_started = Time.get_ticks_msec()
	var watchdog := Timer.new()
	watchdog.process_mode = Node.PROCESS_MODE_ALWAYS
	watchdog.ignore_time_scale = true
	watchdog.wait_time = 0.05
	watchdog.timeout.connect(_check_suite_deadline)
	add_child(watchdog)
	watchdog.start()
	load_cache_from_disk()

	match run_mode:
		RunMode.FailedOnly: await run_dirty_or_failed_tests()
		RunMode.All: await run_all_tests()
		RunMode.Single: await run_test_by_name("test_" + single_test_to_run)

	finish_run()


func _check_suite_deadline() -> void:
	if _suite_started > 0 and not _finished and Time.get_ticks_msec() - _suite_started >= suite_timeout * 1000:
		failed += 1
		aborted = true
		push_error("TEST_SUITE_TIMEOUT")
		finish_run()


func finish_run() -> void:
	if _finished: return
	_finished = true
	save_cache_to_disk()
	print("GDSCRIPT_TEST_RESULT total=", passed + failed, " failed=", failed, " skipped=", skipped)
	# Drain queued scene deletion and both manager loops before engine shutdown.
	for _frame in 4:
		await get_tree().process_frame
	get_tree().quit(0 if failed == 0 else 1)


func configure_from_command_line() -> void:
	var args := OS.get_cmdline_user_args()
	if args.has("--all-tests"):
		run_mode = RunMode.All
	use_cache = not args.has("--no-cache")
	for arg in args:
		if arg.begins_with("--test="):
			run_mode = RunMode.Single
			single_test_to_run = arg.trim_prefix("--test=").trim_prefix("test_")
		elif arg.begins_with("--test-timeout=") or arg.begins_with("--suite-timeout="):
			var value_text := arg.get_slice("=", 1)
			var value := value_text.to_float()
			if not value_text.is_valid_float() or not is_finite(value) or value <= 0 or value > 86400:
				configuration_valid = false
				push_error("Invalid timeout: " + arg)
			elif arg.begins_with("--test-timeout="):
				test_timeout = value
			else:
				suite_timeout = value


func load_cache_from_disk() -> void:
	if not use_cache: return
	cached_results = {}
	_cache_identity = get_cache_identity()
	if _cache_identity.is_empty():
		use_cache = false
		print("Cache disabled: could not identify the addon library")
		return
	if not FileAccess.file_exists(CACHE_PATH): return

	var json := FileAccess.get_file_as_string(CACHE_PATH)
	var parser := JSON.new()
	var result := parser.parse(json)
	if result == OK and parser.data is Dictionary:
		var data: Dictionary = parser.data
		if data.get("schema") == CACHE_SCHEMA and data.get("identity") == _cache_identity and data.get("results") is Dictionary:
			print("Loaded matching cached results")
			cached_results = data.results
		else:
			print("Ignoring stale test cache")
	else:
		print("Failed to load cached results: Err={0}, data={1}".format([result, parser.data]))

func save_cache_to_disk():
	if not use_cache: return
	if _cache_identity.is_empty(): _cache_identity = get_cache_identity()
	if _cache_identity.is_empty(): return

	var json = JSON.stringify({"schema": CACHE_SCHEMA, "identity": _cache_identity, "results": cached_results}, "\t")
	var fa := FileAccess.open(CACHE_PATH, FileAccess.WRITE_READ)
	if fa == null:
		push_error("Unable to save test cache: " + str(FileAccess.get_open_error()))
		failed += 1
		return
	fa.store_string(json)
	fa.close()


func _validate_property(property: Dictionary) -> void:
	if property["name"] != "single_test_to_run": return

	property["hint"] = PROPERTY_HINT_ENUM

	var all_methods: PackedStringArray = []
	for prefab in TESTS:
		for method: String in get_prefab_test_names(prefab):
			all_methods.append(method.trim_prefix("test_"))

	property["hint_string"] = ",".join(all_methods)


func run_dirty_or_failed_tests():
	for prefab in test_scenes:
		var cache: Dictionary = cached_results.get_or_add(prefab.resource_path, {})
		var cached_hash: int = cache.get("hash", -1)
		var curr_hash := get_prefab_hash(prefab)
		var is_dirty := cached_hash != curr_hash
		cache["hash"] = curr_hash

		var prefab_names := get_prefab_test_names(prefab)
		for method in prefab_names:
			if aborted: return
			var previously_failed: bool = !cache.get(method, false)
			if previously_failed or is_dirty:
				await run_test(prefab, method)
			else:
				skipped += 1

		for key: String in cache.keys():
			if key.begins_with("test_") and !prefab_names.has(key):
				cache.erase(key)

		save_cache_to_disk()


func run_all_tests():
	for prefab in test_scenes:
		print("Scene: " + str(prefab.resource_path))

		for method in get_prefab_test_names(prefab):
			if aborted: return
			await run_test(prefab, method)

		save_cache_to_disk()


func run_test(prefab: PackedScene, method_name: String) -> bool:
	var failures_before := TestBase.assertion_failures
	cached_results.get_or_add(prefab.resource_path, {})[method_name] = false
	var node: Node = prefab.instantiate()
	add_child(node)

	var method := Callable(node, method_name)
	var state := {"done": false, "result": false}
	var started := Time.get_ticks_msec()
	_execute_test(node, method, state)
	while not state.done and not aborted and Time.get_ticks_msec() - started < test_timeout * 1000:
		await get_tree().process_frame
	var result: bool = state.done and state.result is bool and state.result and TestBase.assertion_failures == failures_before
	if not state.done:
		aborted = true
		push_error("TEST_TIMEOUT: " + prefab.resource_path + "::" + method_name)
	if result:
		passed += 1
	else:
		failed += 1
	print("TEST_PASS " if result else "TEST_FAIL ", prefab.resource_path, "::", method_name)

	cached_results.get_or_add(prefab.resource_path, {})
	cached_results[prefab.resource_path][method_name] = result

	node.queue_free()
	await get_tree().process_frame
	return result


func _execute_test(node: Node, method: Callable, state: Dictionary) -> void:
	state.result = await node.call(&"run_test", method)
	state.done = true


func run_test_by_name(method_name: String):
	var matches := 0
	for prefab in test_scenes:
		for method in get_prefab_test_names(prefab):
			if method == method_name:
				matches += 1
				await run_test(prefab, method)
	if matches == 0:
		failed += 1
		push_error("No test matched: " + method_name)


static func get_prefab_test_names(prefab: PackedScene) -> Array:
	var node: Node = prefab.instantiate()
	var methods := node.get_method_list()
	node.queue_free()

	return methods\
		.map(func(m): return m["name"])\
		.filter(func(n): return n.begins_with("test_"))


func get_prefab_hash(prefab: PackedScene) -> int:
	var node := prefab.instantiate()
	var script: GDScript = node.get_script()
	var source_code := script.source_code
	node.free()
	return hash(source_code)


func get_cache_identity() -> Dictionary:
	var manifest_path := "res://addons/spire_tween/spire_tween.gdextension"
	var manifest := ConfigFile.new()
	if manifest.load(manifest_path) != OK: return {}
	var library_path := ""
	var specificity := -1
	for key in manifest.get_section_keys("libraries"):
		var tags := key.split(".")
		var matches := true
		for tag in tags:
			matches = matches and OS.has_feature(tag)
		if matches and tags.size() > specificity:
			specificity = tags.size()
			library_path = manifest.get_value("libraries", key)
	if library_path.is_empty(): return {}
	if not library_path.begins_with("res://") and not library_path.is_absolute_path():
		library_path = manifest_path.get_base_dir().path_join(library_path).simplify_path()
	var binary_hash := FileAccess.get_sha256(library_path)
	if binary_hash.is_empty(): return {}
	var sources: Dictionary = {}
	_hash_test_sources("res://examples", sources)
	var clock_args: Array[String] = []
	var args := OS.get_cmdline_args()
	for i in args.size():
		if args[i] in ["--fixed-fps", "--max-fps"] and i + 1 < args.size():
			clock_args.append(args[i])
			clock_args.append(args[i + 1])
	return {"binary": binary_hash, "engine": FileAccess.get_sha256(OS.get_executable_path()),
		"engine_version": Engine.get_version_info().string, "sources": sources,
		"manifest": FileAccess.get_sha256(manifest_path), "project": FileAccess.get_sha256("res://project.godot"),
		"clock_args": clock_args, "physics_ticks": str(Engine.physics_ticks_per_second)}


func _hash_test_sources(path: String, hashes: Dictionary) -> void:
	var directory := DirAccess.open(path)
	if directory == null: return
	for file in directory.get_files():
		if file.get_extension() in ["gd", "tscn"]:
			var full_path := path.path_join(file)
			hashes[full_path] = FileAccess.get_sha256(full_path)
	for child in directory.get_directories():
		_hash_test_sources(path.path_join(child), hashes)
