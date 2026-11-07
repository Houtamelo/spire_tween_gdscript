# Intentional loop-boundary stepping

Tweens intentionally finish at most one loop per frame. When an update reaches or overshoots a loop boundary, the tween applies that loop's endpoint and carries excess time forward for the next update. Progress halts at the boundary until that next update. The same policy applies to individual manual processing calls.

The following observation was initially misclassified as a sampling defect during native character Blueprint validation on Godot 4.7.2. Houtamelo confirmed that it is intended behavior; no timing-core correction is needed.

After entering the scene tree, use a SpireRichTextLabel containing `A`, then run:

```gdscript
var tween := label.offset_char(0, Vector2(8, 0), 1.0)
tween.set_process_mode(Spire.PROCESS_MODE_MANUAL)
tween.set_loops(2, Spire.LOOP_MODE_YOYO)
tween.custom_step(1.5)
print(label.get_char_offset(0)) # Expected: (8, 0), the first loop's endpoint.
tween.custom_step(0.0)
print(label.get_char_offset(0)) # Expected: (4, 0), using the carried 0.5 seconds.
```

In `spire_tween/src/tweens/method/data.rs`, interpolation applies the current loop's endpoint before `handle_loop_finished` advances the loop counter and carries the excess time forward. It does not sample the next loop during that same update. The zero-duration manual step above is another processing call, so it can apply the carried time without adding more elapsed time.

Normal stepped playback and natural Yoyo completion also passed the character-authoring tests and rendered showcase.

The retained reproduction is `dist/text-blueprint-authoring-review-2026-11-06/large_step_probe.gd`, with output in `large-step-probe.log` beside it.
