extends "res://examples/tests/test_base.gd"


func _manual(tween):
	assert_true(Spire.is_registered(tween))
	tween.set_process_mode(Spire.PROCESS_MODE_MANUAL)
	if tween.has_method("set_ease"):
		tween.set_ease(Spire.EASE_LINEAR)
	return tween


func _near(a: float, b: float) -> void:
	assert_true(abs(a - b) < 0.0001, "%s != %s" % [a, b])


func _v2(a: Vector2, b: Vector2) -> void:
	_near(a.distance_to(b), 0.0)


func _v3(a: Vector3, b: Vector3) -> void:
	_near(a.distance_to(b), 0.0)


func test_group2_jump_2d() -> bool:
	var node := Node2D.new()
	add_child(node)
	var tween: SpireMethodFloat = _manual(DoNode2D.jump(node, Vector2(10, 10), 4, 1, 1))
	node.position = Vector2(2, 2)
	tween.custom_step(0.25)
	_v2(node.position, Vector2(4, 2.5))
	tween.custom_step(0.25)
	_v2(node.position, Vector2(6, 4))
	tween.force_complete()
	_v2(node.position, Vector2(10, 10))
	node.free()
	return true


func test_group2_jump_3d_and_global() -> bool:
	var parent := Node3D.new()
	var node := Node3D.new()
	add_child(parent)
	parent.position = Vector3(10, 20, 30)
	parent.add_child(node)
	var start := node.global_position
	var target := start + Vector3(8, 8, 4)
	var tween: SpireMethodFloat = _manual(DoNode3D.global_jump(node, target, 4, 2, 1))
	tween.custom_step(0.25)
	_v3(node.global_position, start + Vector3(2, 7.5, 1))
	tween.force_complete()
	_v3(node.global_position, target)
	parent.free()
	return true


func test_group2_jump_ui_and_loops() -> bool:
	var node := Control.new()
	add_child(node)
	var tween: SpireMethodFloat = _manual(DoControl.jump(node, Vector2(8, 8), 4, 0, 1))
	tween.set_loops(2, Spire.LOOP_MODE_YOYO)
	tween.custom_step(1.0)
	_v2(node.position, Vector2(8, 8))
	tween.custom_step(0.5)
	_v2(node.position, Vector2(4, 2))
	tween.custom_step(0.5)
	_v2(node.position, Vector2.ZERO)
	var replay: SpireMethodFloat = _manual(DoControl.jump(node, Vector2(8, 8), 4, 2, 1))
	replay.set_loops(2, Spire.LOOP_MODE_RESTART)
	replay.custom_step(1.0)
	replay.custom_step(0.25)
	_v2(node.position, Vector2(2, -0.5))
	replay.force_complete()
	node.free()
	return true


func test_group2_scales_and_blendables() -> bool:
	var node := Node2D.new()
	var ui := Control.new()
	var node3 := Node3D.new()
	for owner in [node, ui, node3]: add_child(owner)
	var scales := [DoNode2D.uniform_scale(node, 3, 1), DoControl.uniform_scale(ui, 3, 1), DoNode3D.uniform_scale(node3, 3, 1)]
	for tween in scales: _manual(tween).custom_step(0.5)
	_v2(node.scale, Vector2(2, 2))
	_v2(ui.scale, Vector2(2, 2))
	_v3(node3.scale, Vector3(2, 2, 2))
	var a = _manual(DoNode2D.blendable_position(node, Vector2(4, 6), 1))
	var b = _manual(DoNode2D.blendable_position(node, Vector2(2, 4), 1))
	a.custom_step(0.5)
	b.custom_step(0.5)
	_v2(node.position, Vector2(3, 5))
	node.position = Vector2(20, 30)
	a.force_complete()
	b.force_complete()
	_v2(node.position, Vector2(23, 35))
	_manual(DoControl.blendable_global_position(ui, Vector2(3, 4), 1)).force_complete()
	_v2(ui.global_position, Vector2(3, 4))
	_manual(DoNode3D.blendable_scale(node3, Vector3(1, 2, 3), 1)).force_complete()
	_v3(node3.scale, Vector3(3, 4, 5))
	for tween in scales: tween.stop()
	for owner in [node, ui, node3]: owner.free()
	return true


