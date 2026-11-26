@tool
extends EditorPlugin

const Custom := preload("blueprint_preview_custom.gd")
const Uncaptured := preload("blueprint_preview_uncaptured.gd")
var failures: Array[String] = []
var checks := 0
var scene: Node2D
var target: Node2D

func _enter_tree() -> void:
	_run.call_deferred()

func check(condition: bool, message: String) -> void:
	checks += 1
	if not condition:
		failures.append(message)
		push_error("BLUEPRINT_PREVIEW_ASSERTION: " + message)

func _frames(count: int = 3) -> void:
	for _frame in count: await get_tree().process_frame

func _button(name: StringName) -> Button:
	var label := "Play Animation Preview" if name == &"play_preview_button" else "Stop Animation Preview"
	for button: Button in EditorInterface.get_inspector().find_children("*", "Button", true, false):
		if button.text == label: return button
	return null

func _click(button: Button) -> void:
	if button == null: return
	EditorInterface.get_inspector().ensure_control_visible(button)
	await _frames()
	check(button.is_visible_in_tree() and not button.disabled, "Inspector button is visible and enabled: " + button.text)
	var motion := InputEventMouseMotion.new()
	motion.position = button.get_global_rect().get_center()
	motion.global_position = motion.position
	# Godot routes the click using the physical cursor's hovered control.
	# This cursor belongs exclusively to the harness's private Xvfb display.
	DisplayServer.warp_mouse(DisplayServer.screen_get_size() - Vector2i.ONE)
	await _frames(2)
	Input.warp_mouse(motion.position)
	Input.parse_input_event(motion)
	await _frames(1)
	check(button.is_hovered(), "Virtual cursor hovers Inspector button: " + button.text)
	for pressed in [true, false]:
		var event := InputEventMouseButton.new()
		event.position = button.get_global_rect().get_center()
		event.global_position = event.position
		event.button_index = MOUSE_BUTTON_LEFT
		event.pressed = pressed
		event.button_mask = MOUSE_BUTTON_MASK_LEFT if pressed else 0
		Input.parse_input_event(event)
		await _frames(1)
	await _frames()

func _screenshot(name: String) -> void:
	var image := EditorInterface.get_base_control().get_viewport().get_texture().get_image()
	check(image != null and image.save_png("user://" + name + ".png") == OK, "Rendered preview screenshot: " + name)

func _bp(parent: Node, property: StringName = &"position", end: Variant = Vector2(220, 130)) -> TweenBlueprint:
	var bp := TweenBlueprint.new()
	parent.add_child(bp)
	bp.owner = scene
	bp.kind = Spire.BLUEPRINT_KIND_PROPERTY
	bp.target = target
	bp.property_to_animate = property
	bp.property_end_value = end
	bp.property_speed_or_duration = 1.0
	bp.property_ease_enum = Spire.EASE_LINEAR
	return bp

func _start(bp: TweenBlueprint) -> SpireSequence:
	var result: Array[SpireSequence] = []
	var spawned := func(sequence: SpireSequence) -> void: result.append(sequence)
	bp.tween_spawned.connect(spawned, CONNECT_ONE_SHOT)
	bp.play_animation_preview()
	if result.is_empty():
		bp.tween_spawned.disconnect(spawned)
		return null
	return result[0]

func _pose(node: Node) -> Array:
	return [node.position, node.scale, node.rotation, node.modulate]

func _characters(label: SpireRichTextLabel) -> Array:
	var result: Array = []
	for index in label.get_character_count():
		result.append([label.get_char_offset(index), label.get_char_rotation(index), label.get_char_scale(index), label.get_char_color(index), label.get_char_alpha(index), label.get_char_pivot(index)])
	return result

func _run() -> void:
	await _frames(8)
	# Leave room outside the editor on the private display. Native hover tracking
	# needs an actual window exit/enter, including on Godot 4.5 without a WM.
	get_window().mode = Window.MODE_WINDOWED
	get_window().size = Vector2i(1440, 1000)
	get_window().position = Vector2i.ZERO
	await _frames(2)
	EditorInterface.open_scene_from_path("res://preview_scene.tscn")
	EditorInterface.set_main_screen_editor("2D")
	await _frames(10)
	scene = EditorInterface.get_edited_scene_root()
	target = scene.get_node("Target")
	await _native_and_buttons()
	await _composition()
	await _aliases()
	await _custom_and_failures()
	await _native_text()
	await _templates()
	await _scene_switch()
	print("BLUEPRINT_PREVIEW_EDITOR_RESULT checks=", checks, " failed=", failures.size())
	get_tree().quit(0 if failures.is_empty() else 1)

