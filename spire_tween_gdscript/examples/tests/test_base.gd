extends Node
class_name TestBase

var start_time_usec: int = 0
var timer: float = 0

var start_frame: int = 0

func _time_since_start_seconds() -> float:
	return timer

func _frames_since_start() -> int:
	return Engine.get_process_frames() - start_frame


func _ready():
	self.process_priority = -10
	self.process_physics_priority = -10


func _process(delta: float) -> void:
	timer += delta


func run_test(callable: Callable) -> bool:
	timer = 0
	start_time_usec = Time.get_ticks_usec()
	start_frame = Engine.get_process_frames()
	print("---------------------------------")
	print("Test Started: " + callable.get_method())
	var result: bool = await callable.call()
	print("Test Finished: " + callable.get_method())
	return result


func next_frame(): await get_tree().process_frame


func wait_seconds(seconds: float):
	debug("Waiting " + str(seconds) + " seconds...")
	await get_tree().create_timer(seconds).timeout


func wait_loop_finished(tween, expected_end_time: float, tween_name: String = ""):
	await tween.loop_finished
	_assert_within_tolerance(_time_since_start_seconds() - expected_end_time)
	
	if tween_name.is_empty(): debug("Loop completed")
	else: debug("Tween {0}: Loop completed".format([tween_name]))


func wait_finished(tween, expected_end_time: float, tween_name: String = ""):
	await tween.finished
	_assert_within_tolerance(_time_since_start_seconds() - expected_end_time)
	
	if tween_name.is_empty(): debug("Finished")
	else: debug("Tween {0}: Finished".format([tween_name]))


func debug(message: String):
	var frame := String.num(_frames_since_start()).pad_zeros(6).pad_decimals(0)
	var time := String.num(_time_since_start_seconds()).pad_zeros(2).pad_decimals(4)
	print("[{0}f, {1}s] {2}".format([frame, time, message]))

const TIME_TOLERANCE: float = 0.017 # slightly bigger than the length of 1 frame (at 60 fps)

static func _assert_within_tolerance(num: float, tolerance: float = TIME_TOLERANCE):
	assert(abs(num) <= tolerance, "Abs({0}) is bigger than tolerance `{1}`".format([num, tolerance]))

static func assert_eq(a, b): assert(a == b, fmt(a) + " != " + fmt(b))

static func assert_ne(a, b): assert(a != b, fmt(a) + " == " + fmt(b))

static func assert_le(small, big): assert(small <= big, fmt(small) + " > " + fmt(big))

static func assert_ge(big, small): assert(big >= small,  fmt(big) + " < " +  fmt(small))

static func fmt(a) -> String:
	if a is int: return String.num_int64(a)
	elif a is float: return String.num(a, 5)
	elif a is Vector2: return "({0},{1})".format([String.num(a.x, 5), String.num(a.y, 5)])
	else: return str(a)
