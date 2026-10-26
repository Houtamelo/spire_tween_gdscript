# Shutdown warnings resolved by repairing sequence-constructor tests

The source test now uses weak captures and executes its self/ancestor factories
through playing sequences. It verifies rejection, subsequent callbacks, completion,
replay, and preservation of the original nested child for append/join/insert.
The pending weak-parent lifetime test also covers all three constructor operations.
Full verbose GDScript runs pass all 200 cases on Godot 4.4 and 4.7.2 without
ObjectDB leaks or resources remaining in use at shutdown.

## Initial isolation

On Godot 4.7.2, `test_self_and_ancestor_results_are_rejected` alone reproduces all
12 leaked objects and both retained resources seen in the full GDScript suite.
Changing only its two factory captures to weak references in a temporary fixture
removes the shutdown diagnostics from both the isolated case and the full suite.
Those initial controls did not change the source test or addon implementation.
The temporary fixture was restored byte-for-byte after the experiment; the source
test was subsequently repaired as described above.

## Observations

| Run | Functional result | Shutdown diagnostics |
|---|---|---|
| Full suite, original captures | 200 passed, zero failed/skipped | 12 objects, 2 resources |
| Isolated self/ancestor test, original captures | 1 passed | 12 objects, 2 resources |
| Isolated self/ancestor test, weak captures | 1 passed | None |
| Full suite, weak captures | 200 passed, zero failed/skipped | None |

All four engine processes exited 0. Runs used the same API-4.4 addon binary,
Godot 4.7.2, headless mode, `--verbose`, fixed 120 FPS, and `--no-cache`.
Godot's verbose output names the retained resources as
`res://examples/tests/test_base.gd` and `res://examples/tests/sequence_ctors.gd`.
It also names one `GDScriptNativeClass`, two `GDScript` objects, and nine objects
without class names after extension shutdown. The two resources are part of the
reported object population; the counts do not establish 14 distinct leaks.

## Ownership and test coverage defect

The original test at `spire_tween_gdscript/examples/tests/sequence_ctors.gd:219` installed
these strong ownership cycles for each append/join/insert operation:

```text
sequence -> pending factory callable -> sequence
ancestor -> nested sequence -> pending factory callable -> ancestor
```

That accounts for three sequence wrappers per operation, nine in total. Removing
manager registration in `_exit_tree()` does not break either reference cycle.
The retained callables also keep script resources alive.

The original test stopped the parent before `custom_step()`. The canonical step
returns immediately for a stopped tween, so the factories remained pending. Its
assertions therefore did not establish rejection of a self/ancestor factory result. Both the
original and weak-capture controls pass that ineffective test; the control is an
ownership experiment, not a completed test repair.

The repair removes strong self/ancestor captures, invokes the factories, and
asserts that invocation and rejection occurred. The addon does not need to collect
arbitrary cycles created by user callbacks.

## Repair verification

The focused rejection test passes on both Godot versions and emits exactly three
self-rejection errors and three ancestor-cycle warnings, one pair for each
constructor operation. Invocation counters must be one, following callbacks must
run, both sequences must complete, and replay must not reconstruct rejected
children. The ancestor case is driven through the playing parent, which starts
the child paused by adoption. The original nested child must remain removable.

The pending weak-parent test passes on both engines for append/join/insert: after
stopping, unregistering, and releasing the caller, the weak reference expires
without invoking the factory.

A temporary regression control restores `stop()` before the first self and
ancestor steps. The repaired test exits 1 and reports
`GDSCRIPT_TEST_RESULT total=1 failed=1 skipped=0`, including both invocation-counter
failures. The fixture is restored after that control. See
`stopped_before_step_control.patch` and `stopped-before-step-control.log`.

`fixed-4.4-full.log` and `fixed-4.7.2-full.log` each report
`GDSCRIPT_TEST_RESULT total=200 failed=0 skipped=0`, exit 0, and contain no shutdown
leak diagnostics. `fixed-*-focused.log` and `fixed-*-lifetime.log` preserve the
focused positive checks. The addon binary is unchanged by this test-only repair.

## Reproduction

The isolated baseline command was:

```sh
timeout 45 dist/engine-4.7.2/editor/Godot_v4.7.2-stable_linux.x86_64 \
  --headless --verbose --log-file /tmp/spire-shutdown-20261025-self-strong-engine.log \
  --path dist/group2-20261022/gdscript-4.7.2 --fixed-fps 120 \
  res://examples/tests/test_runner.tscn -- \
  --test=test_self_and_ancestor_results_are_rejected --no-cache
```

The full command replaces the final test selector with `--all-tests`. The weak
control changes only the two captures shown in `weak_capture_control.patch` in
the ignored fixture copy. Saved logs preserve the successful focused controls
and the full runs. Deliberate invalid-enum diagnostics inside the full suite's
`EXPECTED_INVALID_ENUM_BEGIN/END` block are separate from shutdown diagnostics.
