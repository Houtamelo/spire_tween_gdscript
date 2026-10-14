# Callable invalidation and owner retention — 2026-10-14

Method and delayed-call validity guards now stop invalid tweens silently and report invalidity without changing manager registration. This follows the author's clarification that the owner decides when to release an unwanted stopped handle.

## Current behavior

- During automatic processing, the guard stops and returns false, processing reports `ShouldDespawn`, and the manager removes its root registration. Sequences similarly remove invalid children.
- Manual stepping reports the same result (or false through GDScript). The caller decides whether to unregister or release its handle.
- Forced completion of an invalid callable stops without invocation, interpolation, or completion events. The stopped tween remains registered while an external owner retains it, in idle/physics/manual modes.
- Dropping the final external handle lets the manager collect the stopped tween. Explicit unregistration also remains available and does not destroy a handle retained by the caller.
- Raw unregistered builder validation no longer enqueues removal by an unrelated allocation address or otherwise touches manager bookkeeping.

Both `SpireTween<LerpMethodData<T>>::check_callable_validity` and `SpireTween<Callable>::check_callable_validity` now contain only validity testing and `stop()`. Manager retention and sequence processing policies are unchanged. Valid completion/forced completion/replay retain their existing events; invalidation emits neither finish nor loop-finish events.

## Integration proof

The native method/delayed-call suites and GDScript equivalents now assert owner retention and collection after release. Forced-stopping checks cover all three processing modes and playing/paused states. Manual stepping checks both explicit caller unregistration and handle release, using real weak references. Raw builder checks compare read-only manager snapshots. Existing automatic cancellation and sequence removal checks still require the processor to remove invalid tweens.

Before removing the helper-driven unregistration, these updated native suites fail four method and two delayed-call methods; the GDScript suites fail three method and one delayed-call method. All four runners exit 1 without script errors. [before-removal-results.json](before-removal-results.json) and the corresponding logs preserve that proof.

After removing those side effects, all focused suites pass in debug and release: **66 native and 98 GDScript methods per profile**. All 30 fixture processes exit zero with matching result markers/counts. No unexpected native panic or script error occurs; the API compatibility suite still deliberately catches two rejected built-in tween calls per profile.

## Verification scope

Godot 4.7.2 (`ed1daf0bf`), godot-rust 0.5.5, default API 4.6. Runtime checks were headless with copied existing caches and isolated writable user data. No editor import or graphical session was launched.

Workspace checking, native/standalone addon debug/release builds, and default native rustdoc builds pass. Existing rustdoc private-link/BBCode warnings remain in `rustdoc.log`. Fresh Linux libraries were installed into the actual GDScript project, while the native project's tracked library symlinks were preserved. Both installed-project method and delayed-call suites also pass (four additional processes).

[build-results.json](build-results.json), [runtime-results.json](runtime-results.json), [installed-runtime-results.json](installed-runtime-results.json), and individual logs record exact commands and outcomes. [installed-libraries.json](installed-libraries.json) records fresh source/installed hashes and symlink targets. [source-verification.json](source-verification.json) confirms production/test sources did not change during verification.

This supersedes the helper-driven unregistration assumption tested in [the initial method fix](../method_validity_2026_10_13/README.md). Earlier artifacts remain historical evidence. No commit was made for this follow-up; the broader release audit remains separate.
