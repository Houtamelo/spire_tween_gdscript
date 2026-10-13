extends SceneTree
func _initialize(): call_deferred("run")
func observe(label): print("AUDIT_BOOKKEEPING ", label, " nodes_dead_weak_roots=", SpireAuditDiagnostics.counts(), " objects=", Performance.get_monitor(Performance.OBJECT_COUNT))
func run():
    SpireAuditDiagnostics.counts()
    await process_frame
    await process_frame
    observe("warm")
    for batch in range(2):
        for i in range(1000):
            var node := Node2D.new()
            root.add_child(node)
            var tween := DoNode2D.position(node, Vector2.ONE, 1.0)
            tween.force_complete()
            node.queue_free()
        await process_frame
        await process_frame
        await process_frame
        observe("freed_node_batch_" + str(batch + 1))
    var bound := Node.new()
    root.add_child(bound)
    for batch in range(2):
        for i in range(1000):
            var tween := Spire.sequence().append_interval(0.0).bind_node(bound)
            tween.force_complete()
        await process_frame
        await process_frame
        await process_frame
        observe("expired_binding_batch_" + str(batch + 1))
    bound.queue_free()
    await process_frame
    await process_frame
    observe("bound_node_freed")
    quit(0)
