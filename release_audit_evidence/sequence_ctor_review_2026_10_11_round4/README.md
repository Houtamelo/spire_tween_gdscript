# Sequence constructor review — round 4, 2026-10-11

Current source passes `cargo check -p spire_tween --features
include_gdext_lib,include_gdscript_bridge`. Runtime verification used an unmodified
copy of that crate in a separate cdylib and real headless Godot 4.7.2 SceneTree.
No source repairs or installed-library replacement were applied.

The probe reports 26 checks: 21 pass and 5 fail. The failures share two causes:

1. `process()` computes insert excess, but only uses it to decide whether playback
   can finish. It still calls `handle_loop_finished(shared_excess_queue)`, instead
   of using the minimum of queue and insert excess. An insert at 0.25 seconds with
   duration 0.25 completes in a 0.6-second update with reported excess 0.6 instead
   of 0.1. A nested successor likewise receives 0.6 instead of 0.1.
2. Inserted RustFn and Callable branches never reduce shared excess on their first
   successful invocation. With a 0.3-second queue and an inserted callback at 0.5,
   a 0.6-second update reports excess 0.3 instead of 0.1. Even fixing the first
   cause would leave this case incorrect: the computed insert excess remains the
   full frame delta. A two-loop callback at 0.5 still invokes its second callback
   at total elapsed time 0.61 instead of waiting until 1.0. First invocation must
   constrain excess to the already computed clipped time past insertion; previously
   invoked callbacks should contribute no constraint.

Passing checks include the previously failing stopped-future-child case and two
new interleaved pruning cases with 18 deleted and 18 surviving slots, covering
SmallVec spill in normal processing and forced completion. The prior constructor
admission, registration, easing, deferred construction, delta clipping, replay,
forced completion, explicit binding, and actual queued-node deletion cases pass.

Reverse-index Vec::remove preserves indices and survivors, but still has
quadratic worst-case compaction for alternating removals and survivors. A single
retain/compaction pass can use the existing sorted deletion indices. This is an
algorithmic observation, not a measured performance benchmark.

The original fixture lives at `/tmp/spire-sequence-review4-99q8pg9u` and deliberately
exits 5, its failed-check count. Artifacts here preserve the real-engine probe,
complete output, named results, reviewed source hashes, and probe library hash.
Expected diagnostics are emitted for deliberately rejected self-cycle/duplicate
children and nil constructor returns. No full-suite success is claimed.
