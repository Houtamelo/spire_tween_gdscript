extends "res://examples/tests/test_base.gd"

const CUSTOM := preload("res://examples/tests/blueprint_custom.gd")
const PROPERTY_TARGET := preload("res://examples/tests/blueprint_property_target.gd")

class ValueTarget extends Node2D:
	@export var scalar: float = 0.0
	@export var count: int = 0
	@export var text_value: String = ""
	@export var color_value: Color = Color(0, 0, 0, 0)
	@export var vector2i_value: Vector2i = Vector2i.ZERO
	@export var vector3_value: Vector3 = Vector3.ZERO
	@export var vector3i_value: Vector3i = Vector3i.ZERO
	@export var vector4_value: Vector4 = Vector4.ZERO
	@export var rotation_value: Quaternion = Quaternion.IDENTITY
	@export var rect_value: Rect2 = Rect2()
	@export var rect2i_value: Rect2i = Rect2i()

var failures: Array[String] = []
var sequences: Array[SpireSequence] = []

func check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		push_error(message)

func near(actual: float, expected: float) -> bool:
	return abs(actual - expected) < 0.00001

func same(actual: Variant, expected: Variant) -> bool:
	if typeof(expected) == TYPE_FLOAT: return is_equal_approx(actual, expected)
	if typeof(expected) in [TYPE_VECTOR2, TYPE_VECTOR3, TYPE_VECTOR4, TYPE_COLOR, TYPE_QUATERNION, TYPE_RECT2]:
		return actual.is_equal_approx(expected)
	return actual == expected

func blueprint(parent: Node, kind: int = Spire.BLUEPRINT_KIND_CHAIN_BLOCK) -> TweenBlueprint:
	var result := TweenBlueprint.new()
	result.kind = kind
	result.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	parent.add_child(result)
	return result

func property_step(parent: Node, target: Node, property: StringName, end: Variant, duration: float = 1.0) -> TweenBlueprint:
	var result := blueprint(parent, Spire.BLUEPRINT_KIND_PROPERTY)
	result.target = target
	result.animated_property = property
	result.end_value = end
	result.speed_or_duration = duration
	result.ease_basic = Spire.EASE_LINEAR
	return result

func play(effect: TweenBlueprint) -> SpireSequence:
	var sequence := effect.play()
	check(sequence != null, "Blueprint.play must return a usable sequence")
	if sequence:
		sequences.append(sequence)
	return sequence

func _exit_tree() -> void:
	for sequence in sequences:
		sequence.stop()
		sequence.unregister()

func test_blueprint_property_and_restart() -> bool:
	var sprite := Node2D.new()
	add_child(sprite)
	var effect := property_step(self, sprite, &"position", Vector2(10, 0))
	var first := play(effect)
	if not first: return false
	first.custom_step(0.4)
	check(near(sprite.position.x, 4.0), "Property tween must interpolate")
	var second := play(effect)
	if not second: return false
	check(second != first and first.is_stopped(), "Every play must stop the previous run and create a fresh sequence")
	second.custom_step(0.5)
	check(near(sprite.position.x, 7.0), "Implicit start must use the interrupted value")
	second.force_complete()
	check(sprite.position == Vector2(10, 0), "Completion must reach the endpoint")
	return failures.is_empty()

func test_blueprint_explicit_start_delay_and_configuration() -> bool:
	var sprite := Node2D.new()
	add_child(sprite)
	var effect := property_step(self, sprite, &"position", Vector2(10, 0))
	effect.set("start_enabled", true)
	effect.start_value = Vector2(2, 0)
	effect.delay = 0.5
	effect.speed_scale = 2.0
	effect.loops = 2
	effect.ignore_time_scale = true
	var sequence := play(effect)
	if not sequence: return false
	check(near(sequence.get_speed_scale(), 2.0), "Speed must be forwarded")
	check(sequence.get_ignore_time_scale(), "Time-scale setting must be forwarded")
	check(sequence.get_loops() == 2, "Loop count must be forwarded")
	sequence.custom_step(0.2)
	check(sprite.position == Vector2.ZERO, "Construction and pre-delay processing must not write an explicit start")
	sequence.custom_step(0.3)
	check(near(sprite.position.x, 6.0), "Delay and explicit start must use the native clock")
	return failures.is_empty()

