# Typed Godot value verification

Built-in `Vector4`, `Quaternion`, and `Rect2` families and shared Variant
interpolation are covered by 16 native and 16 GDScript integration cases. They
exercise real Godot properties, script
fields, generated shortcuts, typed accessors, callables, shader uniforms,
sequences, registration, weak handles, and lifetime cancellation.

## Results

| Engine | Native suite | GDScript suite |
|---|---|---|
| Godot 4.4 stable, Linux x86_64 | `NATIVE_TEST_RESULT total=140 failed=0` | `GDSCRIPT_TEST_RESULT total=231 failed=0 skipped=0` |
| Godot 4.7.2 stable, Linux x86_64 | `NATIVE_TEST_RESULT total=140 failed=0` | `GDSCRIPT_TEST_RESULT total=231 failed=0 skipped=0` |

Both runtimes loaded libraries built with the optional `api-4-4` feature. The
default API feature remains unchanged and also passes compilation with the
GDScript bridge. The native runtime fixtures use the final implementation; the
addon fixtures also include the new per-class Godot editor documentation.

The 16 existing library tests pass. Regeneration is byte-identical across all
generated class adapters, bridge files, and type data. Formatting and diff checks
pass. Native rustdoc with broken links denied passes; its 32 existing warnings
from enum-diagram code blocks remain. Strict rustdoc with the bridge feature also
encounters pre-existing Godot BBCode links such as `[b]`; that formatting belongs
to Godot's editor documentation.

Strict runtime log checks found no unexpected panics/script errors, failed
assertions, ObjectDB leaks, or leaked resources. The existing invalid-enum tests
deliberately print call error 1337 inside `EXPECTED_INVALID_ENUM_BEGIN/END`.
Native API failure controls deliberately print panics inside
`EXPECTED_API_FAILURE_BEGIN/END`. Those bounded controls remain successful.
Endpoint-conversion failure fixtures log their deliberate errors inside
`EXPECTED_VARIANT_CONVERSION_BEGIN/END`.

Full logs from the 2026-10-27 completion review are in
`/tmp/spire-completion-full-native-{version}.log`
and `/tmp/spire-completion-full-gdscript-{version}.log`, with companion
`-engine.log` files. Isolated imported projects are under
`dist/completion-20261027/{native,gdscript}-{version}`.

## Behaviors checked

- All three types interpolate through typed method/property APIs and Rust
  getter/setter closures. Typed wrapper `build` entry points register correctly.
- Generated JSON adapters cover Node3D's quaternion, Sprite2D's rectangle and its
  position/size components, and a vec4 shader constant's four numeric components.
  The latter intentionally converts Godot's Quaternion storage to Vector4 data.
- Relative updates preserve external writes. Quaternion composition is tested
  with rotations around different axes, making multiplication order observable.
- Speed-based updates use 4D component distance or rotation angle, preserve exact
  endpoints, and report unscaled excess correctly across a sped-up sequence child.
- Restart, Yoyo, Incremental extrapolation, replay, dynamic targets, type erasure,
  and weak-pointer release work through the ordinary processing pipeline.
- Quaternion fixtures cover sign-equivalent endpoints, nonunit normalization,
  small angular motion, a half-turn, positive/negative easing overshoot, and
  forced completion. Zero/NaN inputs become identity and warnings include the
  actual rejected components.
- Completion preparation is handled by `SpireLerp::prepare_completion_value`;
  quaternion property-data setters only forward values. The completion matrix
  covers Absolute/Relative/SpeedBased while playing or paused, nonunit/zero/NaN
  endpoints, coincident speed-based orientations, and stopped no-op behavior.
  Method completion still normalizes through interpolation. Custom property
  completion preserves its endpoint without invoking interpolation callbacks.
- Invalid owners/callables stop without finish signals. Shader uniforms exercise
  successful Vector4 interpolation, invalid names/types, and shader replacement.
- Variant absolute, relative (native explicit baseline), and speed-based dispatch
  use the new built-ins. The initial Variant regression fixture failed against a
  binary without those dispatch entries, then passed with them installed.
- Common Variant interpolation also shares the typed implementations for ints,
  floats, Vector2/2i/3/3i, Color, and String (including StringName/NodePath
  conversion). A 13-row matrix exercises real property and method tweens and
  asserts both midpoint values and result types. Integer endpoints round to ints;
  mixed numeric endpoints promote to floats. Unicode string fixtures operate on
  characters. Failed conversions log inside `EXPECTED_VARIANT_CONVERSION_BEGIN/END`
  and retain the start; a custom callable overrides the built-ins.

The common-type fixture fails against the previous addon binary, which dispatched
only Vector4/Quaternion/Rect2. For example, integer `0 -> 3` at half duration
formerly produced float `1.5`; the shared typed implementation produces int `2`.
That negative control is recorded at `/tmp/spire-variant-before.log`.

## Reproduction

```sh
cargo run --locked -p gdscript_bindgen
cargo build --locked -p spire_tween_rust --features api-4-4
cargo build --locked -p spire_tween \
  --features api-4-4,include_gdext_lib,include_gdscript_bridge
cargo test --locked -p spire_tween --lib --features api-4-4
cargo check --locked -p spire_tween --features include_gdscript_bridge
RUSTDOCFLAGS='-D rustdoc::broken_intra_doc_links' \
  cargo doc --locked --no-deps -p spire_tween --features api-4-4
```

Install the freshly built native/addon libraries into separate imported native
and GDScript test projects. Copy the current test scripts, including
`typed_values.gd`, its scene, and the test runner, into the GDScript fixture.
Do not overwrite libraries while an engine process maps them.

```sh
timeout 660 "$GODOT" --headless --verbose --log-file /tmp/typed-native-engine.log \
  --path "$NATIVE_PROJECT" --max-fps 120 res://test_runner.tscn -- --no-cache
timeout 120 "$GODOT" --headless --verbose --log-file /tmp/typed-gdscript-engine.log \
  --path "$GDSCRIPT_PROJECT" --fixed-fps 120 \
  res://examples/tests/test_runner.tscn -- --all-tests --no-cache
```

Native timing tests require the real clock. GDScript tests use fixed FPS. These
results establish the two listed Linux engine versions; other release platforms
remain the responsibility of the existing CI matrix.
