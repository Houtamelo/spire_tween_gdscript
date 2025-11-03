extends Node2D

@export var initial_speed: float = 1000
@onready var ball: RigidBody2D = $BouncyBall

func _ready():
	var angle := randf_range(0, 2 * PI)
	ball.linear_velocity = Vector2.from_angle(angle) * initial_speed
