extends BaseTestCase

const PREFAB: PackedScene = preload("res://benchmarks/prefabs/square_4x4.tscn")

func _test_name() -> String: return "tween-global-position_black-4x4-dots"

func _run(root: Node, is_builtin: bool, duration: float, amount: int) -> void:
	var nodes := SETUPS.spawn_nodes(PREFAB, root, amount)
	for node in nodes: node.modulate = Color.BLACK
	
	SETUPS.tween_global_positions(duration, is_builtin, nodes, [])
