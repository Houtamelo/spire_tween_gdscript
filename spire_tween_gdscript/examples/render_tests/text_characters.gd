extends Control
## A rendered integration fixture. All character motion comes from Spire tweens.
## Run with scripts/render_text_characters.sh; no host display is needed.

const FPS := 25
const FRAME_COUNT := 250
const SIZE := Vector2i(960, 820)
const INK := Color("edf3ff")
const MUTED := Color("93a3c0")
const MINT := Color("67e8c2")
const CORAL := Color("ff9a9e")
const GOLD := Color("ffd38a")
const LILAC := Color("baa8ff")

var _animators: Dictionary = {}
var _labels: Dictionary = {}
var _active: Array = []
var _events: Dictionary = {}
var _regions: Dictionary = {}
var _pixel_states: Dictionary = {}
var _failures: Array[String] = []
var _phase: Label
var _lifetime_status: Label
var _progress: ColorRect
var _finished_count := 0
var _rebuild_events := [0, 0]
var _rebuild_tween: SpireCharTween
var _output := ""
var _captured_frames := 0


func _ready() -> void:
	for argument in OS.get_cmdline_user_args():
		if argument.begins_with("--frames-dir="):
			_output = argument.trim_prefix("--frames-dir=")
	if _output.is_empty() or DisplayServer.get_name() == "headless":
		push_error("Render fixture requires a rendered display and --frames-dir=PATH.")
		get_tree().quit(1)
		return
	if DirAccess.make_dir_recursive_absolute(_output) != OK:
		push_error("Cannot create frame directory: " + _output)
		get_tree().quit(1)
		return
	get_window().size = SIZE
	get_window().content_scale_size = SIZE
	get_window().content_scale_mode = Window.CONTENT_SCALE_MODE_DISABLED
	_build_showcase()
	# Baseline BBCode colors and cluster mappings must come from a real draw.
	await RenderingServer.frame_post_draw
	await get_tree().process_frame
	_prepare_reveal()
	_schedule()
	for frame in FRAME_COUNT:
		await get_tree().process_frame
		if _events.has(frame):
			for action in _events[frame]:
				action.call()
		if frame > 0:
			for tween in _active:
				tween.custom_step(1.0 / FPS)
			_active = _active.filter(func(tween): return tween.get_state() != Spire.STATE_STOPPED)
		_progress.size.x = 896.0 * float(frame) / (FRAME_COUNT - 1)
		await RenderingServer.frame_post_draw
		var image := get_viewport().get_texture().get_image()
		_check(image != null and not image.is_empty(), "Rendered viewport must contain an image.")
		if image == null or image.is_empty():
			break
		_check(image.get_size() == SIZE, "Capture size must be 960x820.")
		if frame % 5 == 0:
			for key in _regions:
				var digest := hash(image.get_region(_regions[key]).get_data())
				_pixel_states[key][digest] = true
		var error := image.save_png(_output.path_join("frame_%04d.png" % frame))
		_check(error == OK, "PNG write failed at frame %d: %s" % [frame, error])
		if error != OK:
			break
		_captured_frames += 1
	_validate_final_state()
	_check(_captured_frames == FRAME_COUNT, "All 250 frames must be written.")
	var changes := {}
	for key in _pixel_states:
		changes[key] = _pixel_states[key].size()
		_check(changes[key] >= 3, "%s must visibly animate, got %d distinct samples." % [key, changes[key]])
	var report := {
		"name": "text_characters", "frames": _captured_frames, "fps": FPS,
		"size": [SIZE.x, SIZE.y], "engine": Engine.get_version_info().string,
		"display": DisplayServer.get_name(), "renderer": RenderingServer.get_video_adapter_name(),
		"distinct_text_images": changes, "completed_tweens": _finished_count,
		"missing_index_stopped": _rebuild_events[0], "missing_index_finished": _rebuild_events[1],
		"failures": _failures,
	}
	var report_file := FileAccess.open(_output.path_join("../result.json"), FileAccess.WRITE)
	_check(report_file != null, "Result report must be writable.")
	if report_file != null:
		report_file.store_string(JSON.stringify(report, "\t") + "\n")
		report_file.close()
	for tween in _active:
		tween.stop()
	_active.clear()
	_rebuild_tween = null
	print("RENDER_TEST_RESULT name=text_characters frames=%d panels=8 samples=%d failed=%d fps=%d" % [
		_captured_frames, _regions.size(), _failures.size(), FPS])
	get_tree().quit(0 if _failures.is_empty() else 1)


func _check(condition: bool, message: String) -> void:
	if not condition:
		_failures.append(message)
		push_error(message)


