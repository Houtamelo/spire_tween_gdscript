# Blueprint editor preview validation

Validated on 2026-11-20 with Linux x86_64 debug libraries and real Godot editors on private Xvfb displays. Every fixture has isolated project files and XDG directories. Installed development-project binaries and caches were not changed.

Preview rows include the latest invalid-Callable contract update. The remaining regression rows describe the initial implementation validation.

| Check | Godot 4.4 | Godot 4.7.2 |
|---|---|---|
| Preview editor assertions | 228 passed | 228 passed |
| Preview runtime isolation | 2 passed | 2 passed |
| Existing Blueprint runtime suite | 29 passed | 29 passed |
| Existing Blueprint editor suite | Passed | Passed |
| Supplied template runtime/editor suites | 33 / 33 passed | 33 / 33 passed |
| Text/preset runtime suite | 65 passed | 65 passed |
| Text/preset editor and typed-array rejection checks | Passed | Passed |

Preview coverage includes native Inspector Play/Stop clicks, controller-driven playback, direct sequence cancellation, completion, replacement, parent-managed button disabling, nested serial/parallel composition, reverse custom callbacks, capture before reset hooks, unsupported capture, missing properties, freed roots/targets, unrelated tweens, save/reload, and actual scene-tab switching. All 33 templates restore on interruption, replacement and forced completion.

Character checks cover existing offsets/pivots, inherited theme and BBCode colors, invalid/stale/uninitialized capture, and text replacement composed with Text steps in either child order. Combining characters and ligatures exercise restoration of shaped glyph mappings, including a shared glyph root outside the selected range. A failing text-replacement case was reproduced before the fix.

The preview harness accounts for Godot 4.4's physical-cursor hover routing by moving the cursor only inside its private Xvfb display. The editor mouse test uses a long animation so software-rendering load cannot complete playback before Stop is clicked.

## Reproduce

Build the normal addon, then run:

```sh
cargo build --locked -p spire_tween --features include_gdext_lib,include_gdscript_bridge,api-4-4
bash scripts/test_blueprint_preview.sh /path/to/godot /tmp/spire-preview-evidence
```

The harness rejects engine/script errors and failed assertions. Expected configuration and editor-only API warnings remain in the logs. It also rejects a missing addon library before starting the editor. The ordinary Blueprint/template runners and instrumented text/preset runner remain separate regression checks.

## Evidence

Preview logs and four rendered screenshots per engine are retained in `dist/preview-validation/4.4/` and `dist/preview-validation/4.7.2/`. The initially validated normal library is `dist/preview-validation/libspire_tween.so`, SHA-256 `d30b54250d5169f1c2ed1b4ec33a5107fd7d92b10e804f0c415ce0128dad48a7`; it matched `target/debug/libspire_tween.so` at that validation point.

The text/preset tests used the same source with `integration-tests` instrumentation, SHA-256 `fe879cc4662ee4ef807447dced051cfb58acbcbd0327f05bd2d47a7eeb01ea4d`. The normal production build was restored afterward.

Regression logs are retained alongside preview evidence under `dist/preview-validation/regressions/`. These are focused Linux checks; remote CI, other host platforms, and release/export validation were not run. Editing animated properties, Undo/Redo interception, deferred restoration and native hot reload remain outside the approved scope.

## Follow-up: refresh owned by the label callback

`capture_character_rest_state()` now owns the full restoration path, including refreshing empty/uninitialized whole-label captures. Blueprint no longer casts the target back to SpireRichTextLabel during restoration. Capture stays read-only; a missing current snapshot produces a callback that rebuilds defaults synchronously. Explicit invalid/uninitialized character ranges still fail capture.

At that point, the real-editor suite ran 212 assertions on both Godot 4.4 and 4.7.2. All new callback, empty-label, pending-text and uninitialized-label checks passed. Each full run had one failure: `Unsupported child refuses entire preview without target writes`. The test expected rejection, while containers skipped invalid child restorers. These historical runs were not fully passing acceptance runs; the contract decision below resolves the mismatch.

Follow-up logs are retained under `dist/preview-validation/refresh-callback-4.4/` and `dist/preview-validation/refresh-callback-4.7.2/`.

## Latest contract: invalid means nothing to restore

An invalid `_capture_rest_state()` Callable now means there is no restoration action, for both standalone blueprints and children. Preview starts normally, remains stoppable and participates in the single-active-preview lifecycle. Custom scripts without a capture override use this default without warnings. Containers play all children and invoke only valid callbacks in reverse child order.

The updated suite passed all 228 editor assertions and both runtime-isolation checks on each engine. New cases cover a child without restoration among valid siblings, serial/parallel compositions with no restorers, standalone Stop/completion/replacement with no callback, and a script that uses the default capture hook. The old rejection assertion was replaced with checks for the approved behavior.

Latest logs are retained under `dist/preview-validation/nothing-to-restore-4.4/` and `dist/preview-validation/nothing-to-restore-4.7.2/`. The tested production library is `dist/preview-validation/nothing-to-restore.so`, SHA-256 `44525595ee6eae751f3de1e7c804d1d08e80935cddae9e0bdeff7a423c5671c8`.
