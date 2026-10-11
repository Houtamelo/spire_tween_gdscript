extends Node
func _ready():
	var a := Spire.sequence()
	var b := Spire.sequence()
	a.append(b)
	print("FIRST_APPEND")
	b.append(a)
	print("SECOND_APPEND")
	a.register()
	print("ROOT_REGISTERED")
	await get_tree().process_frame
	a = null
	b = null
	for _i in 4:
		await get_tree().process_frame
	get_tree().quit()
