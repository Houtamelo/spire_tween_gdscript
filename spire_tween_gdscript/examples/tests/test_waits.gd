extends RefCounted

# Signals precede node callbacks. Allow queued registration to reach each manager,
# then another turn to process/collect it, and resume after the last idle callback.
static func manager_turns(tree: SceneTree) -> void:
	for _turn in 2:
		await tree.physics_frame
		await tree.process_frame
	await tree.process_frame

static func until(tree: SceneTree, condition: Callable, timeout_seconds: float = 2.0) -> bool:
	var deadline := Time.get_ticks_usec() + int(timeout_seconds * 1_000_000)
	while not condition.call():
		if Time.get_ticks_usec() >= deadline:
			return false
		await tree.process_frame
	return true