func _native_and_buttons() -> void:
	var rest := target.transform
	var bp := _bp(target)
	# Keep the mouse-driven run alive even on slow software-rendered CI editors.
	bp.property_speed_or_duration = 30.0
	EditorInterface.edit_node(bp)
	await _frames(5)
	var play := _button(&"play_preview_button")
	check(play != null, "Native tool-button annotation creates Play Animation Preview")
	check(_button(&"stop_preview_button") != null and _button(&"stop_preview_button").disabled, "Stop is disabled while idle")
	await _click(play)
	check(target.transform != rest, "Virtual mouse starts real controller-driven animation")
	_screenshot("preview-playing")
	await _click(_button(&"stop_preview_button"))
	check(target.transform.is_equal_approx(rest), "Virtual mouse Stop restores synchronously")
	_screenshot("preview-restored")
	check(_button(&"stop_preview_button") != null and _button(&"stop_preview_button").disabled, "Stop is disabled after restoration")
	bp.property_speed_or_duration = 1.0
	var run := _start(bp)
	check(run != null, "Preview emits its native sequence")
	if run != null:
		run.custom_step(0.3)
		check(target.transform != rest, "Native preview advances")
		bp.stop()
		check(target.transform.is_equal_approx(rest) and run.is_stopped(), "Ordinary stop restores preview")
		run = _start(bp)
		run.custom_step(0.3)
		run.stop()
		check(target.transform.is_equal_approx(rest), "Stopping returned handle restores preview")
		run = _start(bp)
		run.force_complete()
		check(target.transform.is_equal_approx(rest), "Natural completion restores root once")
		run = _start(bp)
		run.custom_step(0.25)
		var replacement := _start(bp)
		check(run.is_stopped() and target.transform.is_equal_approx(rest), "Restart restores before recapturing")
		replacement.custom_step(0.2)
		var other := _bp(target, &"rotation", 2.0)
		var next := _start(other)
		check(replacement.is_stopped() and target.transform.is_equal_approx(rest), "Starting another root restores previous preview")
		next.custom_step(0.2)
		other.stop_animation_preview()
		check(target.transform.is_equal_approx(rest), "Replacement has fresh baseline")
		other.free()
		# Unrelated registered editor tweens must survive preview cleanup.
		var unrelated := Spire.do_property(target, ^"modulate:r", 0.5, 10.0)
		run = _start(bp)
		run.custom_step(0.2)
		bp.stop_animation_preview()
		check(unrelated.is_playing(), "Preview cleanup does not stop unrelated tweens")
		unrelated.stop()
		target.modulate = Color.WHITE
		# Save the real edited scene mid-animation and reload the file immediately.
		run = _start(bp)
		run.custom_step(0.4)
		EditorInterface.save_scene_as("res://preview_saved.tscn")
		check(run.is_stopped() and target.transform.is_equal_approx(rest), "Pre-save stops and restores synchronously")
		var packed := ResourceLoader.load("res://preview_saved.tscn", "PackedScene", ResourceLoader.CACHE_MODE_IGNORE) as PackedScene
		var loaded := packed.instantiate()
		check(loaded.get_node("Target").transform.is_equal_approx(rest), "Saved scene contains authored pose")
		var text := FileAccess.get_file_as_string("res://preview_saved.tscn")
		check(not text.contains("play_preview_button") and not text.contains("stop_preview_button") and not text.contains("Callable"), "Buttons and restoration callbacks are never serialized")
		loaded.free()
		var runtime := bp.play()
		runtime.custom_step(0.2)
		bp.stop_animation_preview()
		check(runtime.is_playing(), "Preview Stop does not stop ordinary playback")
		bp.stop()
		target.transform = rest
	bp.free()

