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
	
	seq.join(
		DoCanvasItem.color_a(self, 0, duration)
			.from(1)
			.set_ease(ease_mode)
	)
	
	seq.finished.connect(self.queue_free)
