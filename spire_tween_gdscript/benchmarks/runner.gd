extends Node

const UTIL := preload("res://benchmarks/util.gd")

var setup_call: Callable
var duration: float

# Measurements
var amount: int
var setup_time: float
var frames: Array[float] = []
var engine_fps: Array[float] = []
var is_builtin: bool
var begin_sum: float
var skip_frames: int = 2

func _init(p_setup_call: Callable, p_amount: int, p_duration: float, p_is_builtin):
	setup_call = p_setup_call
	duration = p_duration
	amount = p_amount
	is_builtin = p_is_builtin


func _process(delta: float) -> void:
	begin_sum += delta
	skip_frames -= 1
	if skip_frames < 0:
		frames.push_back(delta)


func _physics_process(_delta: float) -> void:
	# get_frames_per_second will give innacurate results at the beginning of the test 
	# due to it reporting the average on the last second.
	if begin_sum > 2.0:
		engine_fps.push_back(Engine.get_frames_per_second())


func run():
	var start_time := Time.get_ticks_usec()
	setup_call.call(self, amount, duration, is_builtin)
	var end_time := Time.get_ticks_usec()
	
	setup_time = (end_time - start_time) / 1000000.0
	
	await get_tree().create_timer(duration).timeout


static func run_tests(
	test_name: String,
	setup: Callable,
	_is_builtin: bool,
	_duration: float = UTIL.STD_DURATION,
	amounts: Array[int] = UTIL.STD_AMOUNTS,
):
	var results: Array[Dictionary] = []
	
	var tree: SceneTree = Engine.get_main_loop()
	var root: Node = tree.root
	
	for _amount in amounts:
		await tree.create_timer(2.0).timeout
		
		print("[test case began; amount: {0}]".format([_amount]))
		
		var runner := new(setup, _amount, _duration, _is_builtin)
		root.add_child(runner)
		
		await runner.run()
		
		results.push_back({
			"amount" = _amount,
			"setup_time" = runner.setup_time,
			"engine_fps" = runner.engine_fps,
			"frames" = runner.frames,
		})
		
		runner.queue_free()
		print("[test case ended]")
	
	var engine_name: String
	if _is_builtin: engine_name = "Built-in Tween"
	else: engine_name = "Spire Tween"
	
	UTIL.print_results(engine_name + " - " + test_name, results)
