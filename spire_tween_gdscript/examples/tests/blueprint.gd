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
	if kind == Spire.BLUEPRINT_KIND_CHARACTERS:
		result.character_delay_ease_source = Spire.EASE_CURVE
		result.character_delay_ease_curve = delay_curve(0, 0)
	parent.add_child(result)
	return result

func property_step(parent: Node, target: Node, property: StringName, end: Variant, duration: float = 1.0) -> TweenBlueprint:
	var result := blueprint(parent, Spire.BLUEPRINT_KIND_PROPERTY)
	result.target = target
	result.animated_property = property
	result.end_value = end
	result.speed_or_duration = duration
	result.ease_enum = Spire.EASE_LINEAR
	return result

func play(effect: TweenBlueprint) -> SpireSequence:
	var sequence := effect.play()
	check(sequence != null, "Blueprint.play must return a usable sequence")
	if sequence:
		sequences.append(sequence)
	return sequence

func character_step(parent: Node, target: RichTextLabel, resource: Resource, end: Variant) -> TweenBlueprint:
	var result := blueprint(parent, Spire.BLUEPRINT_KIND_CHARACTERS)
	result.target = target
	var type := resource.get_class()
	resource.set("punch" if "Punch" in type else ("strength" if "Shake" in type else "to"), end)
	result.character_effect = resource
	return result

func test_blueprint_character_constructors() -> bool:
	var label := SpireRichTextLabel.new()
	label.text = "ABC"
	add_child(label)
	var animator := label
	var cases := [
		[SpireTextOffsetEffect.new(), Vector2(12, 8), "get_char_offset", Vector2.ZERO],
		[SpireTextFadeEffect.new(), 0.2, "get_char_alpha", 1.0],
		[SpireTextColorEffect.new(), Color(0.2, 0.4, 0.6), "get_char_color", Color.WHITE],
		[SpireTextRotationEffect.new(), Vector3(0, 0, 1), "get_char_rotation", Vector3.ZERO],
		[SpireTextScaleEffect.new(), Vector2(2, 3), "get_char_scale", Vector2.ONE],
		[SpireTextPunchOffsetEffect.new(), Vector2(12, 8), "get_char_offset", Vector2.ZERO],
		[SpireTextPunchRotationEffect.new(), Vector3(0.2, 0.3, 0.4), "get_char_rotation", Vector3.ZERO],
		[SpireTextPunchScaleEffect.new(), Vector2(0.2, 0.4), "get_char_scale", Vector2.ONE],
		[SpireTextShakeOffsetEffect.new(), Vector2(12, 8), "get_char_offset", Vector2.ZERO],
		[SpireTextShakeRotationEffect.new(), Vector3(0.2, 0.3, 0.4), "get_char_rotation", Vector3.ZERO],
		[SpireTextShakeScaleEffect.new(), Vector2(0.2, 0.4), "get_char_scale", Vector2.ONE],
	]
	for item in cases:
		animator.reset()
		for index in 3: animator.set_char_color(index, Color.WHITE)
		var effect := character_step(self, label, item[0], item[1])
		var sequence := play(effect)
		sequence.custom_step(0.173)
		for index in 3:
			check(not same(animator.call(item[2], index), item[3]), "Character constructor must advance every selected index: %s" % item[0])
		sequence.force_complete()
		for index in 3:
			var impulse: bool = "Punch" in item[0].get_class() or "Shake" in item[0].get_class()
			check(same(animator.call(item[2], index), item[3] if impulse else item[1]), "Character completion must reach its endpoint or restore its contribution: %s" % item[0])
		# Every supported resource must also reject invalid timing in a Blueprint.
		animator.reset()
		item[0].duration = 0.0
		play(effect).force_complete()
		check(same(animator.call(item[2], 0), item[3]), "Invalid resource timing must skip the character step: %s" % item[0])
	return failures.is_empty()

func delay_curve(from: float, to: float) -> Curve:
	var curve := Curve.new()
	curve.min_value = minf(0, minf(from, to))
	curve.max_value = maxf(1, maxf(from, to))
	curve.add_point(Vector2(0, from), 0, to - from)
	curve.add_point(Vector2(1, to), to - from, 0)
	return curve

