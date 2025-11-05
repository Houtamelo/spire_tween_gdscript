extends Sprite2D

@onready var repelled_particle: Sprite2D = $"RepelledParticle"

var distance_to_repeller: float:
	get: return self.global_position.distance_to(repelled_particle.global_position)
	set(new_distance): 
		var old_distance := distance_to_repeller
		var distance_diff := new_distance - old_distance
		var direction := self.global_position.direction_to(repelled_particle.global_position)
		repelled_particle.global_position += direction * distance_diff

func _ready():
	# Signature: func(obj: Object, property_path: NodePath, to: float, duration: float) -> SpirePropertyFloat
	Spire.do_property_float(self, ^"distance_to_repeller", 400, 3.0)\
		.set_ease(Spire.EASE_IN_OUT_SINE)\
		.set_loops(-1, Spire.LOOP_MODE_YOYO)
	
	DoNode2D.rotation(self, PI * 2, 4.0).from(0).set_loops(-1)
	
	get_tree().physics_frame.connect(spawn_trail)

func spawn_trail():
	var trail: Sprite2D = repelled_particle.duplicate()
	add_child(trail)
	trail.top_level = true
	trail.global_position = repelled_particle.global_position
	
	DoCanvasItem.color_a(trail, 0.0, 4.0)\
		.finished.connect(trail.queue_free)
