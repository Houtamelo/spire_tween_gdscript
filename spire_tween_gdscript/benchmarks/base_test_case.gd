extends Node
class_name BaseTestCase

const RUNNER := preload("res://benchmarks/runner.gd")
const UTIL := preload("res://benchmarks/util.gd")
const SETUPS := preload("res://benchmarks/tween_setups.gd")

func _ready():
	await RUNNER.run_tests(call(&"_test_name"), Callable(self, &"_run"))
	
	var tree := get_tree()
	await tree.create_timer(0.5).timeout
	tree.quit()

## Virtual funcs
func _run(_root: Node, _is_builtin: bool, _duration: float, _amount: int) -> void: pass
func _test_name() -> String: return "Unnamed"
