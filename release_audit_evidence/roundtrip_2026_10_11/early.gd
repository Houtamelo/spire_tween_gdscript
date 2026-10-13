extends SceneTree
var hits := [0]
var early = Spire.do_delayed_call(func(): hits[0] += 1, 0.01)
func _initialize():
    call_deferred("observe")
func observe():
    await create_timer(0.1).timeout
    var later_hits := [0]
    var later := Spire.do_delayed_call(func(): later_hits[0] += 1, 0.01)
    await create_timer(0.1).timeout
    print("AUDIT_EARLY ", JSON.stringify({"early_hits":hits[0],"later_hits":later_hits[0],"registered":later.is_registered(),"controller":root.has_node("tweens_controller")}))
    quit(0)