func test_blueprint_ease_sources_and_serialization() -> bool:
	var target := Node2D.new()
	add_child(target)
	var property := property_step(target, target, &"position", Vector2(10, 0))
	property.name = "Property"
	property.owner = target
	var curve := delay_curve(0.25, 0.25)
	property.ease_curve = curve
	play(property).custom_step(0.5)
	check(same(target.position, Vector2(5, 0)), "Enum mode must ignore the assigned inactive Curve")
	target.position = Vector2.ZERO
	property.ease_source = Spire.EASE_CURVE
	play(property).custom_step(0.5)
	check(same(target.position, Vector2(2.5, 0)), "Curve mode must use the assigned Curve")
	target.position = Vector2.ZERO
	property.ease_curve = null
	play(property).custom_step(0.5)
	check(same(target.position, Vector2(5, 0)), "A missing Curve must fall back to the selected Ease")
	property.ease_curve = curve
	property.ease_source = Spire.EASE_ENUM
	var label := SpireRichTextLabel.new()
	label.text = "AB"
	label.name = "Text"
	target.add_child(label)
	label.owner = target
	var resource := SpireTextOffsetEffect.new()
	resource.ease = Spire.EASE_IN_QUAD
	var characters := character_step(label, label, resource, Vector2(10, 0))
	characters.name = "Characters"
	characters.owner = target
	characters.ease_enum = Spire.EASE_OUT_QUAD
	characters.ease_curve = curve
	play(characters).custom_step(0.5)
	check(same(label.get_char_offset(0), Vector2(7.5, 0)), "An explicit Enum must override resource easing")
	label.reset()
	characters.ease_source = Spire.EASE_CURVE
	play(characters).custom_step(0.5)
	check(same(label.get_char_offset(0), Vector2(2.5, 0)), "Character Curve mode must override the inactive Enum")
	label.reset()
	characters.ease_curve = null
	play(characters).custom_step(0.5)
	check(same(label.get_char_offset(0), Vector2(7.5, 0)), "Characters must fall back to their selected Ease when Curve mode has no resource")
	characters.ease_curve = curve
	characters.ease_source = Spire.EASE_ENUM
	characters.character_delay_ease_curve = delay_curve(0, 0.5)
	characters.character_delay_ease_source = Spire.EASE_ENUM
	characters.character_delay_ease_enum = Spire.EASE_OUT_QUAD
	var packed := PackedScene.new()
	check(packed.pack(target) == OK, "Source selectors and inactive settings must pack")
	var restored := packed.instantiate()
	var restored_property: TweenBlueprint = restored.get_node("Property")
	check(restored_property.ease_source == Spire.EASE_ENUM and restored_property.ease_enum == Spire.EASE_LINEAR and restored_property.ease_curve == curve, "Packing must preserve inactive property curves")
	var restored_characters: TweenBlueprint = restored.get_node("Text/Characters")
	check(restored_characters != null, "The authored character Blueprint must serialize")
	if restored_characters != null:
		check(restored_characters.ease_source == Spire.EASE_ENUM and restored_characters.ease_enum == Spire.EASE_OUT_QUAD and restored_characters.ease_curve == curve, "Packing must preserve inactive character easing curves")
		check(restored_characters.character_delay_ease_source == Spire.EASE_ENUM and restored_characters.character_delay_ease_enum == Spire.EASE_OUT_QUAD and restored_characters.character_delay_ease_curve is Curve and near(restored_characters.character_delay_ease_curve.sample(1), 0.5), "Packing must preserve inactive character-delay curves")
	restored.free()
	return failures.is_empty()

