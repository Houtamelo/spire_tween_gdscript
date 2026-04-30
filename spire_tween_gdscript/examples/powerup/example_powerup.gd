extends TestBase

@export_group("Spiral")
@export var duration: float = 3.8
@export var shear: float = 0.0
@export var from: float = 4 * PI
@export var to: float = 0.0
@export var scale: Vector2 = Vector2.ONE * 64
@export var balls_count: int = 4


@onready var actor: Sprite2D = $Actor
@onready var ball_prefab: PackedScene = preload("res://examples/powerup/ball_prefab.tscn")


func spawn_ball() -> Sprite2D:
	var spawned: Sprite2D = ball_prefab.instantiate()
	add_child(spawned)
	return spawned


func _ready():
	actor.modulate = Color(0.5, 0.5, 0.5)
	var frame_time := 0.2
	var speed := 1 / frame_time
	var color_speed := sqrt(3 * 0.5 * 0.5) * 2
	
	var balls: Array[Sprite2D] = []
	for _i in range(0, balls_count): 
		balls.append(spawn_ball())
	
	var angle_interval := 2.0 / balls.size()
	var center: Vector2 = actor.global_position
	var ball_seq := Spire.sequence()
	for i in range(0, balls.size()):
		var rotation: float = PI * i * angle_interval
		var ball: Sprite2D = balls[i]
		# Rotate the energy ball around the character in a spiral pattern
		ball_seq.join(DoNode2D.spiral(ball, center, from, to, scale, duration, rotation, shear, Spire.SPIRAL_FERMAT))
		# Fade-in the energy ball
		ball_seq.join(DoCanvasItem.color_a(ball, 0.8, duration).from(0.0))
		# Steadily scale the ball towards 0.25, starting at 0.
		ball_seq.join(DoNode2D.scale(ball, Vector2(0.25, 0.25), duration).from(Vector2.ZERO))
	
	ball_seq.append_interval(0)
	
	for ball in balls:
		ball_seq.join(DoCanvasItem.color(ball, Color(1, 1, 1, 0), 2.0).set_ease(Spire.EASE_OUT_EXPO))
		ball_seq.join(DoNode2D.scale(ball, Vector2.ONE * 3, 2.0).set_ease(Spire.EASE_OUT_EXPO))
	
	var actor_seq := Spire.sequence()
	actor_seq.append(DoSprite2D.frame(actor, 6, speed).from(0).as_speed_based())
	actor_seq.append(DoSprite2D.frame(actor, 8, speed).as_speed_based().set_loops(5, Spire.LOOP_MODE_YOYO))
	actor_seq.join(DoCanvasItem.color(actor, Color.WHITE, color_speed).as_speed_based().set_loops(5, Spire.LOOP_MODE_YOYO).set_ease(Spire.EASE_IN_OUT_SINE))
	actor_seq.append(DoSprite2D.frame(actor, 9, speed).as_speed_based())
	actor_seq.append_interval(frame_time)
	actor_seq.append_call(func(): actor.frame = 0)