func _composition() -> void:
	var rest := target.position
	for kind in [Spire.BLUEPRINT_KIND_CHAIN_BLOCK, Spire.BLUEPRINT_KIND_PARALLEL_BLOCK]:
		var root := TweenBlueprint.new()
		target.add_child(root)
		root.kind = kind
		var first := _bp(root, &"position", rest + Vector2(100, 0))
		first.property_speed_or_duration = 0.2
		var nested := TweenBlueprint.new()
		root.add_child(nested)
		nested.kind = Spire.BLUEPRINT_KIND_CHAIN_BLOCK
		var second := _bp(nested, &"position", rest + Vector2(200, 0))
		second.property_speed_or_duration = 1.0
		EditorInterface.edit_node(first)
		await _frames()
		var button := _button(&"play_preview_button")
		check(button != null and button.disabled, "Parent-managed child's Play button is disabled")
		check(_start(first) == null, "Parent-managed child also rejects direct preview invocation")
		var run := _start(root)
		if run == null:
			check(false, "Composition starts")
		else:
			run.custom_step(0.25)
			check(target.position != rest, "Child completion does not restore root composition")
			root.stop_animation_preview()
			check(target.position.is_equal_approx(rest), "Nested composition restores original shared property")
			run = _start(root)
			run.force_complete()
			check(target.position.is_equal_approx(rest), "Composition completion restores shared property")
		root.free()

func _aliases() -> void:
	var parent := Node2D.new()
	scene.add_child(parent)
	parent.transform = Transform2D(0.3, Vector2(1.3, 0.8), 0.1, Vector2(80, 40))
	var child := Node2D.new()
	parent.add_child(child)
	child.position = Vector2(15, 23)
	var root := TweenBlueprint.new()
	scene.add_child(root)
	root.kind = Spire.BLUEPRINT_KIND_PARALLEL_BLOCK
	var a := _bp(root, &"position", Vector2(300, 200))
	a.target = parent
	var b := _bp(root, &"global_position", Vector2(400, 300))
	b.target = child
	var c := _bp(root, &"rotation_degrees", 120.0)
	c.target = child
	var d := _bp(root, &"global_position:y", 400.0)
	d.target = child
	var parent_rest := parent.transform
	var child_rest := child.transform
	var run := _start(root)
	run.custom_step(0.4)
	root.stop_animation_preview()
	check(parent.transform.is_equal_approx(parent_rest) and child.transform.is_equal_approx(child_rest), "Local canonical transforms restore parent/child global aliases")
	root.free()
	parent.free()
	var node3d := Node3D.new()
	scene.add_child(node3d)
	node3d.position = Vector3(10, 20, 30)
	node3d.rotation = Vector3(0.2, 0.3, 0.4)
	var rest3d := node3d.transform
	var bp := _bp(scene, &"global_rotation_degrees", Vector3(80, 30, 45))
	bp.target = node3d
	run = _start(bp)
	run.custom_step(0.4)
	bp.stop_animation_preview()
	check(node3d.transform.is_equal_approx(rest3d), "Node3D aliases restore local transform")
	bp.free()
	node3d.free()
	var control := Control.new()
	scene.add_child(control)
	control.position = Vector2(30, 40)
	control.size = Vector2(120, 80)
	control.pivot_offset = Vector2(8, 12)
	control.rotation = 0.3
	control.scale = Vector2(0.8, 1.2)
	var rest_control := [control.position, control.size, control.pivot_offset, control.rotation, control.scale]
	for property in [&"position", &"global_position", &"size", &"scale", &"pivot_offset"]:
		bp = _bp(scene, property, Vector2(200, 100))
		bp.target = control
		run = _start(bp)
		run.custom_step(0.4)
		bp.stop_animation_preview()
		check([control.position, control.size, control.pivot_offset, control.rotation, control.scale] == rest_control, "Control layout/transform restores: " + property)
		bp.free()
	control.free()

