extends "res://examples/tests/test_base.gd"

@onready var shape: CollisionShape2D = $BouncyBall/CollisionShape2D


func test_camera_clip_shortcuts() -> bool:
	var camera := Camera3D.new()
	add_child(camera)
	_assert_scalar_shortcut(DoCamera3D.near(camera, 1.0, 1.0), camera, ^"near", 0.25, 1.0)
	_assert_scalar_shortcut(DoCamera3D.far(camera, 300.0, 1.0), camera, ^"far", 100.0, 300.0)
	assert_eq(camera.near, 1.0)
	camera.free()
	return true


func test_camera_clip_shortcuts_freed_owner() -> bool:
	var camera := Camera3D.new()
	add_child(camera)
	_assert_cancelled_shortcuts(camera, [DoCamera3D.near(camera, 1.0, 1.0), DoCamera3D.far(camera, 300.0, 1.0)])
	return true


func test_light_shortcuts() -> bool:
	var lights: Array[Light3D] = [DirectionalLight3D.new(), OmniLight3D.new(), SpotLight3D.new()]
	for light in lights:
		add_child(light)
		var color := light.light_color
		_assert_scalar_shortcut(DoLight3D.light_energy(light, 3.0, 1.0), light, ^"light_energy", 1.0, 3.0)
		_assert_scalar_shortcut(DoLight3D.shadow_opacity(light, 0.0, 1.0), light, ^"shadow_opacity", 1.0, 0.0)
		assert_eq(light.get_param(Light3D.PARAM_ENERGY), 3.0)
		assert_eq(light.light_color, color)
		light.free()
	return true


func test_light_shortcuts_freed_owner() -> bool:
	var light := OmniLight3D.new()
	add_child(light)
	_assert_cancelled_shortcuts(light, [DoLight3D.light_energy(light, 3.0, 1.0), DoLight3D.shadow_opacity(light, 0.0, 1.0)])
	return true


func test_audio_fade_aliases() -> bool:
	var player := AudioStreamPlayer.new()
	var player_2d := AudioStreamPlayer2D.new()
	var player_3d := AudioStreamPlayer3D.new()
	var players: Array[Node] = [player, player_2d, player_3d]
	var helpers: Array[StringName] = [&"DoAudioStreamPlayer", &"DoAudioStreamPlayer2D", &"DoAudioStreamPlayer3D"]
	for owner in players:
		add_child(owner)
	var visual := player_2d.modulate
	for index in players.size():
		assert_true(ClassDB.class_has_method(helpers[index], &"pitch_scale"))
		assert_true(not ClassDB.class_has_method(helpers[index], &"pitch"))
	# Keep these calls typed to check each public GDScript signature.
	var fades: Array[SpirePropertyFloat] = [
		DoAudioStreamPlayer.fade(player, 2.0, 1.0),
		DoAudioStreamPlayer2D.fade(player_2d, 2.0, 1.0),
		DoAudioStreamPlayer3D.fade(player_3d, 2.0, 1.0),
	]
	var canonical: Array[SpirePropertyFloat] = [
		DoAudioStreamPlayer.volume_linear(player, 2.0, 1.0),
		DoAudioStreamPlayer2D.volume_linear(player_2d, 2.0, 1.0),
		DoAudioStreamPlayer3D.volume_linear(player_3d, 2.0, 1.0),
	]
	for index in players.size():
		assert_eq(fades[index].get_property_path(), canonical[index].get_property_path())
		Spire.unregister(canonical[index])
		_assert_scalar_shortcut(fades[index], players[index], ^"volume_linear", 0.5, 2.0)
		assert_eq(players[index].get("pitch_scale"), 1.0)
	var silence: Array[SpirePropertyFloat] = [
		DoAudioStreamPlayer.fade(player, 0.0, 1.0),
		DoAudioStreamPlayer2D.fade(player_2d, 0.0, 1.0),
		DoAudioStreamPlayer3D.fade(player_3d, 0.0, 1.0),
	]
	for index in players.size():
		_assert_scalar_shortcut(silence[index], players[index], ^"volume_linear", 2.0, 0.0)
	assert_eq(player_2d.modulate, visual)
	for owner in players:
		owner.free()
	return true


