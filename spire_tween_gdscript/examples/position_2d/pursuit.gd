extends Node2D

@onready var _chaser: Node2D = $Chaser
@onready var _target: Node2D = $Target
@export var _chaser_speed: float = 500.0
@export var _target_speed: float = 400.0
@export var _bounds_min: Vector2 = Vector2(0, 0)
@export var _bounds_max: Vector2 = Vector2(1920, 1080)

var _corners: Array[Vector2]

func _ready():
	_corners = [
		_bounds_min, 
		Vector2(_bounds_max.x, _bounds_min.y),
		_bounds_max,
		Vector2(_bounds_min.x, _bounds_max.y),
	]
	
	DoNode2D.follow(_chaser, _target, _chaser_speed) \
		.set_ease(Spire.EASE_IN_CIRC)
	
	var sequence := Spire.sequence()
	for corner in _corners:
		sequence.append(DoNode2D.move(_target, corner, _target_speed).as_speed_based())
	
	sequence.set_loops(-1)


func _clamp_within_bounds(goal: Vector2) -> Vector2:
	goal.x = clampf(goal.x, _bounds_min.x, _bounds_max.x)
	goal.y = clampf(goal.y, _bounds_min.y, _bounds_max.y)
	return goal
