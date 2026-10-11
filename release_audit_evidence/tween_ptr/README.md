# Tween ownership verification

The original binary failed all ten initial GDScript ownership regressions, including retaining all 1,000 recorded wrapper IDs. The candidate collects those wrappers and passes the expanded 15-case GDScript and 10-case native ownership suites.

Run the focused suites with the rebuilt local Linux libraries:

```sh
godot --headless --path spire_tween_gdscript --fixed-fps 120 res://examples/tests/ownership_runner.tscn
godot --headless --path spire_tween_rust --fixed-fps 120 res://examples/tests/ownership_runner.tscn
```

Both focused runners report each case and return a nonzero exit code on failure. Tests also wait for short SceneTree timers, so they can run without fixed FPS. The ordinary GDScript and native all-tests runners include these cases, but their pre-existing failures/reporting limitations remain as recorded in the audit.

- `baseline-tests.log`: old binary, initial ten cases; all fail and all 1,000 wrapper IDs remain live.
- `candidate-gdscript-tests-60.log`, `candidate-gdscript-realtime.log`: debug library, expanded fifteen cases.
- `candidate-release-gdscript-tests.log`: release library, expanded fifteen cases.
- `candidate-native-final-tests.log`: final debug harness, ten native cases.
- `candidate-release-native-tests.log`: release harness, ten native cases.
- `candidate-*-full*.log`: broader suites, including known pre-existing failures.
- `installed-gdscript-tests.log`, `installed-native-tests.log`: focused tests using the actual project addon libraries after rebuilding them.
- `candidate-godot46-tests.log`: fifteen GDScript ownership cases on Godot 4.6.
- `spire-tweenptr-check-*.log`, `spire-tweenptr-cargo-final-tests.log`: feature/type/compile verification.
- `artifact-sha256.json`: rebuilt local Linux binary hashes.

The original isolated run directory is `/tmp/spire-tweenptr-ipaol1qh`. Godot configuration, data, and caches were redirected there. The native harness manifest intentionally points its Linux debug entry to a release-named path; the isolated debug check installed an actual debug-profile library at that path.