func test_audio_fade_aliases_freed_owner() -> bool:
	var player := AudioStreamPlayer.new()
	var player_2d := AudioStreamPlayer2D.new()
	var player_3d := AudioStreamPlayer3D.new()
	for owner in [player, player_2d, player_3d]:
		add_child(owner)
	_assert_cancelled_shortcuts(player, [DoAudioStreamPlayer.fade(player, 0.0, 1.0)])
	_assert_cancelled_shortcuts(player_2d, [DoAudioStreamPlayer2D.fade(player_2d, 0.0, 1.0)])
	_assert_cancelled_shortcuts(player_3d, [DoAudioStreamPlayer3D.fade(player_3d, 0.0, 1.0)])
	return true


func test_visual_3d_fade_aliases() -> bool:
	var sprite := Sprite3D.new()
	var animated := AnimatedSprite3D.new()
	var label := Label3D.new()
	var outline := label.outline_modulate
	var owners: Array[Node3D] = [sprite, animated, label]
	for owner in owners:
		add_child(owner)
		owner.set("modulate", Color(0.125, 0.25, 0.5, 1.0))
	var fades: Array[SpirePropertyFloat] = [
		DoSpriteBase3D.fade(sprite, 0.0, 1.0),
		DoSpriteBase3D.fade(animated, 0.0, 1.0),
		DoLabel3D.fade(label, 0.0, 1.0),
	]
	var canonical: Array[SpirePropertyFloat] = [
		DoSpriteBase3D.modulate_a(sprite, 0.0, 1.0),
		DoSpriteBase3D.modulate_a(animated, 0.0, 1.0),
		DoLabel3D.modulate_a(label, 0.0, 1.0),
	]
	var color_aliases: Array[SpirePropertyFloat] = [
		DoSpriteBase3D.color_a(sprite, 0.0, 1.0),
		DoSpriteBase3D.color_a(animated, 0.0, 1.0),
		DoLabel3D.color_a(label, 0.0, 1.0),
	]
	for index in owners.size():
		assert_eq(fades[index].get_property_path(), canonical[index].get_property_path())
		assert_eq(color_aliases[index].get_property_path(), canonical[index].get_property_path())
		Spire.unregister(canonical[index])
		Spire.unregister(color_aliases[index])
		_assert_scalar_shortcut(fades[index], owners[index], ^"modulate:a", 1.0, 0.0)
		assert_eq(owners[index].get("modulate"), Color(0.125, 0.25, 0.5, 0.0))
	assert_eq(label.outline_modulate, outline)
	for owner in owners:
		owner.free()
	return true


func test_visual_3d_fade_aliases_freed_owner() -> bool:
	var sprite := Sprite3D.new()
	var label := Label3D.new()
	add_child(sprite)
	add_child(label)
	_assert_cancelled_shortcuts(sprite, [DoSpriteBase3D.fade(sprite, 0.0, 1.0)])
	_assert_cancelled_shortcuts(label, [DoLabel3D.fade(label, 0.0, 1.0)])
	return true


func _assert_scalar_shortcut(tween: SpirePropertyFloat, owner: Node, path: NodePath, initial: float, target: float) -> void:
	assert_true(Spire.is_registered(tween), "GDScript shortcuts return registered handles")
	assert_eq(tween.get_property_path(), path)
	owner.set_indexed(path, initial)
	tween.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)
	var events: Array[int] = [0, 0]
	tween.stopped.connect(func(): events[0] += 1)
	tween.finished.connect(func(): events[1] += 1)
	assert_true(tween.custom_step(0.5))
	assert_true(absf(float(owner.get_indexed(path)) - (initial + target) * 0.5) < 0.00001, "Unexpected midpoint")
	assert_eq(events, [0, 0])
	assert_true(not tween.custom_step(0.75))
	assert_true(absf(float(owner.get_indexed(path)) - target) < 0.00001, "Unexpected final value")
	assert_true(tween.is_stopped())
	assert_eq(events, [1, 1])
	Spire.unregister(tween)