func test_blueprint_serial_and_parallel_children() -> bool:
	for kind in [Spire.BLUEPRINT_KIND_CHAIN_BLOCK, Spire.BLUEPRINT_KIND_PARALLEL_BLOCK]:
		var sprite := Node2D.new()
		add_child(sprite)
		var container := blueprint(self, kind)
		container.target = sprite
		property_step(container, sprite, &"position:x", 10.0)
		property_step(container, sprite, &"position:y", 20.0)
		var sequence := play(container)
		if not sequence: return false
		sequence.custom_step(0.5)
		check(near(sprite.position.x, 5.0), "First child must advance")
		check(near(sprite.position.y, 10.0 if kind == Spire.BLUEPRINT_KIND_PARALLEL_BLOCK else 0.0), "Container kind must control composition")
		sequence.force_complete()
		check(sprite.position == Vector2(10, 20), "Both composed children must finish")
	return failures.is_empty()

func test_blueprint_supported_property_types() -> bool:
	var target := ValueTarget.new()
	add_child(target)
	var cases := {
		&"scalar": 12.0, &"count": 12, &"text_value": "finished",
		&"color_value": Color(0.2, 0.4, 0.6, 0.8),
		&"position": Vector2(4, 8), &"vector2i_value": Vector2i(4, 8),
		&"vector3_value": Vector3(4, 8, 12), &"vector3i_value": Vector3i(4, 8, 12),
		&"vector4_value": Vector4(4, 8, 12, 16),
		&"rotation_value": Quaternion(Vector3.UP, PI / 2),
		&"rect_value": Rect2(2, 4, 6, 8), &"rect2i_value": Rect2i(2, 4, 6, 8),
	}
	for property in cases:
		var initial: Variant = target.get(property)
		var effect := property_step(self, target, property, cases[property])
		var sequence := play(effect)
		if not sequence: return false
		sequence.custom_step(0.5)
		check(target.get(property) != cases[property], "Typed tween must have an intermediate value: " + property)
		sequence.force_complete()
		check(same(target.get(property), cases[property]), "Typed tween must reach its endpoint: " + property)
		target.set(property, initial)
		var instant := property_step(self, target, property, cases[property], 0.0)
		sequence = play(instant)
		if not sequence: return false
		sequence.custom_step(0.1)
		check(same(target.get(property), cases[property]), "Zero-duration steps must support the same property types: " + property)
	return failures.is_empty()

func test_blueprint_relative_speed_and_curve() -> bool:
	var target := ValueTarget.new()
	add_child(target)
	for property in [&"position", &"vector2i_value", &"vector3i_value", &"rect2i_value"]:
		var offset: Variant = {&"position": Vector2(4, 8), &"vector2i_value": Vector2i(4, 8), &"vector3i_value": Vector3i(4, 8, 12), &"rect2i_value": Rect2i(2, 4, 6, 8)}[property]
		var effect := property_step(self, target, property, offset)
		effect.lerp_mode = Spire.LERP_MODE_KIND_RELATIVE
		var sequence := play(effect)
		if not sequence: return false
		sequence.custom_step(0.5)
		sequence.custom_step(0.6)
		check(target.get(property) == offset, "Relative mode must add its full offset for " + property)
	var speed := property_step(self, target, &"scalar", 10.0, 5.0)
	speed.lerp_mode = Spire.LERP_MODE_KIND_SPEED_BASED
	var sequence := play(speed)
	if not sequence: return false
	sequence.custom_step(0.5)
	check(near(target.scalar, 2.5), "Speed-based mode must advance in units per second")
	sequence.force_complete()
	var curve := Curve.new()
	curve.add_point(Vector2(0, 0))
	curve.add_point(Vector2(1, 0))
	var curved := property_step(self, target, &"scalar", 20.0)
	curved.ease_curve = curve
	sequence = play(curved)
	if not sequence: return false
	sequence.custom_step(0.5)
	check(near(target.scalar, 10.0), "Custom Curve must override basic easing")
	sequence.force_complete()
	return failures.is_empty()

