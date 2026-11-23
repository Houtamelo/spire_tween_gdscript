@tool
extends "template.gd"


func _capture_target_rest_state(node: Node) -> Callable:
	var color: Color = node.modulate
	if node is Node2D:
		var transform: Transform2D = node.transform
		return func() -> void:
			if is_instance_valid(node):
				node.transform = transform
				node.modulate = color
	var position: Vector2 = node.position
	var rotation: float = node.rotation
	var scale: Vector2 = node.scale
	return func() -> void:
		if is_instance_valid(node):
			node.position = position
			node.rotation = rotation
			node.scale = scale
			node.modulate = color


func _target_warnings(node: Node) -> PackedStringArray:
	if not (node is Control or node is Node2D):
		return PackedStringArray(["`target` must be a Control or Node2D."])
	if node is Control and node.get_parent() is Container:
		return PackedStringArray(["`target` is positioned by a Container; animate a visual child beneath a layout wrapper instead."])
	return PackedStringArray()
