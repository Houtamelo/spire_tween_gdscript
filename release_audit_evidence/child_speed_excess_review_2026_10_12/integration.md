# Sequence speed regressions — 2026-10-12

The excess-time documentation and regular integration coverage gaps are resolved.
`SpireTweener::process` and `AdvanceTimeResult::Completed::unscaled_excess_time`
describe the caller's input units, scaled internal bookkeeping, and the strict
near-zero cutoff. The new `sequence_speed.tscn` suite is registered in the regular
GDScript test catalog.

The suite has nine test methods containing 121 parameter cases. Coverage includes
all tween types and property modes, child/parent scaling, parallel joins, insertion,
three-level nesting, delays, Restart/Yoyo/Incremental loops, lazy constructors,
zero speed, and speeds below, at, and above the cutoff. Checks accumulate failures
and return false explicitly rather than relying on the older assertion helpers.

## Run

From the repository root, after building the addon:

```sh
godot --headless --path spire_tween_gdscript --script res://examples/tests/sequence_speed_runner.gd
```

The focused runner discovers the suite's test methods and exits 1 on failed checks
or its 30-second watchdog. The regular runner can also run these methods through
its normal all-tests, selective, and cached modes.

## Verification

| Check | Result |
|---|---|
| Focused debug and release suites | 9/9 methods and 121/121 parameter cases pass in each profile |
| Normal `TestBase.run_test` and catalog runner path | All nine methods pass |
| Catalog and CLI checks | 13/13 pass, including suite inclusion and discovery |
| Previous installed implementation | Eight methods fail with 90 failed checks; focused runner exits 1 |
| Injected failed check in a copied fixture | One method fails; focused runner exits 1 |
| Stalled test in a copied fixture | Watchdog reports timeout; runner exits 1 |
| Final debug/release builds and Rustdoc | Complete; no diagnostics at the updated trait documentation |

The installed Linux debug and release addon libraries were rebuilt and refreshed.
The focused suite passes against the actual debug addon path and a fixture linked
directly to the actual release addon path. `installed-results.json` records their
hashes and commands. Original binary backups are retained in the named `/tmp` paths.

All engine runs used Godot 4.7.2, API 4.6 bindings, and existing project caches in
headless runtime mode. No editor import or graphical runs were performed. Existing
Rustdoc warnings and unrelated full-suite release findings remain. This verification
does not claim that the full project test suite passes.

Earlier `validation.json` and probe logs preserve the initial review snapshot.
The `integration-*.json` files and adjacent logs record this follow-up's completed
documentation, registered suite, failure handling, and installed-library checks.
