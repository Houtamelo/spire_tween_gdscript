extends Node

const RUNNER := preload("res://benchmarks/runner.gd")
const PREFAB: PackedScene = preload("res://benchmarks/prefab_2d.tscn")
const UTIL := preload("res://benchmarks/util.gd")


func _ready():
	await RUNNER.run_tests("Global Position (Random)", run, false)
	await RUNNER.run_tests("Global Position (Random)", run, true)
	
	var tree := get_tree()
	await tree.create_timer(0.5).timeout
	tree.quit()


static func run(root: Node, amount: int, duration: float, is_builtin: bool):
	var nodes := UTIL.spawn_nodes(PREFAB, root, amount)
	move_nodes_to_random_positions(nodes, duration, is_builtin)


static func move_nodes_to_random_positions(nodes: Array[Node2D], duration: float, is_builtin: bool):
	var amount := nodes.size()
	var origins := UTIL.generate_random_positions(amount)
	var destinations := UTIL.generate_random_positions(amount)
	
	tween_global_positions(nodes, origins, destinations, duration, is_builtin)


static func tween_global_positions(
	nodes: Array[Node2D], 
	origins: Array[Vector2], 
	destinations: Array[Vector2],
	duration: float,
	is_builtin: bool,
):
	if is_builtin:
		for idx in range(origins.size()):
			var origin := origins[idx]
			var destination := destinations[idx]
			var node := nodes[idx]
			var tween := node.create_tween()
			tween.tween_property(node, "global_position", destination, duration).from(origin)
	else:
		for idx in range(origins.size()):
			var origin := origins[idx]
			var destination := destinations[idx]
			var node := nodes[idx]
			Spire2D.do_global_position(node, destination, duration).from(origin)