func test_blueprint_independent_targets_and_fallback() -> bool:
	var first := Node2D.new()
	var second := Node2D.new()
	var unrelated := Node2D.new()
	add_child(first)
	add_child(second)
	add_child(unrelated)
	var effect := blueprint(first, Spire.BLUEPRINT_KIND_PROPERTY)
	check(not effect.has_method("play_on"), "Playback must not expose an override-target API")
	check(not effect.get_property_list().any(func(info): return info.name == &"override_children_targets"), "The Inspector must not expose target propagation")
	effect.animated_property = &"position"
	effect.end_value = Vector2(6, 0)
	effect.speed_or_duration = 1.0
	check(effect.get_resolved_target() == first, "An unset target must resolve the first non-blueprint ancestor")
	var sequence := play(effect)
	if not sequence: return false
	sequence.force_complete()
	check(first.position == Vector2(6, 0) and second.position == Vector2.ZERO, "Playback must use the blueprint's own fallback target")
	var container := blueprint(first)
	container.target = unrelated
	var child := property_step(container, second, &"position", Vector2(9, 0))
	var inner := blueprint(container, Spire.BLUEPRINT_KIND_PARALLEL_BLOCK)
	inner.target = second
	var nested := blueprint(inner, Spire.BLUEPRINT_KIND_PROPERTY)
	nested.animated_property = &"position"
	nested.end_value = Vector2(12, 0)
	check(child.get_resolved_target() == second, "A container must preserve each child's configured target")
	check(nested.get_resolved_target() == first, "Fallback must skip blueprint ancestors and their targets")
	sequence = play(container)
	if not sequence: return false
	unrelated.free()
	sequence.force_complete()
	check(first.position == Vector2(12, 0) and second.position == Vector2(9, 0), "Nested children must animate independent targets even when a container's unused target is freed")
	check(child.target == second, "Composition must preserve authored target references")
	return failures.is_empty()

func test_blueprint_invalid_steps_do_not_block_siblings() -> bool:
	var target := Node2D.new()
	add_child(target)
	var container := blueprint(self)
	container.target = target
	property_step(container, target, &"missing_property", 5.0)
	property_step(container, target, &"position", "wrong type")
	property_step(container, target, &"position", Vector2(10, 0), -1.0)
	var authoring_node := blueprint(target)
	property_step(container, authoring_node, &"delay", 4.0)
	property_step(container, target, &"position", Vector2(7, 0))
	var sequence := play(container)
	if not sequence: return false
	sequence.force_complete()
	check(target.position == Vector2(7, 0), "Invalid steps must warn and leave following valid steps usable")
	check(authoring_node.delay == 0.0, "A property step must reject another TweenBlueprint as its target")
	for path in [&" position ", &" position:x "]:
		var normalized := String(path).strip_edges()
		var end: Variant = Vector2(12, 3) if normalized == "position" else 18.0
		var padded := property_step(self, target, path, end)
		padded.end_value = null
		padded.initialize_blueprint_values()
		var expected: Variant = target.position if normalized == "position" else target.position.x
		check(padded.start_value == expected and padded.end_value == expected, "Endpoint initialization must use the validated, trimmed property path")
		padded.end_value = end
		sequence = play(padded)
		sequence.force_complete()
		check(target.get_indexed(NodePath(normalized)) == end, "Playback must use the same trimmed property path as validation")
	var dynamic := PROPERTY_TARGET.new()
	add_child(dynamic)
	var validation_steps := blueprint(self)
	property_step(validation_steps, dynamic, &"readonly_scalar", 9.0)
	property_step(validation_steps, dynamic, &"unsupported_toggle", true)
	property_step(validation_steps, dynamic, &"dynamic_scalar", 8.0)
	property_step(validation_steps, dynamic, &"seed", 6.0)
	property_step(validation_steps, dynamic, &"editor_description", "Authored explicitly")
	property_step(validation_steps, dynamic, &"shape:distance", 12.0)
	sequence = play(validation_steps)
	sequence.force_complete()
	check(dynamic.get("readonly_scalar") == 4.0 and dynamic.get("unsupported_toggle") == false, "Read-only and unsupported properties must be skipped")
	check(dynamic.dynamic_value == 8.0 and dynamic.seed == 6.0, "Dynamic properties and script variables must remain writable tween targets")
	check(dynamic.editor_description == "Authored explicitly", "Suggestion exclusions must not reject an explicitly authored property")
	check(dynamic.shape.distance == 12.0, "An unsupported root type must still allow a supported indexed value")
	return failures.is_empty()

