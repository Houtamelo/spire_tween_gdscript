extends SceneTree


func _initialize() -> void:
	var cases := [
		[Node.new(), &"process_mode", &"ProcessMode"],
		[Control.new(), &"mouse_filter", &"MouseFilter"],
		[TextureRect.new(), &"stretch_mode", &"StretchMode"],
		[Camera2D.new(), &"anchor_mode", &"AnchorMode"],
		[Line2D.new(), &"joint_mode", &"LineJointMode"],
	]
	var failures := 0
	print("Engine: ", Engine.get_version_info()["string"])
	print("TYPE_INT=", TYPE_INT, ", PROPERTY_HINT_ENUM=", PROPERTY_HINT_ENUM,
		", PROPERTY_USAGE_STORAGE=", PROPERTY_USAGE_STORAGE,
		", PROPERTY_USAGE_EDITOR=", PROPERTY_USAGE_EDITOR,
		", PROPERTY_USAGE_CLASS_IS_ENUM=", PROPERTY_USAGE_CLASS_IS_ENUM)
	for entry in cases:
		var object: Object = entry[0]
		var property_name: StringName = entry[1]
		var enum_name: StringName = entry[2]
		var class_name_value := object.get_class()
		# no_inheritance=true verifies that this class declares the enum itself.
		var class_declares_enum := ClassDB.class_has_enum(class_name_value, enum_name, true)
		print("\n", class_name_value, ".", property_name, " : ", class_name_value, ".", enum_name)
		print("ClassDB.class_has_enum(class, enum, true): ", class_declares_enum)
		if not class_declares_enum:
			failures += 1
		var matches := 0
		for property in object.get_property_list():
			if property["name"] != property_name:
				continue
			matches += 1
			# Print the actual Dictionary, preserving StringName notation and every key.
			print(property)
			if property["type"] != TYPE_INT or property["hint"] != PROPERTY_HINT_ENUM:
				failures += 1
			if not property["usage"] & PROPERTY_USAGE_EDITOR:
				failures += 1
		if matches != 1:
			push_error("Expected one property named %s; found %d." % [property_name, matches])
			failures += 1
		object.free()
	print("\nENUM_PROPERTY_PROBE_RESULT cases=", cases.size(), " failed=", failures)
	quit(0 if failures == 0 else 1)
