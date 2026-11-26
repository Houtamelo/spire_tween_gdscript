extends RefCounted


static func snapshot(target: Node) -> Array:
	if target is SpireRichTextLabel:
		var values: Array = []
		for index in target.get_character_count():
			values.append_array([target.get_char_offset(index), target.get_char_scale(index),
				target.get_char_rotation(index), target.get_char_pivot(index), target.get_char_color(index)])
		return values
	return [target.position, target.scale, target.rotation, target.modulate]


static func same(a: Array, b: Array) -> bool:
	if a.size() != b.size(): return false
	for index in a.size():
		match typeof(a[index]):
			TYPE_FLOAT:
				if not is_equal_approx(a[index], b[index]): return false
			TYPE_VECTOR2, TYPE_VECTOR3, TYPE_COLOR:
				if not a[index].is_equal_approx(b[index]): return false
			_:
				if a[index] != b[index]: return false
	return true