func _text(content: String, at: Vector2, font_size: int, color: Color = INK) -> Label:
	var label := Label.new()
	label.position = at
	label.text = content
	label.add_theme_font_override("font", ThemeDB.fallback_font)
	label.add_theme_font_size_override("font_size", font_size)
	label.add_theme_color_override("font_color", color)
	add_child(label)
	return label


func _rect(at: Vector2, extent: Vector2, color: Color) -> ColorRect:
	var rect := ColorRect.new()
	rect.position = at
	rect.size = extent
	rect.color = color
	add_child(rect)
	return rect


func _card(key: String, title: String, subtitle: String, area: Rect2i, accent: Color) -> void:
	var panel := Panel.new()
	panel.position = area.position
	panel.size = area.size
	var style := StyleBoxFlat.new()
	style.bg_color = Color("172035")
	style.border_color = Color("29354e")
	style.set_border_width_all(1)
	style.set_corner_radius_all(12)
	panel.add_theme_stylebox_override("panel", style)
	add_child(panel)
	_rect(Vector2(area.position) + Vector2(18, 20), Vector2(3, 14), accent)
	_text(title, Vector2(area.position) + Vector2(30, 15), 15, accent)
	_text(subtitle, Vector2(area.position) + Vector2(18, 42), 12, MUTED)
	# Exclude the card title/captions: activity checks measure the rendered glyphs.
	_regions[key] = Rect2i(area.position + Vector2i(8, 66), area.size - Vector2i(16, 76))
	_pixel_states[key] = {}


func _characters(key: String, content: String, at: Vector2, width: float, font_size: int,
		color: Color = INK) -> SpireRichTextLabel:
	var label := SpireRichTextLabel.new()
	label.position = at
	label.size = Vector2(width, 68)
	label.bbcode_enabled = true
	label.scroll_active = false
	label.mouse_filter = Control.MOUSE_FILTER_IGNORE
	label.clip_contents = false
	label.add_theme_font_override("normal_font", ThemeDB.fallback_font)
	label.add_theme_font_size_override("normal_font_size", font_size)
	label.add_theme_color_override("default_color", color)
	label.text = content
	add_child(label)
	var animator := label
	# A caller-specified visual pivot near the middle of this font's capitals.
	for index in animator.get_character_count():
		animator.set_char_pivot(index, Vector2(font_size * 0.30, -font_size * 0.34))
	_animators[key] = animator
	_labels[key] = label
	return animator


func _build_showcase() -> void:
	_rect(Vector2.ZERO, Vector2(SIZE), Color("0c1220"))
	_text("SPIRE TWEEN   /   RENDERED TEST 01", Vector2(32, 20), 13, MINT)
	_text("Characters in motion", Vector2(30, 42), 34)
	_text("SpireRichTextLabel  ·  11 tween constructors  ·  real rendered glyphs", Vector2(32, 90), 14, MUTED)
	_phase = _text("01  /  ARRIVE", Vector2(752, 62), 14, GOLD)
	_card("reveal", "01  STAGGERED REVEAL", "Offset + alpha / per-character delays", Rect2i(32, 134, 288, 164), MINT)
	_characters("reveal", "HELLO SPIRE", Vector2(48, 220), 268, 30, MINT)
	_card("color", "02  COLOR", "BBCode start colors / yoyo", Rect2i(336, 134, 288, 164), CORAL)
	_characters("color", "[color=#ffd38a]PR[/color][color=#ff9a9e]IS[/color][color=#baa8ff]M[/color]", Vector2(375, 215), 240, 44)
	_card("rotation", "03  THREE-AXIS ROTATION", "X tilt          Y tilt          Z spin", Rect2i(640, 134, 288, 164), LILAC)
	_characters("rotation", "X   Y   Z", Vector2(666, 212), 262, 40, LILAC)
	_card("scale", "04  SCALE", "Independent axes / centered pivots", Rect2i(32, 314, 288, 234), GOLD)
	_characters("scale", "BOUNCE", Vector2(65, 414), 250, 38, GOLD)
	_text("Each letter has its own clock.", Vector2(50, 508), 12, MUTED)
	_card("punch", "05  PUNCH", "Additive impulses / restores on finish", Rect2i(336, 314, 288, 234), MINT)
	_card("shake", "06  SHAKE", "Interpolated random paths / fade out", Rect2i(640, 314, 288, 234), CORAL)
	for lane in 3:
		var name := ["OFFSET", "ROTATION", "SCALE"][lane] as String
		_text(name, Vector2(354, 395 + lane * 46), 10, MUTED)
		_text(name, Vector2(658, 395 + lane * 46), 10, MUTED)
		_characters("punch_%d" % lane, "Spire", Vector2(438, 384 + lane * 46), 180, 29, MINT)
		_characters("shake_%d" % lane, "Spire", Vector2(742, 384 + lane * 46), 180, 29, CORAL)
		for prefix in ["punch", "shake"]:
			var key := "%s_%d" % [prefix, lane]
			_regions[key] = Rect2i(430 if prefix == "punch" else 734, 382 + lane * 46, 186, 46)
			_pixel_states[key] = {}
	_card("composition", "07  COMPOSE", "Offset + rotation + scale + color / joined in a sequence", Rect2i(32, 564, 592, 186), LILAC)
	_characters("composition", "SPIRE TWEEN", Vector2(105, 650), 510, 45, LILAC)
	_card("refresh", "08  TEXT REBUILD", "Numeric index disappears → ignored", Rect2i(640, 564, 288, 186), GOLD)
	_characters("refresh", "LONGER", Vector2(686, 640), 230, 31, GOLD)
	_lifetime_status = _text("index 5 is animating", Vector2(658, 712), 12, MUTED)
	_rect(Vector2(32, 776), Vector2(896, 2), Color("29354e"))
	_progress = _rect(Vector2(32, 776), Vector2(0, 2), MINT)
	_text("GODOT %s   /   Xvfb + software OpenGL" % Engine.get_version_info().string, Vector2(32, 789), 11, MUTED)
	_text("10 SECONDS   ·   25 FPS", Vector2(766, 789), 11, MUTED)


