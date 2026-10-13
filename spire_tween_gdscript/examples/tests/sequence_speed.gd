extends "res://examples/tests/test_base.gd"

var failures: Array[String] = []
var cases := 0
var roots: Array[SpireSequence] = []

func sequence() -> SpireSequence:
	var seq := Spire.sequence().set_process_mode(Spire.PROCESS_MODE_MANUAL)
	roots.append(seq)
	return seq

func _exit_tree() -> void:
	for seq in roots:
		seq.stop()
		seq.unregister()

func check(condition: bool, label: String, detail: String = "") -> void:
	if not condition:
		failures.append(label + ": " + detail)
		push_error(failures[-1])

func near(actual: float, expected: float) -> bool:
	return abs(actual - expected) < 0.000001

func child(kind: String) -> Variant:
	match kind:
		"method": return Spire.do_call_float(func(_x): pass, 0.0, 1.0, 1.0)
		"delayed": return Spire.do_delayed_call(func(): pass, 1.0)
		"sequence": return Spire.sequence().append_interval(1.0)
		"absolute", "relative", "speed_based":
			var node := Node2D.new()
			add_child(node)
			var tween := DoNode2D.position(node, Vector2(1, 0), 1.0).set_ease(Spire.EASE_LINEAR)
			if kind == "relative": tween.as_relative()
			if kind == "speed_based": tween.as_speed_based()
			return tween
	return null

func successor() -> SpireMethodFloat:
	return Spire.do_call_float(func(_x): pass, 0.0, 1.0, 10.0)

func cleanup(seq: SpireSequence) -> void:
	seq.force_complete()
	seq.unregister()

func test_excess_uses_parent_time_for_all_tween_types() -> bool:
	for kind in ["method", "delayed", "sequence", "absolute", "relative", "speed_based"]:
		for scale in [0.5, 1.0, 2.0]:
			for parent_scale in [0.5, 1.0, 2.0]:
				cases += 1
				var first: Variant = child(kind).set_speed_scale(scale)
				var next := successor()
				var seq := sequence().set_speed_scale(parent_scale)
				seq.append(first).append(next)
				seq.custom_step((1.0 / scale + 0.1) / parent_scale)
				var label := "serial %s speed=%s parent=%s" % [kind, scale, parent_scale]
				check(first.is_stopped(), label, "first child did not complete")
				check(near(next.get_total_elapsed_time(), 0.1), label, "successor elapsed=%s, expected=0.1" % next.get_total_elapsed_time())
				cleanup(seq)
	return failures.is_empty()


func test_join_excess_uses_latest_finisher_in_parent_time() -> bool:
	for reverse in [false, true]:
		cases += 1
		var fast: Variant = child("method").set_speed_scale(2.0)
		var slow: Variant = child("absolute").set_speed_scale(0.5)
		var next := successor()
		var seq := sequence()
		if reverse: seq.append(slow).join(fast)
		else: seq.append(fast).join(slow)
		seq.append(next)
		seq.custom_step(2.1)
		check(near(next.get_total_elapsed_time(), 0.1), "join reverse=%s" % reverse, "elapsed=%s" % next.get_total_elapsed_time())
		cleanup(seq)
	return failures.is_empty()


func test_inserted_child_excess_crosses_sequence_boundaries() -> bool:
	for scale in [0.5, 2.0]:
		cases += 1
		var inserted: Variant = child("method").set_speed_scale(scale)
		var nested := Spire.sequence().append_interval(0.1).insert(0.25, inserted).set_speed_scale(2.0)
		var next := successor()
		var seq := sequence().append(nested).append(next)
		seq.custom_step((0.25 + 1.0 / scale) / 2.0 + 0.1)
		check(nested.is_stopped(), "insert speed=%s" % scale, "nested sequence did not complete")
		check(near(next.get_total_elapsed_time(), 0.1), "insert speed=%s" % scale, "elapsed=%s" % next.get_total_elapsed_time())
		cleanup(seq)
	return failures.is_empty()


func test_nested_sequences_unscale_at_each_level() -> bool:
	for outer_scale in [0.5, 2.0]:
		for middle_scale in [0.5, 2.0]:
			for leaf_scale in [0.5, 2.0]:
				cases += 1
				var leaf: Variant = child("method").set_speed_scale(leaf_scale)
				var middle := Spire.sequence().append(leaf).set_speed_scale(middle_scale)
				var outer := Spire.sequence().append(middle).set_speed_scale(outer_scale)
				var next := successor()
				var seq := sequence().append(outer).append(next)
				seq.custom_step(1.0 / (leaf_scale * middle_scale * outer_scale) + 0.1)
				check(near(next.get_total_elapsed_time(), 0.1), "nested %s/%s/%s" % [outer_scale, middle_scale, leaf_scale], "elapsed=%s" % next.get_total_elapsed_time())
				cleanup(seq)
	return failures.is_empty()