func test_blueprint_handles_do_not_retain_themselves() -> bool:
	var effect: TweenBlueprint = CUSTOM.new()
	effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	add_child(effect)
	var sequence := effect.play()
	if not sequence: return false
	var observer: WeakRef = weakref(sequence)
	sequence.unregister()
	effect.stop()
	sequence = null
	check(observer.get_ref() == null, "Lifecycle forwarding must not create a sequence ownership cycle")
	return failures.is_empty()

func test_blueprint_stop_and_freed_bindings() -> bool:
	var target := Node2D.new()
	add_child(target)
	var effect := property_step(self, target, &"position", Vector2(10, 0))
	var events: Array[String] = []
	effect.tween_stopped.connect(func(_sequence): events.append("stop"))
	effect.tween_finished.connect(func(_sequence): events.append("finish"))
	var sequence := play(effect)
	if not sequence: return false
	sequence.custom_step(0.25)
	effect.stop()
	effect.stop()
	check(events == ["stop"] and near(target.position.x, 2.5), "Stop must be idempotent and preserve the interrupted pose")
	events.clear()
	sequence = play(effect)
	if not sequence: return false
	target.free()
	sequence.custom_step(0.5)
	check(sequence.is_stopped() and events == ["stop"], "A freed target must cancel without successful completion")
	var retained := play(effect)
	if not retained: return false
	effect.free()
	check(retained.is_stopped(), "A freed authoring node must stop its retained run")
	return failures.is_empty()

func test_blueprint_invalid_runtime_scalars_and_missing_builder() -> bool:
	var target := Node2D.new()
	add_child(target)
	var effect := property_step(self, target, &"position", Vector2(10, 0))
	effect.delay = INF
	effect.speed_scale = NAN
	var sequence := play(effect)
	if not sequence: return false
	check(sequence.get_delay() == 0.0 and sequence.get_speed_scale() == 1.0, "Invalid timing values must normalize to operational defaults")
	sequence.custom_step(0.5)
	check(near(target.position.x, 5.0), "Normalized invalid timing must allow playback")
	sequence.stop()
	var instant := property_step(self, target, &"position", Vector2(20, 0), 0.0)
	instant.delay = 0.5
	sequence = play(instant)
	if not sequence: return false
	sequence.custom_step(0.25)
	check(near(target.position.x, 5.0), "A zero-duration step must still honor its delay")
	sequence.custom_step(0.3)
	check(target.position == Vector2(20, 0) and sequence.is_stopped(), "Zero-duration steps must complete at activation without division by zero")
	var missing_builder := GDScript.new()
	missing_builder.source_code = "extends TweenBlueprint\n"
	check(missing_builder.reload() == OK, "The missing-builder fixture must compile")
	var custom: TweenBlueprint = missing_builder.new()
	custom.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	add_child(custom)
	sequence = play(custom)
	if not sequence: return false
	sequence.force_complete()
	check(sequence.is_stopped(), "A missing script override must return a usable empty sequence")
	var invalid: TweenBlueprint = CUSTOM.new()
	invalid.trigger_condition = Spire.TRIGGER_CONDITION_MOUSE_ENTER
	invalid.trigger_source = target
	add_child(invalid)
	await get_tree().process_frame
	check(invalid.get("build_count") == 0, "An unsupported mouse source must not invoke the effect")
	invalid.trigger_source.free()
	invalid.free()
	return failures.is_empty()

