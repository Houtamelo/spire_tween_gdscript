@tool
extends EditorPlugin

func _enter_tree() -> void:
	_run.call_deferred()

func _settle() -> void:
	for _frame in 24:
		await get_tree().process_frame

func _batch(tweened: bool) -> void:
	for _index in 500:
		var node := Node2D.new()
		if tweened:
			var tween := DoNode2D.position(node, Vector2.ONE, 0.1)
			tween.set_process_mode(Spire.PROCESS_MODE_MANUAL)
			tween.force_complete()
		node.free()

func _count() -> int:
	return int(Performance.get_monitor(Performance.OBJECT_COUNT))

func _run() -> void:
	await _settle()
	var control := [_count()]
	for _round in 3:
		_batch(false)
		await _settle()
		control.append(_count())
	# Warm manager/class initialization before measuring tweened batches.
	var warm := Node2D.new()
	DoNode2D.position(warm, Vector2.ONE, 0.1).force_complete()
	warm.free()
	await _settle()
	var tweened := [_count()]
	for _round in 3:
		_batch(true)
		await _settle()
		tweened.append(_count())
	print("EDITOR_LIFETIME_OBSERVATION control=", control, " tweened=", tweened)
	get_tree().quit()