func test_scaled_excess_accounts_for_parent_and_child_delays() -> bool:
	for kind in ["method", "absolute", "relative", "speed_based", "sequence"]:
		for scale in [0.5, 2.0]:
			cases += 1
			var first: Variant = child(kind).set_delay(0.4).set_speed_scale(scale)
			var next := successor()
			var seq := sequence().set_delay(0.3).set_speed_scale(2.0).append(first).append(next)
			seq.custom_step((0.3 + 1.4 / scale + 0.1) / 2.0)
			check(near(next.get_total_elapsed_time(), 0.1), "delay %s/%s" % [kind, scale], "elapsed=%s" % next.get_total_elapsed_time())
			cleanup(seq)
	return failures.is_empty()


func test_only_final_loop_excess_is_unscaled() -> bool:
	for kind in ["method", "absolute", "relative"]:
		for mode in [Spire.LOOP_MODE_RESTART, Spire.LOOP_MODE_YOYO, Spire.LOOP_MODE_INCREMENTAL]:
			for scale in [0.5, 2.0]:
				cases += 1
				var first: Variant = child(kind).set_speed_scale(scale).set_loops(2, mode)
				var next := successor()
				var seq := sequence().append(first).append(next)
				seq.custom_step(1.2 / scale)
				check(near(next.get_total_elapsed_time(), 0.0), "loop before final %s/%s/%s" % [kind, mode, scale])
				seq.custom_step(1.2 / scale)
				check(first.is_stopped(), "loop completion %s/%s/%s" % [kind, mode, scale])
				check(near(next.get_total_elapsed_time(), 0.4 / scale), "loop excess %s/%s/%s" % [kind, mode, scale], "elapsed=%s expected=%s" % [next.get_total_elapsed_time(), 0.4 / scale])
				cleanup(seq)
	return failures.is_empty()


func test_near_zero_speed_suppresses_excess_with_strict_cutoff() -> bool:
	for kind in ["method", "delayed", "absolute", "relative", "speed_based", "sequence"]:
		for scale in [0.000005, 0.00001, 0.00002]:
			cases += 1
			var first: Variant = child(kind).set_speed_scale(scale)
			var next := successor()
			var seq := sequence().append(first).append(next)
			seq.custom_step(1.0 / scale + 0.1)
			var expected := 0.0 if scale < 0.00001 else 0.1
			check(first.is_stopped(), "cutoff completion %s/%s" % [kind, scale])
			check(near(next.get_total_elapsed_time(), expected), "cutoff %s/%s" % [kind, scale], "elapsed=%s expected=%s" % [next.get_total_elapsed_time(), expected])
			cleanup(seq)
	return failures.is_empty()


func test_zero_speed_blocks_successor() -> bool:
	for kind in ["method", "delayed", "absolute", "relative", "speed_based", "sequence"]:
		cases += 1
		var frozen: Variant = child(kind).set_speed_scale(0.0)
		var next := successor()
		var seq := sequence().append(frozen).append(next)
		seq.custom_step(100.0)
		check(frozen.is_playing() and near(next.get_total_elapsed_time(), 0.0), "zero speed %s" % kind)
		cleanup(seq)
	return failures.is_empty()

func test_lazy_constructor_excess_uses_parent_time() -> bool:
	for operation in [0, 1, 2]:
		cases += 1
		var make := func(): return Spire.do_call_float(func(_x): pass, 0.0, 1.0, 1.0).set_speed_scale(2.0)
		var nested := Spire.sequence().set_speed_scale(2.0)
		if operation == 0: nested.append_ctor(make)
		elif operation == 1: nested.join_ctor(make)
		else: nested.insert_ctor(0.25, make)
		var next := successor()
		var seq := sequence().append(nested).append(next)
		var duration := 0.375 if operation == 2 else 0.25
		seq.custom_step(duration + 0.1)
		check(near(next.get_total_elapsed_time(), 0.1), "ctor %s" % operation, "elapsed=%s" % next.get_total_elapsed_time())
		cleanup(seq)
	return failures.is_empty()