func _custom_and_failures() -> void:
	var rest := target.position
	var log: Array[String] = []
	var captures: Array[String] = []
	var root := TweenBlueprint.new()
	scene.add_child(root)
	for marker in 3:
		var child := Custom.new()
		root.add_child(child)
		child.target = target
		child.marker = marker
		child.restore_log = log
		child.capture_log = captures
	var run := _start(root)
	check(captures == ["0", "1", "2"], "Every custom child captures before playback")
	check(target.position != rest, "Custom reset is allowed to mutate after capture")
	run.custom_step(0.2)
	root.stop_animation_preview()
	check(log == ["2", "1", "0"], "Custom restorers run in reverse child order")
	check(target.position.is_equal_approx(rest), "Custom callback restores state captured before reset")
	root.stop_animation_preview()
	check(log.size() == 3, "Repeated stop restores exactly once")
	log.clear()
	captures.clear()
	root.kind = Spire.BLUEPRINT_KIND_PARALLEL_BLOCK
	run = _start(root)
	run.custom_step(0.2)
	root.stop_animation_preview()
	check(captures == ["0", "1", "2"] and log == ["2", "1", "0"], "Parallel Block captures first and restores callbacks in reverse order")
	check(target.position.is_equal_approx(rest), "Parallel custom callbacks restore pre-reset baseline")
	root.get_child(1).restore_state = false
	for kind in [Spire.BLUEPRINT_KIND_CHAIN_BLOCK, Spire.BLUEPRINT_KIND_PARALLEL_BLOCK]:
		root.kind = kind
		log.clear()
		captures.clear()
		run = _start(root)
		check(run != null and target.position != rest, "A child with nothing to restore still participates in preview")
		if run != null: run.custom_step(0.2)
		root.stop_animation_preview()
		check(captures == ["0", "1", "2"] and log == ["2", "0"], "Containers skip invalid callbacks and restore valid siblings in reverse order")
		check(target.position.is_equal_approx(rest), "Valid sibling callbacks still restore their captured state")
	for child in root.get_children(): child.restore_state = false
	for kind in [Spire.BLUEPRINT_KIND_CHAIN_BLOCK, Spire.BLUEPRINT_KIND_PARALLEL_BLOCK]:
		root.kind = kind
		log.clear()
		run = _start(root)
		check(run != null, "Composition with no restoration actions can preview")
		if run != null: run.custom_step(0.2)
		var pose := target.position
		root.stop_animation_preview()
		check(log.is_empty() and target.position == pose, "Composition with no restorers leaves its final pose unchanged")
		target.position = rest
	root.free()
	var invalid := _bp(scene, &"does_not_exist", 1.0)
	run = _start(invalid)
	check(run != null and target.position.is_equal_approx(rest), "Invalid property is skipped by playback without target writes")
	invalid.stop_animation_preview()
	invalid.free()
	var restoreless := Custom.new()
	scene.add_child(restoreless)
	restoreless.target = target
	restoreless.restore_state = false
	restoreless.restore_log = log
	restoreless.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	log.clear()
	run = _start(restoreless)
	check(run != null, "Standalone invalid restoration callback does not refuse preview")
	EditorInterface.edit_node(restoreless)
	await _frames()
	var stop := _button(&"stop_preview_button")
	check(stop != null and not stop.disabled, "A preview with no restorer still enables Stop")
	if run != null:
		run.custom_step(0.2)
		var pose := target.position
		restoreless.stop_animation_preview()
		check(run.is_stopped() and target.position == pose and log.is_empty(), "Standalone Stop skips invalid restoration callback")
		run = _start(restoreless)
		run.force_complete()
		check(target.position == Vector2(200, 140) and log.is_empty(), "Standalone completion with no restorer keeps its endpoint")
		target.position = rest
		run = _start(restoreless)
		run.custom_step(0.2)
		pose = target.position
		var next := _bp(scene)
		var replacement := _start(next)
		check(run.is_stopped() and target.position == pose, "Replacement stops a preview that has no restoration action")
		if replacement != null: replacement.custom_step(0.2)
		next.stop_animation_preview()
		check(target.position.is_equal_approx(pose), "Next preview captures the pose left by a restoreless run")
		next.free()
	restoreless.free()
	target.position = rest
	var uncaptured := Uncaptured.new()
	scene.add_child(uncaptured)
	uncaptured.target = target
	run = _start(uncaptured)
	check(run != null and target.position == rest + Vector2(100, 0), "Default script capture has nothing to restore and still runs its builder")
	uncaptured.stop_animation_preview()
	check(target.position == rest + Vector2(100, 0), "Default script capture performs no restoration on Stop")
	uncaptured.free()
	target.position = rest
	var bp := _bp(scene)
	run = _start(bp)
	run.custom_step(0.2)
	bp.free()
	check(run.is_stopped() and target.position.is_equal_approx(rest), "Freeing preview root restores surviving target")
	bp = _bp(scene)
	var removed := Node2D.new()
	scene.add_child(removed)
	bp.target = removed
	run = _start(bp)
	run.custom_step(0.2)
	removed.free()
	bp.stop_animation_preview()
	check(run.is_stopped(), "Restoration skips freed targets")
	bp.free()

