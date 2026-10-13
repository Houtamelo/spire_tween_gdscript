# Ellipse/circle regression fixes — 2026-10-12

The 3D circle shortcut now supplies `(radius, radius, 0)` as both ellipse radius
endpoints. Its constant-radius path stays in the plane through the global center,
perpendicular to the normalized axis. The generator and generated wrapper both
contain the fix and method documentation. All 115 generated Rust files match a
fresh generator run in an isolated output directory.

The user's normalized-progress changes in the 2D/3D ellipse templates handle equal
angles without dividing by the angular span. The receiver binding cancels ordinary
standalone playback when its node is freed. A new test found that binding alone
left immediate deletion unsafe for sequence children, whose processing bypasses
the manager's bound-node check. Setting the explicit method owner in both templates
activates their existing owner-validity checks and resolves that case.

## Integration coverage

The regular GDScript catalog includes
[`ellipsis_regressions.gd`](../../spire_tween_gdscript/examples/tests/ellipsis_regressions.gd),
with eleven methods and 119 parameter cases. Checks accumulate failures explicitly.

- Circles: constant radius, zero axial displacement, zero radius, translated centers,
  principal/oblique/non-unit/zero axes, forward/reverse/multiple full turns, endpoint
  closure, and global coordinates under translated/rotated/scaled parents.
- Ellipses: finite equal-angle radius interpolation in 2D/3D, fixed direction,
  and preservation of intentional helical displacement.
- Lifecycle: automatic receiver binding, normalized progress endpoints, valid normal
  and forced completion, immediate/deferred deletion, nodes outside the tree, and
  cancellation/continuation inside sequences without child completion signals.

```sh
godot --headless --path spire_tween_gdscript --script res://examples/tests/ellipsis_regressions_runner.gd
```

The focused runner exits 1 on a failed Boolean result, empty discovery, or its
30-second watchdog. Copied fixtures verify failed-check and timeout handling.

## Results

| Check | Result |
|---|---|
| User's fixes before the companion changes | 4/11 methods fail; 16 native panics from immediate deletion of sequence receivers |
| Explicit method owner, before the circle change | All lifetime checks pass, zero native panics; three circle geometry methods still fail |
| Corrected debug and release libraries | 11/11 methods, 119/119 parameter cases pass per profile; zero errors or native panics |
| Regular catalog and `TestBase.run_test` path | All eleven methods pass |
| Catalog/CLI checks | 15/15 pass, including new suite inclusion and discovery |
| Previous sequence-speed suite | 121/121 parameter cases still pass |
| Injected failed check | Focused runner exits 1 |
| Stalled copied test | Watchdog reports timeout and exits 1 |
| Workspace/all-target compilation | Passes with six existing native demo warnings |
| Fresh generator output | All 115 generated files match the checked-in tree |

The Linux debug and release addon libraries were refreshed atomically in their
actual installed paths. The focused suite passes directly in the real debug
project and through a fixture linked to the installed release library. Original
library backups remain in the recorded `/tmp` paths.

Commands, library/source hashes, complete logs, intermediate failing results, and
the fixture used to exercise the regular harness are saved beside this file.
Engine checks used Godot 4.7.2, API 4.6 bindings, and existing project caches in
headless runtime mode. No editor imports or graphical runs were performed.

These checks verify the repaired geometry and normal-playback cancellation.
Unrelated release findings, including invalid-method forced-completion semantics
in audit #15, remain. This is not a full-suite or all-platform success claim.