func test_group2_blendable_colors() -> bool:
	var node := Node2D.new()
	add_child(node)
	node.modulate = Color(0.2, 0.2, 0.2, 1)
	var a = _manual(DoCanvasItem.blendable_modulate(node, Color(0.4, 0.2, 0.2, 1), 1))
	var b = _manual(DoCanvasItem.blendable_modulate(node, Color(0.2, 0.6, 0.2, 1), 1))
	node.modulate = Color(0.3, 0.3, 0.3, 0.5)
	a.force_complete()
	b.force_complete()
	_near(node.modulate.r, 0.5)
	_near(node.modulate.g, 0.7)
	_near(node.modulate.a, 0.5)
	node.free()
	return true


func test_group2_gradient_sampling() -> bool:
	var gradient := Gradient.new()
	gradient.set_color(0, Color(0, 0, 0, 0.2))
	gradient.set_color(1, Color(1, 1, 1, 0.8))
	var node := Node2D.new()
	add_child(node)
	node.modulate = Color(0.1, 0.2, 0.3, 0.9)
	var rgb = _manual(DoCanvasItem.gradient_modulate(node, gradient, 1, false))
	rgb.custom_step(0.5)
	_near(node.modulate.r, 0.5)
	_near(node.modulate.a, 0.9)
	rgb.stop()
	var rgba = _manual(DoCanvasItem.gradient_modulate(node, gradient, 1))
	gradient.interpolation_mode = Gradient.GRADIENT_INTERPOLATE_CONSTANT
	rgba.custom_step(0.5)
	_near(node.modulate.r, 0)
	_near(node.modulate.a, 0.2)
	rgba.force_complete()
	_near(node.modulate.r, 1)
	_near(node.modulate.a, 0.8)
	assert_eq(DoCanvasItem.gradient_modulate(node, null, 1), null)
	node.free()
	return true


func test_group2_color_adapters() -> bool:
	var sprite := Sprite3D.new()
	var label := Label3D.new()
	var light := OmniLight3D.new()
	for owner in [sprite, label, light]: add_child(owner)
	var material := StandardMaterial3D.new()
	var gradient := Gradient.new()
	gradient.set_color(1, Color(0.2, 0.4, 0.6, 1))
	var tweens := [
		DoSpriteBase3D.gradient_modulate(sprite, gradient, 1),
		DoLabel3D.gradient_modulate(label, gradient, 1),
		DoLight3D.gradient_light_color(light, gradient, 1),
		DoBaseMaterial3D.gradient_albedo_color(material, gradient, 1),
	]
	for tween in tweens: _manual(tween).force_complete()
	_near(sprite.modulate.r, 0.2)
	_near(label.modulate.g, 0.4)
	_near(light.light_color.b, 0.6)
	_near(material.albedo_color.r, 0.2)
	var blends := [
		DoSpriteBase3D.blendable_modulate(sprite, Color(0.4, 0.4, 0.6, 1), 1),
		DoLabel3D.blendable_modulate(label, Color(0.2, 0.6, 0.6, 1), 1),
		DoLight3D.blendable_light_color(light, Color(0.2, 0.4, 0.8, 1), 1),
		DoBaseMaterial3D.blendable_albedo_color(material, Color(0.4, 0.4, 0.6, 1), 1),
	]
	for tween in blends: _manual(tween).force_complete()
	_near(sprite.modulate.r, 0.4)
	_near(label.modulate.g, 0.6)
	_near(light.light_color.b, 0.8)
	_near(material.albedo_color.r, 0.4)
	for owner in [sprite, label, light]: owner.free()
	return true


func test_group2_resource_properties() -> bool:
	var env := Environment.new()
	env.background_mode = Environment.BG_SKY
	var tween = _manual(DoEnvironment.background_color(env, Color(1, 0, 0, 0.5), 1))
	env.background_color = Color(0, 0, 0, 1)
	tween.custom_step(0.5)
	_near(env.background_color.r, 0.5)
	assert_eq(env.background_mode, Environment.BG_SKY)
	tween.force_complete()
	var material := StandardMaterial3D.new()
	material.albedo_color = Color(0.2, 0.3, 0.4, 1)
	_manual(DoBaseMaterial3D.fade(material, 0, 1)).force_complete()
	assert_eq(material.albedo_color, Color(0.2, 0.3, 0.4, 0))
	var uv = _manual(DoBaseMaterial3D.uv1_scale(material, Vector3(3, 3, 3), 1))
	uv.custom_step(0.5)
	_v3(material.uv1_scale, Vector3(2, 2, 2))
	uv.force_complete()
	var retained: WeakRef = weakref(material)
	material = null
	assert_true(retained.get_ref() != null)
	return true


