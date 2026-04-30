# Test Port Review: GDScript -> Rust

Comparing tests in `spire_tween_gdscript/examples/tests/` (source of truth) against `spire_tween_rust/src/tests/`.

---

## error_handling.gd -> error_handling.rs

GDScript defines **4 tests**. Rust has **3**.

Some GDScript error-handling tests target mistakes that Rust's type system prevents at compile time. These are analyzed individually below.

### test_invalid_enum -- JUSTIFIED OMISSION (should be deleted)

- GDScript passes `-1 as Spire.LoopMode` to test runtime handling of an invalid enum value.
- In Rust, `LoopMode` is a proper enum -- invalid variants cannot be constructed. The compiler prevents this entirely.
- The current Rust port passes a valid `LoopMode::Restart`, making it a normal 2-loop test -- behavior already covered by other test files (delays, play_pause_stop). **Should be deleted rather than kept as a misleading duplicate.**

### test_invalid_tween -- JUSTIFIED OMISSION

- GDScript passes a `RefCounted` (wrong type) and `null` to `register`/`unregister`/`is_registered`.
- In Rust, these functions take typed parameters -- you can't pass a `RefCounted` where a tween handle is expected, and `null` doesn't exist. The compiler prevents this.
- **Correctly omitted.**

### test_free_while_playing

- **Valid runtime scenario** -- a `Gd<Node>` can be `queue_free`'d while a tween targeting it is active. The TweenManager must detect and clean up.
- Faithfully ported.
- **Relaxed tolerance**: Rust waits 4 frames (line 106) vs GDScript's 2 frames (lines 51-52) before checking post-free state.
- Assertions are identical.

### test_invalid_sequence_adding

- **Self-append skipped but should be ported**: GDScript does `seq.append(seq)`. Rust comment (line 76-77) claims ownership prevents it, but this is incorrect -- after calling `register()` you get an `RcPtr`, which can be cloned and passed to `append_ptr`. Self-referencing a sequence via `RcPtr` is a valid runtime scenario that should be tested.
- Double-append and join-same-tween are **valid runtime scenarios** (possible via `RcPtr::clone`) and are faithfully ported.

### Verdict

`test_invalid_enum` and `test_invalid_tween` are correctly omitted -- Rust's type system prevents those errors at compile time. `test_invalid_enum` should be deleted entirely rather than kept as a misleading duplicate. The self-append test in `test_invalid_sequence_adding` should be ported since `RcPtr` makes it possible at runtime.

---

## misc.gd -> misc.rs

GDScript defines **3 tests**. Rust has **3**. All present.

### test_force_complete

Faithful port. All assertions, values, and sequencing match exactly.

### test_dyn_target

Faithful port. Dynamic target via `RcPtr<Vector2>` with interior mutability replaces GDScript closure capture -- idiomatic Rust equivalent. All values, tolerances (0.01 distance), and assertions match.

### test_property_path

Faithful port. Both assert `scale:x` and `global_position` paths.

### Verdict

Clean port. No issues found.

---

## play_pause_stop.gd -> play_pause_stop.rs

GDScript defines **3 tests**. Rust has **3**. All present.

### test_play_works

- **Conditional wait_finished**: Rust wraps `wait_finished` with `if !handle.is_stopped()` (line 48). GDScript unconditionally awaits both `loop_finished` and `finished`. For a single-loop tween, these may fire in the same frame, so the Rust version can skip the `finished` timing assertion entirely. This is a **relaxation**.
- All other assertions match.

### test_pause_play_works

- Same conditional `wait_finished` relaxation as above.
- **MISSING**: GDScript lines 41-42 connect error-printing callbacks to `loop_finished` and `finished` to verify the stopped tween never emits signals again. Rust omits this verification entirely.
- Core pause/play logic is otherwise faithfully ported.

### test_play_stop_play_works

- Same conditional `wait_finished` relaxation.
- Otherwise faithfully ported.

### Verdict

