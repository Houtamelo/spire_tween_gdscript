@tool
extends Node

const CACHE_PATH := "res://examples/tests/results.json"

const TESTS: Array[PackedScene] = [
	preload("res://examples/tests/delays.tscn"),
	preload("res://examples/tests/error_handling.tscn"),
	preload("res://examples/tests/misc.tscn"),
	preload("res://examples/tests/pause_process_modes.tscn"),
	preload("res://examples/tests/play_pause_stop.tscn"),
	preload("res://examples/tests/register_unregister.tscn"),
	preload("res://examples/tests/sequences.tscn"),
	preload("res://examples/tests/lerp_modes.tscn"),
	preload("res://examples/tests/lerp_callable.tscn"),
]


enum RunMode {
	FailedOnly,
	All,
	Single,
}

@export var run_mode: RunMode = RunMode.FailedOnly
@export var single_test_to_run: String = ""

var cached_results: Dictionary = {}


func _ready():
	if Engine.is_editor_hint(): return
	
	var json := FileAccess.get_file_as_string(CACHE_PATH)
	var parser := JSON.new()
	var result := parser.parse(json)
	if result == OK:
		print("Succesfully loaded cached results")
		cached_results = parser.data
	else:
		print("Failed to load cached results: Err={0}, data={1}".format([result, parser.data]))
	
	match run_mode:
		RunMode.FailedOnly: await run_dirty_or_failed_tests()
		RunMode.All: await run_all_tests()
		RunMode.Single: await run_test_by_name("test_" + single_test_to_run)
	
	save_cache_to_disk()
	
	print("Test Runner finished!")
	
	await get_tree().create_timer(1.0).timeout
	get_tree().quit()


func save_cache_to_disk():
	var json = JSON.stringify(cached_results, "\t")
	var fa := FileAccess.open(CACHE_PATH, FileAccess.WRITE_READ)
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
	for prefab in TESTS:
		var cache: Dictionary = cached_results.get_or_add(prefab.resource_path, {})
		var cached_hash: int = cache.get("hash", -1)
		var curr_hash := get_prefab_hash(prefab)
		var is_dirty := cached_hash != curr_hash
		cache["hash"] = curr_hash
		
		var prefab_names := get_prefab_test_names(prefab)
		for method in prefab_names:
			var previously_failed: bool = !cache.get(method, false)
			if previously_failed or is_dirty:
				await run_test(prefab, method)
		
		for key: String in cache.keys():
			if key.begins_with("test_") and !prefab_names.has(key):
				cache.erase(key)
		
		save_cache_to_disk()


func run_all_tests():
	for prefab in TESTS:
		print("Scene: " + str(prefab.resource_path))
		
		for method in get_prefab_test_names(prefab):
			await run_test(prefab, method)
		
		save_cache_to_disk()


func run_test(prefab: PackedScene, method_name: String):
	var node: Node = prefab.instantiate()
	add_child(node)
	
	var method := Callable(node, method_name)
	var result: bool = await node.call(&"run_test", method)
	
	cached_results.get_or_add(prefab.resource_path, {})
	cached_results[prefab.resource_path][method_name] = result
	
	node.queue_free()


func run_test_by_name(method_name: String):
	for prefab in TESTS:
		for method in get_prefab_test_names(prefab):
			if method == method_name:
				await run_test(prefab, method)


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
	return hash(source_code)
