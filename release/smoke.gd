extends Node

const TemplateCatalog := preload("res://smoke_templates.gd")


func _check(condition: bool, message: String) -> bool:
	if not condition:
		push_error("SPIRE_RELEASE_SMOKE_FAILED " + message)
	return condition


func _snapshot(target: Node) -> Array:
	if target is SpireRichTextLabel:
		var values: Array = []
		for index in target.get_character_count():
			values.append_array([target.get_char_offset(index), target.get_char_scale(index),
				target.get_char_rotation(index), target.get_char_pivot(index), target.get_char_color(index)])
		return values
	return [target.position, target.scale, target.rotation, target.modulate]


func _same(a: Array, b: Array) -> bool:
	if a.size() != b.size(): return false
	for index in a.size():
		match typeof(a[index]):
			TYPE_FLOAT:
				if not is_equal_approx(a[index], b[index]): return false
			TYPE_VECTOR2, TYPE_VECTOR3, TYPE_COLOR:
				if not a[index].is_equal_approx(b[index]): return false
			_:
				if a[index] != b[index]: return false
	return true


func _exercise_template(entry: Dictionary) -> bool:
	var path: String = "res://addons/spire_tween/templates/" + entry.scene
	var scene := load(path) as PackedScene
	if not _check(scene != null, "template scene cannot load: " + path): return false
	var effect := scene.instantiate() as TweenBlueprint
	if not _check(effect != null, "template root is not a TweenBlueprint: " + path): return false
	# A missing GDScript can leave a valid native node behind. Require the script
	# and its inherited methods before treating instantiation as successful.
	if not _check(effect.get_script() != null and effect.has_method("get_template_warnings"),
		"template script cannot load: " + path):
		effect.free()
		return false
	var target: Node
	if entry.text:
		var label := SpireRichTextLabel.new()
		label.text = "SPIRE TWEEN"
		label.size = Vector2(500, 150)
		label.add_theme_color_override("default_color", Color.WHITE)
		target = label
	else:
		target = Node2D.new()
	add_child(target)
	target.add_child(effect)
	effect.duration = 0.1
	effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	var warnings: PackedStringArray = effect.get_template_warnings()
	if not _check(warnings.is_empty(), "invalid template: " + path + " " + str(warnings)):
		target.free()
		return false
	var original := _snapshot(target)
	var sequence := effect.play()
	var changed := false
	for step in [0.017, 0.029, 0.031, 0.023]:
		sequence.custom_step(step)
		changed = changed or not _same(original, _snapshot(target))
	var valid := _check(changed, "template did not animate: " + path)
	effect.restore()
	valid = _check(_same(original, _snapshot(target)), "template did not restore: " + path) and valid
	target.free()
	return valid


func _ready() -> void:
	var target := Node2D.new()
	add_child(target)
	var calls := [0]
	var seq := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
	seq.append(DoNode2D.position(target, Vector2(10.0, 20.0), 0.1))
	seq.append_call(func(): calls[0] += 1)
	seq.custom_step(0.2)
	if not target.position.is_equal_approx(Vector2(10.0, 20.0)) or calls[0] != 1:
		push_error("SPIRE_RELEASE_SMOKE_FAILED")
		get_tree().quit(1)
		return
	# Also exercise cancellation through the actual installed bridge.
	var invalid := Spire.do_delayed_call(Callable(), 0.1)
	invalid.set_process_mode(Spire.PROCESS_MODE_MANUAL)
	if invalid.custom_step(0.2):
		push_error("SPIRE_RELEASE_SMOKE_FAILED invalid callable stayed alive")
		get_tree().quit(1)
		return
	seq = null
	invalid = null
	target.queue_free()
	for entry in TemplateCatalog.ENTRIES:
		if not _exercise_template(entry):
			get_tree().quit(1)
			return
	for frame in range(4):
		await get_tree().process_frame
	print("SPIRE_RELEASE_TEMPLATES_OK count=", TemplateCatalog.ENTRIES.size())
	print("SPIRE_RELEASE_SMOKE_OK")
	get_tree().quit(0)
