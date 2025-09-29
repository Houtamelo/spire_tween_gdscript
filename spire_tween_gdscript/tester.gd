extends Node

#@onready var sprite_b: Sprite2D = $sprite_b

@onready var sprite: Sprite2D = $sprite_a

func _ready():
	Spire.do_color(sprite, Color.RED, 10.0)
	#TweenBuilder.do_follow(sprite_a, sprite_b, 100.0)
	#TweenBuilder.do_move(sprite_b, Vector2(1000, -1000), 10)