The conditional `wait_finished` pattern appears in all 3 tests and relaxes timing validation on the `finished` signal. The missing stopped-tween signal guard in `test_pause_play_works` is a notable omission -- it was a deliberate check in GDScript to verify stopped tweens don't re-emit.

---

## lerp_modes.gd -> lerp_modes.rs

GDScript defines **4 tests**. Rust has **4**. All present.

### test_relative, test_two_relatives, test_speed_based, test_speed_based_plus_relative

All faithfully ported. Same values, same tolerances (0.1 distance, 0.017 timing), same assertions.

Minor cosmetic omissions: two `target_vfx` debug sprites (visual-only) are omitted in Rust. No effect on test correctness.

### Verdict

Clean port. No issues found.

---

## delays.gd -> delays.rs

GDScript defines **4 tests**. Rust has **4**. All present.

### test_delay_is_respected

Faithful port. No issues.

### test_extra_loops_dont_have_delay

- Defensive `if !tween.is_stopped()` guard before `wait_finished` (same pattern as play_pause_stop). Justified to avoid async deadlock.
- Otherwise faithful.

### test_sequence_respects_delay

- Explicit `LoopMode::Restart` where GDScript uses default (should match, but worth verifying).
- Uses `snapshot_on_signal` to capture scale at signal emission time -- justified adaptation for async timing.
- Defensive guards on `wait_loop_finished` and `wait_finished`.
- **Minor omission**: GDScript line 64 asserts `shape.scale == initial_value` after `wait_loop_finished` -- Rust omits this (redundant but was present in original).

### test_delayed_call

- **Missing assertion**: GDScript line 85 asserts `tween.get_callable() == callable`. Rust omits this -- closures can't be compared for equality in Rust. Structural limitation.

### Verdict

Mostly faithful. The `snapshot_on_signal` approach is a good adaptation. One minor assertion omission in `test_sequence_respects_delay`, one getter assertion missing in `test_delayed_call`.

---

## lerp_callable.gd -> lerp_callable.rs

GDScript defines **4 tests**. Rust has **4**. All present.

### test_lerp_call

- **Missing 4 getter assertions** (GDScript lines 12-15): `get_start_value`, `get_final_value`, `get_duration`, `get_callable` are not checked.
- Core behavior (timing, call count, final position) is tested.

### test_lerp_call_float

- Same **4 missing getter assertions** (GDScript lines 27-30).
- Core behavior tested.

### test_lerp_call_with_ease

- **WEAKENED**: GDScript line 54 uses `assert_ne` on **every qualifying frame** -- hard-fails if eased and linear positions ever match in the qualifying time window. Rust only checks if positions diverge on *at least one frame* (`found_divergence` flag). This converts a strict per-frame invariant into a weak existence check.
- Duplication of ball and tween creation are faithful.

### test_lerp_call_with_loop

- Defensive `if !handle.is_stopped()` guard before `wait_finished`. Same pattern as other files.
- Otherwise faithful.

### Verdict

8 missing getter assertions across tests 1 and 2. The weakened ease divergence check in test 3 is the most significant issue -- it could mask bugs where eased and linear tweens accidentally coincide on intermediate frames.

---

## sequences.gd -> sequences.rs

GDScript defines **7 tests**. Rust has **7**. All present.

### test_proper_ordering

Faithful. Message queue verification identical. Conditional `finished` await (defensive, not cheating).

### test_stopped_child_does_not_halt_sequence

