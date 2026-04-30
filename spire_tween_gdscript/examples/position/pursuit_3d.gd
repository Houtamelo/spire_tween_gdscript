extends Node3D

const PREFAB: PackedScene = preload("res://examples/position/marker.tscn")

@onready var _chaser: Node3D = $Chaser
@onready var _target: Node3D = $Target
@export var _chaser_speed: float = 500.0
@export var _target_speed: float = 400.0
@export var _bounds_min: Vector3 = Vector3(0, 0, 0)
@export var _bounds_max: Vector3 = Vector3(1920, 1080, 1080)

var _corners: Array[Vector3]

func _ready():
	_corners = [
		_bounds_min, 
		Vector3(_bounds_min.x, _bounds_min.y, _bounds_max.z),
		Vector3(_bounds_min.x, _bounds_max.y, _bounds_max.z),
		_bounds_max,
		Vector3(_bounds_max.x, _bounds_min.y, _bounds_max.z),
		Vector3(_bounds_max.x, _bounds_min.y, _bounds_min.z),
		Vector3(_bounds_max.x, _bounds_max.y, _bounds_min.z),
		Vector3(_bounds_min.x, _bounds_max.y, _bounds_min.z),
	]
	
	DoNode3D.follow(_chaser, _target, _chaser_speed) \
		.set_ease(Spire.EASE_IN_CIRC)
	
	var sequence := Spire.sequence()
	for corner in _corners:
		var tween := DoNode3D.move(_target, corner, _target_speed).as_speed_based()
		tween.finished.connect(spawn_marker.bind(corner), CONNECT_ONE_SHOT)
		sequence.append(tween)
	
	sequence.set_loops(-1)


func spawn_marker(pos: Vector3):
	var instance: Node3D = PREFAB.instantiate()
	add_child(instance)
	instance.global_position = pos
