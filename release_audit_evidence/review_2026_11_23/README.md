# Evidence for the 2026-11-23 release review

Read [the review](../../release_audit_round3.md) for findings and interpretation.
Source revision: `d3d993e754e48630150624a4f4a76675e18a0305`.
The worktree was clean at the start. Test/workflow/documentation repairs are
listed in the review; addon runtime Rust source and installed addon libraries
were not changed. The subsequent
[template packaging follow-up](template_packaging_followup/README.md) changes
release tooling and records its separate fresh build and archive checks.

Current disposition: the pointer architecture is accepted and its blocker was
withdrawn. The desktop matrix is a validation checklist. Godot's BBCode is
intentional and required; the rustdoc finding is withdrawn with no formatting
changes or follow-up needed. Original observations and logs below remain the
record of the initial audit, before the packaging fix.

## Artifact identity

The tested Linux libraries are from the successful Linux job in
[CI run 45291109254](https://github.com/Houtamelo/spire_tween_gdscript/actions/runs/45291109254).
`build.json` records their hashes, source snapshot, toolchain, API, notices,
and measured GLIBC requirement. No locally installed example binary was used.

| Library | SHA-256 |
|---|---|
| Customer debug | `a0bd70aaa15eacd53294c687782049dcc0b6d8f62acaf6e0196df2497dc996a1` |
| Customer release | `815f3388009b7d536004c006859757c8f3ff09d75d618229176b296d876226cd` |
| Partial Linux candidate archive | `8e1f0f3c0f1b9d4dbf79c8a926c6f9ff0a177eef09354ee2eb030790e40bf8d0` |

Godot versions: `4.4.stable.official.4c311cbee` and
`4.7.2.stable.official.ed1daf0bf`. Rust:
`1.99.0-nightly (12c36e253 2026-08-10)`; godot-rust: `0.5.5`.
Native aggregate runs use real time capped at 120 FPS. Ordinary GDScript
aggregates use fixed 120 FPS. Rendered/editor checks use private Xvfb,
software rendering, isolated XDG directories, and isolated project copies.

Full temporary artifacts, projects, exports and original logs are under
`/tmp/spire-release-review-20261123`. The large native libraries, archives,
exported executables, editor caches, and dependency-source payload are not
duplicated here.

## Logs and probes

- `baseline-*.log`: ordinary GDScript runs against production libraries before
  the test repair. The glyph-sampling method is unavailable, type inference
  fails, and discovery fails. Exit code was 1 on both engines.
- `gdscript-ENGINE-PROFILE.log`: repaired ordinary catalog, 343 cases per run.
- `native-ENGINE-debug.log`: complete native catalog, 189 cases, preceded by
  seven focused suites totaling 67 checks.
- `native-ENGINE-release-interrupted.log`: release aggregates terminated by the
  Codex server restart. These are incomplete runs, not passing aggregate logs.
  `native-ENGINE-release-resumed/` contains the remaining cases run individually,
  selected by comparing completed case IDs with the debug catalog. Successful
  cases before the interruption were not repeated.
- `runner-controls/`: actual passing and deliberately failing runner probes.
  Deliberate failures must exit 1 and print `failed=1`; those logs are expected.
- `archive-validation-ENGINE.json`: extracted Linux candidate's built-in
  checksum/install/export results. Both pass despite the missing templates.
- `feature_smoke.gd` and `feature-*.log`: an additional customer-library probe
  for native property/text Blueprints, character ranges, runtime preset adapter
  creation, and code-after-preset color composition. It also checks that the
  customer library has no glyph-observation method. Twelve assertions run in
  each source and debug/release exported process on both engines.
- `alias_compile_probe.rs` and `alias-cargo-check.log`: compile-only evidence
  that the safe pointer API accepts overlapping mutable references. The probe
  was checked against the current source through a path dependency. **Do not
  execute its functions.** This is not an engine crash or Miri reproduction.
- `editor_lifetime.gd` and `editor-lifetime.log`: real editor plugin comparing
  three batches of 500 ordinary node allocations with three tweened batches.
  Stable object counts do not constitute a census of manager records.
- `templates/`, `preview/`, and `presets/`: ordinary graphical test-script logs
  for each engine. Instrumented preset rendering uses the CI native integration
  extension; it is separate from the customer addon.
- `rustdoc-*.log`: strict broken-link checks, with/without the bridge feature.
  These fail on Godot BBCode interpreted as Rust links.
- `ci-checks.jsonl`, `macos-annotations.json`, and `releases.json`: current CI
  status, rejected runner configuration, and the empty published-release list.

`review-results.json` records review observations. It is not a release-tool
`validation.json`, is not accepted as platform validation evidence, and must not
be substituted for the complete release pipeline.

## Reproduction commands

Run from the repository root. Set engine/artifact paths to the installed engines
and the downloaded `build-linux` artifact as appropriate.
The three artifact integration tests include packaging; run those in the clean,
matching checkout described below as well.

```bash
gh run download 45291109254 --name build-linux --dir "$AUDIT_DIR/staging/linux"
cargo test --locked -p spire_release
cargo check --workspace --all-targets --locked
cargo test --locked -p spire_tween --features api-4-4

cargo run --locked -p spire_release -- test \
  --stage "$AUDIT_DIR/staging/linux" --godot "$GODOT" \
  --templates "$EXPORT_TEMPLATES" --out "$AUDIT_DIR/full"

SPIRE_RELEASE_STAGE="$AUDIT_DIR/staging/linux" \
SPIRE_RELEASE_GODOT="$GODOT" \
  cargo test --locked -p spire_release --test pipeline -- --ignored

SPIRE_TEMPLATE_LIBRARY="$AUDIT_DIR/staging/linux/addon/lib/x86_64-unknown-linux-gnu/debug/libspire_tween.so" \
  bash scripts/test_scene_templates.sh "$GODOT" "$AUDIT_DIR/templates"
SPIRE_PREVIEW_LIBRARY="$AUDIT_DIR/staging/linux/addon/lib/x86_64-unknown-linux-gnu/debug/libspire_tween.so" \
  bash scripts/test_blueprint_preview.sh "$GODOT" "$AUDIT_DIR/preview"
SPIRE_PRESET_LIBRARY="$AUDIT_DIR/staging/linux/tests/lib/x86_64-unknown-linux-gnu/debug/libspire_tween_rust.so" \
  bash scripts/test_text_effect_presets.sh "$GODOT" "$AUDIT_DIR/presets"

RUSTDOCFLAGS='-D rustdoc::broken_intra_doc_links' \
  cargo doc --locked -p spire_tween --no-deps
```

Packaging requires the checkout's exact source bytes to match the artifact
snapshot. The original local checkout had five CRLF/LF differences despite a
clean Git status; the packager correctly rejected that mismatch. A fresh local
clone at the same commit supplied matching LF-normalized bytes, and successfully
assembled the candidate. No hash or source-identity gate was weakened.

Run the package commands from that clean checkout using its release-tool build:

```bash
cargo run --locked -p spire_release -- package \
  --staging "$AUDIT_DIR/staging" --reports "$AUDIT_DIR/reports" \
  --out "$AUDIT_DIR/archive" --candidate --platforms linux
cargo run --locked -p spire_release -- test-archive \
  --archive "$ARCHIVE" --godot "$GODOT" --templates "$EXPORT_TEMPLATES" \
  --out "$AUDIT_DIR/archive-validation"
```

To reproduce the extra exported-feature probe, replace `smoke.gd` in the archive
test's generated export project with `feature_smoke.gd`, export both profiles
using its existing preset, and run each executable with `--headless`. Require
`RELEASE_FEATURE_SMOKE_RESULT checks=12 failed=0` and exit 0.

For the aliasing compile witness, create a temporary Cargo library with a path
dependency on this checkout's `spire_tween`, enable `api-4-4`, use
`alias_compile_probe.rs` as its `src/lib.rs`, copy the root lockfile, and run
`cargo check --offline`. Only compilation was performed.