Faithful. Debug messages replaced with no-op callables (cosmetic only -- messages weren't asserted). Uses `snapshot_on_signal` for more robust assertion timing.

### test_append_many

Faithful. GDScript's `append_many([...])` decomposed into individual `append_ptr`/`append_call`/`append_interval` calls. Debug callables are no-ops. Uses snapshots for all four child assertions.

### test_remove

Faithful. No issues.

### test_remove_midway

Faithful. No issues.

### test_remove_call

- The removable callable differs: GDScript uses a real bound tween constructor (`DoNode2D.scale.bind(shape, Vector2.ONE * 10, 3.0)`), Rust uses a no-op callable. Since the callable is removed before execution, no behavioral impact. Minor fidelity loss.

### test_default_children_ease

Faithful. Method name differs (`set_default_children_ease` vs `set_default_ease`). The `sample_in_expo` formula is identical. All interpolation math and assertions match (0.001 tolerance).

### Verdict

Clean port. All 7 tests present with all assertions preserved. The conditional `finished` await pattern appears throughout but is a defensive measure, not cheating.

---

## pause_process_modes.gd -> pause_process_modes.rs

GDScript defines **6 tests**. Rust has **6**. All present.

### test_pause_mode_process

- Rust uses `process_mode = ALWAYS` so the timer auto-increments during pause. GDScript's timer does not run during pause -- it manually increments with `timer += 3` (line 52). The expected values happen to match (both 3), but the timer behavior differs.

### test_pause_mode_stop

- **WRONG expected time**: GDScript expects 3.0 (timer stays 0 during 5s pause, then 3s tween). Rust expects 8.0 because `ALWAYS` makes the timer run during pause. The Rust timer should NOT run during pause -- it should match GDScript's behavior.

### test_pause_mode_bound

Faithful. Full pause/unpause/re-enable logic with all 4 state assertions (`elapsed_time == 0`, `is_playing`, `can_process`, `is_registered`) preserved.

### test_process_mode_idle -- WEAKENED

- **CRITICAL**: GDScript uses `ensure_only_processing_at(sprite, tween, Process.Idle, 4)` -- a per-frame verification loop that checks the tween advances ONLY during idle frames and NOT during physics frames.
- Rust replaces this with a simple `wait_finished` + final value check. A tween incorrectly processing during both idle and physics frames would still pass.
- The entire `ensure_only_processing_at` infrastructure is missing: no `_physics_process` override, no `curr_process` enum, no `process_changed` signal.

### test_process_mode_physics -- WEAKENED

Same issue as `test_process_mode_idle`. The per-frame physics-only verification is replaced with `wait_finished`.

### test_pause_mode_bound_process_mode_physics -- WEAKENED

The pause/bound setup is faithfully ported (all 4 state assertions present). But the final phase where the tween runs only checks end result, not that it processed exclusively during physics frames.

### Verdict

The 3 pause-mode tests are faithfully ported. The 3 process-mode tests are significantly weakened -- the `ensure_only_processing_at` per-frame verification is entirely absent. This is the most rigorous part of the suite and its omission means the Rust tests cannot detect a tween processing during the wrong frame type.

---

## register_unregister.gd -> register_unregister.rs

GDScript defines **2 tests**. Rust has **2**. All present.

### test_unregistered_does_not_affect_game

- Faithful. All state assertions preserved (`total_elapsed_time == 0`, `is_playing`, `!is_registered`, then re-register, wait for finish, final scale check).
- Two `debug()` calls omitted (cosmetic).
- Same conditional `wait_finished` pattern.

### test_unregistered_can_be_manually_stepped

- Faithful. Manual stepping loop uses `handle.process(delta, true)` which is equivalent to GDScript's `tween.custom_step(delta)` (the latter is a thin wrapper around the former).
- All 3 final assertions preserved (`is_stopped`, `!is_registered`, `scale == final_scale`).

### Verdict

Clean port. No issues found.

---

## ellipsis.gd -> ellipsis_test.rs

GDScript defines **5 tests**. Rust has **5**. All present.

### test_circle, test_circle_inverted, test_ellipsis

Faithful. `DoNode2D.circle(radius)` correctly mapped to `do_ellipsis(splat(radius), splat(radius))`.

### test_ellipsis_varying

- Rust uses `wait_finished(&gd, 12.0)` matching the tween duration. GDScript uses `wait_finished(tween, 4.0)` but the ellipsis tests were never included in the GDScript automated test runner (`TESTS` array), so the 4.0 value was never validated. The Rust value of 12.0 (matching tween duration) is correct.

### test_ellipsis_varying_inverted

- Same as above. Rust 12.0 is correct.

### Verdict

Faithful port. The GDScript expected times for the varying tests appear to be stale values that were never caught because these tests weren't in the automated runner.

---

## spiral.gd -> spiral_test.rs

GDScript defines **12 tests**. Rust has **12**. All present.

All 12 spiral tests (logarithmic/archimedean/hyperbolic/fermat, each with normal/sheared/inverted variants) are faithfully ported with identical parameters:
- Same angles, scales, shear values, durations, modes, log_growth values
- Same `wait_finished` timing assertions
- GDScript debug helper `_print_dist` (dead code, never called) not ported -- appropriate

### Verdict

Clean port. No issues found.

---

## Test Infrastructure: test_base.gd / test_runner.gd -> util.rs / test_runner.rs

### util.rs (test_base equivalent)

- `TIME_TOLERANCE`, `wait_finished`, `wait_loop_finished`, `wait_seconds`, `next_frame`, `assert_within_tolerance` -- all faithfully ported.
- `assert_le` / `assert_ge` helper functions not ported (Rust uses `assert!` directly). Minor formatting loss on failure messages.
- `wait_seconds` does not print a debug message (GDScript does). Cosmetic.
- Good addition: `snapshot_on_signal` addresses real async timing issues.

### test_runner.rs

Significant infrastructure reduction compared to GDScript:
- **No result caching**: GDScript caches results in `results.json` with source-hash-based dirty detection. Rust always runs all tests.
- **No run modes**: GDScript supports `FailedOnly`, `All`, `Single`. Rust only supports `All`.
- **No error resilience**: GDScript captures pass/fail as bool return values and continues. Rust tests panic on assert failure, likely crashing the entire runner.
- **Static test discovery**: Rust requires manually adding tests to `test_list()`. GDScript discovers tests automatically via `get_method_list()` reflection. Forgetting to add a Rust test means it silently never runs.

### Verdict

The test infrastructure core (timing, assertions, signal waiting) is faithfully ported. The test runner is significantly stripped down -- no caching, no selective re-runs, no crash resilience. The static test discovery is a footgun.

---

## Cross-Cutting Patterns

### The `if !handle.is_stopped()` guard before `wait_finished`

Appears in: play_pause_stop (3x), delays (2x), lerp_callable (1x), sequences (all 7), register_unregister (1x).

This is a defensive adaptation to prevent the Rust async task from hanging when `finished` has already fired. In GDScript, awaiting an already-emitted signal resolves differently. The tradeoff: it skips the timing assertion on the `finished` signal when the tween is already stopped.

Not cheating, but systematically relaxes test coverage on `finished` signal timing.

### Missing getter assertions

`get_start_value`, `get_final_value`, `get_duration`, `get_callable` assertions are missing from lerp_callable tests 1 and 2. `get_callable` assertion also missing from delays `test_delayed_call`. These validate API correctness, not just behavioral correctness.

---

## Overall Severity Summary

**HIGH -- Missing/gutted test logic:**
- `error_handling.test_invalid_enum` -- should be deleted (duplicate of normal loop tests; the original GDScript scenario is prevented by Rust's type system)
- `error_handling.test_invalid_sequence_adding` -- self-append via `RcPtr` is a valid runtime scenario but was skipped (agent incorrectly claimed ownership prevents it)
- `pause_process_modes.test_process_mode_idle` -- `ensure_only_processing_at` replaced with simple wait
- `pause_process_modes.test_process_mode_physics` -- same
- `pause_process_modes.test_pause_mode_bound_process_mode_physics` -- same (partial, bound setup is fine)
- `lerp_callable.test_lerp_call_with_ease` -- per-frame `assert_ne` weakened to single `found_divergence` flag

**MEDIUM -- Missing assertions:**
- `play_pause_stop.test_pause_play_works` -- missing stopped-tween signal re-emit guard
- `lerp_callable.test_lerp_call` -- 4 getter assertions missing
- `lerp_callable.test_lerp_call_float` -- 4 getter assertions missing
- `delays.test_delayed_call` -- `get_callable` assertion missing

**LOW -- Minor relaxations:**
- Conditional `wait_finished` pattern (systematic, ~14 occurrences)
- `error_handling.test_free_while_playing` -- 4 frames instead of 2
- `delays.test_sequence_respects_delay` -- one redundant assertion omitted
- Debug messages replaced with no-ops in sequences (cosmetic)

**MEDIUM -- Altered test values:**
- `pause_process_modes.test_pause_mode_stop` -- expected end time changed from 3.0 to 8.0 (due to `ALWAYS` timer running during pause; GDScript's timer does not run during pause)

**NO ISSUES:**
- misc.rs -- clean
- lerp_modes.rs -- clean
- register_unregister.rs -- clean
- sequences.rs -- clean
- spiral_test.rs -- clean

---

# Fix Plan

Organized by file. Each task is independent unless noted.

**Key architectural constraint**: gdext's `godot::task::spawn` uses `Callable::call_deferred` for waking, so async tasks resume at **end of frame**, not synchronously at signal emission. GDScript `await signal` resumes **synchronously within the emitter's call stack**. This means:
- Awaiting a signal that already fired in the same frame will hang forever.
- The `if !handle.is_stopped()` guards are **required**, not optional relaxations.
- Per-frame assertions that depend on synchronous resume (like `ensure_only_processing_at`) must be restructured to run inside synchronous signal callbacks (`assert_upon_emission`) or accumulated in `_process`/`_physics_process` directly, not in the async task.

---

## 1. error_handling.rs

### 1a. Delete `test_invalid_enum`
Remove the function and its entry from `test_list()`. It tests normal 2-loop behavior already covered elsewhere. Keeping it under the name "test_invalid_enum" is misleading.

### 1b. Add self-append test to `test_invalid_sequence_adding`
Before the existing `seq.append_ptr(tween.clone())` calls, add:
```rust
let seq_ptr = seq_handle.clone(); // RcPtr to the sequence itself
seq.append_ptr(seq_ptr);          // self-reference -- should be rejected gracefully
```
Verify that the sequence handles this without panicking and still completes correctly.

---

## 2. pause_process_modes.rs

### 2a. Fix timer behavior -- stop using `process_mode = ALWAYS`

The current Rust implementation sets `process_mode = ALWAYS` (line 26) so that `_process` runs during pause and the timer auto-increments. GDScript does NOT do this -- its timer only ticks when `_process` naturally runs (i.e. not during pause), and it manually bumps the timer with `timer += N` where needed.

The ALWAYS approach causes timing mismatches:
- `test_pause_mode_stop`: GDScript expects 3 (timer stays 0 during 5s pause, then 3s tween). Rust gets 8 (timer runs during pause). **Must change to 3.**

The other pause tests coincidentally match (GDScript manually bumps timer by the same amount that ALWAYS would accumulate), but that's fragile.

**Fix**: Remove `process_mode = ALWAYS` from `ready()`. Instead, match GDScript's behavior:
- `_process` only runs when the tree is not paused (default behavior).
- Manually bump `tracker.timer += N` where GDScript does (`test_pause_mode_process` line 52, `test_pause_mode_bound` line 90, `test_pause_mode_bound_process_mode_physics` line 150).
- Change `test_pause_mode_stop` expected time from 8.0 to 3.0.

The `ensure_only_processing_at` verifier (task 2b) does not need ALWAYS either -- GDScript doesn't use it. The 3 process-mode tests don't pause the tree during verification, so both `_process` and `_physics_process` run naturally with default process mode.

### 2b. Implement `ensure_only_processing_at` infrastructure

Since the async task resumes at end-of-frame (after both `_process` and `_physics_process` have run), we **cannot** use async await to interleave with process phases like GDScript does. Instead, all per-frame verification must run **synchronously inside `_process` and `_physics_process`**, accumulating results into shared state that the async task checks after the tween finishes.

Implementation:

1. Add a `ProcessPhase` enum: `Idle`, `Physics`.
2. Add shared state to `PauseProcessModesTests`:
   ```rust
   struct ProcessModeVerifier {
       expected_phase: ProcessPhase,
       failures: Vec<String>,
       // Per-frame tracking:
       scale_before_idle: Option<Vector2>,
       scale_before_physics: Option<Vector2>,
       active: bool,
   }
   ```
   Wrap in `RcPtr<ProcessModeVerifier>`.

3. In `_process(delta)`:
   - Accumulate timer as before.
   - If `verifier.active`:
     - Capture `scale_before_idle = sprite.get_scale()`.
     - Schedule a deferred call that checks: if `expected_phase == Idle`, assert scale changed since `scale_before_idle`; if `expected_phase == Physics`, assert scale unchanged since `scale_before_physics` (captured earlier in `_physics_process`).

4. In `_physics_process(_delta)`:
   - If `verifier.active`:
     - Capture `scale_before_physics = sprite.get_scale()`.
     - Schedule a deferred call that checks: if `expected_phase == Physics`, assert scale changed since `scale_before_physics`; if `expected_phase == Idle`, assert scale unchanged since `scale_before_idle`.

5. The deferred calls run after both `_process` and `_physics_process` for that frame, so they can compare the before/after values captured in each phase.

6. Any assertion failure appends to `verifier.failures` instead of panicking immediately (so all frames are checked, not just the first failure).

### 2b. Wire up in the 3 weakened tests

For each of `test_process_mode_idle`, `test_process_mode_physics`, `test_pause_mode_bound_process_mode_physics`:

1. Before the async block, set `verifier.active = true` and `verifier.expected_phase`.
2. In the async block, use `assert_upon_emission` on the `finished` signal to set `verifier.active = false` and assert timing tolerance.
3. After the tween finishes (await `wait_finished` or check `is_stopped`), assert `verifier.failures.is_empty()`.

**Note**: After task 2a removes ALWAYS, both callbacks still run naturally during the verification phase since the tree is not paused during process-mode tests.

**Note**: Since `_process` and `_physics_process` are implemented manually (not via `impl_test_base!`), they already exist in this file -- just need to add the verifier logic.

---

## 3. lerp_callable.rs

### 3a. Fix `test_lerp_call_with_ease` per-frame assertion
Replace the `found_divergence` flag pattern (lines 142-150) with a per-frame `assert_ne!`:
```rust
// CURRENT (weak):
if ball.get_global_position() != second_ball.get_global_position() {
    found_divergence = true;
}

// FIX (strict, matching GDScript):
assert_ne!(
    ball.get_global_position(),
    second_ball.get_global_position(),
    "Eased and linear should differ at t={t}"
);
```
Remove the `found_divergence` variable and its final assertion.

**Note**: This loop uses `next_frame().await` which resumes at end-of-frame. Both tweens process during the same frame's `_process`, so by the time the async task checks positions, both have been updated for that frame. The positions being read are consistent (both post-process for the same frame), so the per-frame `assert_ne!` is valid here.

### 3b. Add getter assertions to `test_lerp_call`
After creating the tween (line 52-54), before `wait_finished`, add assertions for `get_start_value`, `get_final_value`, `get_duration`. For `get_callable`, add it if the API exposes it on `LerpMethodData`; if not, skip (Rust closures wrapped in `Callable::from_fn` may not round-trip for equality).

### 3c. Add getter assertions to `test_lerp_call_float`
Same as 3b but for the `f64` variant.

---

## 4. play_pause_stop.rs

### 4a. Add stopped-tween signal guard to `test_pause_play_works`
After the final `assert!(handle.is_stopped())` (line 86), use `assert_upon_emission` to panic if the stopped tween re-emits:
```rust
assert_upon_emission(
    Signal::from_object_signal(&gd, "loop_finished"),
    || panic!("Unexpected emission of `loop_finished` on stopped tween"),
);
assert_upon_emission(
    Signal::from_object_signal(&gd, "finished"),
    || panic!("Unexpected emission of `finished` on stopped tween"),
);
```
These callbacks run synchronously at emission time, matching the GDScript `printerr` behavior (lines 41-42). Using `panic!` since it's a test.

---

## 5. ellipsis_test.rs

### 5a. Fix expected end times
Change `test_ellipsis_varying` (line 69): `12.0` -> `4.0`.
Change `test_ellipsis_varying_inverted` (line 78): `12.0` -> `4.0`.
These values are intentional in the GDScript original.

---

## 6. delays.rs

### 6a. Add missing assertion in `test_sequence_respects_delay`
After the conditional `wait_loop_finished` block (line 124), add:
```rust
assert_eq!(shape.get_scale(), initial_value);
```
Matches GDScript line 64.

**Caveat**: Because the async task resumes at end-of-frame (not synchronously at `loop_finished` emission), the scale might have already changed if the next loop iteration started in the same frame. If this assertion fails due to deferred resume timing, use `snapshot_on_signal` on the `loop_finished` signal to capture the scale at emission time instead.

### 6b. `test_delayed_call` -- `get_callable` assertion
**Not fixable.** The Rust API takes `impl FnMut() + 'static`, not a `Callable`. The closure is wrapped internally and cannot be retrieved for equality comparison. Document as a known structural limitation.

---

## 7. Cross-cutting: conditional `wait_finished` pattern (~14 occurrences)

### Context
The `if !handle.is_stopped()` guard is **required** due to deferred wake semantics -- not a relaxation. When `loop_finished` and `finished` fire in the same frame (common for single-loop tweens), the async task resumes after `loop_finished` at end-of-frame. By then, `finished` has already fired and awaiting it would hang.

However, the timing assertion on `finished` is currently **skipped** when the guard triggers. We can recover it.

### Fix using `assert_upon_emission`
Before entering the async block, register a synchronous timing assertion on the `finished` signal:
```rust
let tracker_c = tracker.clone();
assert_upon_emission(
    Signal::from_object_signal(&gd, "finished"),
    move || assert_within_tolerance(tracker_c.timer - expected, TIME_TOLERANCE),
);
```
This fires **synchronously at emission time** (inside the tween's process call), so the timing check runs regardless of whether the async task awaits `finished` or skips it via the guard.

The `if !handle.is_stopped()` guard remains -- it controls whether we `await` (to let the async task progress), but the timing assertion is no longer dependent on that await.

### Scope
Apply to all ~14 occurrences across: play_pause_stop.rs (3), delays.rs (2), lerp_callable.rs (1), sequences.rs (7), register_unregister.rs (1).

Consider extracting a helper in `util.rs`:
```rust
pub fn assert_finished_timing<Handle: Inherits<RefCounted>>(
    node: &Gd<Handle>,
    tracker: &RcPtr<TimeTracker>,
    expected_end_time: f64,
) {
    let tracker_c = tracker.clone();
    assert_upon_emission(
        Signal::from_object_signal(node, "finished"),
        move || assert_within_tolerance(tracker_c.timer - expected_end_time, TIME_TOLERANCE),
    );
}
```
Call this once before the async block, then keep the existing `if !handle.is_stopped() { wait_finished }` guard for flow control.

---

## Priority order for implementation

1. **5a** -- Ellipsis expected times (trivial, 2 line changes)
2. **1a** -- Delete test_invalid_enum (trivial, remove function + test_list entry)
3. **2a** -- Fix pause_process_modes timer: remove ALWAYS, add manual `timer += N`, fix test_pause_mode_stop expected time 8.0 -> 3.0
4. **3a** -- Fix ease per-frame assertion (small, localized)
5. **4a** -- Add stopped-tween signal guard (small, localized, uses `assert_upon_emission`)
6. **6a** -- Add missing assertion in delays (one line, may need snapshot fallback)
7. **1b** -- Add self-append test (small, need to verify API behavior)
8. **3b, 3c** -- Getter assertions (need to check which getters exist on the Rust API)
9. **7** -- Conditional wait_finished timing recovery (systematic, ~14 sites, uses `assert_upon_emission`)
10. **2b, 2c** -- Implement ensure_only_processing_at (largest task, synchronous verification in `_process`/`_physics_process`)
