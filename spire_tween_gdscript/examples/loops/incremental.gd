extends Node2D

@export var initial_speed: float = 1000
@onready var ball: RigidBody2D = $BouncyBall
@onready var label: Label = $Label

func _ready():
	var angle := randf_range(0, 2 * PI)
	ball.linear_velocity = Vector2.from_angle(angle) * initial_speed
	DoRigidBody2D.gravity_scale(ball, 2, 2) \
		.set_loops(10, Spire.LOOP_MODE_INCREMENTAL)


func _process(_delta: float) -> void:
	label.text = "Gravity: " + str(ball.gravity_scale)
