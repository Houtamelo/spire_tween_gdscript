extends SceneTree

const TEMPLATES := "res://addons/spire_tween/templates/"
var failures: Array[String] = []
var total := 0
var stage: Control
var output := ""
var completed := false


func _initialize() -> void:
	for argument in OS.get_cmdline_user_args():
		if argument.begins_with("--output="): output = argument.trim_prefix("--output=")
	_run.call_deferred()


func check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		push_error("SCENE_TEMPLATE_ASSERTION: " + message)


func _draw() -> void:
	await process_frame
	await RenderingServer.frame_post_draw


func _snapshot(node: Node) -> Dictionary:
	if node is SpireRichTextLabel:
		var values := {}
		for index in node.get_character_count():
			for channel in [&"offset", &"scale", &"rotation", &"pivot", &"alpha", &"color"]:
				values[str(index) + "/" + channel] = node.call("get_char_" + channel, index)
		return values
	return {"position": node.position, "scale": node.scale, "rotation": node.rotation, "modulate": node.modulate}


func _same(a: Dictionary, b: Dictionary) -> bool:
	if a.size() != b.size(): return false
	for key in a:
		var value: Variant = a[key]
		if value is float:
			if not is_equal_approx(value, b[key]): return false
		elif value is Vector2 or value is Vector3 or value is Color:
			if not value.is_equal_approx(b[key]): return false
		elif value != b[key]: return false
	return true


func _target(text: bool) -> Node:
	var node: Node
	if text:
		var label := SpireRichTextLabel.new()
		label.text = "[color=#67e8c2]SPIRE[/color] TWEEN"
		label.bbcode_enabled = true
		label.scroll_active = false
		label.size = Vector2(520, 130)
		label.position = Vector2(100, 100)
		label.add_theme_font_size_override("normal_font_size", 48)
		node = label
	else:
		var label := Label.new()
		label.text = "SPIRE TWEEN"
		label.size = Vector2(520, 130)
		label.position = Vector2(100, 100)
		label.pivot_offset = Vector2(150, 35)
		label.add_theme_font_size_override("font_size", 48)
		node = label
	stage.add_child(node)
	return node


func _advance(sequence: SpireSequence, seconds: float) -> void:
	for _tick in ceili(seconds / 0.01):
		sequence.custom_step(0.01)


func _exercise(entry: Dictionary, index: int) -> void:
	var before_failures := failures.size()
	var scene := load(TEMPLATES + entry.scene) as PackedScene
	check(scene != null, entry.scene + ": scene loads")
	if scene == null: return
	var node := _target(entry.text)
	var effect := scene.instantiate() as TweenBlueprint
	check(effect != null and effect.get_script() != null, entry.scene + ": GDScript Blueprint root")
	if effect == null:
		node.free()
		return
	node.add_child(effect)
	effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	effect.duration = 0.3
	if entry.text:
		effect.first_character = 1
		effect.last_character = 5
	await _draw()
	check(effect.get_template_warnings().is_empty(), entry.scene + ": default configuration is valid")
	var original := _snapshot(node)
	var baseline_image := root.get_texture().get_image()
	var sequence := effect.play()
	check(_same(original, _snapshot(node)), entry.scene + ": building does not modify targets")
	var rendered_change := false
	var moving_image: Image
	for seconds in [0.07, 0.06, 0.08]:
		_advance(sequence, seconds)
		await _draw()
		moving_image = root.get_texture().get_image()
		rendered_change = rendered_change or baseline_image.get_data() != moving_image.get_data()
	check(not _same(original, _snapshot(node)), entry.scene + ": animation modifies its channel")
	check(rendered_change, entry.scene + ": real rendered glyphs visibly change")
	if not output.is_empty():
		check(moving_image.save_png(output.path_join("%02d-%s.png" % [index, entry.scene.replace("/", "-").trim_suffix(".tscn")])) == OK, entry.scene + ": screenshot saved")
	if entry.text:
		var now := _snapshot(node)
		for key: String in original:
			if key.begins_with("0/") or key.begins_with("6/"):
				check(original[key] == now[key], entry.scene + ": characters outside selection unchanged")
	effect.stop()
	var stopped := _snapshot(node)
	_advance(sequence, 0.2)
	check(_same(stopped, _snapshot(node)), entry.scene + ": stop preserves pose")
	effect.restore()
	check(_same(original, _snapshot(node)), entry.scene + ": restore returns captured channels")
	sequence = effect.play()
	_advance(sequence, 0.12)
	var replacement := effect.play()
	check(sequence.get_state() == Spire.STATE_STOPPED, entry.scene + ": replay stops prior handle")
	check(_same(original, _snapshot(node)), entry.scene + ": interrupted replay restores baseline without drift")
	_advance(replacement, 4.0)
	if entry.looping:
		check(replacement.get_state() == Spire.STATE_PLAYING, entry.scene + ": continuous template remains playing")
	else:
		check(replacement.get_state() == Spire.STATE_STOPPED, entry.scene + ": finite template completes")
		if entry.family == "text_emphasis":
			# Pivot is authored by rotating/scaling effects; verify channel recovery.
			var recovered := _snapshot(node)
			for key: String in original:
				if not key.ends_with("/pivot"):
					check(_same({key: original[key]}, {key: recovered[key]}), entry.scene + ": emphasis recovers " + key)
		if entry.family == "interaction" and effect.style != 0:
			check(_same(original, _snapshot(node)), entry.scene + ": press/selection returns to baseline")
	effect.restore()
	check(_same(original, _snapshot(node)), entry.scene + ": completed/looping run restores")
	if entry.family.ends_with("entrances"):
		effect.exiting = true
		sequence = effect.play()
		_advance(sequence, 4.0)
		check(sequence.get_state() == Spire.STATE_STOPPED, entry.scene + ": exit completes")
		if entry.text:
			check(is_zero_approx(node.get_char_alpha(3)), entry.scene + ": exit hides selected glyphs")
		else:
			check(is_zero_approx(node.modulate.a), entry.scene + ": exit hides target")
		effect.restore()
		check(_same(original, _snapshot(node)), entry.scene + ": exit restores")
	# Verify scene persistence after playback; transient snapshots are not stored.
	var packed := PackedScene.new()
	check(packed.pack(effect) == OK, entry.scene + ": scene packs after playback")
	var saved := "user://template-%d.tscn" % index
	check(ResourceSaver.save(packed, saved) == OK, entry.scene + ": scene saves")
	var reloaded := (load(saved) as PackedScene).instantiate()
	check(is_equal_approx(reloaded.duration, 0.3) and reloaded.style == effect.style, entry.scene + ": exported controls persist")
	reloaded.free()
	# Every template handles invalid input without animating a target.
	effect.duration = NAN
	check(not effect.get_template_warnings().is_empty(), entry.scene + ": invalid duration is reported")
	var invalid := effect.play()
	_advance(invalid, 0.05)
	check(_same(original, _snapshot(node)), entry.scene + ": invalid run does not modify target")
	node.free()
	await process_frame
	total += 1
	print("SCENE_TEMPLATE_CASE ", entry.scene, " failed=", failures.size() - before_failures)


