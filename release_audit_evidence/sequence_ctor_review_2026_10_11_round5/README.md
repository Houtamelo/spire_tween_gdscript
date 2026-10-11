# Sequence constructor review — round 5, 2026-10-11

The two excess-time findings from round 4 are repaired. The actual current source
passes `cargo check -p spire_tween --features include_gdext_lib,include_gdscript_bridge`.
An unmodified copied crate was built into a separate probe cdylib and run in a real
headless Godot 4.7.2 SceneTree with default API 4.6 bindings. All 28 focused checks
pass; the probe exits zero. Source files and installed addon libraries were not
modified. Exact reviewed source hashes and the probe library hash accompany this
report.

## Timing evidence

- Insert at 0.25 seconds with duration 0.25, processed with delta 0.6, reports
  excess approximately 0.1 and final loop time 0.5.
- Its nested successor receives approximately 0.1, rather than 0.6.
- The first loop of a callback inserted at 0.5 carries approximately 0.1 after
  processing delta 0.6. Advancing another 0.01 leaves the callback count at one.
- Both Rust and Godot-callable inserted callbacks constrain excess correctly when
  the block queue is shorter than their insertion offset.
- The second callback completes at the second offset when the next step crosses
  it; stopped future inserted children are skipped immediately.

## Focused coverage

The passing cases include lazy block activation, one-time construction and replay,
pending constructors with empty queues, inserts outliving the queue, per-update
delta and clipping at the offset, constructor registration cleanup and easing,
duplicate and self-cycle rejection, callable factory conversion, forced block and
insert constructor completion, failed-then-valid traversal, explicit binding
completion, actual queued-node deletion, and interleaved pruning with 18 deleted
and 18 surviving entries in normal and forced playback (SmallVec spill).

Deliberate invalid fixtures emit their expected duplicate, self-cycle, and nil
factory result diagnostics. This is focused verification, not a full-suite result.

## Remaining earlier performance observation

The reverse-order Vec::remove loops retain quadratic worst-case compaction for
interleaved deleted and kept entries. Indices and survivors are handled correctly;
a single compaction pass could address the remaining algorithmic cost. No timing
benchmark was performed for this observation.

## Artifacts

- `probe.rs`: real-engine harness and named checks.
- `runtime.log`: complete engine output and timing observations.
- `runtime-result.json`: engine command, exit code, and 28 check results.
- `source-hashes.json`: exact source snapshots.
- `artifact-hash.json`: probe library path and SHA-256.

The isolated runnable fixture is `/tmp/spire-sequence-review5-qn068f66`.
