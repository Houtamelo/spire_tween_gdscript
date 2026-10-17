extends SceneTree
class Case extends "res://examples/tests/test_base.gd":
    func test_false_pass() -> bool:
        assert_eq(1, 2)
        return true
func _initialize(): call_deferred("run")
func run():
    var c = Case.new()
    root.add_child(c)
    var result = await c.run_test(Callable(c, "test_false_pass"))
    print("HARNESS_RED helper_result=", result)
    c.free()
    quit(0 if result else 1)