func test_group2_label_styles() -> bool:
	var label := Label.new()
	var other := Label.new()
	add_child(label)
	add_child(other)
	label.add_theme_font_size_override("font_size", 20)
	var font = _manual(DoLabel.font_size(label, 40, 1))
	font.custom_step(0.5)
	assert_eq(label.get_theme_font_size("font_size"), 30)
	font.stop()
	var settings := LabelSettings.new()
	settings.font_size = 20
	label.label_settings = settings
	other.label_settings = settings
	_manual(DoLabel.font_size(label, 40, 1)).force_complete()
	assert_eq(settings.font_size, 40)
	assert_eq(other.label_settings.font_size, 40)
	settings.font_color = Color(0.2, 0.3, 0.4, 1)
	_manual(DoLabel.font_color_a(label, 0.5, 1)).force_complete()
	assert_eq(settings.font_color, Color(0.2, 0.3, 0.4, 0.5))
	_manual(DoLabel.outline_color(label, Color(0.4, 0.3, 0.2, 1), 1)).force_complete()
	_manual(DoLabel.outline_size(label, 7, 1)).force_complete()
	assert_eq(settings.outline_size, 7)
	_manual(DoLabelSettings.font_size(settings, 24, 1)).force_complete()
	assert_eq(settings.font_size, 24)
	var generic = _manual(Spire.do_property_int(label, ^"font_size", 40, 1))
	generic.custom_step(0.5)
	assert_eq(settings.font_size, 32)
	generic.force_complete()
	assert_eq(settings.font_size, 40)
	assert_eq(other.label_settings.font_size, 40)
	_manual(Spire.do_property_float(label, ^"font_color:a", 0.25, 1)).force_complete()
	assert_eq(settings.font_color, Color(0.2, 0.3, 0.4, 0.25))
	other.label_settings = null
	other.add_theme_font_size_override("font_size", 10)
	var themed = _manual(Spire.do_property_int(other, ^"font_size", 20, 1))
	themed.custom_step(0.5)
	assert_eq(other.get_theme_font_size("font_size"), 15)
	themed.force_complete()
	assert_eq(other.get_theme_font_size("font_size"), 20)
	assert_eq(settings.font_size, 40)
	label.free()
	other.free()
	return true


func test_group2_richtext_styles() -> bool:
	var label := RichTextLabel.new()
	add_child(label)
	var tweens := [
		DoRichTextLabel.normal_font_size(label, 24, 1),
		DoRichTextLabel.bold_font_size(label, 25, 1),
		DoRichTextLabel.italics_font_size(label, 26, 1),
		DoRichTextLabel.bold_italics_font_size(label, 27, 1),
		DoRichTextLabel.mono_font_size(label, 28, 1),
		DoRichTextLabel.default_color(label, Color(0.2, 0.3, 0.4, 1), 1),
		DoRichTextLabel.outline_color(label, Color(0.4, 0.3, 0.2, 1), 1),
		DoRichTextLabel.outline_size(label, 6, 1),
	]
	for tween in tweens: _manual(tween).force_complete()
	for index in 5:
		assert_eq(label.get_theme_font_size(["normal", "bold", "italics", "bold_italics", "mono"][index] + "_font_size"), 24 + index)
	assert_eq(label.get_theme_constant("outline_size"), 6)
	assert_eq(label.get_theme_color("default_color"), Color(0.2, 0.3, 0.4, 1))
	var generic = _manual(Spire.do_property_int(label, ^"normal_font_size", 40, 1))
	generic.custom_step(0.5)
	assert_eq(label.get_theme_font_size("normal_font_size"), 32)
	generic.force_complete()
	assert_eq(label.get_theme_font_size("normal_font_size"), 40)
	_manual(Spire.do_property_float(label, ^"default_color:a", 0.5, 1)).force_complete()
	assert_eq(label.get_theme_color("default_color"), Color(0.2, 0.3, 0.4, 0.5))
	label.free()
	return true


