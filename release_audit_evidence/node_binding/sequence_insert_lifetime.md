# Separate finding: insert-only sequences complete before their children

This behavior reproduces with the pre-binding-fix local addon and the binding candidate on Godot 4.7.2 at fixed 120 FPS, using an unbound child:

```gdscript
var child := Spire.sequence().append_interval(0.1)
var parent := Spire.sequence().insert(0.0, child)
await get_tree().create_timer(0.2).timeout
print(parent.is_stopped(), child.is_playing(), child.get_total_elapsed_time())
```

Both runs report:

```text
INSERT_ONLY parent_stopped=true child_playing=true child_elapsed=0.01666666666667
```

In `spire_tween/src/tweens/sequence/data.rs`, `process()` advances inserted items, then advances the block queue. Once that queue is exhausted, it calls `handle_loop_finished()` without waiting for inserted tweens to finish. An empty queue therefore allows the parent to stop while the inserted child is still playing. This conflicts with `insert()` documentation promising that inserted tweens affect total duration.

This requires a separate sequence playback fix and regression coverage for empty/shorter queues, delayed inserts, completion signals, replay, and loops. The binding suites keep their parent queue active long enough to test insertion and re-registration independently of this defect. Sequence playback was left unchanged in the binding fix.

See `baseline-insert-only.log` and `candidate-insert-only.log` for the engine output. The minimal probe calls `quit()` while retaining its local wrappers, so its shutdown warning is not evidence of another permanent ownership leak.
