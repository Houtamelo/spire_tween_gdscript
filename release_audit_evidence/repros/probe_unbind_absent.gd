extends Node
func _ready():
	var n := Node.new()
	add_child(n)
	var a := Spire.sequence().append_interval(10.0)
	var b := Spire.sequence().append_interval(10.0)
	b.bind_node(n)
	a.unbind_node(n)
	print("UNBIND_DONE")
	get_tree().quit()
