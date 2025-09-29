class_name TweenProperty

const State = preload("res://handle/tween_handle.gd").State

var tween: InternalTweenProperty

func _init(p_tween: InternalTweenProperty):
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

func is_absolute() -> bool: return tween.is_absolute()

func is_relative() -> bool: return tween.is_relative()

func is_speed_based() -> bool: return tween.is_speed_based()

func set_absolute() -> void: tween.set_absolute()

func set_speed_based(speed: float) -> void: tween.set_speed_based(speed)
