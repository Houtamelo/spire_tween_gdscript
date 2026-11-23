@tool
extends TweenBlueprint

func _play_internal(_sequence: SpireSequence) -> void:
	get_resolved_target().position += Vector2(100, 0)
