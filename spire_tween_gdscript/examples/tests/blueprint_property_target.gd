@tool
extends Node2D

@export var seed: float = 0.0
@export_enum("First", "Second") var discrete_value: int = 0
@export_file var file_value: String = ""
@export var shape: WorldBoundaryShape2D = WorldBoundaryShape2D.new()

var dynamic_value: float = 2.0

func _get_property_list() -> Array[Dictionary]:
	return [
		{"name": "dynamic_scalar", "type": TYPE_FLOAT, "usage": PROPERTY_USAGE_DEFAULT},
		{"name": "readonly_scalar", "type": TYPE_FLOAT, "usage": PROPERTY_USAGE_EDITOR | PROPERTY_USAGE_READ_ONLY},
		{"name": "unsupported_toggle", "type": TYPE_BOOL, "usage": PROPERTY_USAGE_EDITOR},
	]

func _get(property: StringName) -> Variant:
	match property:
		&"dynamic_scalar": return dynamic_value
		&"readonly_scalar": return 4.0
		&"unsupported_toggle": return false
	return null

func _set(property: StringName, value: Variant) -> bool:
	if property == &"dynamic_scalar":
		dynamic_value = value
		return true
	return false
