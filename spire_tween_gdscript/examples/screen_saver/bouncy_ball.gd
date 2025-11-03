extends RigidBody2D

const COLLISION_FX := preload("res://examples/screen_saver/collision_fx.tscn")
var collision_pos: Vector2

func _ready():
	self.body_entered.connect(_on_body_entered)

func _integrate_forces(state):
	if(state.get_contact_count() >= 1):
		collision_pos = state.get_contact_local_position(0)


func _on_body_entered(__):
	self.linear_velocity = self.linear_velocity.rotated(randf_range(-PI / 8, PI / 8))
	var fx := COLLISION_FX.instantiate()
	get_tree().root.add_child(fx)
	fx.global_position = collision_pos
