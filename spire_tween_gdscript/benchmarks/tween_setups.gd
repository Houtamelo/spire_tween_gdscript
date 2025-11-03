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


static func generate_random_colors(amount: int) -> PackedColorArray:
	var array: PackedColorArray = []
	array.resize(amount)
	
	for idx in range(amount):
		var r := randf()
		var g := randf()
		var b := randf()
		var a := randf()
		array[idx] = Color(r, g, b, a)
	
	return array


static func ensure_builtin_tweeners(nodes: Array[Node2D], tweeners: Array[Tween]) -> void:
	var tweeners_size := tweeners.size()
	var nodes_size := nodes.size()
	if tweeners_size >= nodes_size: return
	
	for i in range(tweeners_size, nodes_size):
		tweeners.push_back(nodes[i].create_tween().set_parallel(true))

static func tween_global_positions(
	duration: float,
	is_builtin: bool,
	nodes: Array[Node2D],
	tweeners: Array[Tween],
	froms: PackedVector2Array = [], 
	tos: PackedVector2Array = [],
):
	var amount := nodes.size()
	if froms.is_empty(): froms = generate_random_positions(amount)
	if tos.is_empty(): tos = generate_random_positions(amount)
	
	if is_builtin:
		ensure_builtin_tweeners(nodes, tweeners)
		for i in range(froms.size()):
			tweeners[i].tween_property(nodes[i], "global_position", tos[i], duration).from(froms[i])
	else:
		for i in range(froms.size()):
			DoNode2D.global_position(nodes[i], tos[i], duration).from(froms[i])


static func tween_modulates(
	duration: float,
	is_builtin: bool,
	nodes: Array[Node2D],
	tweeners: Array[Tween],
	froms: PackedColorArray = [], 
	tos: PackedColorArray = [],
):
	var amount := nodes.size()
	if froms.is_empty(): froms = generate_random_colors(amount)
	if tos.is_empty(): tos = generate_random_colors(amount)
	
	if is_builtin:
		ensure_builtin_tweeners(nodes, tweeners)
		for i in range(froms.size()):
			tweeners[i].tween_property(nodes[i], "modulate", tos[i], duration).from(froms[i])
	else:
		for i in range(froms.size()):
			DoCanvasItem.color(nodes[i], tos[i], duration).from(froms[i])
