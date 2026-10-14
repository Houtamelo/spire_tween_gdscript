# Method callable validity — 2026-10-13

**Superseded retention assumption:** this run tested helper-driven unregistration. The author's 2026-10-14 clarification leaves removal to the processor and preserves externally retained stopped registrations after manual stepping/forced stopping. See the [ownership follow-up](../callable_retention_2026_10_14/README.md) for the current implementation and tests. The logs below preserve the earlier run.

Finding #15 is resolved under the chosen contract: invalidation stops and unregisters a method tween without invoking its callable or emitting completion events. Normal successful completion and valid forced completion retain their existing signals.

## Implementation

`SpireTween<LerpMethodData<T>>::check_callable_validity` now shares the delayed-call pattern: valid callables proceed; invalid callables stop and enqueue removal by the native allocation address. Processing checks this before time advancement. Forced completion checks it before interpolation or invocation, and calls `handle_finished` only on the successful path.

The cancellation contract is documented in Rust constructors, Godot method-tween class/signal docs, the GDScript constructors, and shared `force_complete` docs. The method signal documentation also clarifies the existing behavior that valid forced completion emits `finished` even when natural looping is infinite.

## Real-engine regression tests

[Native tests](../../spire_tween_rust/src/tests/method_validity.rs) contain 10 methods. They cover freed Node/non-Node Object targets, empty/missing-method callables, idle/physics/manual processing, playing/paused forced cancellation, repeated cancellation, unregistered builders, custom interpolation, valid delay/completion/replay, and invalid sequence children. Native signal callbacks observe actual tween completion events; no mocked engine is used.

[GDScript tests](../../spire_tween_gdscript/examples/tests/method_validity.gd) contain seven methods covering Godot signal delivery, typed/Variant/custom method tweens, invalid callables and freed nodes, clock modes, manual stepping, playing/paused forced cancellation, skipped custom lerpers, valid interpolation/completion/infinite forced completion/replay, and sequence children. Checks return their Boolean result directly; the focused runner exits nonzero on failed checks. Both suites are registered in the normal test catalogs.

Before the production fix, the native suite reports **10 methods, eight failures, exit 1** and the GDScript suite reports **seven methods, six failures, exit 1**. Valid completion and forced replay controls pass. [red-results.json](red-results.json) and the `red-*` logs preserve that proof.

After the fix, all 10 native and seven GDScript methods pass in both debug and release. Cancellation reports stopped/unregistered, skips invocation/interpolation, emits no finish/loop-finish events, and lets sequences continue after removing invalid children. Valid controls still apply the expected values and emit the expected events.

## Verification scope

Godot 4.7.2 (`ed1daf0bf`), godot-rust 0.5.5, default API 4.6. All runtime processes were headless, with copied existing caches and isolated writable user data. No editor import or graphical session was launched.

- Workspace checking and native/standalone addon debug/release builds pass.
- All 30 fixture processes exit zero with matching result markers: **66 focused native and 98 focused GDScript methods per profile**.
- The API compatibility suite deliberately catches two engine-rejected built-in tween calls per profile. No unexpected native panic or script error occurs in the passing fixture suites.
- Fresh Linux addon libraries were installed into the actual GDScript project; native installed symlinks were preserved and resolve to the fresh builds. Actual installed-project method-validity suites also pass. Source/installed hashes are recorded in [installed-libraries.json](installed-libraries.json).
- Default native rustdoc builds successfully. Its existing private-link/BBCode warnings remain recorded in `rustdoc.log`.
- [source-verification.json](source-verification.json) confirms the production and integration-test sources did not change during verification.

Exact commands and result counts are recorded in [build-results.json](build-results.json), [runtime-results.json](runtime-results.json), and [installed-runtime-results.json](installed-runtime-results.json). The broader release audit and older aggregate-runner limitations remain separate from these focused passes. No commit was made for this fix.