func test_blueprint_runtime_trigger_reconfiguration_and_3d_input() -> bool:
	var first := Control.new()
	var second := Control.new()
	add_child(first)
	add_child(second)
	var effect: TweenBlueprint = CUSTOM.new()
	effect.trigger_condition = Spire.TRIGGER_CONDITION_MOUSE_ENTER
	effect.trigger_source = first
	add_child(effect)
	effect.trigger_source = second
	await get_tree().process_frame
	first.mouse_entered.emit()
	second.mouse_entered.emit()
	check(effect.get("build_count") == 1, "Changing a source must disconnect the old source and connect the new one")
	effect.trigger_condition = Spire.TRIGGER_CONDITION_MANUAL
	await get_tree().process_frame
	second.mouse_entered.emit()
	check(effect.get("build_count") == 1, "Switching to Manual must remove automatic subscriptions")
	effect.trigger_condition = Spire.TRIGGER_CONDITION_MOUSE_ENTER
	await get_tree().process_frame
	remove_child(effect)
	second.mouse_entered.emit()
	check(effect.get("build_count") == 1, "An exited blueprint must disconnect its mouse trigger")
	add_child(effect)
	await get_tree().process_frame
	second.mouse_entered.emit()
	check(effect.get("build_count") == 2, "Reentry must reconnect the mouse trigger once")
	for target in [Area2D.new(), Area3D.new()]:
		add_child(target)
		var click: TweenBlueprint = CUSTOM.new()
		click.trigger_source = target
		click.trigger_condition = Spire.TRIGGER_CONDITION_MOUSE_CLICK_LEFT
		add_child(click)
		var event := InputEventMouseButton.new()
		event.button_index = MOUSE_BUTTON_LEFT
		event.pressed = true
		if target is Area2D:
			target.input_event.emit(get_viewport(), event, 0)
		else:
			target.input_event.emit(get_viewport().get_camera_3d(), event, Vector3.ZERO, Vector3.UP, 0)
		check(click.get("build_count") == 1, "Collision input signatures must use the event argument")
	return failures.is_empty()

func test_blueprint_ready_enter_and_parent_managed_triggers() -> bool:
	var effect: TweenBlueprint = CUSTOM.new()
	effect.trigger_condition = Spire.TRIGGER_CONDITION_AFTER_READY
	effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	add_child(effect)
	check(effect.get("build_count") == 1, "AfterReady must play once")
	remove_child(effect)
	add_child(effect)
	await get_tree().process_frame
	check(effect.get("build_count") == 1, "AfterReady must not repeat on ordinary reentry")
	effect.trigger_condition = Spire.TRIGGER_CONDITION_AFTER_ENTER_TREE
	remove_child(effect)
	add_child(effect)
	await get_tree().process_frame
	check(effect.get("build_count") == 2, "AfterEnterTree must play on each new entry")
	var parent := TweenBlueprint.new()
	parent.trigger_condition = Spire.TRIGGER_CONDITION_AFTER_READY
	parent.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	var child: TweenBlueprint = CUSTOM.new()
	child.trigger_condition = Spire.TRIGGER_CONDITION_AFTER_READY
	parent.add_child(child)
	add_child(parent)
	check(child.get("build_count") == 1, "A composed child's automatic trigger must not double-play it")
	return failures.is_empty()