func test_blueprint_character_default_basic_curve_and_serialization() -> bool:
	var label := SpireRichTextLabel.new()
	label.text = "ABC"
	add_child(label)
	var step := TweenBlueprint.new()
	step.name = "Animation"
	step.kind = Spire.BLUEPRINT_KIND_CHARACTERS
	step.target = label
	step.tween_process_mode = Spire.PROCESS_MODE_MANUAL
	var effect := SpireTextOffsetEffect.new()
	effect.to = Vector2(10, 0)
	step.character_effect = effect
	label.add_child(step)
	step.owner = label
	check(step.character_delay_ease_source == Spire.EASE_ENUM and step.character_delay_ease_enum == Spire.EASE_LINEAR, "Character delays must default to the Linear Ease")
	var sequence := play(step)
	sequence.custom_step(0.5)
	check(same(label.get_char_offset(0), Vector2(5, 0)) and same(label.get_char_offset(1), Vector2.ZERO), "The default basic curve must provide linear per-character start delays")
	sequence.custom_step(0.75)
	check(same(label.get_char_offset(2), Vector2(2.5, 0)) and sequence.is_playing(), "The last character must receive its full per-character duration")
	sequence.custom_step(0.75)
	check(same(label.get_char_offset(2), Vector2(10, 0)), "Each character must finish by the nominal two-second range duration")
	# Sequences finish on positive excess after an exact child boundary.
	sequence.custom_step(0.001)
	check(sequence.is_stopped(), "The default linear delay curve must finish the range after two per-character durations")
	label.reset()
	step.character_delay_ease_enum = Spire.EASE_OUT_QUAD
	play(step).custom_step(1.0)
	check(same(label.get_char_offset(1), Vector2(2.5, 0)), "A built-in ease must define the spatial start-delay curve")
	var packed := PackedScene.new()
	check(packed.pack(label) == OK, "The basic curve variant must pack")
	var restored := packed.instantiate()
	check(restored.get_node("Animation").character_delay_ease_source == Spire.EASE_ENUM and restored.get_node("Animation").character_delay_ease_enum == Spire.EASE_OUT_QUAD, "Scene serialization must preserve the delay source and Ease")
	restored.free()
	return failures.is_empty()

func test_blueprint_character_delay_start_loops_and_restart() -> bool:
	var label := SpireRichTextLabel.new()
	label.text = "ABC"
	add_child(label)
	var animator := label
	var effect := character_step(self, label, SpireTextOffsetEffect.new(), Vector2(10, 0))
	effect.character_first = 1
	effect.character_last = 2
	effect.character_delay_ease_curve = delay_curve(0, 0.25)
	effect.delay = 0.5
	effect.start_enabled = true
	effect.character_effect.from = Vector2(2, 0)
	var first := play(effect)
	check(animator.get_char_offset(1) == Vector2.ZERO, "Building must not apply explicit character starts")
	first.custom_step(0.25)
	check(animator.get_char_offset(1) == Vector2.ZERO, "The Blueprint delay must precede explicit character starts")
	first.custom_step(0.35)
	check(same(animator.get_char_offset(2), Vector2(2, 0)), "Explicit starts must initialize the whole selection before staggered animation begins")
	first.custom_step(0.16)
	check(same(animator.get_char_offset(1), Vector2(4.08, 0)), "The first selected character must interpolate from its explicit start")
	check(same(animator.get_char_offset(2), Vector2(2.08, 0)), "Stagger must begin at the selected range, after that character's delay: %s" % animator.get_char_offset(2))
	check(animator.get_char_offset(0) == Vector2.ZERO, "Unselected characters must remain untouched")
	effect.start_enabled = false
	effect.delay = 0.0
	effect.character_delay_ease_curve = delay_curve(0, 0)
	var second := play(effect)
	check(first.is_stopped() and second != first, "Character replay must stop the previous run and build a fresh sequence")
	animator.set_char_offset(1, Vector2(6, 0))
	second.custom_step(0.5)
	check(same(animator.get_char_offset(1), Vector2(8, 0)), "Implicit character starts must be captured when processing begins")
	second.stop()
	animator.reset()
	effect.character_loops = 2
	effect.character_effect.loop_mode = Spire.LOOP_MODE_YOYO
	var yoyo := play(effect)
	for _tick in 3: yoyo.custom_step(0.5)
	check(same(animator.get_char_offset(1), Vector2(5, 0)), "Character loops must support yoyo independently of sequence loops: %s" % animator.get_char_offset(1))
	yoyo.custom_step(0.5)
	check(same(animator.get_char_offset(1), Vector2.ZERO), "Yoyo must restore the captured start: %s" % animator.get_char_offset(1))
	animator.reset()
	var sprite := Node2D.new()
	add_child(sprite)
	var serial := blueprint(self)
	property_step(serial, sprite, &"position:x", 10.0)
	var later := character_step(serial, label, SpireTextOffsetEffect.new(), Vector2(10, 0))
	later.start_enabled = true
	later.character_effect.from = Vector2(2, 0)
	var chain := play(serial)
	chain.custom_step(0.5)
	check(animator.get_char_offset(0) == Vector2.ZERO, "A later serial character step must not initialize its start early")
	chain.custom_step(0.75)
	check(same(animator.get_char_offset(0), Vector2(4, 0)), "A serial character step must initialize and advance when reached")
	return failures.is_empty()

