# Group 4 template validation

Scope: native/GDScript waypoint paths, rich text and scrambling, and integer
margins. Public contracts and mappings are in
[`docs/paths_text_and_values.md`](../../docs/paths_text_and_values.md).

Paths and margins use the existing float method tween pipeline. Text options
use the existing string property tween pipeline, including relative and
speed-based processing. No scheduler, ownership, completion-signal, or default
Godot API feature changes were required.
No DOTween source was copied and no new dependency was added.

## Coverage

There are 26 new native cases and 27 new GDScript cases, registered with the
existing runners. Expected full totals are 166 native and 258 GDScript tests.
`release/config.json` contains those exact totals.

Coverage includes:

* Unicode replacement and source capture; nested BBCode and cross-style slices;
  bracket/control tags; atomic images; quoted attributes; malformed text; every
  scramble alphabet; invalid custom alphabets; cancellation and stopped replay.
* Unified text property return types and options; removal of the separate effect
  APIs; opt-in rich defaults; changing fixed/dynamic endpoints and options;
  shrinking half-step rounding; cached buffer growth/shrinkage and empty inputs;
  nested style stacks beyond inline storage; long/custom alphabet replacement;
  relative append/remove and forced completion;
  speed-based edit budgets, excess time, formatting-only changes, atomic images
  and Yoyo. Unsupported scrambling warns once per configuration, preserves the
  selected mode, and is ignored for both rich and plain relative/speed-based text.
* Unequal-length polylines; known cubic and Catmull-Rom points; distance inversion;
  2D, 3D, and Control receivers; transformed parents and local/global coordinates;
  closed paths; stationary/duplicate points; locks before distance measurement;
  plane modes, custom model forward, fixed/moving look-at, and copied options.
* Zero scale and top-level nodes; invalid owner/target cancellation; ignored
  look-at targets; Restart/Yoyo/Incremental; stop versus force-complete;
  sped-up child sequence handoff; invalid options and incomplete cubic triples.
* Effective theme margin capture, signed margins, reflected Vector4i properties,
  typed Rust accessors, wrong reflected types, and replay of invalid builders.
* Existing i64 property/method APIs with negative values, values beyond the
  32-bit range, invalid callables, and receiver invalidation. GDScript reflection
  also checks that the removed numeric helpers are no longer registered.

## Regression controls

With the new low-resolution cubic-loop test and the preceding addon binary,
`test_group4_path_curve_edges` fails: a nonconstant curve whose start equals end
was treated as stationary with one sample per segment. Captured in
`/tmp/spire-group4-resolution-before.log` (`total=1 failed=1`, exit 1).
Resolution now clamps to at least three samples, with a warning containing the
rejected value. A nonconstant cubic cannot equal its start at all four sample
points (t=0, 1/3, 2/3, 1).

The initial native cancellation assertion was corrected to distinguish explicit
binding invalidation (`NoLongerValid`) from owner invalidation (`ShouldDespawn`).
Both stop without successful completion; no runtime cancellation change was needed.

The integer API regression fails against the preceding addon binary because the
four removed numeric helpers remain registered there. Captured in
`/tmp/spire-signed-before.log` (`total=1 failed=1`, exit 1). The same regression
passes with the rebuilt addon and exercises the existing signed integer APIs.

## Results before the text API merge (2026-11-01)

| Engine (Linux x86_64) | Native | GDScript | Exit codes |
|---|---|---|---|
| Godot 4.4 stable | 162 passed, 0 failed | 254 passed, 0 failed, 0 skipped | 0 / 0 |
| Godot 4.7.2 stable | 162 passed, 0 failed | 254 passed, 0 failed, 0 skipped | 0 / 0 |

All 22 native and 23 GDScript group-4 cases were discovered and passed on both
engines. Logs contain no unexpected script errors, panics, ObjectDB leaks,
resource leaks, or orphan StringNames. Existing deliberate invalid-enum errors
and native API-failure panics remain inside their expected markers.

Additional checks passed: 16 library tests, default-API bridge compilation,
native rustdoc broken-link checking, targeted rustfmt, and `git diff --check`.
Regeneration reproduced all 137 generated files byte for byte. Rustdoc retains
the 32 existing warnings about diagram code blocks; no new broken links occurred.

The verification fixtures live in `dist/signed-integers-20261101/` (ignored build artifacts).
Logs use `/tmp/spire-signed-{native,gdscript}-{4.4,4.7.2}.log` and companion
`-engine.log` files. Native fixtures are `native-*`; GDScript fixtures are
`gdscript-*`. Libraries are copied only after builds complete, and never overwritten
while mapped by a running engine.

## Results after merging text options (2026-11-01)

The separate text effect template and bridge methods were removed. Native
`do_text(...).with_text_options(...)`, generated GDScript `text(...)`,
`Spire.do_property_string(...)`, and `SpirePropertyString.set_text_options(...)`
share the string property tween. BBCode is opt-in in every constructor and works
in all property modes; relative/speed-based modes warn once per options
configuration and ignore scrambling. The requested mode remains unchanged.