func _native_text() -> void:
	var label := SpireRichTextLabel.new()
	label.bbcode_enabled = true
	label.text = "[color=red]AB[/color] C"
	label.position = Vector2(140, 180)
	label.size = Vector2(320, 90)
	label.add_theme_font_size_override(&"normal_font_size", 48)
	scene.add_child(label)
	await _frames(6)
	check(label.get_character_count() == 4, "Editor prepares character state before capture")
	label.set_char_offset(1, Vector2(3, 7))
	label.set_char_pivot(2, Vector2(5, 9))
	var rest := _characters(label)
	var bp := _bp(scene)
	bp.target = label
	bp.kind = Spire.BLUEPRINT_KIND_TEXT
	bp.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	var effect := SpireTextEffect.new()
	effect.kind = Spire.TEXT_EFFECT_KIND_COLOR
	effect.to = Color.GREEN
	effect.duration = 1.0
	bp.text_effect = effect
	EditorInterface.edit_node(bp)
	await _frames(4)
	var run := _start(bp)
	run.custom_step(0.4)
	check(_characters(label) != rest, "Native Text preview animates real character state")
	await _frames(3)
	_screenshot("text-preview-playing")
	bp.stop_animation_preview()
	check(_characters(label) == rest, "Native Text restores pre-existing channels and BBCode color")
	await _frames(3)
	_screenshot("text-preview-restored")
	var restore := label.capture_character_rest_state(0, -1)
	label.set_char_color(0, Color.BLUE)
	restore.call()
	check(_characters(label) == rest, "Public character capture is reusable by custom scripts")
	check(not label.capture_character_rest_state(-1, -1).is_valid(), "Invalid character capture range is rejected")
	var inherited := Color(0.2, 0.7, 0.4)
	label.add_theme_color_override(&"default_color", inherited)
	await _frames(4)
	check(label.get_char_color(3).is_equal_approx(inherited), "Restored inherited color follows later theme changes")
	check(label.get_char_color(0).is_equal_approx(Color.RED), "Restored BBCode color still overrides the theme")
	var old_restore := label.capture_character_rest_state(0, -1)
	label.text = "Changed source"
	check(not label.capture_character_rest_state(0, 0).is_valid(), "Capture refuses unsynchronized text without refreshing or changing it")
	var pending_restore := label.capture_character_rest_state(0, -1)
	check(pending_restore.is_valid() and not label.capture_character_rest_state(0, 0).is_valid(), "Whole-label capture provides a fallback without initializing pending text")
	label.refresh()
	label.set_char_offset(0, Vector2(99, 88))
	old_restore.call()
	check(label.get_char_offset(0) == Vector2(99, 88), "Old character snapshot does not overwrite rebuilt data")
	pending_restore.call()
	check(label.get_char_offset(0) == Vector2.ZERO, "Pending-text callback refreshes and removes preview deformations synchronously")
	bp.free()
	label.text = "e\u0301"
	label.refresh()
	await _frames(3)
	label.set_char_offset(1, Vector2(4, 8))
	var cluster_rest := _characters(label)
	var cluster_restore := label.capture_character_rest_state(1, 1)
	label.refresh()
	label.set_char_offset(0, Vector2(99, 88))
	cluster_restore.call()
	check(_characters(label) == cluster_rest, "Partial character capture restores a shared glyph root outside the selection")
	# Text replacement is an animation too: restoring its authored text must
	# recover pre-existing character state, regardless of callback ordering.
	for kind in [Spire.BLUEPRINT_KIND_CHAIN_BLOCK, Spire.BLUEPRINT_KIND_PARALLEL_BLOCK]:
		for text_first in [true, false]:
			label.text = "e\u0301 ffi Original"
			label.refresh()
			await _frames(3)
			label.set_char_offset(1, Vector2(4, 8))
			var original_chars := _characters(label)
			var root := TweenBlueprint.new()
			scene.add_child(root)
			root.kind = kind
			root.tween_process_mode = Spire.PROCESS_MODE_MANUAL
			var replacement := _bp(root, &"text", "Replacement")
			replacement.target = label
			replacement.property_speed_or_duration = 0.1
			var animation := _bp(root)
			animation.target = label
			animation.kind = Spire.BLUEPRINT_KIND_TEXT
			animation.text_effect = effect
			if not text_first: root.move_child(animation, 0)
			run = _start(root)
			run.custom_step(0.3 if text_first or kind == Spire.BLUEPRINT_KIND_PARALLEL_BLOCK else 2.05)
			await _frames(3)
			root.stop_animation_preview()
			check(label.text == "e\u0301 ffi Original" and _characters(label) == original_chars, "Composed text replacement restores source, shaped glyph aliases and exact original character state")
			root.free()
	label.free()
	old_restore.call()
	var uninitialized := SpireRichTextLabel.new()
	uninitialized.text = "Not ready"
	var initialize_restore := uninitialized.capture_character_rest_state(0, -1)
	check(initialize_restore.is_valid(), "Uninitialized whole-label capture returns a refresh callback")
	check(not uninitialized.capture_character_rest_state(0, 0).is_valid(), "Creating the fallback does not initialize the label")
	uninitialized.refresh()
	uninitialized.set_char_offset(0, Vector2(11, 12))
	initialize_restore.call()
	check(uninitialized.get_char_offset(0) == Vector2.ZERO, "Uninitialized-label callback restores defaults after playback initialized it")
	uninitialized.free()
	initialize_restore.call()
	var empty := SpireRichTextLabel.new()
	scene.add_child(empty)
	var empty_restore := empty.capture_character_rest_state(0, -1)
	check(empty_restore.is_valid(), "Initialized empty labels have a valid whole-label restorer")
	check(not empty.capture_character_rest_state(0, 0).is_valid(), "Empty labels reject explicit character indices")
	empty.text = "Temporary"
	empty.refresh()
	empty.text = ""
	empty_restore.call()
	check(empty.get_character_count() == 0, "Empty-label callback synchronizes the restored source without caller refresh")
	var replacement := _bp(scene, &"text", "Replacement")
	replacement.target = empty
	replacement.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	var empty_run := _start(replacement)
	check(empty_run != null, "Property preview starts from an empty label")
	if empty_run != null:
		empty_run.custom_step(0.4)
		await _frames(3)
		check(empty.get_character_count() > 0, "Text replacement initializes temporary characters")
		replacement.stop_animation_preview()
		check(empty.text == "" and empty.get_character_count() == 0, "Property callback restores empty text and derived state synchronously")
	replacement.free()
	empty.free()

