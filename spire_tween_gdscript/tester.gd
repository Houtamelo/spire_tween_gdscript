extends Node

#@onready var sprite_b: Sprite2D = $sprite_b

@onready var sprite: Sprite2D = $sprite_a
@export var pause_mode: Spire.PauseMode


func _ready():
	pass
	
	DoNode2D.position(sprite, Vector2(2, 1), 5)
	
	var tween := SpireSequence.build()
	tween.bind_node(sprite)
	
	#var tween := SpirePropertyColor.instantiate(sprite, "modulate", Color.AQUA, 1.0, false)
	#tween.bind_node()
	
	#var tween := sprite.create_tween()
	#var prop := tween.tween_property(sprite, ^"modulate", Color.RED, 10.0)
	#prop.as_relative()
	
	#SpirePropert
	
	#Spire.do_color(sprite, Color.RED, 10.0)
	#TweenBuilder.do_follow(sprite_a, sprite_b, 100.0)
	#TweenBuilder.do_move(sprite_b, Vector2(1000, -1000), 10)


func test():
	pass
