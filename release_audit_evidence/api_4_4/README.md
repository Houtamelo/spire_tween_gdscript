# Godot API 4.4 verification

Both crates expose the opt-in `api-4-4` feature. It forwards to `godot/api-4-4`; the native crate also forwards to `spire_tween/api-4-4`, matching its local compilation guards. Ordinary builds retain godot-rust's default API selection. The initial default-feature selection was removed at the user's request; the results below record the API 4.4 experiment.

The baseline feature-only check failed with eight native compilation errors. The guarded code resolves the 4.4 bindings' nullable timer, `Node::create_tween`, and `Tween::tween_property` results. No addon playback behavior was changed.

| Verification | Result |
| --- | --- |
| API 4.4 core addon and native demo/test builds | Debug and release pass |
| 34 focused GDScript cases | Pass on Godot 4.4.1 and 4.7.2, both profiles |
| 36 focused native cases | Pass on Godot 4.4.1 and 4.7.2, both profiles |
| Five native API cases with defaults disabled, API 4.6 | Pass on Godot 4.7.2 |
| Actual installed Linux project libraries | All 70 focused cases pass on Godot 4.4.1 |
| Cargo tests | Eight benchmark smoke checks and one compile-fail doctest pass; `must_compile` runs zero tests and 78 doctests remain ignored |
| Full GDScript suite, debug libraries | 83 started/finished on each engine; invalid-enum null-call failure remains |
| Full native suite, release libraries | 36 focused cases pass, then 68 older cases start and 65 finish on each engine; prior shake/timing/freed-node failures remain |

Build commands:

```sh
cargo build --offline --locked -p spire_tween --features api-4-4,include_gdext_lib,include_gdscript_bridge
cargo build --offline --locked -p spire_tween --release --features api-4-4,include_gdext_lib,include_gdscript_bridge
cargo build --offline --locked -p spire_tween_rust --features api-4-4
cargo build --offline --locked -p spire_tween_rust --release --features api-4-4
cargo build --offline --locked -p spire_tween_rust --no-default-features
cargo test --offline --locked -p spire_tween --all-features
```

Engine commands use `--headless --fixed-fps 120 --path <project>` and the corresponding `res://examples/tests/*_runner.tscn`. The native API scene is `api_compatibility_runner.tscn`. The two official runtimes report `4.4.1.stable.official.49a5bc7b6` and `4.7.2.stable.official.ed1daf0bf`.

The five new API cases exercise timer completion, built-in position/color playback including `from` values, and Godot refusing position/color tweeners appended after a tween starts. Each refusal prints an engine error followed by a caught panic, enclosed by `EXPECTED_API_FAILURE_BEGIN`/`EXPECTED_API_FAILURE_END`. Validation requires exactly two such blocks, the expected refusal/panic, and `NATIVE_API_RESULT total=5 failed=0`; errors outside those blocks fail the focused gate. Cycle suites retain their eight GDScript/nine native intentional warnings. The broader-suite logs also contain intentional errors from existing invalid-input cases; the failures above are actual script errors/assertion panics and incomplete test execution.

The 29 focused profile/API gates and seven installed-path gates pass. Full suites are recorded as failing despite their zero exit status. Their fixed-FPS time-scale comparison uses wall-clock time, so its failure does not establish an addon time-scale defect.

Each isolated profile check places its intended library at the manifest entry selected by the editor executable. GDScript release-profile checks therefore use the isolated debug manifest path; the native manifest uses a release-named path for both profiles. The installed-path results and artifact hashes below describe the experimental API 4.4 libraries. The installed libraries are subsequently restored to ordinary builds, with the native symlink preserved; correction results and current hashes are recorded under `default_api_restore/`.

Logs, command/result JSON, source/artifact hashes, and the baseline compiler errors are retained here. Cached project resources were used. Other platform artifacts, the intervening Godot 4.5/4.6 runtimes with these API 4.4 libraries, clean first import, and export were not verified in this task.

The correction under `default_api_restore/` passes four ordinary builds and 14 focused profile gates: all 70 cases on Godot 4.7.2 in debug/release using API 4.6. Installed project checks confirm API 4.6, current artifact hashes, and the preserved native symlink. Explicit opt-in API 4.4 compilation still passes. No default feature selects API 4.4 in either crate.
