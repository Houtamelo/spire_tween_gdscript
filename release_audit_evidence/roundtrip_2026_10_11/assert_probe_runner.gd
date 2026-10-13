extends SceneTree
func _initialize():call_deferred("run")
func run():
    var node = load("res://assert_probe_case.gd").new()
    root.add_child(node)
    var result: bool = await node.run_test(Callable(node, "test_deliberate_mismatch"))
    print("AUDIT_ASSERT_HELPER returned_success=", result)
    node.queue_free()
    await process_frame
    quit(0 if result else 1)