func test_blueprint_character_build_refreshes_text() -> bool:
	var label := SpireRichTextLabel.new()
	label.text = "A"
	add_child(label)
	var step := character_step(self, label, SpireTextOffsetEffect.new(), Vector2(3, 4))
	label.text = "ABCD"
	check(label.get_character_count() == 1, "Character getters must keep the last refreshed state")
	var sequence := play(step)
	check(label.get_character_count() == 4, "Blueprint range selection must explicitly synchronize current text")
	sequence.force_complete()
	check(label.get_char_offset(3) == Vector2(3, 4), "A fresh blueprint run must select the updated full range")
	return failures.is_empty()

func test_blueprint_character_invalid_configuration_and_lifetime() -> bool:
	var label := SpireRichTextLabel.new()
	label.text = "ABCDE"
	add_child(label)
	var animator := label
	var sprite := Node2D.new()
	add_child(sprite)
	var plain_label := RichTextLabel.new()
	plain_label.text = "ABCDE"
	add_child(plain_label)
	var invalid_cases := [
		["target", sprite], ["target", plain_label], ["character_first", -1], ["character_first", 99], ["character_last", 99],
		["character_last", -2], ["character_delay_ease_curve", delay_curve(-0.1, -0.1)],
		["character_loops", 0], ["character_effect", null], ["character_effect", Resource.new()],
		["start_value", "wrong type"],
	]
	for item in invalid_cases:
		var container := blueprint(self)
		var invalid := character_step(container, label, SpireTextPunchOffsetEffect.new() if item[0] == "start_value" else SpireTextOffsetEffect.new(), Vector2(10, 0))
		if item[0] == "start_value": invalid.start_enabled = true
		invalid.set(item[0], item[1])
		property_step(container, sprite, &"position:x", 5.0)
		sprite.position = Vector2.ZERO
		play(container).force_complete()
		check(near(sprite.position.x, 5.0), "Invalid character steps must leave valid following siblings playable: " + item[0])
		check(animator.get_char_offset(0) == Vector2.ZERO, "Invalid character settings must not mutate the label")
	var invalid_resources := [
		[SpireTextRotationEffect.new(), "pivot", Vector2(INF, 0)],
		[SpireTextOffsetEffect.new(), "duration", NAN],
		[SpireTextOffsetEffect.new(), "to", Vector2(NAN, 0)],
		[SpireTextPunchOffsetEffect.new(), "vibrato", 0],
		[SpireTextPunchOffsetEffect.new(), "elasticity", NAN],
		[SpireTextPunchOffsetEffect.new(), "elasticity", 2.0],
		[SpireTextShakeOffsetEffect.new(), "randomness", NAN],
		[SpireTextShakeOffsetEffect.new(), "randomness", 181.0],
	]
	for item in invalid_resources:
		var container := blueprint(self)
		var invalid := blueprint(container, Spire.BLUEPRINT_KIND_CHARACTERS)
		invalid.target = label
		invalid.character_effect = item[0]
		item[0].set(item[1], item[2])
		property_step(container, sprite, &"position:x", 5.0)
		sprite.position = Vector2.ZERO
		play(container).force_complete()
		check(near(sprite.position.x, 5.0) and animator.get_char_offset(0) == Vector2.ZERO, "Invalid resource settings must leave valid siblings playable: " + item[1])
	var delayed := character_step(self, label, SpireTextOffsetEffect.new(), Vector2(0, -24))
	delayed.character_first = 4
	delayed.character_last = 4
	delayed.delay = 0.5
	var sequence := play(delayed)
	label.text = "HI!"
	label.refresh()
	sequence.custom_step(0.6)
	check(not sequence.is_stopped() and animator.get_character_count() == 3, "A disappeared delayed character must preserve playback")
	sequence.force_complete()
	check(sequence.is_stopped(), "Playback with a missing character must still complete normally")
	check(animator.get_char_offset(0) == Vector2.ZERO, "Missing indices must not redirect writes to a surviving character")
	label.text = "ABCDE"
	sequence = play(delayed)
	label.free()
	sequence.custom_step(2.0)
	check(sequence.is_stopped(), "Freeing the RichTextLabel must cancel character playback")
	return failures.is_empty()

