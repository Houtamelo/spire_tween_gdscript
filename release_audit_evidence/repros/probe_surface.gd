extends Node
func _ready():
	var t := Spire.sequence()
	print("HAS_CLOSE_CURRENT_BLOCK ", t.has_method("close_current_block"))
	t.unregister()
	get_tree().quit()
