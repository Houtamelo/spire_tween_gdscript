extends Node

const UTIL := preload("res://benchmarks/util.gd")

const STD_AMOUNTS: Array[int] = [
	1000,
	10000,
	50000,
]

const STD_DURATION: float = 10.0

var _setup_call: Callable
var _duration: float

# Measurements
var _amount: int
var _setup_time: float
var _delta_times: Array[float] = []
var _engine_fps: Array[float] = []
var _is_builtin: bool
var _begin_sum: float
var _skip_frames: int = 2

func _init(setup_call: Callable, is_builtin: bool, duration: float, amount: int):
	_setup_call = setup_call
	_duration = duration
	_amount = amount
	_is_builtin = is_builtin


func _process(delta: float) -> void:
	_begin_sum += delta
	_skip_frames -= 1
	if _skip_frames < 0: _delta_times.push_back(delta)


func _physics_process(_delta: float) -> void:
	# get_frames_per_second will give innacurate results at the beginning of the test 
	# due to it reporting the average on the last second.
	if _begin_sum > 2.0:
		_engine_fps.push_back(Engine.get_frames_per_second())


func run_test():
	var start_time := Time.get_ticks_usec()
	_setup_call.call(self, _is_builtin, _duration, _amount)
	var end_time := Time.get_ticks_usec()
	
	_setup_time = (end_time - start_time) / 1000000.0
	
	await get_tree().create_timer(_duration).timeout


static func run_tests(
	test_name: String,
	setup_call: Callable,
	duration: float = STD_DURATION,
	amounts: Array[int] = STD_AMOUNTS,
):
	for amount in amounts:
		var results_builtin := await run_test_case(setup_call, true, duration, amount)
		var results_spire := await run_test_case(setup_call, false, duration, amount)
		UTIL.print_results(test_name + " | " + str(amount) + " Nodes" , results_builtin, results_spire)


static func run_test_case(
	setup_call: Callable,
	is_builtin: bool,
	duration: float,
	amount: int,
) -> Dictionary:
	var tree: SceneTree = Engine.get_main_loop()
	var root: Node = tree.root
	
	await tree.create_timer(2.0).timeout
	
	var runner := new(setup_call, is_builtin, duration, amount)
	root.add_child(runner)
	
	await runner.run_test()
	
	var result = {
		"setup_time" = runner._setup_time,
		"engine_fps" = runner._engine_fps,
		"delta_times" = runner._delta_times,
	}
	
	runner.queue_free()
	
	return result
