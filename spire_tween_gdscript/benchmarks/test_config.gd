extends RefCounted

var duration: Variant = null
var amount: Variant = null
var is_builtin: Variant = null

func _init() -> void:
	var amount_env := OS.get_environment("TEST_NODE_AMOUNT")
	if amount_env != "":
		amount = amount_env.to_int()
	
	var duration_env := OS.get_environment("TEST_DURATION")
	if duration_env != "":
		duration = duration_env.to_float()
	
	var is_builtin_env := OS.get_environment("TEST_IS_BUILTIN")
	if is_builtin_env != "":
		is_builtin = bool(is_builtin_env.to_int())
