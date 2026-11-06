@tool
extends EditorPlugin

var output: FileAccess
var tree_id := 0
var root_id := 0
var controller_id := 0
var a_id := 0
var b_id := 0
var clock_completions := 0
var target_stops := 0
var target_finishes := 0
var failures: Array[String] = []
var expect_clock_ticks := true
var last_ticks := Vector2i.ZERO

func _enter_tree() -> void:
	output = FileAccess.open(OS.get_environment("SPIRE_PROBE_OUTPUT"), FileAccess.WRITE)
	expect_clock_ticks = OS.get_environment("SPIRE_EXPECT_TICKS") != "0"
	scene_changed.connect(_scene_changed)
	scene_closed.connect(_scene_closed)
	call_deferred("_run")

func _record(event: String, extra: Dictionary = {}) -> void:
	var row := {"event": event, "ms": Time.get_ticks_msec(), "pid": OS.get_process_id(), "variant": OS.get_environment("SPIRE_PROBE_VARIANT")}
	row.merge(extra)
	var line := JSON.stringify(row)
	print("EDITOR_CONTROLLER_PROBE ", line)
	if output != null:
		output.store_line(line)
		output.flush()

func _node_info(node: Node) -> Dictionary:
	if not is_instance_valid(node):
		return {"valid": false}
	var info := {"valid": true, "id": node.get_instance_id(), "class": node.get_class(), "name": str(node.name), "inside_tree": node.is_inside_tree(), "owner_id": node.owner.get_instance_id() if node.owner != null else 0}
	if node.is_inside_tree():
		info["path"] = str(node.get_path())
		info["tree_id"] = node.get_tree().get_instance_id()
	if node.get_parent() != null:
		info["parent_id"] = node.get_parent().get_instance_id()
	if node is Node2D:
		info["position_x"] = node.position.x
	return info

func _by_id(id: int) -> Node:
	if id == 0 or not is_instance_id_valid(id):
		return null
	return instance_from_id(id) as Node

func _ticks(controller: Node) -> Vector2i:
	if not is_instance_valid(controller):
		return Vector2i(-1, -1)
	for property in controller.get_property_list():
		if property.name == "probe_process_ticks":
			return Vector2i(controller.get("probe_process_ticks"), controller.get("probe_physics_ticks"))
	return Vector2i(-1, -1)

func _snapshot(stage: String) -> void:
	var tree := get_tree()
	var controller := tree.root.get_node_or_null("tweens_controller")
	var child_names: Array[String] = []
	for child in tree.root.get_children():
		child_names.append(str(child.name))
	_record(stage, {"main_loop_class": Engine.get_main_loop().get_class(), "tree_id": tree.get_instance_id(), "root": _node_info(tree.root), "root_children": child_names, "current_scene": _node_info(tree.current_scene), "edited_scene": _node_info(EditorInterface.get_edited_scene_root()), "open_scenes": EditorInterface.get_open_scenes(), "controller": _node_info(controller), "ticks": str(_ticks(controller)), "scene_a": _node_info(_by_id(a_id)), "scene_b": _node_info(_by_id(b_id)), "clock_completions": clock_completions, "target_stops": target_stops, "target_finishes": target_finishes})
	_check(tree.get_instance_id() == tree_id, stage + ": same SceneTree")
	_check(tree.root.get_instance_id() == root_id, stage + ": same root Window")
	if controller_id != 0:
		_check(is_instance_valid(controller) and controller.get_instance_id() == controller_id, stage + ": same native controller")
		_check(controller.get_parent() == tree.root, stage + ": controller remains a direct child of root")
		_check(controller.owner == null, stage + ": controller has no scene owner")
		if expect_clock_ticks:
			var ticks := _ticks(controller)
			# Production builds have no diagnostic counters. Their real sequences and
			# target property changes below still verify editor playback and cleanup.
			if ticks.x >= 0 and ticks.y >= 0:
				_check(ticks.x > last_ticks.x and ticks.y > last_ticks.y, stage + ": both native callbacks keep advancing")
				last_ticks = ticks

func _check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		_record("ASSERTION_FAILED", {"message": message})

func _settle(seconds: float = 0.25) -> void:
	await get_tree().create_timer(seconds).timeout
	await get_tree().process_frame
	await get_tree().process_frame

func _scene_changed(scene: Node) -> void:
	_record("signal_scene_changed", {"scene": _node_info(scene)})

func _scene_closed(path: String) -> void:
	_record("signal_scene_closed", {"path": path, "a_valid": is_instance_id_valid(a_id), "b_valid": is_instance_id_valid(b_id)})

func _target_stopped() -> void:
	target_stops += 1
	_record("target_tween_stopped")

func _target_finished() -> void:
	target_finishes += 1
	_record("target_tween_finished")

