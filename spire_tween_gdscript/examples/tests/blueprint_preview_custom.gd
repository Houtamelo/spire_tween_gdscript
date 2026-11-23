@tool
extends TweenBlueprint

@export var restore_state := true
@export var marker := 0
@export var capture_log: Array[String] = []
@export var restore_log: Array[String] = []

func _capture_rest_state() -> Callable:
	capture_log.append(str(marker))
	if not restore_state:
		return Callable()
	var node := get_resolved_target() as Node2D
	var position := node.position
	var label := str(marker)
	var log := restore_log
	return func() -> void:
		log.append(label)
		if is_instance_valid(node): node.position = position

func _reset_state() -> void:
	# Capture must precede even this synchronous mutation.
	get_resolved_target().position += Vector2(11, 7)

func _play_internal(sequence: SpireSequence) -> void:
	sequence.append(Spire.do_property(get_resolved_target(), ^"position", Vector2(200, 140), 1.0))
