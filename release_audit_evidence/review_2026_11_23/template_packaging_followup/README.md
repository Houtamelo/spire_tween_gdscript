# Template packaging follow-up

The repaired builder stages and verifies all 55 authored template files. The
customer candidate includes 33 scenes, their GDScript and UID sidecars, catalog,
and README exactly once. No addon runtime Rust code was changed.

This is a fresh local Linux stage produced by the actual release builder from
the current uncommitted source snapshot. `build.json` records the exact source
hashes and marks that snapshot dirty. It is a development candidate, not evidence
that the complete desktop release matrix has passed.

Candidate: `SpireTween-1.4.0-d3d993e754e4-candidate-linux.zip`

SHA-256: `1fef3cec5b704914000da6a8c0c833571e2dd047f5918b956f3bbf627000cfb7`

| Check | Godot 4.4 | Godot 4.7.2 |
|---|---|---|
| Real-artifact integration tests | 3 pass | 3 pass |
| Extracted archive, debug/release library profiles | All 33 templates load, animate and restore | All 33 templates load, animate and restore |
| Debug/release exports and exported execution | All 33 templates load, animate and restore | All 33 templates load, animate and restore |
| Missing shared script, with valid archive checksums | Rejected | Rejected |
| Missing staged script, also removed from the artifact-hash map | Rejected using the source inventory | Rejected using the source inventory |

The archive has 504 files, 55 under `addons/spire_tween/templates/`, and no
duplicate entries. Integration assertions compare every delivered template file
with its source bytes. The missing-script control regenerates the archive's
checksum manifest so the runtime check, rather than a stale checksum, must fail.

`rustdoc-default.log` records a successful ordinary
`cargo doc --locked -p spire_tween --no-deps`. It emits warnings but generates the
documentation. The earlier strict audit command promoted those warnings to
errors; it is not an existing release CI gate or evidence of broken Godot help.
The BBCode is intentional and necessary for Godot. The finding is withdrawn;
no documentation formatting changes or follow-up are needed.

Full build stages, archives and exports remain under
`/tmp/spire-template-packaging-20261123`. Logs and build/validation metadata are
retained here; native libraries and exported executables are not duplicated.
The initial stage attempt compiled all four libraries but stopped while unpacking
an uncached Cargo dependency for notices into a read-only cache. After populating
that cache, a new staging directory completed successfully without weakening
provenance checks.

Reproduction from the repository root, with a fresh output directory:

```bash
SPIRE_RELEASE_TARGET_DIR="$PWD/target" cargo run --locked -p spire_release -- \
  build --platform linux --out "$AUDIT_DIR/staging/linux"
SPIRE_RELEASE_STAGE="$AUDIT_DIR/staging/linux" SPIRE_RELEASE_GODOT="$GODOT" \
  cargo test --locked -p spire_release --test pipeline -- --ignored
cargo run --locked -p spire_release -- package \
  --staging "$AUDIT_DIR/staging" --reports "$AUDIT_DIR/reports" \
  --out "$AUDIT_DIR/archive" --candidate --platforms linux
cargo run --locked -p spire_release -- test-archive \
  --archive "$ARCHIVE" --godot "$GODOT" --templates "$EXPORT_TEMPLATES" \
  --out "$AUDIT_DIR/archive-validation"
```

Run the integration/archive checks with each supported engine and matching
export templates. A complete Windows/macOS run still belongs to the configured
desktop workflow after these changes are committed and pushed.