func test_group2_counters() -> bool:
	var label := Label.new()
	var rich := RichTextLabel.new()
	var label3 := Label3D.new()
	for node in [label, rich, label3]: add_child(node)
	var a = _manual(DoLabel.counter(label, -2000, 2000, 1, "·"))
	a.custom_step(0.5)
	assert_eq(label.text, "0")
	a.force_complete()
	assert_eq(label.text, "2·000")
	_manual(DoRichTextLabel.counter(rich, 0, -1234567, 1)).force_complete()
	assert_eq(rich.text, "-1,234,567")
	var ungrouped = _manual(DoLabel3D.counter(label3, -2000, 2000, 1, ""))
	ungrouped.custom_step(0.25)
	assert_eq(label3.text, "-1000")
	ungrouped.custom_step(0.75)
	assert_eq(label3.text, "2000")
	for node in [label, rich, label3]: node.free()
	return true


func test_group2_arrays() -> bool:
	var node := Node2D.new()
	add_child(node)
	var values: Array[Vector2] = [Vector2(4, 0), Vector2(8, 0)]
	var seq = _manual(Spire.do_property_array_vec2(node, ^"position", values, PackedFloat64Array([1, 1])))
	# At a segment boundary easing does not affect the expected endpoint.
	seq.custom_step(1.0)
	_v2(node.position, Vector2(4, 0))
	seq.custom_step(1.5)
	_v2(node.position, Vector2(8, 0))
	assert_true(seq.is_stopped())
	var node3 := Node3D.new()
	add_child(node3)
	var values3: Array[Vector3] = [Vector3(1, 2, 3), Vector3(4, 5, 6)]
	var instant = _manual(Spire.do_property_array_vec3(node3, ^"position", values3, PackedFloat64Array([0, 0])))
	instant.force_complete()
	_v3(node3.position, Vector3(4, 5, 6))
	node.free()
	node3.free()
	return true


func test_group2_array_errors() -> bool:
	var node := Node2D.new()
	add_child(node)
	var one: Array[Vector2] = [Vector2.ONE]
	var empty: Array[Vector2] = []
	assert_eq(Spire.do_property_array_vec2(node, ^"position", one, PackedFloat64Array([])), null)
	assert_eq(Spire.do_property_array_vec2(node, ^"position", empty, PackedFloat64Array([])), null)
	for invalid in [-1.0, NAN, INF]:
		assert_eq(Spire.do_property_array_vec2(node, ^"position", one, PackedFloat64Array([invalid])), null)
	assert_eq(Spire.do_property_array_vec2(null, ^"position", one, PackedFloat64Array([1])), null)
	_v2(node.position, Vector2.ZERO)
	assert_true(not ClassDB.class_has_method(&"Spire", &"do_get_set_float"))
	assert_true(not ClassDB.class_has_method(&"Spire", &"do_get_set_alpha"))
	node.free()
	return true


func test_group2_line_colors() -> bool:
	var line := Line2D.new()
	add_child(line)
	var gradient := Gradient.new()
	gradient.add_point(0.5, Color.GREEN)
	line.gradient = gradient
	var tween = _manual(DoLine2D.endpoint_colors(line, Color.RED, Color.BLUE, 1))
	assert_eq(line.gradient, gradient)
	tween.custom_step(0.5)
	assert_ne(line.gradient, gradient)
	assert_eq(line.gradient.get_color(1), Color.GREEN)
	assert_eq(gradient.get_color(0), Color.BLACK)
	tween.force_complete()
	assert_eq(line.gradient.get_color(0), Color.RED)
	assert_eq(line.gradient.get_color(2), Color.BLUE)
	line.gradient = null
	line.default_color = Color(0.2, 0.3, 0.4, 1)
	var fallback = _manual(DoLine2D.endpoint_colors(line, Color.RED, Color.BLUE, 1))
	fallback.custom_step(0.5)
	_near(line.gradient.get_color(0).r, 0.6)
	fallback.force_complete()
	var single := Gradient.new()
	single.remove_point(1)
	single.set_color(0, Color(0.2, 0.3, 0.4, 1))
	line.gradient = single
	var single_tween = _manual(DoLine2D.endpoint_colors(line, Color.RED, Color.BLUE, 1))
	single_tween.custom_step(0.5)
	_near(line.gradient.get_color(0).r, 0.6)
	assert_eq(single.get_point_count(), 1)
	single_tween.force_complete()
	line.free()
	return true