func _assert_cancelled_shortcuts(owner: Node, tweens: Array[SpirePropertyFloat]) -> void:
	var events: Array[int] = [0, 0]
	for tween in tweens:
		tween.set_process_mode(Spire.PROCESS_MODE_MANUAL)
		tween.stopped.connect(func(): events[0] += 1)
		tween.finished.connect(func(): events[1] += 1)
	owner.free()
	for tween in tweens:
		assert_true(not tween.custom_step(0.5))
		assert_true(tween.is_stopped())
		tween.stop()
		Spire.unregister(tween)
	assert_eq(events, [tweens.size(), 0])


func test_fade_alias() -> bool:
	var plain_node := Node2D.new()
	add_child(plain_node)
	var nodes: Array[Node2D] = [plain_node, shape]
	for node in nodes:
		var initial := Color(0.125, 0.25, 0.5, 1.0)
		node.modulate = initial
		var self_modulate := node.self_modulate
		var canonical := DoCanvasItem.modulate_a(node, 0.0, 1.0)
		var existing_alias := DoCanvasItem.color_a(node, 0.0, 1.0)
		assert_eq(existing_alias.get_property_path(), canonical.get_property_path())
		Spire.unregister(existing_alias)
		var tween: SpirePropertyFloat = DoCanvasItem.fade(node, 0.0, 1.0)
		assert_true(Spire.is_registered(tween))
		assert_eq(tween.get_property_path(), canonical.get_property_path())
		assert_eq(tween.get_property_path(), ^"modulate:a")
		Spire.unregister(canonical)
		tween.set_process_mode(Spire.PROCESS_MODE_MANUAL).set_ease(Spire.EASE_LINEAR)
		assert_true(tween.custom_step(0.5))
		assert_eq(node.modulate, Color(initial.r, initial.g, initial.b, 0.5))
		assert_true(not tween.custom_step(0.75))
		assert_true(tween.is_stopped())
		assert_eq(node.modulate, Color(initial.r, initial.g, initial.b, 0.0))
		assert_eq(node.self_modulate, self_modulate)
		Spire.unregister(tween)
	plain_node.free()
	return true


func test_fade_alias_freed_owner() -> bool:
	var node := Node2D.new()
	add_child(node)
	var tween := DoCanvasItem.fade(node, 0.0, 1.0).set_process_mode(Spire.PROCESS_MODE_MANUAL)
	var finishes: Array[bool] = []
	tween.finished.connect(func(): finishes.append(true))
	node.free()
	assert_true(not tween.custom_step(0.5))
	assert_true(tween.is_stopped())
	assert_true(finishes.is_empty(), "Freed owners must cancel rather than complete the fade")
	Spire.unregister(tween)
	return true


func test_force_complete() -> bool:
	var final_value := Vector2.ONE * 2.5
	var tween := DoNode2D.scale(shape, final_value, 5.0)
	tween.force_complete()
	assert_true(tween.is_stopped())
	assert_eq(shape.scale, final_value)
	
	await wait_seconds(2.0)
	
	shape.scale = Vector2.ONE
	
	tween.play()
	await wait_finished(tween, 7)
	assert_eq(shape.scale, final_value)
	return true


var dyn_target: Vector2

func test_dyn_target() -> bool:
	dyn_target = Vector2.ONE * 5
	var initial_scale := shape.scale
	var speed := 0.5
	var tween := DoNode2D.scale(shape, Vector2.ZERO, speed)\
		.as_speed_based()\
		.set_dynamic_target(func(): return dyn_target)
	
	var wait_time := 3.0
	await wait_seconds(wait_time)
	var expected_scale := initial_scale.move_toward(dyn_target, speed * wait_time)
	assert_le(shape.scale.distance_to(expected_scale), 0.01)
	
	dyn_target = Vector2.ONE
	var expected_duration := shape.scale.distance_to(dyn_target) / speed
	await wait_finished(tween, wait_time + expected_duration)
	assert_eq(shape.scale, dyn_target)
	
	return true


func test_property_path() -> bool:
	var scale_tween := DoNode2D.scale_x(shape, 5, 1.0)
	assert_eq(scale_tween.get_property_path(), ^"scale:x")
	var pos_tween = DoNode2D.move(shape, Vector2(200, 500), 2.0)
	assert_eq(pos_tween.get_property_path(), ^"global_position")
	
	return true
