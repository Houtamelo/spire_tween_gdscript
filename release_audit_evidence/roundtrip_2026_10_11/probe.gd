extends SceneTree

func _initialize():
    call_deferred("run")

func record(label: String, facts: Dictionary):
    print("AUDIT_PROBE ", label, " ", JSON.stringify(facts))

func run():
    # Valid controls, and invalid-target method tween cancellation.
    var sink := [0.0]
    var valid := Spire.do_call_float(func(x): sink[0] = x, 0.0, 1.0, 1.0)
    valid.set_process_mode(Spire.PROCESS_MODE_MANUAL)
    valid.custom_step(0.25)
    record("valid_method", {"value":sink[0], "playing":valid.is_playing()})
    valid.force_complete()
    var n := Node.new()
    root.add_child(n)
    var cb := Callable(n, "set_process_priority")
    var invalid := Spire.do_call_int(cb, 0, 10, 1.0)
    n.free()
    await process_frame
    await process_frame
    record("freed_method_natural", {"callable_valid":cb.is_valid(), "registered":invalid.is_registered(), "playing":invalid.is_playing(), "stopped":invalid.is_stopped()})
    var count := [0]
    var missing := Spire.do_call_float(Callable(self, "missing_method"), 0.0, 1.0, 1.0)
    missing.finished.connect(func(): count[0] += 1)
    missing.force_complete()
    record("invalid_method_force", {"finished_count":count[0], "registered":missing.is_registered(), "stopped":missing.is_stopped()})
    missing.unregister()
    # Consecutive blocks must pass unused time in the parent time domain.
    for speed in [1.0, 2.0, 0.5]:
        var delta: float = 1.0 / speed + 0.1
        var a := Spire.do_call_float(func(_x): pass, 0.0, 1.0, 1.0).set_speed_scale(speed)
        var b := Spire.do_call_float(func(_x): pass, 0.0, 1.0, 1.0)
        var seq := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
        seq.append(a).append(b)
        seq.custom_step(delta)
        record("child_speed_excess", {"speed":speed,"delta":delta,"successor_elapsed":b.get_total_elapsed_time(),"expected":0.1})
        seq.force_complete()
        seq.unregister()
    # Valid, already-immediate-free Node owners must cancel without cloning a freed Gd.
    var p := Node2D.new()
    root.add_child(p)
    var prop := DoNode2D.position(p, Vector2.ONE, 1.0)
    p.free()
    await process_frame
    await process_frame
    record("immediate_free_property", {"registered":prop.is_registered(),"stopped":prop.is_stopped()})
    # Finite inputs control versus nonfinite clock state.
    for scale in [1.0, -1.0, NAN, INF]:
        var t := Spire.do_call_float(func(_x): pass, 0.0, 1.0, 1.0).set_process_mode(Spire.PROCESS_MODE_MANUAL).set_speed_scale(scale)
        t.custom_step(0.1)
        record("speed_input", {"scale":str(scale),"elapsed":str(t.get_total_elapsed_time()),"playing":t.is_playing(),"stopped":t.is_stopped()})
        t.unregister()
    quit(0)