func test_blueprint_native_signal_connection_survives_packing() -> bool:
	var scene := Node.new()
	var button := Button.new()
	button.name = "PlayButton"
	scene.add_child(button)
	button.owner = scene
	var effect: TweenBlueprint = CUSTOM.new()
	effect.name = "Effect"
	effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	scene.add_child(effect)
	effect.owner = scene
	check(not ClassDB.class_get_enum_constants(&"Spire", &"TriggerCondition").has("TRIGGER_CONDITION_SIGNAL"), "The Signal trigger constant must not be registered")
	check(not effect.get_property_list().any(func(info): return info.name == &"trigger_signal"), "The signal picker must not be exported")
	check(not effect.has_method("set_trigger_signal"), "The signal configuration API must be removed")
	check(button.pressed.connect(Callable(effect, &"play"), Object.CONNECT_PERSIST) == OK, "Godot must connect a native signal directly to play")
	var packed := PackedScene.new()
	check(packed.pack(scene) == OK, "A scene with a persistent native connection must pack")
	scene.free()
	var restored := packed.instantiate()
	add_child(restored)
	var restored_button: Button = restored.get_node("PlayButton")
	var restored_effect: TweenBlueprint = restored.get_node("Effect")
	check(restored_effect.trigger_condition == Spire.TRIGGER_CONDITION_MANUAL, "Editor-connected playback must work in Manual mode")
	check(restored_effect.get("build_count") == 0, "Restoring a connection must not play automatically")
	restored_button.pressed.emit()
	check(restored_effect.get("build_count") == 1, "A saved native connection must invoke play without an addon Signal trigger")
	return failures.is_empty()

func test_blueprint_mouse_trigger_filters_buttons_and_release() -> bool:
	var target := Control.new()
	add_child(target)
	for trigger in [Spire.TRIGGER_CONDITION_MOUSE_ENTER, Spire.TRIGGER_CONDITION_MOUSE_EXIT, Spire.TRIGGER_CONDITION_MOUSE_CLICK_LEFT, Spire.TRIGGER_CONDITION_MOUSE_CLICK_RIGHT]:
		var effect: TweenBlueprint = CUSTOM.new()
		effect.trigger_condition = trigger
		effect.trigger_source = target
		effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
		add_child(effect)
		if trigger == Spire.TRIGGER_CONDITION_MOUSE_ENTER:
			target.mouse_entered.emit()
		elif trigger == Spire.TRIGGER_CONDITION_MOUSE_EXIT:
			target.mouse_exited.emit()
		else:
			var event := InputEventMouseButton.new()
			event.button_index = MOUSE_BUTTON_RIGHT if trigger == Spire.TRIGGER_CONDITION_MOUSE_CLICK_LEFT else MOUSE_BUTTON_LEFT
			event.pressed = true
			target.gui_input.emit(event)
			check(effect.get("build_count") == 0, "Other mouse buttons must not play")
			event.button_index = MOUSE_BUTTON_LEFT if trigger == Spire.TRIGGER_CONDITION_MOUSE_CLICK_LEFT else MOUSE_BUTTON_RIGHT
			event.pressed = false
			target.gui_input.emit(event)
			check(effect.get("build_count") == 0, "Mouse releases must not play")
			event.pressed = true
			target.gui_input.emit(event)
		check(effect.get("build_count") == 1, "The configured mouse event must play exactly once")
	return failures.is_empty()

func test_blueprint_custom_hooks_and_signal_identity() -> bool:
	var effect: TweenBlueprint = CUSTOM.new()
	effect.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	add_child(effect)
	var events: Array = []
	effect.tween_spawned.connect(func(sequence): events.append(["spawn", sequence]))
	effect.tween_loop_finished.connect(func(sequence): events.append(["loop", sequence]))
	effect.tween_stopped.connect(func(sequence): events.append(["stop", sequence]))
	effect.tween_finished.connect(func(sequence): events.append(["finish", sequence]))
	var sequence := play(effect)
	if not sequence: return false
	check(effect.get("reset_count") == 1 and effect.get("build_count") == 1, "GDScript virtuals must access their own fields without borrowed guards")
	sequence.custom_step(0.6)
	check(events.map(func(event): return event[0]) == ["spawn", "loop", "stop", "finish"], "Blueprint signals must preserve native order")
	for event in events:
		check(event[1] == sequence, "Every event must carry the exact sequence")
	return failures.is_empty()
