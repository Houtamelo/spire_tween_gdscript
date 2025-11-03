extends Control

const DIALOGUES := [
	"Hi there.",
	"Thank you for trying out Spire Tween!",
	"I hope these examples give you inspiration to build wonderful games :)"
]

@onready var _label: Label = $Panel/Label
@onready var _button_next: Button = $Button_Next
@onready var _button_previous: Button = $Button_Previous
@export var _chars_per_second: float = 30.0

var _idx: int = -1
var _tween: SpirePropertyInt

func _ready() -> void:
	_button_next.pressed.connect(_next_line)
	_button_previous.pressed.connect(_previous_line)


func _next_line():
	if (_idx + 1) >= DIALOGUES.size(): return
	
	Spire.unregister(_tween) # Ensure previous tween is no more.
	
	_idx += 1
	_label.text = DIALOGUES[_idx]
	_label.visible_characters = 0 # Hide all characters when starting.
	
	# When going forward, show characters one by one.
	_tween = DoLabel.visible_characters(_label, _label.text.length(), _chars_per_second) \
		.as_speed_based()


func _previous_line():
	if (_idx - 1) < 0: return
	
	Spire.unregister(_tween) # Ensure previous tween is no more.
	_tween = null # Drop reference
	
	_idx -= 1
	_label.text = DIALOGUES[_idx]
	_label.visible_ratio = 1.0 # When going backwards, show text immediately (mirrors the behavior of Ren'py).
