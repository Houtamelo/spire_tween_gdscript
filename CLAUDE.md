# spire_tween_asset_plugin

Godot 4 tween library written in Rust via gdext, with a GDScript-facing API. Inspired by Unity DOTween.

## Workspace

- `spire_tween/` — core library (default workspace member)
- `spire_tween_rust/` — Rust integration test runner (Godot project)
- `spire_tween_plugin/` — Godot editor plugin
- `spire_tween_gdscript/` — GDScript reference implementation + addon
- `gdscript_bindgen/` — generates GDScript bindings from Rust API
- `build_scripts/` — release packaging

## Build

```bash
cargo build --release   # always release: .gdextension hardcodes the release path for both debug and release
```

Never `cargo build` alone before running tests — godot will load a stale `.so`.

## Test

```bash
cd spire_tween_rust
godot4 --path . test_runner.tscn   # canonical runner at project root
```

The godot binary is `godot4` (not `godot`). Run with a display, not headless — frame-timing assertions tolerate ±1 frame.

**Pitfall:** there is a second `test_runner.tscn` at `spire_tween_rust/examples/tests/` whose root node has `type="Node"` (not `type="TestRunner"`), so it instantiates an empty scene and hangs. Always run the project-root copy.

## Conventions

- **GDScript tests are the source of truth.** When a Rust test diverges from its GDScript counterpart, fix the Rust side. The GDScript suite is deterministic and verified.
- **Never hand-edit generated files** under `spire_tween_gdscript/` — modify the generator in `gdscript_bindgen/`.
- **Don't paper over bugs with fuzzy/distance asserts.** If a test needs a tolerance, document why.
- **Suspected gdext bug?** Reproduce in 5 lines of GDScript and check the godot docs page first. gdext mirrors godot semantics faithfully, including surprising ones (e.g. `Callable::bind()` *appends* args — there is no `bind_left`/`rbind`/`prepend`).
- **Callable equality** uses reference counting; clone before passing to retain a comparable handle.

## Gotchas

- gdext async tasks resume at end of frame via `call_deferred` (unlike GDScript's synchronous `await`). Plan test timing accordingly.
- "Parameter obj is null" warnings during teardown originate from gdext `FallibleSignalFuture::drop` TOCTOU — harmless.
