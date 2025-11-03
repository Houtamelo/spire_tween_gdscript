extends Node2D

@onready var ball: Sprite2D = $Ball


func _ready():
	DoNode2D.move_x(ball, 512, 1.0) \
		.from(-512) \
		.set_loops(-1, Spire.LOOP_MODE_YOYO) \
		.set_ease(Spire.EASE_IN_OUT_SINE)
