# GDScript integration test clock fixes

The test harness now waits for the relevant processing clocks and bounded tween
state changes. Tween runtime behavior, including the one-loop-per-frame policy,
is unchanged. The historical observations below were collected while validating
the SpireRichTextLabel migration on Godot 4.7.2 with Xvfb and software OpenGL.

Before the fixes, two groups of assertions depended on frame cadence:

- Some elapsed-time assertions allowed 0.017 seconds of error. At a 60 FPS cap,
  `test_sequence_respects_delay` and `test_force_complete` accumulated about two
  frames of error (0.0333 seconds). `test_invalid_enum` also checked progress after
  two idle frames against a small fixed threshold.
- Callable-validity fixtures used a fixed number of idle frames to wait for
  elapsed time or physics-manager processing. For example,
  `test_valid_anonymous_call_respects_delay` waited 40 frames for a 0.2-second
  delay; that wait could be too short when rendering was uncapped. Eight idle
  frames could also elapse before the physics manager got a cleanup tick.

The uncapped full run completed 297 tests with five failures, all in
`delayed_call_validity.gd` and `method_validity.gd`. Isolated rendered runs also
failed with the saved pre-migration library from the text-authoring review:

| Rendered configuration | Delayed-call validity | Method validity |
|---|---|---|
| Saved pre-migration library, uncapped | 9 tests, 2 failures | 7 tests, 2 failures |
| Migrated library, uncapped | 9 tests, 4 failures | 7 tests, 3 failures |
| Migrated library, 144 FPS cap | 9 tests, 0 failures | 7 tests, 0 failures |

Failure counts vary with frame cadence. Headless runs of both libraries passed
these isolated suites as well. These observations establish a harness scheduling
dependency; they do not establish a production tween defect.

Before these test fixes, the full migrated suite passed with a 144 FPS cap on the same
rendered Godot 4.7.2 setup: 297 tests, zero failures, zero skips, without cached
results. The frame-rate cap is part of that validation configuration.

## Fixes

- `test_waits.gd` provides a shared wait for actual physics and idle manager
  turns, plus a predicate wait with a wall-clock timeout. Callable-validity and
  manager stop-event tests use these instead of guessing how many idle frames
  are sufficient for physics processing.
- The delayed-call pre-delay assertion uses a precise manual step. The tween
  then resumes automatic processing, and the test waits for completion with a
  deadline. A real paused-tween regression verifies that an expired wait returns
  false without finishing the tween.
- Completion assertions allow one actual idle/physics tick, retaining the prior
  0.017-second precision floor. Second sequence loops are measured from their
  observed loop boundary. Replays, resumes, registration, sequence removal, and
  dynamic-target changes use the time those operations actually occur after
  the SceneTree timer. The speed-based/relative test similarly measures the
  remaining travel time after the relative tween actually finishes. This
  avoids accumulating earlier timer or completion quantization.
- Dynamic-target intermediate positions use the tween's elapsed processing
  time. The sequence's elapsed-time lower bound accepts approximate equality
  at seven seconds, avoiding a strict floating-point boundary comparison.
- Process-phase probes measure completion in the `finished` callback. Physics
  durations use the physics-frame counter, keeping the independent idle clock
  and the deferred observation code out of that duration assertion. Checks
  that properties change only during the selected processing phase remain.
- The enum-rejection test verifies second-loop state and elapsed loop time,
  replacing its fixed 0.02-second threshold after two idle frames.

The original timing probe confirmed that a nominal two-second SceneTree timer
returned at about 2.011 seconds, and that the first sequence loop finished at
about 7.016 seconds. Adding ideal durations to zero had charged those earlier
offsets to the later replay or loop. The corrected assertions retain a single
tick allowance for each newly started operation.

## Verification

All 35 focused cases, including adjacent pause/process and registration cases,
passed on Godot 4.4 and 4.7.2 at 30 FPS, 60 FPS, and uncapped rendering. Tests
specifically about deferred idle-frame delivery retain their idle-frame waits.
The physics-processing case also passed five consecutive fresh-scene runs in
each of these six configurations.

Final full-suite runs used the final test sources, private Xvfb displays,
software OpenGL, and `--all-tests --no-cache`. The new timeout regression brings
the total from 297 to 298 cases.

| Godot | Rendering frame cap | Passed | Failed | Skipped |
|---|---|---|---|---|
| 4.7.2 | 30 FPS | 298 | 0 | 0 |
| 4.7.2 | 60 FPS | 298 | 0 | 0 |
| 4.7.2 | Uncapped | 298 | 0 | 0 |
| 4.4 | 60 FPS | 298 | 0 | 0 |

Evidence, including the initial 60 FPS attempt, uncapped aggregate run, isolated
baseline comparisons, and the rendered runner scripts, is retained under
`dist/spire-rich-text-label-migration-2026-11-06/`.

Fix validation, including the original failing timing probe and subsequent
cross-version/frame-rate runs, is retained under
`dist/test-clock-fixes-2026-11-06/`. The final full-suite logs are
`full-accepted-*.log`; `validated-test-sources.json` records the test source
hashes used for those runs.
