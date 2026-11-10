@tool
extends RichTextEffect

var bbcode = "foreign_glow"

func _process_custom_fx(fx: CharFXTransform) -> bool:
	fx.color = Color(0, 1, 0, 0.75)
	return true