func test_blueprint_character_resource_from_values() -> bool:
	var label := SpireRichTextLabel.new()
	label.text = "AB"
	add_child(label)
	var cases := [
		[SpireTextColorEffect.new(), Color.RED, Color.BLUE, Color(0.75, 0, 0.25), "get_char_color"],
		[SpireTextFadeEffect.new(), 0.2, 0.6, 0.3, "get_char_alpha"],
		[SpireTextOffsetEffect.new(), Vector2(-8, 0), Vector2(8, 4), Vector2(-4, 1), "get_char_offset"],
		[SpireTextScaleEffect.new(), Vector2(2, 3), Vector2(4, 7), Vector2(2.5, 4), "get_char_scale"],
		[SpireTextRotationEffect.new(), Vector3(0.2, 0, 0), Vector3(0.6, 0, 0), Vector3(0.3, 0, 0), "get_char_rotation"],
	]
	for item in cases:
		label.reset()
		var entry: Resource = item[0]
		entry.set("from", item[1])
		var step := character_step(self, label, entry, item[2])
		step.start_enabled = true
		step.character_delay_ease_curve = delay_curve(0, 0.5)
		var sequence := play(step)
		sequence.custom_step(0.25)
		check(same(label.call(item[4], 0), item[3]), "Basic resource from/to must interpolate the first selected character")
		check(same(label.call(item[4], 1), item[1]), "Basic resource from must initialize the remaining characters before their delay")
		sequence.stop()
	return failures.is_empty()

func test_blueprint_character_resource_timing_and_curve() -> bool:
	var label := SpireRichTextLabel.new()
	label.text = "AB"
	add_child(label)
	var entry := SpireTextOffsetEffect.new()
	entry.duration = 2.0
	entry.ease = Spire.EASE_IN_QUAD
	entry.character_phase_offset = 0.75
	var step := character_step(self, label, entry, Vector2(16, 0))
	step.start_enabled = true
	entry.from = Vector2(4, 0)
	var sequence := play(step)
	sequence.custom_step(1.0)
	for index in 2:
		check(same(label.get_char_offset(index), Vector2(7, 0)), "Resource duration/ease/from must drive the tween; preset phase must not act as a start delay")
	sequence.stop()
	var curve := Curve.new()
	curve.add_point(Vector2(0, 0.25))
	curve.add_point(Vector2(1, 0.25))
	step.ease_source = Spire.EASE_CURVE
	step.ease_curve = curve
	sequence = play(step)
	sequence.custom_step(1.5)
	check(same(label.get_char_offset(0), Vector2(7, 0)), "The Blueprint curve must override the resource's basic easing")
	return failures.is_empty()

func test_blueprint_hidden_character_resource_preserves_property_values() -> bool:
	var target := Node2D.new()
	add_child(target)
	var step := property_step(self, target, &"position", Vector2(5, 6))
	step.character_effect = SpireTextPunchScaleEffect.new()
	check(step.start_value == null, "A hidden character resource must not initialize property values")
	step.start_value = Vector2.ONE
	step.character_effect = SpireTextPunchRotationEffect.new()
	check(typeof(step.start_value) == TYPE_VECTOR2 and step.start_value == Vector2.ONE, "Replacing a hidden character resource must preserve the authored property start")
	step.start_value = null
	step.kind = Spire.BLUEPRINT_KIND_CHARACTERS
	check(typeof(step.start_value) == TYPE_VECTOR3 and step.start_value == Vector3.ZERO, "Switching to Characters must initialize an unset impulse start synchronously")
	return failures.is_empty()

