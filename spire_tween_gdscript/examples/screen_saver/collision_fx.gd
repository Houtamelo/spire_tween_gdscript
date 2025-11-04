extends Sprite2D

@export var duration: float = 2.0
@export var max_scale: float = 3.0
@export var ease_mode: Spire.Ease = Spire.EASE_IN_EXPO

func _ready():
	var seq := Spire.sequence().bind_node(self)
	
	seq.append(
		DoNode2D.scale(self, Vector2(max_scale, max_scale), duration)
			.from(Vector2.ZERO)
			.set_ease(ease_mode)
	)
	
	var rand_color = Color(randf_range(0, 1), randf_range(0, 1), randf_range(0, 1), 1)
	
	seq.join(
		DoCanvasItem.color(self, Color(1, 1, 1, 0), duration)
			.from(rand_color)
			.set_ease(ease_mode)
	)
	
	seq.finished.connect(self.queue_free)
