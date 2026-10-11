extends "res://examples/tests/test_base.gd"

var failures: Array[String] = []

func check(condition: bool, message: String) -> void:
	if not condition:
		failures.append(message)
		push_error(message)

func add_tween(parent: SpireSequence, child: RefCounted, operation: int) -> void:
	match operation:
		0: parent.append(child)
		1: parent.join(child)
		2: parent.insert(0.0, child)

func settle() -> void:
	for _i in 4:
		await get_tree().process_frame

func test_two_node_cycles() -> bool:
	for operation in 3:
		var a := Spire.sequence().append_interval(0.03)
		var b := Spire.sequence().append_interval(0.01)
		a.append(b)
		add_tween(b, a, operation)
		var stayed_registered := a.is_registered()
		var stayed_playing := a.is_playing()
		# This also breaks the cycle when running against the unfixed library.
		var accepted := b.remove(a)
		check(not accepted, "Two-node cycle must be rejected for operation %d" % operation)
		check(stayed_registered and stayed_playing, "Rejected insertion must preserve the root")
		a.register()
		a.force_complete()
		check(a.is_stopped() and b.is_stopped(), "The remaining valid hierarchy must complete")
	await settle()
	return failures.is_empty()

func test_deeper_mixed_cycles() -> bool:
	for operation in 3:
		var a := Spire.sequence().append_interval(0.03)
		var b := Spire.sequence().append_interval(0.02)
		var c := Spire.sequence().append_interval(0.01)
		a.append(b)
		b.join(c)
		add_tween(c, a, operation)
		var stayed_registered := a.is_registered()
		var accepted := c.remove(a)
		check(not accepted, "Three-node cycle must be rejected for operation %d" % operation)
		check(stayed_registered, "Ancestor registration must survive a rejected deep insertion")
		a.register()
		a.force_complete()
		check(a.is_stopped() and b.is_stopped() and c.is_stopped(), "Valid nesting must remain usable")
	await settle()
	return failures.is_empty()

func test_rejection_preserves_state_binding_and_signals() -> bool:
	var target := Node.new()
	add_child(target)
	var a := Spire.sequence().append_interval(0.03).bind_node(target)
	var b := Spire.sequence().append_interval(0.01)
	var calls: Array[int] = [0]
	a.finished.connect(func(): calls[0] += 1)
	a.append(b)
	b.insert(0.0, a)
	var stayed_registered := a.is_registered()
	var stayed_playing := a.is_playing()
	var accepted := b.remove(a)
	check(not accepted, "Cycle insertion must fail")
	check(stayed_registered and stayed_playing, "Rejected child must retain playback and registration")
	check(a.get_bound_nodes() == [target], "Rejected child must retain its bindings")
	a.register()
	a.force_complete()
	check(calls[0] == 1 and a.is_stopped(), "Its manager binding and signal must still work once")
	a.play()
	target.queue_free()
	await settle()
	check(not a.is_registered() and a.is_stopped(), "Preserved binding must still cancel on node deletion")
	return failures.is_empty()

func test_repeated_incoming_graph_is_rejected() -> bool:
	var shared := Spire.sequence().append_interval(0.01)
	var left := Spire.sequence().append_interval(0.02).append(shared)
	var right := Spire.sequence().append_interval(0.02).append(shared)
	var candidate := Spire.sequence().append_interval(0.03).append(left).append(right)
	candidate.unregister()
	var parent := Spire.sequence().append_interval(0.05)
	parent.append(candidate)
	var accepted := parent.remove(candidate)
	check(not accepted, "An incoming graph with a repeated tween must be rejected")
	check(not candidate.is_registered() and candidate.is_playing(), "Rejected candidate must remain unchanged")
	# Reuse is unsupported; clean up this deliberately invalid fixture before ticking.
	right.remove(shared)
	candidate.stop()
	parent.stop()
	await settle()
	return failures.is_empty()

func test_valid_nested_append_join_insert_and_replay() -> bool:
	var children: Array[SpireSequence] = []
	var calls: Array[int] = [0]
	var root := Spire.sequence().append_interval(0.05)
	for operation in 3:
		var child := Spire.sequence().append_interval(0.01)
		child.finished.connect(func(): calls[0] += 1)
		children.append(child)
		add_tween(root, child, operation)
	root.register()
	await get_tree().create_timer(0.15).timeout
	check(root.is_stopped() and calls[0] == 3, "Valid nested children must all finish once")
	for child in children:
		check(child.is_stopped() and not child.is_registered(), "Nested children must not remain roots")
	root.unregister()
	await settle()
	root.register()
	root.play()
	await get_tree().create_timer(0.15).timeout
	check(root.is_stopped() and calls[0] == 6, "Re-registration must preserve replay and child signals")
	return failures.is_empty()

func test_wide_and_deep_valid_hierarchies() -> bool:
	var wide := Spire.sequence().append_interval(0.001)
	for _i in 24:
		wide.append(Spire.sequence().append_interval(0.001))
	var deep := Spire.sequence().append_interval(0.001)
	var cursor := deep
	for _i in 32:
		var child := Spire.sequence().append_interval(0.001)
		cursor.append(child)
		cursor = child
	wide.register()
	deep.register()
	await get_tree().create_timer(0.2).timeout
	check(wide.is_stopped() and deep.is_stopped(), "Valid hierarchies exceeding inline capacities must finish")
	return failures.is_empty()
