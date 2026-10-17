# Test runner verification — 2026-10-14

The audited false-success and stale-cache paths are repaired. All checks below use
real Godot 4.7.2 on Linux x86_64 with fresh default-API-4.6 debug/release libraries.
Existing project import caches are copied; no graphical run or editor import occurs.

| Check | Result |
|---|---|
| Original `TestBase.assert_eq(1, 2)` followed by `return true` | Reproduces false success before the fix: exit 0 |
| Passing/failing runner subprocesses, per-test/suite deadlines, paused-tree deadlines, bad CLI inputs, cache reuse/invalidation | 70/70 expected outcomes across debug/release |
| Full native suites, real time with `--max-fps 120` | 134/134 cases per profile; exit 0 |
| Full GDScript suites, `--fixed-fps 120 --all-tests --no-cache` | 152/152 cases per profile; exit 0 |
| Actual installed-project positive/negative controls | Both runners exit 0 for a passing case and 1 for a deliberate failure |
| Workspace/all-target check with GDScript bridge | Pass |
| Native compile with opt-in API 4.4 | Pass; default API unchanged |
| Installed addon hashes / native library symlinks | Match fresh artifacts; symlinks preserved |

The 134 native cases comprise 66 focused cases and 68 individually named cases.
The full native run uses real time because `ignore_time_scale` compares engine time
with wall time. An initial fixed-FPS run correctly fails that unsuitable assertion.
The 3D ellipse test now samples exact manual steps with a 0.0001 spatial tolerance;
it no longer confuses late timer resumption with geometry errors.

GDScript helper assertions accumulate failures even when the calling body returns
true. Ordinary tests use those helpers. Native setup and asynchronous panics have
explicit outcomes; native callback assertions use checked subscription helpers.
Per-test deadlines are 60 seconds and suite deadlines 900 seconds by default.
Always-processing watchdog timers also work when the tree is paused. The regression
fixtures intentionally return false/nil, assert, panic, or never finish; their
expected nonzero exit is a successful verifier check, not a green failing test.

Cache checks change actual copied test source and swap two valid addon artifacts.
They also reject changed engine identities, preserve cached passes when inputs are
unchanged, and verify that `--no-cache` does not read or modify cache files. SHA-256
identity includes the engine executable, selected addon binary, shared scripts and
scenes, project/extension configuration, and clock settings.

The invalid-enum test expects rejection, preserves the original valid tween, and
checks normal completion. Godot/godot-rust diagnostics from those deliberate calls
are bracketed by `EXPECTED_INVALID_ENUM_BEGIN/END`; native API rejection probes
likewise use `EXPECTED_API_FAILURE_BEGIN/END`. Other assertion failures are not
suppressed. The GDScript aggregate's shutdown warnings remain separately recorded
in [shutdown_warnings.md](shutdown_warnings.md).

`summary.json`, `harness-results.json`, aggregate JSON/logs, library hashes, build
logs, and source hashes record the verified candidate. Reproduce the runner checks
with [scripts/verify_test_runners.py](../../scripts/verify_test_runners.py) after the
build commands in the main README. That verifier isolates fixtures and preserves
the example projects' caches and installed libraries.
