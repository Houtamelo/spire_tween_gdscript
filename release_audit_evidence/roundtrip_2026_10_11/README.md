# Fresh release review evidence — 2026-10-11

See [the review](../../release_audit_round2.md) for findings and priorities. The reviewed source is commit `cc35c19`; `source-hashes-before-feature.json` freezes the source before the subsequent constructor-bridge task.

- `probe.gd`, `probe-{debug,release}.log`: deterministic speed/cancellation controls and failures.
- `early.gd`, `early-{debug,release}.log`: manager initialization before the SceneTree is available, followed by ordinary later use.
- `templates.gd`, `templates-{debug,release}.log`: freed 2D/3D ellipse owners and equal-angle NaN geometry.
- `extra.gd`, `extra-debug.log`: follow stopping on first catch, 3D circle axial displacement, fresh-source future-insert control.
- `bookkeeping.gd`, `bookkeeping.log`, `bookkeeping-instrumentation.rs`: observation-only class added to a copied crate, counting retained node/weak records without changing lifecycle logic.
- `surface.gd`, `installed-surface.log`: existing project-installed Linux debug library, future-insert behavior and method surface.
- `assert_probe_*.gd`, `assert-helper.log`: real test helper failure still returns true.
- `gdscript-debug-fixed.log`, `gdscript-release.log`: fresh-library full uncached runs, fixed 120 FPS.
- `gdscript-debug.log`: capped real-time run, incomplete at 240 seconds.
- `native-debug-corrected-fixture.log`: fresh actual debug native harness at the release path selected by its existing manifest. The earlier `native-debug.log` records a missing-library fixture setup, not a runtime addon failure.
- `*-result.json`, `results-summary.json`, `provenance.json`: commands, exits, artifact hashes and verification boundaries.
- `packaging-inventory.json`, `dependency-inventory.json`: local distribution inventory and declared Linux workspace package licenses. Neither is a completed customer archive or final linked-binary notice inventory.

All engine probes are real headless Godot runtime checks. Installed libraries are read/copied, not replaced. Existing caches are copied to avoid editor import. No graphical session, platform purchase, message to an asset author, or publishing action was taken.
