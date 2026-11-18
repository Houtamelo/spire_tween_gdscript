extends SceneTree


# These are GDScript exports typed with engine enums, not native properties.
class ExportedEngineEnums:
	extends Resource
	@export var node_process_mode: Node.ProcessMode = Node.PROCESS_MODE_INHERIT
	@export var control_mouse_filter: Control.MouseFilter = Control.MOUSE_FILTER_STOP
	@export var texture_stretch_mode: TextureRect.StretchMode = TextureRect.STRETCH_SCALE


func _initialize() -> void:
	print("Engine: ", Engine.get_version_info()["string"])
	print("PROPERTY_USAGE_CLASS_IS_ENUM=", PROPERTY_USAGE_CLASS_IS_ENUM,
		", PROPERTY_USAGE_SCRIPT_VARIABLE=", PROPERTY_USAGE_SCRIPT_VARIABLE,
		", PROPERTY_USAGE_STORAGE=", PROPERTY_USAGE_STORAGE,
		", PROPERTY_USAGE_EDITOR=", PROPERTY_USAGE_EDITOR)
	var classes := ClassDB.get_class_list()
	classes.sort()
	var native_matches := 0
	var native_properties := 0
	for class_name_value in classes:
		# no_inheritance=true counts each class's declared properties once.
		for property in ClassDB.class_get_property_list(class_name_value, true):
			native_properties += 1
			if property["usage"] & PROPERTY_USAGE_CLASS_IS_ENUM:
				print("Native ", class_name_value, ": ", property)
				native_matches += 1
	print("NATIVE_ENUM_SCAN_RESULT classes=", classes.size(),
		" properties=", native_properties, " matches=", native_matches)

	var expected_types := {
		"node_process_mode": &"Node.ProcessMode",
		"control_mouse_filter": &"Control.MouseFilter",
		"texture_stretch_mode": &"TextureRect.StretchMode",
	}
	var fixture := ExportedEngineEnums.new()
	var matches := 0
	var failures := 0
	print("\nGDScript exported properties, from fixture.get_property_list():")
	for property in fixture.get_property_list():
		if not expected_types.has(property["name"]):
			continue
		print(property)
		matches += 1
		if not property["usage"] & PROPERTY_USAGE_CLASS_IS_ENUM:
			failures += 1
		if property["class_name"] != expected_types[property["name"]]:
			failures += 1
		if property["type"] != TYPE_INT or property["hint"] != PROPERTY_HINT_ENUM:
			failures += 1
		if not property["usage"] & PROPERTY_USAGE_EDITOR:
			failures += 1
	if matches != expected_types.size():
		failures += 1
	print("\nCLASS_ENUM_FLAG_PROBE_RESULT matches=", matches, " failed=", failures)
	quit(0 if failures == 0 else 1)
