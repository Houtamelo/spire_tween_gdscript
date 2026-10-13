# Delayed-call validity verification

The baseline real Godot GDScript suite reports `DELAY_VALIDITY_RESULT total=9 failed=6`; valid anonymous, RefCounted-method, and forced-completion/replay controls already pass. The candidate reports
zero failures for nine GDScript and nine native cases in debug and release on Godot 4.7.2 with API 4.6. Library profiles are tested through the manifest entry selected by the editor executable;
release artifacts therefore occupy the isolated debug entry in the GDScript project.

The implementation checks validity before progressing the delay and before forced invocation. Invalid calls stop, enqueue unregistration, and do not emit completion signals. `NoLongerValid` lets a
parent sequence remove the child and continue. Manager removal uses the same allocation address already used by `TweenPtr`; this supports native calls without manufacturing a Godot handle. No target
binding model or default Godot API selection changed.

Coverage includes freed Node/RefCounted targets, initially empty/missing-method callables, active/paused forced completion, raw builders, native manual processing, valid ownerless custom/anonymous
calls, bound arguments, normal delay/finish signals, replay, a callback freeing its looping target, differing receiver/callable targets, and an invalid sequence child followed by valid work. The 70
existing focused ownership/binding/cycle/API cases also pass in both profiles: 88 focused cases total, across 18 suite gates. New delayed-call suites contain no intentional console errors or warnings.
Existing cycle suites retain their expected eight GDScript/nine native warnings; existing API-negative cases retain their two marked expected failure blocks.

Commands:

```sh
cargo build --offline --locked -p spire_tween --features include_gdext_lib,include_gdscript_bridge
cargo build --offline --locked -p spire_tween_rust
# Repeat both builds with --release.
godot --headless --fixed-fps 120 --path spire_tween_gdscript res://examples/tests/delayed_call_validity_runner.tscn
godot --headless --fixed-fps 120 --path spire_tween_rust res://examples/tests/delayed_call_validity_runner.tscn
cargo check --offline --locked -p spire_tween -p spire_tween_rust --features api-4-4
cargo test --offline --locked -p spire_tween --features include_gdext_lib,include_gdscript_bridge
```

The installed Linux projects also pass their nine-case suites after refresh; the native release-library symlink is preserved. Runtime caches/extension lists were prepared directly, and XDG directories
were isolated under `/tmp`. No graphical runs or fresh editor imports were used. Cargo runs eight smoke checks and one compile-fail doctest; `must_compile` runs zero tests and 78 doctests remain
ignored. Rust formatting and `git diff --check` pass.

Validity is checked on processing and forced completion. Never-processed paused/manual tweens are not polled proactively. Unrelated broad-suite failures and other platform artifacts remain outside
this verification. Logs, commands/results, baseline source, and selected source/current artifact hashes are retained here.
