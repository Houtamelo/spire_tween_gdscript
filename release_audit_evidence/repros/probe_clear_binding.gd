extends Node
func _ready():
	var n := Node.new()
	add_child(n)
	var s := Spire.sequence().append_interval(10.0)
	s.bind_node(n)
	s.clear_bound_nodes()
	print("CLEARED_COUNT ", s.get_bound_nodes().size())
	n.queue_free()
	await get_tree().process_frame
	await get_tree().process_frame
	print("AFTER_FREE registered=", s.is_registered(), " stopped=", s.is_stopped())
	get_tree().quit()
