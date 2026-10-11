extends Node
func _ready():
	var warm := Spire.sequence().append_interval(0.01)
	await warm.finished
	warm = null
	await get_tree().process_frame
	await get_tree().process_frame
	var before := Performance.get_monitor(Performance.OBJECT_COUNT)
	for i in range(1000):
		var tween := Spire.sequence().append_interval(0.0)
		tween.force_complete()
		tween = null
	for i in range(10): await get_tree().process_frame
	var after := Performance.get_monitor(Performance.OBJECT_COUNT)
	print("LEAK_MEASURE before=", before, " after=", after, " delta=", after-before)
	get_tree().quit()
