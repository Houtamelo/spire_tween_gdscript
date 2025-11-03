#| Nodes |    min |    max |       avg |
#| ----: | -----: | -----: | --------: |
#|    1k | 5901.0 | 6275.0 | 6135.3948 |
#|   10k | 1083.0 | 1108.0 | 1096.9981 |
#|   50k |   69.0 |   91.0 |   88.7774 |
#|  100k |   32.0 |   34.0 |   33.1702 |
static func print_results(
	test_name: String, 
	results_builtin: Dictionary,
	results_spire: Dictionary,
) -> void:
	var builder := "\n"
	builder += "-----------------------------------------------------------\n\n"
	builder += test_name + "\n\n"
	
	builder += "|--------------------|\n"
	builder += "| tween |  setup(ms) |\n"
	builder += "|-------|------------|\n"
	builder += fmt_setup_time_line(results_builtin["setup_time"], "Godot")
	builder += fmt_setup_time_line(results_spire  ["setup_time"], "Spire")
	builder += "|--------------------|\n"
	builder += "\n"
	builder += "|---------------------------------------------------------|\n"
	builder += "|          frames per second - higher is better           |\n"
	builder += "| tween |     min |     max |  median |    mean |  stddev |\n"
	builder += "|-------|---------|---------|---------|---------|---------|\n"
	builder += fmt_engine_fps_results(results_builtin, "Godot")
	builder += fmt_engine_fps_results(results_spire, "Spire")
	builder += "|---------------------------------------------------------|\n"
	builder += "|         miliseconds per frame - lower is better         |\n"
	builder += "| tween |     min |     max |  median |    mean |  stddev |\n"
	builder += "|-------|---------|---------|---------|---------|---------|\n"
	builder += fmt_delta_times_results(results_builtin, "Godot")
	builder += fmt_delta_times_results(results_spire, "Spire")
	builder += "|---------------------------------------------------------|\n"
	
	print(builder)

static func fmt_engine_fps_results(results: Dictionary, tween_lib: String) -> String:
	var engine_fps: Array[float] = results["engine_fps"]
	engine_fps.sort()
	
	var _min := fmt_fps(engine_fps[0])
	var _max := fmt_fps(engine_fps[engine_fps.size() - 1])
	@warning_ignore("integer_division")
	var median := fmt_fps(engine_fps[engine_fps.size() / 2])
	
	var sum: float = 0.0
	for fps in engine_fps: sum += fps
	sum /= engine_fps.size()
	var mean := fmt_fps(sum)
	
	var stddev := fmt_stddev(std_dev_population(engine_fps))
	
	return "| {0} | {1} | {2} | {3} | {4} | {5} |\n".format([tween_lib, _min, _max, median, mean, stddev])


static func fmt_delta_times_results(results: Dictionary, tween_lib: String) -> String:
	var deltas: Array[float] = results["delta_times"]
	deltas.sort()
	
	for idx in range(deltas.size()):
		deltas[idx] *= 1000.0
	
	var _min := fmt_fps(deltas[0])
	var _max := fmt_fps(deltas[deltas.size() - 1])
	@warning_ignore("integer_division")
	var median := fmt_fps(deltas[deltas.size() / 2])
	
	var sum: float = 0.0
	for fps in deltas: sum += fps
	sum /= deltas.size()
	var mean := fmt_fps(sum)
	
	var stddev := fmt_stddev(std_dev_population(deltas))
	
	return "| {0} | {1} | {2} | {3} | {4} | {5} |\n".format([tween_lib, _min, _max, median, mean, stddev])


static func fmt_setup_time_line(num: float, lib: String) -> String:
	return "| " + lib + " |  " + String.num(num * 1000.0, 4).pad_zeros(3).pad_decimals(4) + "  |\n"


static func fmt_fps(num: float) -> String:
	var formatted := String.num(num, 2).pad_decimals(2)
	
	if num >= 1000 : return formatted
	elif num >= 100: return " " + formatted
	elif num >= 10 : return "  " + formatted
	elif num >= 0  : return "   " + formatted
	else           : return "    " + formatted


static func fmt_stddev(num: float) -> String: 
	return String.num(num, 4).pad_decimals(4).pad_zeros(2)

@warning_ignore_start("integer_division")
static func fmt_amount(num: int) -> String:
	if num >= 100000 : return "  " + str(num / 1000) + "k"
	elif num >= 10000: return "   " + str(num / 1000) + "k"
	elif num >= 1000 : return "    " + str(num / 1000) + "k"
	elif num >= 100  : return "   " + str(num)
	elif num >= 10   : return "    " + str(num)
	else             : return "     " + str(num)


static func std_dev_population(data: Array[float]) -> float:
	var n := data.size()
	
	var mean := 0.0
	for v in data: mean += float(v)
	mean /= n
	
	var variance := 0.0
	for v in data:
		var d := float(v) - mean
		variance += d * d
	
	variance /= n
	return sqrt(variance)

static func std_dev_sample(data: Array[float]) -> float:
	var n := data.size()
	
	var mean := 0.0
	for v in data: mean += float(v)
	mean /= n
	
	var variance := 0.0
	for v in data:
		var d := float(v) - mean
		variance += d * d
	variance /= (n - 1)
	
	return sqrt(variance)
