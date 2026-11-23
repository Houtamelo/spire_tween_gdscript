@tool
extends Control
## The scene contains real editable template instances. This script only wires
## demo buttons; animation configuration lives on each template root.


func _ready() -> void:
	resized.connect(_resize_grid)
	_resize_grid.call_deferred()
	if Engine.is_editor_hint():
		return
	for card in $Scroll/Grid.get_children():
		var effect: TweenBlueprint = card.get_node("Content/TargetSlot/Target/Effect")
		card.get_node("Content/Buttons/Play").pressed.connect(effect.play)
		card.get_node("Content/Buttons/Restore").pressed.connect(effect.restore)


func _resize_grid() -> void:
	$Scroll/Grid.columns = clampi(floori(($Scroll.size.x + 24.0) / 354.0), 1, 3)


func _exit_tree() -> void:
	if Engine.is_editor_hint():
		return
	for card in $Scroll/Grid.get_children():
		var effect: TweenBlueprint = card.get_node("Content/TargetSlot/Target/Effect")
		effect.stop()
