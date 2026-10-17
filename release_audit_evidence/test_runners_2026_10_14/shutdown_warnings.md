# GDScript aggregate shutdown warnings

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