func _special_cases() -> void:
	var text_scene := load(TEMPLATES + "text_entrances/slide.tscn") as PackedScene
	var label := _target(true) as SpireRichTextLabel
	var effect := text_scene.instantiate()
	label.add_child(effect)
	await _draw()
	effect.first_character = 3
	effect.last_character = 1
	check(str(effect.get_template_warnings()).contains("`first_character`"), "Invalid character range is reported")
	effect.first_character = 0
	effect.last_character = -1
	var curve := Curve.new()
	curve.min_value = -1.0
	curve.add_point(Vector2(0, -1))
	effect.start_delay_curve = curve
	check(str(effect.get_template_warnings()).contains("`start_delay_curve`"), "Negative curve samples are reported")
	effect.start_delay_curve = null
	effect.duration = 0.2
	effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	curve = Curve.new()
	curve.max_value = 2.0
	curve.add_point(Vector2.ZERO)
	curve.add_point(Vector2(1, 2))
	effect.start_delay_curve = curve
	effect.last_character = 2
	var delayed: SpireSequence = effect.play()
	_advance(delayed, 0.3)
	check(is_equal_approx(label.get_char_alpha(0), 1.0) and is_zero_approx(label.get_char_alpha(2)), "Custom curve delays final character beyond the first leg")
	check(delayed.get_state() == Spire.STATE_PLAYING, "Range remains active until delayed characters finish")
	_advance(delayed, 0.4)
	check(delayed.get_state() == Spire.STATE_STOPPED, "Custom-curve range eventually completes")
	effect.restore()
	effect.start_delay_curve = null
	effect.target = stage
	check(str(effect.get_template_warnings()).contains("SpireRichTextLabel"), "Incompatible text target is reported")
	label.free()
	var container := HBoxContainer.new()
	stage.add_child(container)
	var control := Control.new()
	container.add_child(control)
	effect = (load(TEMPLATES + "canvas_entrances/slide.tscn") as PackedScene).instantiate()
	control.add_child(effect)
	check(str(effect.get_template_warnings()).contains("Container"), "Layout-owned target is reported")
	container.free()
	# Native serial composition: baseline must be captured after the earlier step.
	var canvas := _target(false)
	var original := canvas.position as Vector2
	var composition := TweenBlueprint.new()
	composition.kind = Spire.BLUEPRINT_KIND_CHAIN_BLOCK
	composition.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	stage.add_child(composition)
	var leading := TweenBlueprint.new()
	leading.kind = Spire.BLUEPRINT_KIND_PROPERTY
	leading.target = canvas
	leading.property_to_animate = &"position"
	leading.property_end_value = original + Vector2(35, 12)
	leading.property_speed_or_duration = 0.3
	composition.add_child(leading)
	effect = (load(TEMPLATES + "canvas_entrances/slide.tscn") as PackedScene).instantiate()
	effect.target = canvas
	effect.duration = 0.2
	composition.add_child(effect)
	var sequence := composition.play()
	check(canvas.position == original, "Composed construction does not mutate targets")
	_advance(sequence, 0.1)
	check(canvas.modulate.a == 1.0, "Later template does not initialize before its step")
	_advance(sequence, 1.0)
	check(canvas.position.is_equal_approx(original + Vector2(35, 12)), "Template captures post-predecessor baseline")
	effect.restore()
	canvas.free()
	# Invalid edits made while a delayed template is queued must be revalidated
	# before its lazy builder initializes any target channel.
	canvas = _target(false)
	effect = (load(TEMPLATES + "canvas_entrances/slide.tscn") as PackedScene).instantiate()
	canvas.add_child(effect)
	effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	effect.delay = 0.2
	var waiting_baseline := _snapshot(canvas)
	sequence = effect.play()
	effect.duration = NAN
	_advance(sequence, 0.8)
	check(_same(waiting_baseline, _snapshot(canvas)), "Queued configuration is validated again before initialization")
	canvas.free()
	composition.free()
	# Node2D support and non-default transform/color baselines on every canvas scene.
	var catalog: Array = JSON.parse_string(FileAccess.get_file_as_string(TEMPLATES + "catalog.json"))
	for entry in catalog:
		if entry.text: continue
		var sprite := Polygon2D.new()
		sprite.polygon = PackedVector2Array([Vector2.ZERO, Vector2(80, 0), Vector2(80, 45), Vector2(0, 45)])
		sprite.position = Vector2(20, 40)
		sprite.scale = Vector2(1.4, 0.8)
		sprite.rotation = 0.2
		sprite.modulate = Color(0.8, 0.9, 1.0, 0.75)
		stage.add_child(sprite)
		effect = (load(TEMPLATES + entry.scene) as PackedScene).instantiate()
		sprite.add_child(effect)
		effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
		effect.duration = 0.3
		check(effect.get_template_warnings().is_empty(), entry.scene + ": Node2D target is valid")
		var baseline := _snapshot(sprite)
		sequence = effect.play()
		_advance(sequence, 0.12)
		check(not _same(baseline, _snapshot(sprite)), entry.scene + ": animates Node2D")
		effect.restore()
		check(_same(baseline, _snapshot(sprite)), entry.scene + ": restores non-default Node2D baseline")
		sprite.free()
	# Repeated completed hover/focus events must keep the same held baseline.
	canvas = _target(false)
	effect = (load(TEMPLATES + "interaction/hover_focus.tscn") as PackedScene).instantiate()
	canvas.add_child(effect)
	effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	for _entry in 3:
		sequence = effect.play()
		_advance(sequence, 0.5)
		check(canvas.scale.is_equal_approx(Vector2(1.06, 1.06)), "Repeated hover/focus does not compound scale")
	effect.restore()
	canvas.free()
	# A retained handle tolerates its target disappearing during playback.
	canvas = _target(false)
	effect = (load(TEMPLATES + "canvas_continuous/pulse.tscn") as PackedScene).instantiate()
	canvas.add_child(effect)
	effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	sequence = effect.play()
	_advance(sequence, 0.1)
	canvas.free()
	_advance(sequence, 0.2)
	check(sequence.get_state() == Spire.STATE_STOPPED, "Target deletion stops retained handle")
	# The shipped gallery uses editable, valid instances and real Button signals.
	var gallery := (load("res://examples/effect_templates/gallery.tscn") as PackedScene).instantiate()
	stage.add_child(gallery)
	await _draw()
	var cards := get_nodes_in_group("effect_template_card")
	check(cards.size() == 33, "Gallery contains all 33 template instances")
	for card in cards:
		var template: TweenBlueprint = card.get_node("Content/TargetSlot/Target/Effect")
		check(template.get_template_warnings().is_empty(), "Gallery configuration valid: " + str(template.scene_file_path))
		card.get_node("Content/Buttons/Play").pressed.emit()
		card.get_node("Content/Buttons/Restore").pressed.emit()
	var second_gallery := (load("res://examples/effect_templates/gallery.tscn") as PackedScene).instantiate()
	stage.add_child(second_gallery)
	await _draw()
	check(get_nodes_in_group("effect_template_card").size() == 66, "Two gallery instances coexist")
	for own_gallery in [gallery, second_gallery]:
		var card: Node = own_gallery.get_node("Scroll/Grid/Card00")
		card.get_node("Content/Buttons/Play").pressed.emit()
		card.get_node("Content/Buttons/Restore").pressed.emit()
	second_gallery.queue_free()
	gallery.queue_free()
	await process_frame


func _run() -> void:
	create_timer(75.0).timeout.connect(func():
		if not completed:
			push_error("Scene template tests timed out")
			quit(1))
	stage = Control.new()
	root.add_child(stage)
	root.size = Vector2i(800, 300)
	var entries: Array = JSON.parse_string(FileAccess.get_file_as_string(TEMPLATES + "catalog.json"))
	check(entries.size() == 33, "Catalog contains the agreed 33 templates")
	for index in entries.size():
		await _exercise(entries[index], index)
	await _special_cases()
	completed = true
	print("SCENE_TEMPLATE_RESULT total=", total, " failed=", failures.size())
	quit(0 if failures.is_empty() else 1)
