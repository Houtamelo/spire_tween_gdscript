extends SceneTree
func _initialize():call_deferred("run")
func run():
    var seq := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
    var child := Spire.do_delayed_call(func():pass, 1.0)
    seq.insert(0.5, child)
    seq.custom_step(0.25)
    print("AUDIT_SURFACE ", JSON.stringify({"close_current_block":seq.has_method("close_current_block"),"append_ctor":seq.has_method("append_ctor"),"future_insert_parent_playing":seq.is_playing(),"future_insert_child_elapsed":child.get_total_elapsed_time()}))
    seq.unregister()
    quit(0)
