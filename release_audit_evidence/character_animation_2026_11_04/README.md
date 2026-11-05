# RichTextLabel character animation validation

Implemented the eleven inventoried character constructors through `TextAnimator`
and `SpireTextAnimator.attach`, with automatic effect scope maintenance, logical
indices, shared shaping-cluster state, refresh/reset, and normal Spire processing.
See [the API guide](../../docs/character_animation.md).

## Behavior and boundaries

- Source `text` and BBCode settings remain caller-owned. The animator rebuilds
  the rendering stack with `push_customfx`; no caller-maintained tags are needed.
- Source/BBCode/translation changes reset transformations without rewinding valid
  tweens. A missing index cancels on the next processing step or forced completion,
  emitting stopped without finished or loop-finished.
- The rendering resource holds weak state ownership. The label owns the runtime
  processing child; freeing either owner invalidates the corresponding tweens.
- Godot duplicates internal children too. The helper has an inert constructor;
  copied helpers bind independent state to the copied label. Existing tweens are
  not copied or redirected.
- Three-axis YXZ rotation is projected orthographically. The default pivot is the
  glyph pen origin, with configurable pixel offsets. Godot 4.4 does not expose
  exact font-size/glyph-bound metadata for automatic center pivots.
- Initial BBCode/preceding-effect colors and shaping clusters become available
  through real drawing. Before that, color reads use theme defaults. The guide
  demonstrates awaiting the label's draw signal.
- Caller alpha inputs normalize to 0..1. Easing/Incremental interpolation can
  overshoot without being mistaken for invalid caller input.
- Normal text-property updates are supported; manual formatting-stack mutations
  are not tracked. Godot's custom-effect processing continues between tweens.

## Real-engine coverage

Thirteen fixtures are registered in each API:

| Suffix after `test_characters_` | Checks |
|---|---|
| `attach` | Reuse, literal text versus BBCode, invalid indices, null GDScript target, inert helper, and independent duplicated labels. |
| `channels` | Offset/scale/color/fade interpolation, first-step capture, RGB preservation and easing overshoot. |
| `refresh` | Reset on text changes, numeric-index continuity, resize preservation, vanished-index cancellation and forced-completion rejection. |
| `lifecycle` | Pause/resume, stop, replay, reset, label deletion and animator deletion. |
| `loops_sequence` | Restart/Yoyo/Incremental behavior and speed-scaled sequence handoff. |
| `effects` | All three punch and all three shake channels; successful completion restoration. |
| `invalid` | Invalid duration/components/alpha/randomness/vibrato and empty/invalid indices. |
| `render` | Actual RichTextEffect calls, BBCode color, X/Y/Z rotation, combined tilts, scale, offsets, explicit pivots and reset. |
| `unicode_render` | Combining-mark cluster sharing and logical bidirectional indices. |
| `rotation` | Shortest angular deltas, retained turns and left/right quaternion composition. |
| `effect_refresh` | Rebuilt targets do not subtract discarded punch contributions twice. |
| `automatic` | Manager-driven completion and effect maintenance without live tweens. |
| `translation` | Locale-driven rebuilding with unchanged source property and effective BBCode color. |

Native rendering fixtures record matrices/colors from actual engine callbacks,
not synthetic CharFXTransform invocations. No GPU pixel-output equivalence is
claimed for headless rendering.

## Results

Linux x86_64 debug builds using the minimum 4.4 bindings:

| Runtime | Full native suite | Final native character fixtures | Final full GDScript suite |
|---|---|---|---|
| Godot 4.4 stable | 184 passed, 0 failed | 13/13 passed | 276 passed, 0 failed, 0 skipped |
| Godot 4.7.2 stable | 184 passed, 0 failed | 13/13 passed | 276 passed, 0 failed, 0 skipped |

The native full-suite runs preceded the final alpha-overshoot and duplication
refinements. All thirteen character fixtures were rerun against the final
bridge-enabled native build on both runtimes. The full GDScript runs use the
final implementation, including those refinements. Expected aggregate counts
are updated from 171/263 to 184/276 in `release/config.json`.

Minimum-API native/bridge builds and default-API native compilation pass. The
default Godot API feature is unchanged. No ObjectDB/resource leaks or orphan
StringNames were reported. Existing intentional invalid-enum script exceptions
remain inside EXPECTED_INVALID_ENUM markers; native failure-path diagnostics
remain intentional existing fixtures.

The alpha regression control ran the new channel fixture against the earlier
implementation. It failed exactly one test, with computed alpha -0.25 clamped
and warned. After separating input normalization from interpolation, that
fixture passes on both runtimes and in both APIs.

## Resource limits and reproduction

After the user's resource-limit request, compilation and engine checks run
serially with `nice -n 10 taskset -c 0,1`. Cargo also uses `CARGO_BUILD_JOBS=1`
and `-j 1`. Full native runs took about 369 seconds each and peaked below
164 MiB RSS. Final full GDScript runs took about 9–11 seconds and peaked below
176 MiB RSS. These measurements cover engine processes, not compilation.

```sh
CARGO_BUILD_JOBS=1 nice -n 10 taskset -c 0,1 cargo build --locked -j 1 \
  -p spire_tween_rust --features api-4-4,spire_tween/include_gdscript_bridge
CARGO_BUILD_JOBS=1 nice -n 10 taskset -c 0,1 cargo build --locked -j 1 \
  -p spire_tween --features api-4-4,include_gdscript_bridge,include_gdext_lib
CARGO_BUILD_JOBS=1 nice -n 10 taskset -c 0,1 cargo check --locked -j 1 \
  -p spire_tween_rust
```

Use the imported fixtures at `dist/animatable-20261103/{native,gdscript}-VERSION/`
and engines under `dist/engine-VERSION/editor/`. Copy the matching native or
bridge library to the fixture and synchronize GDScript tests before running.
Use writable temporary XDG config/cache/data directories.

```sh
nice -n 10 taskset -c 0,1 timeout 720 "$GODOT" --headless --verbose \
  --log-file /tmp/characters-native-engine.log --path "$NATIVE_PROJECT" \
  --max-fps 120 res://test_runner.tscn -- --no-cache
nice -n 10 taskset -c 0,1 timeout 180 "$GODOT" --headless --verbose \
  --log-file /tmp/characters-gdscript-engine.log --path "$GDSCRIPT_PROJECT" \
  --fixed-fps 120 res://examples/tests/test_runner.tscn -- --all-tests --no-cache
```

Focused native tests accept exact names, e.g. `--test=test_characters_render`.
Logs are `/tmp/spire-chars-{native,gdscript}-VERSION.log`, companion engine and
resource logs, and `/tmp/spire-chars-final-native-VERSION-SUFFIX.log` for the
final focused native reruns. The alpha control is
`/tmp/spire-chars-alpha-control.log`. Build/API logs use the same prefix.

Final bridge library SHA-256:
`76a04ad319560b184aef7a6f64167b183c053af9e27bd59ea925a55ea67d5b35`.
Final native bridge-enabled library SHA-256:
`6bbfdf45a093b112383176d4b5d889b72600157f8a702078885eb7d581598ea6`.

The preexisting controller edit and feedback design drafts are outside this
change and remain untouched. Tests use the current working tree in headless
mode; editor interaction and other operating systems were not exercised here.
