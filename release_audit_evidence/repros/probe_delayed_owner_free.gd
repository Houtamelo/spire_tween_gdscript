extends Node
func _ready():
	var n := Node.new()
	add_child(n)
	var t := Spire.do_delayed_call(Callable(n, "queue_free"), 100.0)
	n.queue_free()
	await get_tree().process_frame
	await get_tree().process_frame
	print("DELAY_AFTER_FREE registered=", t.is_registered(), " stopped=", t.is_stopped())
	get_tree().quit()