func _templates() -> void:
	var packed := load("res://examples/effect_templates/gallery.tscn") as PackedScene
	var gallery := packed.instantiate()
	scene.add_child(gallery)
	await _frames(8)
	for card in gallery.get_node("Scroll/Grid").get_children():
		var node := card.get_node("Content/TargetSlot/Target")
		var bp: TweenBlueprint = node.get_node("Effect")
		bp.tween_process_mode = Spire.PROCESS_MODE_MANUAL
		var rest := _pose(node)
		var chars := _characters(node) if node is SpireRichTextLabel else []
		var run := _start(bp)
		check(run != null, "Template supports preview: " + str(bp.scene_file_path))
		if run == null: continue
		run.custom_step(0.13)
		var replay := _start(bp)
		check(run.is_stopped() and _pose(node) == rest and (chars.is_empty() or _characters(node) == chars), "Template replacement restores before capture: " + str(bp.scene_file_path))
		replay.custom_step(0.09)
		bp.stop_animation_preview()
		check(_pose(node) == rest and (chars.is_empty() or _characters(node) == chars), "Interrupted template replay has no drift: " + str(bp.scene_file_path))
		replay = _start(bp)
		replay.force_complete()
		check(_pose(node) == rest and (chars.is_empty() or _characters(node) == chars), "Template completion restores rest state: " + str(bp.scene_file_path))
	gallery.free()

func _scene_switch() -> void:
	var rest := target.position
	var bp := _bp(scene)
	var run := _start(bp)
	run.custom_step(0.3)
	EditorInterface.open_scene_from_path("res://other_scene.tscn")
	await _frames(6)
	check(run.is_stopped() and target.position.is_equal_approx(rest), "Switching scene tabs stops and restores outgoing preview")
	EditorInterface.open_scene_from_path("res://preview_saved.tscn")
	await _frames(6)
	check(run.is_stopped() and target.position.is_equal_approx(rest), "Returning to scene does not resume old preview")
	bp.free()
