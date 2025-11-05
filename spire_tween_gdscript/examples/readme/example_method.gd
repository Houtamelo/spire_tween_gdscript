# Note: Physics mode is used to make the dots spawn in fixed intervals 
extends Node2D

const DOT_PREFAB: PackedScene = preload("res://examples/readme/dot_prefab.tscn")

@export var from: Vector2i = Vector2i(100, 100)
@export var to: Vector2i = Vector2i(900, 600)
var duration: float = 10.0

func _ready():
	# Delay so that the tweening process doesn't start before we focus the game window
	Spire.do_delayed_call(spire_impl, 2.0)
	
	# Uncomment if you want to see built-in version in action
	#get_tree().create_timer(2.0).timeout.connect(godot_impl)


func godot_impl():
	self.create_tween()\
		.set_process_mode(Tween.TWEEN_PROCESS_PHYSICS)\
		.tween_method(create_dot, from, to, duration)


func spire_impl():
	# Signature: func(callable: Callable, from: Vector2i, to: Vector2i, duration: float) -> SpireMethodVector2i
	Spire.do_call_vector2i(create_dot, from, to, duration)\
		.set_process_mode(Spire.PROCESS_MODE_PHYSICS)


func create_dot(at: Vector2i) -> void:
	var dot: Sprite2D = DOT_PREFAB.instantiate()
	add_child(dot)
	dot.global_position = at
