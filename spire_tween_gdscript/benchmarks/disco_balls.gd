extends BaseTestCase

const PREFAB: PackedScene = preload("res://benchmarks/prefabs/circle_16x16.tscn")

func _test_name() -> String: 
	#return "| global-position & modulate | 16x16-circles | parallel | 1 tweener/node"
	return "| global-position & modulate | 16x16-circles | parallel | 1 tweener/property"

func _run(root: Node, is_builtin: bool, duration: float, amount: int) -> void:
	#separate_tweeners(root, is_builtin, duration, amount)
	reusing_tweeners(root, is_builtin, duration, amount)


func reusing_tweeners(root: Node, is_builtin: bool, duration: float, amount: int) -> void:
	var nodes := SETUPS.spawn_nodes(PREFAB, root, amount)
	var tweeners = get_tweeners_array(is_builtin)
	SETUPS.tween_global_positions(duration, is_builtin, nodes, tweeners, true)
	SETUPS.tween_modulates(duration, is_builtin, nodes, tweeners, true)


func separate_tweeners(root: Node, is_builtin: bool, duration: float, amount: int) -> void:
	var nodes := SETUPS.spawn_nodes(PREFAB, root, amount)
	SETUPS.tween_global_positions(duration, is_builtin, nodes, get_tweeners_array(is_builtin))
	SETUPS.tween_modulates(duration, is_builtin, nodes, get_tweeners_array(is_builtin))


static func get_tweeners_array(is_builtin: bool) -> Variant:
	if is_builtin:
		var typed_arr: Array[Tween] = []
		return typed_arr
	else:
		var typed_arr: Array[SpireSequence] = []
		return typed_arr
