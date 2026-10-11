extends Node
func _ready():
	var child = Spire.sequence().append_interval(0.1)
	var parent = Spire.sequence().insert(0.0, child)
	await get_tree().create_timer(0.2).timeout
	print("INSERT_ONLY parent_stopped=", parent.is_stopped(), " child_playing=", child.is_playing(), " child_elapsed=", child.get_total_elapsed_time())
	child.stop()
	parent.stop()
	await get_tree().create_timer(0.05).timeout
	get_tree().quit()
