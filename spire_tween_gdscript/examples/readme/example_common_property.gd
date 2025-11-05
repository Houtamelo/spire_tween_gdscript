extends Node2D

@onready var circle: Sprite2D = $Circle
@export var destination: Vector2 = Vector2(1200, 300)
@export var duration: float = 4.0

func _ready():
	# Delay so that the tweening process doesn't start before we focus the game window
	Spire.do_delayed_call(spire_impl, 2.0)
	#Spire.do_delayed_call(godot_impl, 2.0)

func godot_impl():
	self.create_tween().tween_property(circle, ^"global_position", destination, duration)
	self.create_tween().tween_property(circle, ^"modulate:r", 1.0, duration)

func spire_impl():
	# Signature: func(node: Node2D, to: Vector2, duration: float) -> SpirePropertyVector2 
	DoNode2D.global_position(circle, destination, duration)
	# Signature: func(node: CanvasItem, to: float, duration: float) -> SpirePropertyFloat
	DoCanvasItem.modulate_r(circle, 1.0, duration)
