# Checked tween stepping and documentation — 2026-10-13

The manager, sequences, and manual callers share `TweenPtr::custom_step` for explicit binding and playback checks. Native callers receive `TweenStepResult`; GDScript receives a "wants to live" Boolean. Container retention remains separate from the step outcome.

## Documented contract

- Dead explicit bindings take priority over playback state and cancel without completion events.
- Paused/stopped tweens preserve that result when live bindings are disabled or detached. Only playing tweens report a temporarily blocked step.
- Sequences activate paused children, retain valid completed children for loops/replay, remove invalid children, and pass excess in input-delta units.
- GDScript returns false for completed/stopped/cancelled and true for playing/paused/temporarily blocked. Manual callers choose when to unregister or release a handle.
- The caller supplies the delta; the tween applies speed scale. GDScript treats the tree as unpaused for Stop mode, while Bound mode still respects node processing state.

Rustdoc covers both exported result types and every variant, the hidden internal entry point, and a compiled manual-step example. Godot method docs and both READMEs describe the native/GDScript distinction and include examples.

## Real-engine checks

Godot 4.7.2 (`ed1daf0bf`), godot-rust 0.5.5, default API 4.6. Both libraries were rebuilt from the current source with `include_gdext_lib,include_gdscript_bridge`. All runtime checks were headless, with copied existing caches and isolated writable user data. No editor import or graphical session was launched.

| Focused suite | Methods per profile | Result in debug and release |
|---|---:|---|
| Node bindings and manual returns | 23 | Pass |
| Sequence speed | 9 (121 parameter cases) | Pass |
| Sequence constructors | 18 | Pass |
| Ellipse/circle regressions | 11 (119 parameter cases) | Pass |
| Sequence cycles | 6 | Pass |
| Ownership | 15 | Pass |
| Delayed-call validity | 9 | Pass |
| Total | 91 | Pass |

Four new tests in the existing node-binding suite cover 42 standalone configurations: completion/replay for four tween types, 24 paused/playing/live-binding combinations, 12 dead-binding/state combinations, and two invalid-callable cases. The existing six sequence binding tests still cover their 192 configurations. Every focused runner exited zero; no script errors or native panics occurred. Negative constructor/cycle cases intentionally emit rejection diagnostics.

The Linux addon debug/release libraries were installed atomically and their hashes recorded. The actual installed debug path and a fixture referencing the installed release path both pass all 23 binding methods.

## Documentation and compilation

- `cargo check --workspace --all-targets --locked`: pass.
- `cargo doc -p spire_tween --locked --no-deps`: pass; the 39 existing native-doc warnings remain, with none in the updated stepping/result contract.
- Bridge-enabled rustdoc: pass; Godot BBCode and existing diagram fences still produce warnings when interpreted as Rust documentation.
- `cargo test -p spire_tween --locked --doc custom_step`: one compiled example passes.
- The GDScript example extracted directly from the README runs to completion and releases its manual tween.
- `git diff --check`: pass before committing.

See `debug-results.json`, `release-results.json`, `installed-results.json`, and `summary.json` for results and hashes. Logs preserve the individual runner and build outputs.

## Boundaries

This is focused validation, not a passing aggregate release suite. The native demo/test crate was compile-checked, not relinked or runtime-tested here. No other platform artifact was refreshed. Manager bookkeeping/startup, invalid-method cancellation state/forced completion, follow semantics, runner reliability, and distribution decisions remain separate release items. The work-in-progress node lifetime tracker is not a completed pruning implementation.
