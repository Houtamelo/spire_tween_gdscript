@tool
extends TweenBlueprint

@export var custom_duration: float = 0.5
@export var character_distance: float = 24.0
var reset_count: int = 0
var build_count: int = 0

func _reset_state() -> void:
	reset_count += 1

func _play_internal(sequence: SpireSequence) -> void:
	build_count += 1
	sequence.append_interval(custom_duration)
