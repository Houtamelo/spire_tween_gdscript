extends Sprite2D

const vertices := [
	Vector2(100, 100),
	Vector2(500, 100),
	Vector2(500, 400),
	Vector2(100, 400),
]

var speed := 200.0 # pixels per second
var flash_duration := 0.5

func _ready():
	self.global_position = vertices[3]
	spire_impl()
	# Uncomment if you want to see the built-in version in action
	#godot_impl() 


func godot_impl():
	var tween := create_tween().set_loops(-1)
	var start_pos := vertices[3]
	
	for vert: Vector2 in vertices:
		var duration := vert.distance_to(start_pos) / speed
		tween.chain().tween_property(self, ^"global_position", vert, duration)
		start_pos = vert
		
		tween.parallel()\
			.tween_property(self, ^"modulate", Color.RED, flash_duration)\
			.finished.connect(
				func(): create_tween().tween_property(self, ^"modulate", Color.WHITE, flash_duration)
			)


func spire_impl():
	var seq := Spire.sequence().set_loops(-1) # Make the sequence loop infinitely.
	
	for vert: Vector2 in vertices:
		# `append` creates a new "step" in the sequence, which means that the tween "appended" will run after all previous steps finish.
		seq.append(DoNode2D.move(self, vert, speed).as_speed_based())
		
		# `join` adds another tween to the current step of the sequence.
		# In this case, it makes the tween bellow run(flash) at the same time as the tween above(movement).
		seq.join(
			DoCanvasItem.modulate(self, Color.RED, flash_duration)
				.set_loops(2, Spire.LOOP_MODE_YOYO) # yoyo loop will make it go red then back to the original color.
		)