func _watch(tween) -> void:
	tween.set_process_mode(Spire.PROCESS_MODE_MANUAL)
	tween.finished.connect(func(): _finished_count += 1)
	_active.append(tween)


func _track(tween: SpireCharTween, ease: int = Spire.EASE_OUT_CUBIC, delay: float = 0.0) -> SpireCharTween:
	tween.set_ease(ease).set_delay(delay)
	_watch(tween)
	return tween


func _at(frame: int, action: Callable) -> void:
	if not _events.has(frame):
		_events[frame] = []
	_events[frame].append(action)


func _prepare_reveal() -> void:
	var animator: SpireRichTextLabel = _animators.reveal
	for index in animator.get_character_count():
		animator.set_char_offset(index, Vector2(0, 20))
		animator.set_char_alpha(index, 0.0)


func _schedule() -> void:
	_at(10, _reveal)
	_at(12, _basic_channels)
	_at(25, _effects)
	_at(60, func(): _phase.text = "02  /  COMPOSE")
	_at(65, _compose)
	_at(82, _start_lifetime)
	_at(112, _shorten_text)
	_at(114, _check_missing_index)
	_at(125, func(): _phase.text = "03  /  REBUILD")
	_at(133, _effects)
	_at(169, _restore_text)
	_at(177, _basic_channels)
	_at(206, _check_effect_restoration)
	_at(226, _reset_all)


func _reveal() -> void:
	var animator: SpireRichTextLabel = _animators.reveal
	for index in animator.get_character_count():
		_track(animator.offset_char(index, Vector2.ZERO, 0.65), Spire.EASE_OUT_CUBIC, index * 0.055)
		_track(animator.fade_char(index, 1.0, 0.5), Spire.EASE_LINEAR, index * 0.055)


func _basic_channels() -> void:
	var colors: SpireRichTextLabel = _animators.color
	for index in colors.get_character_count():
		_track(colors.color_char(index, [MINT, LILAC, CORAL][index % 3], 0.7), Spire.EASE_LINEAR, index * 0.07).set_loops(2, Spire.LOOP_MODE_YOYO)
	var rotation: SpireRichTextLabel = _animators.rotation
	_track(rotation.rotate_char(0, Vector3(1.15, 0, 0), 0.8)).set_loops(2, Spire.LOOP_MODE_YOYO)
	_track(rotation.rotate_char(4, Vector3(0, 1.15, 0), 0.8), Spire.EASE_OUT_CUBIC, 0.18).set_loops(2, Spire.LOOP_MODE_YOYO)
	_track(rotation.rotate_char(8, Vector3(0, 0, TAU), 1.3, Spire.ROTATE_MODE_FAST_BEYOND_360), Spire.EASE_OUT_CUBIC, 0.35)
	var scale: SpireRichTextLabel = _animators.scale
	for index in scale.get_character_count():
		_track(scale.scale_char(index, Vector2(0.65, 1.6), 0.55), Spire.EASE_OUT_CUBIC, index * 0.1).set_loops(2, Spire.LOOP_MODE_YOYO)


