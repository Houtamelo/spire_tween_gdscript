class_name TweenPropertyI32
const State = preload("res://handle/tween_handle.gd").State

var tween: InternalTweenPropertyI32

func _init(p_tween: InternalTweenPropertyI32):
	tween = p_tween

var state: State:
	get: return tween.get_state() as State
	set(val): tween.set_state(val)

var is_playing: bool:
	get: return tween.is_playing()
	set(val):
		if val: tween.play()
		else: tween.pause()

var is_paused: bool:
	get: return tween.is_paused()
	set(val):
		if val: tween.pause()
		else: tween.play()

func play() -> void: tween.play()

func pause() -> void: tween.pause()

func stop() -> void: tween.stop()

var property_path: NodePath:
	get: return tween.get_property_path()
	set(val): tween.set_property_path(val)

var owner: Object:
	get: return tween.get_owner()
	set(val): tween.set_owner(val)

var easing: Variant:
	get: return tween.get_ease()
	set(val): tween.set_ease(val)

var duration: float:
	get: return tween.get_duration()
	set(val): tween.set_duration(val)

var start_value: int:
	set(val): tween.set_start_value(val)

var final_value: int:
	get: return tween.get_final_value()
	set(val): tween.set_final_value(val)

func is_absolute() -> bool: return tween.is_absolute()

func is_relative() -> bool: return tween.is_relative()

func is_speed_based() -> bool: return tween.is_speed_based()

## Builder methods

func from(value: int) -> TweenPropertyI32:
	tween.set_start_value(value)
	return self

func with_ease(p_ease: Variant) -> TweenPropertyI32:
	tween = tween.with_ease(p_ease)
	return self

func set_absolute() -> TweenPropertyI32:
	tween.set_absolute()
	return self

func set_speed_based(speed: float) -> TweenPropertyI32:
	tween.set_speed_based(speed)
	return self

func set_relative(to: int = 0) -> TweenPropertyI32:
	tween.set_relative(to)
	return self
