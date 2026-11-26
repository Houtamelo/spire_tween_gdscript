# Changelog

## 2.0.0 — Unreleased

The next release uses version **2.0.0** for the Rust crate, Godot addon archives,
and storefront. Changes below are relative to the published Rust crate **1.4.0**.

### Added

- `TweenBlueprint` scene nodes for Inspector-authored property and character
  animations, serial and parallel compositions, automatic triggers, and custom
  GDScript builders.
- In-editor animation preview with Play/Stop buttons, one active preview, and
  synchronous restoration on completion, cancellation, saving, scene switching,
  and removal. Custom builders can supply a `_capture_rest_state()` Callable;
  compositions capture before playback and restore children in reverse order.
- 33 reusable templates authored entirely in GDScript and Godot scenes, covering
  text effects, visual entrances, continuous motion, and interaction feedback.
- `SpireRichTextLabel`, extending `RichTextLabel` with character-range fade,
  color, offset, rotation, scale, punch, and shake tweens. Ranges support spatial
  start-delay curves, normal tween lifecycle signals, and sequence composition.
  `duration` is the duration of each character's animation per leg; the range
  finishes after every selected character completes its delays and requested legs.
- Reusable BBCode loops through `SpireRichTextEffectPreset` and `SpireTextEffect`
  resources, with independent timing, easing, and loop settings per effect.
  Presets render before code-authored character effects, so code applies last.
- `InPop`, `OutPop`, `InOutPop`, `InBounce`, `OutBounce`, and `InOutBounce` easing.
- Typed `Vector4`, `Quaternion`, and `Rect2` property/method tweens; waypoint paths;
  BBCode-aware string replacement and scrambling; integer margin helpers; shader,
  array, and typed getter/setter adapters; and additional camera, light, audio,
  color, motion, punch, orientation, and UI shortcuts.
- Lazy sequence factories through `append_ctor`, `join_ctor`, and `insert_ctor`,
  allowing child tweens to be constructed when their sequence position is reached.
- A `stopped` event for cancellation and successful completion, alongside
  `finished` and `loop_finished`.

### Changed and migration from 1.4.0

- Native tween ownership uses `TweenPtr<T>` and `WeakTweenPtr<T>`, where `T` is
  the tween data. Replace explicit `RcPtr<SpireTween<T>>` and
  `WeakPtr<SpireTween<T>>` annotations with those types. Existing tween pointer
  aliases, such as `TweenSequencePtr`, use the new representation.
- Native manual stepping uses `TweenPtr::custom_step(delta_time, is_tree_paused)`
  and returns `TweenStepResult`; processing results use `AdvanceTimeResult`.
  GDScript `custom_step(delta_time)` continues returning whether the tween wants
  to remain alive. Use Manual process mode when driving updates yourself.
- Shake now interpolates a generated waypoint path. Replace the former
  `(radius_min, radius_max, vibratio, frequency, duration)` arguments with
  `(duration, strength, vibrato, randomness, fade_out, randomness_mode, blendable)`.
  Rust supplies every option explicitly; GDScript has defaults for the options
  after `strength`. Shake no longer uses fixed-frequency position jumps.
- String property tweens use `StringLerper` instead of `()` for their
  `ILerpable::Lerper`. Update native code that explicitly names the lerper;
  constructors infer it. String method tweens retain the plain lerper.
- `AnyTween` includes Char tweens, and `Ease` includes the six new variants.
  Update exhaustive native matches as needed.
- Successful natural completion emits `loop_finished`, then `stopped`, then
  `finished`. Cancellation emits `stopped` without `finished`; forced completion
  skips loop events. Use `stopped` for cleanup that must also run on cancellation.

### Fixed

- Invalid method and delayed-call targets cancel without reporting successful
  completion; explicitly bound nodes retain lifetime and pause semantics across
  registration and sequence insertion.
- Native/GDScript handle retention and stale manager bookkeeping no longer keep
  completed or invalid ownership records unnecessarily.
- Sequence processing shares checked lifecycle handling, rejects cyclic
  containment, and returns child excess time in the caller's time units when
  speed scales differ.
- Ellipse/circle geometry, including zero-angle cases and 3D circle placement.
- Character rendering recovers after scrolling settings change.

### Distribution

- Release tooling builds editor/debug and exported release libraries, verifies
  artifact hashes and dependency notices, and validates installed archives in
  Godot. Desktop packaging requires the configured Linux, Windows, and macOS
  validation matrix.
- Customer archives include all supplied scene templates and shared scripts,
  this changelog, installation instructions, licensing, and build provenance.
- Windows builds use the static Visual C++ runtime. Universal macOS frameworks
  use ad-hoc signatures and require no Apple developer account to build.