func _run() -> void:
	await _settle(0.8)
	tree_id = get_tree().get_instance_id()
	root_id = get_tree().root.get_instance_id()
	_record("version", {"godot": Engine.get_version_info(), "editor_hint": Engine.is_editor_hint(), "headless": DisplayServer.get_name()})
	# ClassDB uses editor placeholders for non-tool native classes. Unlike the
	# internal new_alloc() path, this makes the tool declaration observable.
	var classdb_probe := ClassDB.instantiate("TweensController") as Node
	_check(classdb_probe != null, "ClassDB can instantiate the controller registration")
	if classdb_probe != null:
		classdb_probe.name = "classdb_controller_probe"
		get_tree().root.add_child(classdb_probe)
		var ready_configured := classdb_probe.process_mode == Node.PROCESS_MODE_ALWAYS and classdb_probe.process_priority == 2147483647 and classdb_probe.process_physics_priority == 2147483647
		var expect_tool_registration := OS.get_environment("SPIRE_EXPECT_TOOL") == "1"
		_record("classdb_registration", {"ready_configured": ready_configured, "expected_tool": expect_tool_registration})
		_check(ready_configured == expect_tool_registration, "ClassDB instantiation reflects the expected tool registration")
		# Remove this temporary instance before any frame can tick the manager twice.
		classdb_probe.set_process(false)
		classdb_probe.set_physics_process(false)
		get_tree().root.remove_child(classdb_probe)
		classdb_probe.free()
	_snapshot("before_first_tween")
	var clock = Spire.sequence().append_interval(0.05).append_call(func(): clock_completions += 1)
	await _settle()
	var controller := get_tree().root.get_node_or_null("tweens_controller")
	_check(controller != null, "first tween attaches native controller to editor root")
	if controller != null:
		controller_id = controller.get_instance_id()
	_snapshot("no_open_scene")
	_check(clock_completions == (1 if expect_clock_ticks else 0), "native clock matches expected callback behavior")

	EditorInterface.open_scene_from_path("res://scene_a.tscn")
	await _settle()
	var a := EditorInterface.get_edited_scene_root() as Node2D
	_check(a != null and a.scene_file_path == "res://scene_a.tscn", "opened scene A")
	if a == null:
		get_tree().quit(2)
		return
	a_id = a.get_instance_id()
	a.tree_entered.connect(func(): _record("scene_a_tree_entered"))
	a.tree_exiting.connect(func(): _record("scene_a_tree_exiting"))
	a.tree_exited.connect(func(): _record("scene_a_tree_exited"))
	var target_tween = DoNode2D.position(a, Vector2(1000.0, 0.0), 100.0)
	target_tween.stopped.connect(_target_stopped)
	target_tween.finished.connect(_target_finished)
	await _settle()
	_snapshot("scene_a_active")
	var a_position_before_switch := a.position.x
	if expect_clock_ticks:
		_check(a_position_before_switch > 0.0, "property tween progresses while scene A is active")

	EditorInterface.open_scene_from_path("res://scene_b.tscn")
	await _settle()
	var b := EditorInterface.get_edited_scene_root()
	_check(b != null and b.scene_file_path == "res://scene_b.tscn", "opened scene B")
	b_id = b.get_instance_id()
	_snapshot("scene_b_active_a_background")
	var a_position_in_background := a.position.x
	await _settle()
	_snapshot("scene_b_still_active")
	_check(is_instance_id_valid(a_id) and not a.is_inside_tree(), "background tab detaches scene A without freeing it")
	_check(is_equal_approx(a.position.x, a_position_in_background), "default node-bound tween does not advance in background tab")
	_record("background_target_motion", {"a_x_before_switch": a_position_before_switch, "a_now": _node_info(_by_id(a_id))})
	if DisplayServer.get_name() == "headless":
		var packed := PackedScene.new()
		_check(packed.pack(b) == OK, "packing scene B succeeds with root controller installed")
		_check(ResourceSaver.save(packed, "res://scene_b.tscn") == OK, "saving packed scene B succeeds")
	else:
		_check(EditorInterface.save_scene() == OK, "saving scene B succeeds with root controller installed")
	_check(not FileAccess.get_file_as_string("res://scene_b.tscn").contains("tweens_controller"), "saving edited scene does not serialize the root controller")

	EditorInterface.open_scene_from_path("res://scene_a.tscn")
	await _settle()
	_check(EditorInterface.get_edited_scene_root().get_instance_id() == a_id, "switching back reuses scene A instance")
	_snapshot("scene_a_reactivated")
	if expect_clock_ticks:
		_check(a.position.x > a_position_in_background, "node-bound tween resumes when scene A becomes active again")

	var close_a: int = EditorInterface.close_scene()
	_record("close_a_result", {"error": close_a})
	await _settle()
	_snapshot("after_close_a")
	_check(not is_instance_id_valid(a_id), "closing scene A frees its target")
	if expect_clock_ticks:
		_check(target_stops == 1 and target_finishes == 0, "closing target cancels its tween exactly once")

	var close_b: int = EditorInterface.close_scene()
	_record("close_b_result", {"error": close_b})
	await _settle()
	_snapshot("after_close_all")
	var saved_scenes := EditorInterface.get_open_scenes()
	if saved_scenes.has(""):
		saved_scenes.remove_at(saved_scenes.find(""))
	_check(saved_scenes.is_empty(), "all saved scene tabs closed")
	_check(not is_instance_id_valid(b_id), "closing scene B frees its target")

	EditorInterface.open_scene_from_path("res://scene_a.tscn")
	await _settle()
	var reopened_a := EditorInterface.get_edited_scene_root()
	_check(reopened_a != null and reopened_a.get_instance_id() != a_id, "reopening A creates a new scene instance")
	_snapshot("scene_a_reopened_new_instance")
	var final_clock = Spire.sequence().append_interval(0.05).append_call(func(): clock_completions += 1)
	await _settle()
	_snapshot("final_clock_after_scene_reopen")
	_check(clock_completions == (2 if expect_clock_ticks else 0), "controller still drives new sequences after reopening")
	_record("RESULT", {"ok": failures.is_empty(), "failures": failures})
	get_tree().quit(0 if failures.is_empty() else 1)