func test_blueprint_character_impulse_default_is_synchronous() -> bool:
	var label := SpireRichTextLabel.new()
	label.text = "A"
	add_child(label)
	for item in [[SpireTextPunchOffsetEffect.new(), Vector2.ZERO], [SpireTextPunchScaleEffect.new(), Vector2.ONE], [SpireTextPunchRotationEffect.new(), Vector3.ZERO]]:
		var step := blueprint(self, Spire.BLUEPRINT_KIND_CHARACTERS)
		step.target = label
		step.character_effect = item[0]
		step.start_enabled = true
		check(step.start_value == item[1], "Assigning a resource must initialize its neutral impulse start before an immediate play")
		play(step).force_complete()
	return failures.is_empty()

func test_blueprint_character_impulse_start_and_blending() -> bool:
	var label := SpireRichTextLabel.new()
	label.text = "A"
	add_child(label)
	for blendable in [true, false]:
		var entry := SpireTextPunchScaleEffect.new()
		entry.punch = Vector2(0.4, 0.2)
		entry.vibrato = 4
		entry.elasticity = 0.4
		var step := blueprint(self, Spire.BLUEPRINT_KIND_CHARACTERS)
		step.target = label
		step.character_effect = entry
		step.start_enabled = true
		step.start_value = Vector2(2, 3)
		step.character_blendable = blendable
		var sequence := play(step)
		sequence.custom_step(0.1)
		check(same(label.get_char_scale(0), Vector2(2.4, 3.2)), "Impulse resources must initialize their Blueprint start before sampling")
		label.set_char_scale(0, label.get_char_scale(0) + Vector2.ONE)
		sequence.force_complete()
		check(same(label.get_char_scale(0), Vector2(3, 4) if blendable else Vector2(2, 3)), "Blendable playback must preserve external contributions; nonblendable playback restores its initial value")
	return failures.is_empty()

func test_blueprint_character_delay_ease_curve_watch_and_snapshot() -> bool:
	var label := SpireRichTextLabel.new()
	label.text = "AB"
	add_child(label)
	var curve := delay_curve(0, 1)
	var step := character_step(self, label, SpireTextOffsetEffect.new(), Vector2(10, 0))
	step.character_delay_ease_curve = curve
	check(curve.get_signal_connection_list("changed").size() == 1, "The Blueprint must watch its delay curve")
	var running := play(step)
	running.custom_step(0.25)
	curve.set_point_value(1, 0.5)
	running.custom_step(0.25)
	check(same(label.get_char_offset(1), Vector2.ZERO), "Editing the curve must preserve an active range's sampled start delays")
	label.reset()
	var next := play(step)
	next.custom_step(0.75)
	check(same(label.get_char_offset(1), Vector2(2.5, 0)), "A subsequent play must snapshot the edited delay curve")
	step.character_delay_ease_curve = curve
	check(curve.get_signal_connection_list("changed").size() == 1, "Reassigning a delay curve must replace its watch")
	var replacement := delay_curve(0, 0)
	step.character_delay_ease_curve = replacement
	check(curve.get_signal_connection_list("changed").is_empty(), "Replacing the curve must disconnect its old watch")
	var connections := replacement.get_signal_connection_list("changed")
	replacement.changed.disconnect(connections[0].callable)
	step.character_delay_ease_curve = null
	step.character_delay_ease_curve = curve
	step.free()
	check(curve.get_signal_connection_list("changed").is_empty(), "Freeing the Blueprint must disconnect the curve watch")
	curve.emit_changed()
	replacement.emit_changed()
	return failures.is_empty()

