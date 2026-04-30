extends Sprite2D

func _process(_delta: float) -> void:
	var copy: Sprite2D = self.duplicate()
	get_parent().add_child(copy)
	copy.set_script(null)
	copy.global_position = self.global_position
	DoCanvasItem.color_a(copy, 0.0, 8.0).from(0.75).finished.connect(copy.queue_free)