func _effects() -> void:
	for index in 5:
		var delay := index * 0.065
		_track(_animators.punch_0.punch_char_offset(index, Vector2(0, -17), 2.0, 5, 0.65), Spire.EASE_LINEAR, delay)
		_track(_animators.punch_1.punch_char_rotation(index, Vector3(0, 0, 0.48), 2.0, 5, 0.65), Spire.EASE_LINEAR, delay)
		_track(_animators.punch_2.punch_char_scale(index, Vector2(0.25, 0.5), 2.0, 5, 0.65), Spire.EASE_LINEAR, delay)
		_track(_animators.shake_0.shake_char_offset(index, 2.0, Vector2(8, 12), 7, 100.0), Spire.EASE_LINEAR, delay)
		_track(_animators.shake_1.shake_char_rotation(index, 2.0, Vector3(0.25, 0.25, 0.35), 7, 100.0), Spire.EASE_LINEAR, delay)
		_track(_animators.shake_2.shake_char_scale(index, 2.0, Vector2(0.22, 0.35), 7, 100.0), Spire.EASE_LINEAR, delay)


func _compose() -> void:
	var animator: SpireRichTextLabel = _animators.composition
	var sequence = Spire.sequence().set_default_children_ease(Spire.EASE_OUT_CUBIC)
	for index in animator.get_character_count():
		var delay := index * 0.07
		sequence.join(animator.offset_char(index, Vector2(0, -18), 0.7).set_delay(delay).set_loops(2, Spire.LOOP_MODE_YOYO))
		sequence.join(animator.rotate_char(index, Vector3(0, 0, -0.22), 0.7).set_delay(delay).set_loops(2, Spire.LOOP_MODE_YOYO))
		sequence.join(animator.scale_char(index, Vector2(1.12, 1.12), 0.7).set_delay(delay).set_loops(2, Spire.LOOP_MODE_YOYO))
		sequence.join(animator.color_char(index, MINT, 0.7).set_delay(delay).set_loops(2, Spire.LOOP_MODE_YOYO))
	_watch(sequence)


func _start_lifetime() -> void:
	_rebuild_tween = _track(_animators.refresh.offset_char(5, Vector2(0, -24), 3.0), Spire.EASE_LINEAR)
	_rebuild_tween.stopped.connect(func(): _rebuild_events[0] += 1)
	_rebuild_tween.finished.connect(func(): _rebuild_events[1] += 1)


func _shorten_text() -> void:
	_labels.refresh.text = "HI!"
	_lifetime_status.text = "text changed  /  rebuilding…"


func _check_missing_index() -> void:
	_check(_rebuild_tween.get_state() == Spire.STATE_PLAYING, "Vanished character must preserve playback.")
	_check(_rebuild_events == [0, 0], "Missing indices must not stop or finish playback early.")
	_check(_animators.refresh.get_character_count() == 3, "Refresh must update the parsed character count.")
	_check(_animators.refresh.get_char_offset(0).is_zero_approx(), "Refresh must reset surviving character transforms.")
	_lifetime_status.text = "playing  /  missing index ignored"
	_lifetime_status.add_theme_color_override("font_color", MINT)


func _restore_text() -> void:
	_labels.refresh.text = "LONGER"
	_lifetime_status.text = "fresh characters  /  transforms reset"
	_lifetime_status.add_theme_font_size_override("font_size", 11)


func _check_effect_restoration() -> void:
	for prefix in ["punch", "shake"]:
		for index in 5:
			_check(_animators[prefix + "_0"].get_char_offset(index).is_zero_approx(), prefix + " offset must restore.")
			_check(_animators[prefix + "_1"].get_char_rotation(index).is_zero_approx(), prefix + " rotation must restore.")
			_check(_animators[prefix + "_2"].get_char_scale(index).is_equal_approx(Vector2.ONE), prefix + " scale must restore.")


func _reset_all() -> void:
	for tween in _active:
		tween.stop()
	_active.clear()
	for animator in _animators.values():
		animator.reset()
	_phase.text = "04  /  RESET"
	_lifetime_status.text = "reset()  /  ready to animate again"
	_lifetime_status.add_theme_color_override("font_color", MUTED)


func _validate_final_state() -> void:
	_check(_finished_count > 100, "Showcase tweens must successfully complete.")
	_check(_rebuild_events == [1, 1], "The tween must complete normally despite its missing index.")
	for animator in _animators.values():
		for index in animator.get_character_count():
			_check(animator.get_char_offset(index).is_zero_approx(), "reset() must clear offsets.")
			_check(animator.get_char_rotation(index).is_zero_approx(), "reset() must clear rotations.")
			_check(animator.get_char_scale(index).is_equal_approx(Vector2.ONE), "reset() must clear scale.")
			_check(is_equal_approx(animator.get_char_alpha(index), 1.0), "reset() must restore alpha.")
