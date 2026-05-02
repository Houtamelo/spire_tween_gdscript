# spire_tween

Tweening library for Godot 4 via [godot-rust](https://github.com/godot-rust/gdext),
inspired by Unity's [DOTween](https://dotween.demigiant.com/).

- Type-safe per-property shortcuts (`do_position`, `do_modulate`, …) generated for every tweenable field of every Godot built-in class.
- Sequences with parallel & sequential blocks, intervals, callbacks, and inserts at absolute time offsets.
- Templates for higher-level patterns: `do_shake`, `do_follow`, `do_spiral`, `do_ellipsis` (2D/3D), `do_contour_shape`, `do_bone`.
- Three lerp modes per property tween — absolute, relative (blends with external forces), speed-based (fixed units/second).
- Pluggable custom lerpers via the `BasicLerp` / `SpireLerp` traits for non-built-in types.
- Two paths to listen for `finished` / `loop_finished`: pure-Rust closures (`finished_connect`) or Godot signals (when you opt into a `Gd<Spire…>` handle via `register_with_gd_handle`).

This crate is the Rust core. There's also a [GDScript-facing addon] built on top of it for non-Rust Godot users.

[GDScript-facing addon]: https://github.com/Houtamelo/spire_tween_gdscript

## Quickstart

```rust
use spire_tween::prelude::*;

// Tween a known property — returns a builder. Don't forget `.register()`.
let handle = my_node
    .do_position(Vector2::new(640.0, 360.0), 2.0)
    .with_ease(EaseKind::Basic(Ease::OutCubic))
    .as_relative(Vector2::ZERO)
    .register();

// Hook a callback for when it finishes (closure-based, Rust-only path).
handle.to_mut().finished_connect(
    || godot_print!("done!"),
    SpireFlags::DEFERRED | SpireFlags::ONE_SHOT,
);

// Sequence multiple tweens.
let mut seq = SpireTween::<Sequence>::new();
seq.append(my_node.do_position(target_a, 1.0));
seq.join(my_node.do_color(Color::RED, 1.0));   // parallel with the above
seq.append(my_node.do_position(target_b, 1.0));
seq.register();
```

## Compatibility

- **Godot 4.4+**
- **godot-rust 0.5.x** (currently pinned at `0.5`)
- **Nightly Rust** required — uses `type_changing_struct_update`, `unboxed_closures`,
  `arbitrary_self_types`, `stmt_expr_attributes`. A `rust-toolchain.toml` in the
  project root pins `nightly`.

## Cargo features

| Feature | Default | Purpose |
|---|---|---|
| `indexmap` | ✅ | Single-threaded ordered map backend for the global tween manager. |
| `dashmap` |  | Alternative concurrent map backend (mutually exclusive with `indexmap`). |
| `standalone` |  | Registers GDScript-facing classes (`Spire`, `Do{Class}`, `SpireSequence`, …). Used by the addon distribution; pure-Rust gdext consumers don't need this. |
| `double-precision` |  | Forwards to godot's `double-precision`. Match this with your gdext build. |
| `nothreads` |  | Forwards to godot's `experimental-wasm-nothreads` for wasm-without-threads builds. |
| `verbose-stdout` |  | Extra diagnostic prints inside the manager loop. |

## The `prelude`

`use spire_tween::prelude::*` brings in:

- **Core** — `SpireTween`, `AnyTween`, `Sequence`, `SpireFlags`.
- **Pointer** — `RcPtr`, `WeakPtr`.
- **Enums** — `Ease`, `EaseKind`, `Evaluator`, `LoopMode`, `PauseMode`, `ProcessMode`, `Spiral`, `State`.
- **Tween-data** — `LerpPropertyData`, `LerpMethodData`, plus the generated per-class adapter enums.
- **Constructor traits** — `DoProperty`, `DoMethod`, `DoVarMethod`, `DoDelayedCall`, `DoDelayedCallable`.
- **Templates** — `DoBone`, `DoContourShape2D`, `DoEllipsis2D`, `DoEllipsis3D`, `DoFollow2D`, `DoFollow3D`, `DoShakeNode2D`, `DoShakeControl`, `DoSpiral`.
- **Lifecycle helpers** — `CompleteBoundTweens`, `KillBoundTweens`.
- **Custom-lerper plumbing** — `BasicLerp`, `SpireLerp`, `CustomLerper`, `LerpMode`, `ITweenable`, `SpireTweener`.

Per-item docs live on [docs.rs](https://docs.rs/spire_tween).

## Two register paths

Pick based on who needs to listen for `finished` / `loop_finished`:

- `SpireTween::register` — pure-Rust path. Returns an `RcPtr` handle. Subscribe to events via `SpireTween::finished_connect` / `loop_finished_connect` (closure-based).
- `SpireTween::register_with_gd_handle` — also attaches a `Gd<Spire…>` wrapper so GDScript code (or any consumer of Godot signals) can connect to the `finished` / `loop_finished` Godot signals on that handle.

## Threading

Spire is single-threaded — same constraint as Godot's main loop. The internal `RcPtr<T>` is a `Rc<UnsafeCell<T>>` wrapper that relies on Godot's main-thread invariant; do not register or access tweens from worker threads.

## License

[MIT](LICENSE).