func test_group2_anchors_and_circle() -> bool:
	var parent := Control.new()
	var node := Control.new()
	add_child(parent)
	parent.position = Vector2(30, 40)
	parent.add_child(node)
	_manual(DoControl.anchor_min(node, Vector2(0.2, 0.3), 1)).force_complete()
	_near(node.anchor_left, 0.2)
	_near(node.anchor_top, 0.3)
	_manual(DoControl.anchor_max(node, Vector2(0.8, 0.9), 1)).force_complete()
	_near(node.anchor_right, 0.8)
	_near(node.anchor_bottom, 0.9)
	var circle = _manual(DoControl.circle(node, Vector2(5, 6), 0, PI, 4, 1))
	circle.custom_step(0.5)
	_v2(node.position, Vector2(5, 10))
	circle.force_complete()
	_v2(node.position, Vector2(1, 6))
	_manual(DoControl.global_circle(node, Vector2(20, 30), 0, PI, 4, 1)).force_complete()
	_v2(node.global_position, Vector2(16, 30))
	_manual(DoControl.ellipsis(node, Vector2.ZERO, 0, PI / 2, Vector2(2, 4), Vector2(2, 4), 1)).force_complete()
	_v2(node.position, Vector2(0, 4))
	_manual(DoControl.global_ellipsis(node, Vector2.ZERO, 0, PI / 2, Vector2(2, 4), Vector2(2, 4), 1)).force_complete()
	_v2(node.global_position, Vector2(0, 4))
	parent.free()
	return true


func test_group2_shader_parameters() -> bool:
	var shader := Shader.new()
	shader.code = "shader_type canvas_item; uniform float gain=1.0; uniform vec3 tint=vec3(0.0); uniform int count=0; uniform vec2 uv=vec2(0.0); uniform ivec2 tiles=ivec2(0); uniform ivec3 grid=ivec3(0); uniform vec4 paint: source_color=vec4(1.0); void fragment(){ COLOR=paint*vec4(tint*gain,1.0); }"
	var material := ShaderMaterial.new()
	material.shader = shader
	material.set_shader_parameter("gain", 1.0)
	material.set_shader_parameter("tint", Vector3.ZERO)
	material.set_shader_parameter("paint", Color.WHITE)
	material.set_shader_parameter("count", 0)
	material.set_shader_parameter("uv", Vector2.ZERO)
	material.set_shader_parameter("tiles", Vector2i.ZERO)
	material.set_shader_parameter("grid", Vector3i.ZERO)
	var tween = _manual(DoShaderMaterial.parameter_float(material, &"gain", 3, 1))
	tween.custom_step(0.5)
	_near(material.get_shader_parameter("gain"), 2)
	tween.force_complete()
	_manual(DoShaderMaterial.parameter_vec3(material, &"tint", Vector3(0.2, 0.3, 0.4), 1)).force_complete()
	_v3(material.get_shader_parameter("tint"), Vector3(0.2, 0.3, 0.4))
	_manual(DoShaderMaterial.parameter_color(material, &"paint", Color(0.2, 0.3, 0.4, 0.5), 1)).force_complete()
	assert_eq(material.get_shader_parameter("paint"), Color(0.2, 0.3, 0.4, 0.5))
	_manual(DoShaderMaterial.parameter_color_a(material, &"paint", 0.25, 1)).force_complete()
	assert_eq(material.get_shader_parameter("paint"), Color(0.2, 0.3, 0.4, 0.25))
	_manual(DoShaderMaterial.parameter_int(material, &"count", 5, 1)).force_complete()
	_manual(DoShaderMaterial.parameter_vec2(material, &"uv", Vector2(2, 3), 1)).force_complete()
	_manual(DoShaderMaterial.parameter_vec2i(material, &"tiles", Vector2i(2, 3), 1)).force_complete()
	_manual(DoShaderMaterial.parameter_vec3i(material, &"grid", Vector3i(2, 3, 4), 1)).force_complete()
	assert_eq(material.get_shader_parameter("count"), 5)
	_v2(material.get_shader_parameter("uv"), Vector2(2, 3))
	assert_eq(material.get_shader_parameter("tiles"), Vector2i(2, 3))
	assert_eq(material.get_shader_parameter("grid"), Vector3i(2, 3, 4))
	assert_eq(DoShaderMaterial.parameter_float(material, &"missing", 1, 1), null)
	assert_eq(DoShaderMaterial.parameter_vec3(material, &"gain", Vector3.ONE, 1), null)
	var cancelled = _manual(DoShaderMaterial.parameter_float(material, &"gain", 5, 1))
	var events := [0, 0]
	cancelled.stopped.connect(func(): events[0] += 1)
	cancelled.finished.connect(func(): events[1] += 1)
	material.shader = null
	assert_true(not cancelled.custom_step(0.5))
	assert_eq(events, [1, 0])
	return true