func test_blueprint_character_shared_resource_and_watch_lifetime() -> bool:
	var entry := SpireTextOffsetEffect.new()
	entry.to = Vector2(10, 0)
	entry.duration = 2.0
	entry.ease = Spire.EASE_IN_QUAD
	var preset := SpireRichTextEffectPreset.new()
	preset.bbcode = "bob"
	preset.effects = [entry]
	var a := SpireRichTextLabel.new()
	a.bbcode_enabled = true
	a.text = "[bob]AB[/bob]"
	a.effect_presets = [preset]
	add_child(a)
	var b := SpireRichTextLabel.new()
	b.text = "AB"
	add_child(b)
	var first := blueprint(self, Spire.BLUEPRINT_KIND_CHARACTERS)
	first.target = a
	first.character_effect = entry
	first.start_enabled = true
	var second := blueprint(self, Spire.BLUEPRINT_KIND_CHARACTERS)
	second.target = b
	second.character_effect = entry
	second.start_enabled = true
	check(entry.get_signal_connection_list("changed").size() == 3, "Two Blueprint consumers and the preset renderer must watch the same resource independently")
	var running := play(first)
	running.custom_step(0.5)
	check(same(a.get_char_offset(0), Vector2(0.625, 0)), "The first run must capture the resource's initial configuration")
	entry.to = Vector2(20, 0)
	entry.duration = 1.0
	for _frame in 4: await next_frame()
	check(a.get_parsed_text() == "AB" and same(a.get_char_offset(0), Vector2(0.625, 0)), "Preset resource refreshes must preserve the Blueprint's code-controlled state")
	running.custom_step(0.5)
	check(same(a.get_char_offset(0), Vector2(2.5, 0)), "Editing a resource must not mutate an already-built tween")
	play(second).custom_step(0.5)
	check(same(b.get_char_offset(0), Vector2(5, 0)), "The next run must use the edited resource")
	# Reassignment must not accumulate connections, including externally removed watches.
	second.character_effect = entry
	check(entry.get_signal_connection_list("changed").size() == 3, "Reassigning the same resource must replace its watch")
	var replacement := SpireTextFadeEffect.new()
	second.character_effect = replacement
	check(entry.get_signal_connection_list("changed").size() == 2, "Replacing a resource must disconnect the old watch")
	var connections := replacement.get_signal_connection_list("changed")
	replacement.changed.disconnect(connections[0].callable)
	second.character_effect = null
	second.free()
	first.free()
	check(entry.get_signal_connection_list("changed").size() == 1, "Freeing Blueprints must leave only the label's subscription")
	a.free()
	check(entry.get_signal_connection_list("changed").is_empty(), "Freeing the label must release its final watch")
	entry.emit_changed()
	replacement.emit_changed()
	return failures.is_empty()

func test_blueprint_character_rotation_pivot_and_serialization() -> bool:
	var root := Node.new()
	var label := SpireRichTextLabel.new()
	label.name = "Label"
	label.text = "X"
	root.add_child(label)
	label.owner = root
	add_child(root)
	var effect := character_step(label, label, SpireTextRotationEffect.new(), Vector3(0, 0, TAU))
	effect.name = "Spin"
	effect.owner = root
	effect.character_effect.pivot = Vector2(12, -14)
	effect.character_effect.rotate_mode = Spire.ROTATE_MODE_FAST_BEYOND_360
	effect.character_delay_ease_curve = delay_curve(0, 1)
	effect.character_loops = 2
	effect.character_effect.loop_mode = Spire.LOOP_MODE_YOYO
	var animator := label
	var sequence := play(effect)
	check(animator.get_char_pivot(0) == Vector2.ZERO, "Construction must not set the pivot")
	sequence.custom_step(0.5)
	check(same(animator.get_char_rotation(0), Vector3(0, 0, PI)), "Beyond-360 rotation must preserve full turns")
	check(animator.get_char_pivot(0) == Vector2(12, -14), "Rotation must apply the authored pixel pivot")
	sequence.stop()
	var packed := PackedScene.new()
	check(packed.pack(root) == OK, "Character settings must pack")
	var copy := packed.instantiate()
	var restored: TweenBlueprint = copy.get_node("Label/Spin")
	check(restored.kind == Spire.BLUEPRINT_KIND_CHARACTERS and restored.character_effect is SpireTextRotationEffect, "Character kind and resource type must serialize")
	check(restored.character_effect.to == Vector3(0, 0, TAU) and restored.character_effect.pivot == Vector2(12, -14), "Typed resource values and pivots must serialize")
	check(restored.character_effect.rotate_mode == Spire.ROTATE_MODE_FAST_BEYOND_360 and restored.character_loops == 2 and restored.character_effect.loop_mode == Spire.LOOP_MODE_YOYO, "Rotation and loop policies must serialize")
	check(restored.character_delay_ease_curve is Curve and near(restored.character_delay_ease_curve.sample(1), 1), "Character start-delay curves must serialize")
	check(restored.target == copy.get_node("Label"), "Character targets must serialize as relative references")
	copy.free()
	return failures.is_empty()

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
	curved.ease_source = Spire.EASE_CURVE
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
