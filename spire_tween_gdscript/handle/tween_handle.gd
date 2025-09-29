extends RefCounted
class_name TweenHandle

enum State {
	DoesNotExist = -1,
	Stopped = 0,
	Paused = 1,
	Playing = 2,
}

var handle: InternalUntypedTween

func _init(p_handle: InternalUntypedTween):
	handle = p_handle

var state: State:
	get: return handle.get_state() as State
	set(val): handle.set_state(val)

var is_playing: bool:
	get: return handle.is_playing()
	set(val):
		if val: handle.play()
		else: handle.pause()

var is_paused: bool:
	get: return handle.is_paused()
	set(val):
		if val: handle.pause()
		else: handle.play()

func play() -> void: handle.play()

func pause() -> void: handle.pause()

func stop() -> void: handle.stop()
