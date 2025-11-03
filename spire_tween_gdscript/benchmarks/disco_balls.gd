extends BaseTestCase

const PREFAB: PackedScene = preload("res://benchmarks/prefabs/circle_16x16.tscn")

func _test_name() -> String: 
	#return "| global-position & modulate | 16x16-circles | parallel | 1 tweener/node"
	return "| global-position & modulate | 16x16-circles | parallel | 1 tweener/property"

func _run(root: Node, is_builtin: bool, duration: float, amount: int) -> void:
	separate_tweeners(root, is_builtin, duration, amount)
	#reusing_tweeners(root, is_builtin, duration, amount)


# TODO: Make this use SpireSequence
func reusing_tweeners(root: Node, is_builtin: bool, duration: float, amount: int) -> void:
	var nodes := SETUPS.spawn_nodes(PREFAB, root, amount)
	var tweeners: Array[Tween] = []
	SETUPS.tween_global_positions(duration, is_builtin, nodes, tweeners)
	SETUPS.tween_modulates(duration, is_builtin, nodes, tweeners)


func separate_tweeners(root: Node, is_builtin: bool, duration: float, amount: int) -> void:
	var nodes := SETUPS.spawn_nodes(PREFAB, root, amount)
	SETUPS.tween_global_positions(duration, is_builtin, nodes, [])
	SETUPS.tween_modulates(duration, is_builtin, nodes, [])
