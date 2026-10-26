# GDScript aggregate shutdown warnings

**Resolved 2026-10-25:** strong captures and ineffective rejection checks in the
sequence-constructor tests are repaired. Full verbose 200-case GDScript runs on
Godot 4.4 and 4.7.2 exit 0 without shutdown leak diagnostics. See the
[repair and controls](../shutdown_2026_10_25/README.md). The earlier observations
below are retained as historical evidence.

Both current debug/release full GDScript runs pass all 152 functional cases and
exit 0, then report **12 ObjectDB instances leaked at exit** and **2 resources still
in use**. See `gdscript-aggregate-final-debug.log` and the release counterpart.
Draining four process frames before quitting does not eliminate these warnings.
The full native runs do not report equivalent shutdown warnings.

These messages do not establish which component retains the objects. No cause is
assigned to SpireTween, Godot, the test scenes, or script coroutine state. They are
outside the repaired assertion/outcome/cache paths and require an isolated lifetime
investigation before claiming leak-free aggregate execution.

One additional real-engine probe invalidates a loop-mode argument on a live tween:
its Godot reference count remains 2 before/after the rejected call; after explicit
completion/unregistration and releasing the caller, its weak reference expires.
That probe (`invalid-enum-refcount-probe.log`) does not attribute the aggregate
warnings to a retained tween wrapper. It is only a narrow ownership control.

Next investigation: isolate suites with verbose shutdown logging, identify the
remaining classes/resources, and compare deliberate error paths with valid controls.
Preserve expected rejection diagnostics; do not suppress shutdown warnings or treat
their presence as proof of an ordinary playback leak.

## 2026-10-25 follow-up: cause isolated

A verbose Godot 4.7.2 run identifies the retained resources as `test_base.gd` and
`sequence_ctors.gd`. The self/ancestor constructor test alone reproduces all 12
objects and both resources. Replacing only its strong self/ancestor captures with
weak references in a temporary fixture removes the shutdown diagnostics from
both the focused case and the full 200-test suite. The original test also stopped
its sequences before stepping them, so its factories were never invoked and its
rejection assertions passed without exercising rejection. The source test now
uses weak captures and checks actual factory invocation, rejection, subsequent
callbacks, completion, and replay. A stopped-before-step regression control fails
as expected. See the [isolated controls and repair logs](../shutdown_2026_10_25/README.md).
