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

SpireTween requires an installed `SceneTree` main loop with an available root before
the tween manager's first use, including registration and node binding. A scene
Node's `ready()` is a suitable starting point. Custom main-loop constructors and
member initializers can run before `Engine::get_main_loop()` exposes the tree;
defer tween setup until the main loop is installed. Failed initialization logs an
error and is not retried: correct the startup call and restart the application.

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

## Lazy sequence children

`append_ctor` and `join_ctor` accept a `FnOnce() -> AnyTween` and defer child construction until the new/last block starts. `insert_ctor(time, factory)` constructs at the insertion offset plus the sequence delay. All factories run synchronously on the main thread.

```rust
use spire_tween::prelude::*;

// `node` is a Gd<Node2D>. Read its position when this block starts.
let node = my_node.clone();
let mut seq = SpireTween::<Sequence>::new();
seq.append_interval(0.5);
seq.append_ctor(move || {
    let target = node.get_position() + Vector2::new(100.0, 0.0);
    TweenPtr::new(node.do_position(target, 0.3)).into()
});
seq.register();
```

The `_callable` counterparts (`append_ctor_callable`, `join_ctor_callable`, `insert_ctor_callable`) take a zero-argument Godot `Callable` returning a Spire tween wrapper. Invalid Callables, nil/non-tween results, and rejected hierarchy additions log diagnostics and skip the slot.

Each factory runs at most once; loops and replay reuse the accepted child. Forced completion immediately constructs pending children and completes them. Inserted constructors keep the parent alive until their offset and child completion. Adoption validates the hierarchy, preserves bindings, removes global registration, and uses the parent's current default ease for eligible children. Return a `TweenPtr::new(builder).into()` or an already registered handle converted to `AnyTween`.

Factories must configure and return their child without mutating the receiving sequence during invocation. Ordinary `append_fn`/`join_fn`/`insert_fn` callbacks are retained and run once per loop; factories are consumed once. Pending constructors are not included in child traversal or removed by `remove_call`; after construction, `remove` accepts the child handle.

Factories retain captures until invocation. Use a `WeakTweenPtr` when capturing a strong parent pointer would create an ownership cycle.

## Manual stepping

Use `TweenPtr::custom_step(delta_time, is_tree_paused)` for a custom processing loop.
The manager and sequences use this same checked path; do not call
`SpireTweener::process_internal` directly. Set `ProcessMode::Manual` on a registered
tween to disable automatic updates, or use `TweenPtr::new(builder)` without registering.

```rust
use spire_tween::prelude::*;

let mut sequence = SpireTween::<Sequence>::new();
sequence.append_interval(1.0);
let tween = TweenPtr::new(sequence);
let result = tween.custom_step(0.25, false);
assert!(matches!(result, TweenStepResult::Stepped(AdvanceTimeResult::Playing)));
```

The caller supplies its chosen delta in seconds; the tween applies its `speed_scale`.
`ignore_time_scale` selects the manager's automatic clock, not the delta supplied
manually. Pass the SceneTree pause state, or `false` to override `PauseMode::Stop`;
`PauseMode::Bound` still checks bound nodes' processing state. Paused/stopped tweens
do not advance automatically: call `play()` to resume or restart them.

| `TweenStepResult` | Meaning |
|---|---|
| `Stepped(Playing)` | Still active or waiting for its initial delay. |
| `Stepped(Paused)` | A type-specific owner check temporarily blocked advancement. |
| `Stepped(Completed { unscaled_excess_time })` | Finished; excess uses the caller's input-delta units. |
| `Stepped(ShouldDespawn)` | Invalid owner/callable or another cancellation condition; remove from the processor. |
| `AlreadyPaused` / `AlreadyStopped` | No advancement, even with temporarily blocked live bindings. |
| `DidNotStepDueToBounds` | Playing, but live bindings or pause mode blocked this step. |
| `NoLongerValid` | A bound node was freed; the tween stopped without completion events. |

Bound-node lifetime checks run even when paused/stopped, and a dead binding takes
priority over playback state. The result reports the step outcome rather than
reference retention: the manager applies its external-owner policy, while sequences
keep valid completed children for loops and replay. Manual processors decide when
to unregister or release their handles. Completed excess is deliberately zero when
`abs(speed_scale) < 0.00001`; exactly the cutoff still divides by the speed.

Method/delayed-call validity checks stop invalid tweens without changing manager
registration. The processor consumes `ShouldDespawn` and removes its reference.
After manual stepping or forced stopping, retained external handles may keep the
stopped tween registered; unregister or drop the handle when it is no longer wanted.

Call only on the main thread, without overlapping mutable access or stepping from
a callback while the same tween is processing. GDScript wrappers expose a separate
Boolean contract, described in the [addon guide](../README.md#manual-stepping).

## Compatibility

- **Godot 4.4+**
- **godot-rust 0.5.x** (the `0.5` dependency constraint permits updates within that series)
- **Nightly Rust** required — uses `type_changing_struct_update`, `unboxed_closures`,
  `arbitrary_self_types`, `stmt_expr_attributes`. A `rust-toolchain.toml` in the
  project root selects a floating `nightly` channel.

Godot 4.4 can be tested by explicitly enabling `api-4-4`, which forwards to
`godot/api-4-4`. Ordinary builds use godot-rust's default API selection.

## The `prelude`

`use spire_tween::prelude::*` brings in:

- **Core** — `SpireTween`, `AnyTween`, `Sequence`, `SpireFlags`.
- **Tween pointers** — `TweenPtr<T>`, `WeakTweenPtr<T>`; `T` is the inner tween data, such as `Sequence`.
- **Processing results** — `TweenStepResult`, `AdvanceTimeResult`.
- **Shared-state pointers** — `RcPtr<T>`, `WeakPtr<T>`.
- **Enums** — `Ease`, `EaseKind`, `Evaluator`, `LoopMode`, `PauseMode`, `ProcessMode`, `Spiral`, `State`.
- **Tween-data** — `LerpPropertyData`, `LerpMethodData`, plus the generated per-class adapter enums.
- **Constructor traits** — `DoProperty`, `DoMethod`, `DoVarMethod`, `DoDelayedCall`, `DoDelayedCallable`.
- **Templates** — `DoBone`, `DoContourShape2D`, `DoEllipsis2D`, `DoEllipsis3D`, `DoFollow2D`, `DoFollow3D`, `DoShakeNode2D`, `DoShakeControl`, `DoSpiral`.
- **Lifecycle helpers** — `CompleteBoundTweens`, `KillBoundTweens`.
- **Custom-lerper plumbing** — `BasicLerp`, `SpireLerp`, `CustomLerper`, `LerpMode`, `ITweenable`, `SpireTweener`.

Per-item docs live on [docs.rs](https://docs.rs/spire_tween).

## Two register paths

Pick based on who needs to listen for `finished` / `loop_finished`:

- `SpireTween::register` — pure-Rust path. Returns a `TweenPtr` handle. Subscribe to events via `SpireTween::finished_connect` / `loop_finished_connect` (closure-based).
- `SpireTween::register_with_gd_handle` — also attaches a `Gd<Spire…>` wrapper so GDScript code (or any consumer of Godot signals) can connect to the `finished` / `loop_finished` Godot signals on that handle.

## Threading

Spire is single-threaded — same constraint as Godot's main loop. The internal `TweenPtr<T>` owns a `Rc<UnsafeCell<SpireTween<T>>>` that relies on Godot's main-thread invariant; do not register or access tweens from worker threads.

## License

[MIT](LICENSE).
