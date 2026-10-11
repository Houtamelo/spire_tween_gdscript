extends Node
func _ready():
	var node := Node2D.new()
	add_child(node)
	var t := DoNode2D.position(node, Vector2(10, 20), 0.1)
	await t.finished
	print("BASELINE ", node.position)
	assert(node.position == Vector2(10, 20))
	get_tree().quit()
