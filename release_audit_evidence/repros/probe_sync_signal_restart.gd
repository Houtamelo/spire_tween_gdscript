extends Node
func _ready():
	var n := Node2D.new()
	add_child(n)
	var t := DoNode2D.position(n, Vector2.ONE, 0.1)
	t.finished.connect(func(): t.play())
	for i in range(30): await get_tree().process_frame
	print("SYNC_RESTART registered=", t.is_registered(), " playing=", t.is_playing())
	get_tree().quit()
