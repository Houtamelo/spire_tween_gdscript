extends Node
func _ready():
	var n := Node.new()
	add_child(n)
	var s := Spire.sequence().append_interval(10.0)
	s.bind_node(n)
	s.bind_node(n)
	print("DUPLICATE_DONE")
	get_tree().quit()
