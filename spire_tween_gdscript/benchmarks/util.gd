const STD_AMOUNTS: Array[int] = [
	1000,
	10000,
	50000,
]

const STD_DURATION: float = 10.0

static func spawn_nodes(prefab: PackedScene, root: Node, amount: int) -> Array[Node2D]:
	var nodes: Array[Node2D] = []
	nodes.resize(amount)
	
	for idx in range(amount):
		var node: Node2D = prefab.instantiate()
		root.add_child(node)
		nodes[idx] = node
	
	return nodes


static func generate_random_positions(amount: int) -> PackedVector2Array:
	var array: PackedVector2Array = []
	array.resize(amount)
	
	for idx in range(amount):
		var x := randf_range(0, 1920)
		var y := randf_range(0, 1080)
		array[idx] = Vector2(x, y)
	
	return array

#| Nodes |    min |    max |       avg |
#| ----: | -----: | -----: | --------: |
#|    1k | 5901.0 | 6275.0 | 6135.3948 |
#|   10k | 1083.0 | 1108.0 | 1096.9981 |
#|   50k |   69.0 |   91.0 |   88.7774 |
#|  100k |   32.0 |   34.0 |   33.1702 |
static func print_results(test_name: String, results: Array[Dictionary]) -> void:
	var builder := "\n"
	builder += "---------------------------------------------------------------\n\n"
	builder += "## " + test_name + "\n"
	builder += "### Frames per second (higher is better)\n"
	builder += "|  nodes |     min |     max |  median |    mean |  stddev |  setup(ms) |\n"
	builder += "| -----: | ------: | ------: | ------: | ------: | ------: | ---------: |\n"
	
	for dict in results:
		var _amount := fmt_amount(dict["amount"])
		
		var _setup_time := String.num(dict["setup_time"] * 1000.0, 4).pad_zeros(3).pad_decimals(4)
		
		var _engine_fps: Array[float] = dict["engine_fps"]
		_engine_fps.sort()
		
		var _min := fmt_fps(_engine_fps[0])
		var _max := fmt_fps(_engine_fps[_engine_fps.size() - 1])
		@warning_ignore("integer_division")
		var median := fmt_fps(_engine_fps[_engine_fps.size() / 2])
		
		var sum: float = 0.0
		for fps in _engine_fps: sum += fps
		sum /= _engine_fps.size()
		var mean := fmt_fps(sum)
		
		var stddev := fmt_fps(std_dev_population(_engine_fps))
		
		builder += "| {0} | {1} | {2} | {3} | {4} | {5} |   {6} |\n".format([_amount, _min, _max, median, mean, stddev, _setup_time])
	
	builder += "\n\n"
	
	builder += "## Miliseconds per frame (lower is better)\n"
	builder += "|  nodes |     min |     max |  median |    mean |  stddev |\n"
	builder += "| -----: | ------: | ------: | ------: | ------: | ------: |\n"
	
	for dict in results:
		var _amount := fmt_amount(dict["amount"])
		
		var _frames: Array[float] = dict["frames"]
		_frames.sort()
		
		for idx in range(_frames.size()):
			_frames[idx] *= 1000.0
		
		var _min := fmt_fps(_frames[0])
		var _max := fmt_fps(_frames[_frames.size() - 1])
		@warning_ignore("integer_division")
		var median := fmt_fps(_frames[_frames.size() / 2])
		
		var sum: float = 0.0
		for fps in _frames: sum += fps
		sum /= _frames.size()
		var mean := fmt_fps(sum)
		
		var stddev := fmt_fps(std_dev_population(_frames))
		
		builder += "| {0} | {1} | {2} | {3} | {4} | {5} |\n".format([_amount, _min, _max, median, mean, stddev])
	
	builder += "---------------------------------------------------------------\n"
	
	print(builder)


static func fmt_fps(num: float) -> String:
	var formatted := String.num(num, 2).pad_decimals(2)
	
	if num >= 1000 : return formatted
	elif num >= 100: return " " + formatted
	elif num >= 10 : return "  " + formatted
	elif num >= 0  : return "   " + formatted
	else           : return "    " + formatted

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