func test_group2_audio_buses() -> bool:
	var index := AudioServer.bus_count
	AudioServer.add_bus()
	AudioServer.set_bus_name(index, &"SpireGroup2")
	AudioServer.add_bus()
	AudioServer.set_bus_name(index + 1, &"SpireGroup2Other")
	AudioServer.set_bus_volume_linear(index, 1)
	var tween = _manual(DoAudioServer.bus_volume_linear(&"SpireGroup2", 3, 1))
	tween.custom_step(0.5)
	_near(AudioServer.get_bus_volume_linear(index), 2)
	AudioServer.move_bus(index, index + 1)
	index = AudioServer.get_bus_index(&"SpireGroup2")
	tween.force_complete()
	_near(AudioServer.get_bus_volume_linear(index), 3)
	var cancelled = _manual(DoAudioServer.bus_volume_db(&"SpireGroup2", -20, 1))
	var events := [0, 0]
	cancelled.stopped.connect(func(): events[0] += 1)
	cancelled.finished.connect(func(): events[1] += 1)
	AudioServer.remove_bus(index)
	assert_true(not cancelled.custom_step(0.5))
	assert_eq(events, [1, 0])
	assert_eq(DoAudioServer.bus_volume_db(&"SpireGroup2", 0, 1), null)
	AudioServer.remove_bus(AudioServer.get_bus_index(&"SpireGroup2Other"))
	return true


func test_group2_freed_owners() -> bool:
	for forced in [false, true]:
		var node := Node2D.new()
		var ui := Control.new()
		var label := Label.new()
		var rich := RichTextLabel.new()
		var line := Line2D.new()
		for owner in [node, ui, label, rich, line]: add_child(owner)
		var tweens := [
			DoNode2D.jump(node, Vector2.ONE, 4, 1, 1),
			DoCanvasItem.gradient_modulate(node, Gradient.new(), 1),
			DoControl.circle(ui, Vector2.ZERO, 0, PI, 2, 1),
			DoLabel.counter(label, 0, 1000, 1),
			DoLabel.font_size(label, 40, 1),
			Spire.do_property_int(label, ^"font_size", 40, 1),
			Spire.do_property_float(label, ^"font_color:a", 0.5, 1),
			Spire.do_property_int(rich, ^"normal_font_size", 40, 1),
			DoLine2D.endpoint_colors(line, Color.RED, Color.BLUE, 1),
		]
		var events := [0, 0]
		for tween in tweens:
			_manual(tween)
			tween.stopped.connect(func(): events[0] += 1)
			tween.finished.connect(func(): events[1] += 1)
		for owner in [node, ui, label, rich, line]: owner.free()
		for tween in tweens:
			if forced: tween.force_complete()
			else: assert_true(not tween.custom_step(0.5))
			assert_true(tween.is_stopped())
		assert_eq(events, [tweens.size(), 0])
	return true


func test_group2_discovery_rejects_missing_script() -> bool:
	# Exercise discovery against actual PackedScenes without starting another runner.
	var harness = load("res://examples/tests/test_runner.gd").new()
	var node := Node.new()
	var invalid := PackedScene.new()
	assert_eq(invalid.pack(node), OK)
	node.free()
	assert_true(harness.get_prefab_test_names(invalid).is_empty())
	assert_eq(harness.failed, 1)
	assert_eq(harness.get_prefab_hash(invalid), 0)
	assert_eq(harness.failed, 1)
	var valid: PackedScene = load("res://examples/tests/misc.tscn")
	assert_true(harness.get_prefab_test_names(valid).has("test_force_complete"))
	assert_eq(harness.failed, 1)
	harness.free()
	return true