| Engine (Linux x86_64) | Native | GDScript | Exit codes |
|---|---|---|---|
| Godot 4.4 stable | 165 passed, 0 failed | 257 passed, 0 failed, 0 skipped | 0 / 0 |
| Godot 4.7.2 stable | 165 passed, 0 failed | 257 passed, 0 failed, 0 skipped | 0 / 0 |

All 25 native and 26 GDScript group-4 cases were discovered on both engines.
The three new text cases also passed focused Godot 4.4 runs. Their logs confirm
exactly one unsupported-scrambling warning per configured tween across repeated
updates (three distinct relative tweens, two distinct speed-based tweens).
No shutdown ObjectDB/resource leaks or orphan StringNames appeared. The only
script exceptions are inside the existing deliberate invalid-enum markers.

Additional checks passed: 16 library tests, default-API bridge compilation,
strict native rustdoc broken-link checking, targeted rustfmt, and diff whitespace
checking. Generator reproducibility covered all 137 generated files. Existing
compiler/rustdoc warnings were unchanged.

Fixtures: `dist/text-options-20261101/{native,gdscript}-{4.4,4.7.2}/`.
Full logs: `/tmp/spire-text-{native,gdscript}-{4.4,4.7.2}.log` and companion
`-engine.log` files. Focused logs use `/tmp/spire-text-{native,gdscript}-text_*.log`.
Build, unit, default-API, rustdoc, format, and generator logs share the
`/tmp/spire-text-` prefix. Generated file hashes are recorded in
`/tmp/spire-text-generated.json`.

## Results after the text allocation review (2026-11-01)

Text parsing and rendering now use borrowed UTF-32 character slices. Glyphs and
styles store ranges and parent indices into cached Godot strings, without owned
per-glyph text or cloned style stacks. Parsed tables and output storage retain
their capacity as values change. Cached SmallVec style stacks and custom
alphabets retain spilled storage; plain string operations use SmallVec character
buffers. Scrambling renders character overrides directly from cached glyphs.
Closing tags append characters directly, and the control-tag list is a separate
macro. Neither string lerper contains UTF-8 conversion or formatting calls.
Returned Godot strings are still constructed; this is not a claim that complete
engine processing performs no allocations.

| Engine (Linux x86_64) | Native | GDScript | Exit codes |
|---|---|---|---|
| Godot 4.4 stable | 166 passed, 0 failed | 258 passed, 0 failed, 0 skipped | 0 / 0 |
| Godot 4.7.2 stable | 166 passed, 0 failed | 258 passed, 0 failed, 0 skipped | 0 / 0 |

All 26 native and 27 GDScript group-4 cases were discovered and passed on both
engines. The added buffer test reuses a tween across growing, shrinking and empty
text; twelve nested styles; quoted Unicode attributes; rich/plain option changes;
long alphabet spill/replacement; shared styles across source/target; plain-buffer
spill; and every self-closing control tag. Focused buffer tests passed in both
languages on 4.4, and all nine native text cases passed focused runs.
No shutdown leaks or unexpected script exceptions/panics appeared. The existing
deliberate invalid-enum script exceptions remain inside their markers.

Also passed: 16 library tests, default-API bridge compilation, strict native
rustdoc broken-link checking, targeted rustfmt, and diff whitespace checking.
Existing compiler/rustdoc warnings remain unchanged. No generator source or
output changed in this allocation review.

Fixtures: `dist/text-buffers-20261101/{native,gdscript}-{4.4,4.7.2}/`.
Full logs: `/tmp/spire-text-opt-{native,gdscript}-{4.4,4.7.2}.log` and companion
`-engine.log` files. Focused logs use `/tmp/spire-text-opt-native-*.log` and
`/tmp/spire-text-opt-gdscript-buffers.log`. Build/check logs use the same
`/tmp/spire-text-opt-` prefix.

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

Install each library in a separate imported fixture. Set `XDG_CONFIG_HOME`,
`XDG_CACHE_HOME`, and `XDG_DATA_HOME` to writable temporary directories when running
inside the filesystem sandbox. Pass an explicit writable log path.

```sh
timeout 660 "$GODOT" --headless --verbose --log-file /tmp/group4-native-engine.log \
  --path "$NATIVE_PROJECT" --max-fps 120 res://test_runner.tscn -- --no-cache
timeout 120 "$GODOT" --headless --verbose --log-file /tmp/group4-gdscript-engine.log \
  --path "$GDSCRIPT_PROJECT" --fixed-fps 120 \
  res://examples/tests/test_runner.tscn -- --all-tests --no-cache
```

Native timing tests use the real clock; GDScript tests use fixed FPS. Validation
on these Linux engines does not substitute for the existing cross-platform CI.
