# Sequence cycle integration verification

The baseline source library terminated with SIGSEGV (exit -11) on the original two-sequence reproduction. The integrated library rejects the second append with a console warning, registers the remaining valid hierarchy, and exits successfully without unexpected warnings or leaks. The focused baseline tests reported 4/6 failed GDScript cases and 5/9 failed native cases; the iterator termination controls already passed before integration.

The final focused suites run inside real Godot 4.7.2 SceneTrees against API 4.6 libraries:

```sh
godot --headless --path spire_tween_gdscript --fixed-fps 120 res://examples/tests/sequence_cycles_runner.tscn
godot --headless --path spire_tween_rust --fixed-fps 120 res://examples/tests/sequence_cycles_runner.tscn
```

Expected markers are `CYCLE_RESULT total=6 failed=0` and `NATIVE_CYCLE_RESULT total=10 failed=0`. The deliberate invalid cases produce eight GDScript and nine native warnings, each beginning with `[SpireTween] Sequence`. Reject script errors, Rust panic reports, unexpected warnings, and ObjectDB leak warnings; an exit code alone is insufficient.

Coverage includes two-node and deeper cycles through append/join/insert; unchanged state, ease, registration, binding, and completion callbacks after rejection; repeated incoming graphs; refused registration of a repeated graph; terminal forbidden/repeated iterator errors; repeated traversal after the visited set spills; valid nested playback, replay, automatic property owners, and wide/deep graphs beyond inline capacities. Malformed unregistered iterator fixtures deliberately edit hidden storage only to exercise traversal errors; they do not establish support for manual field mutation.

The 13 GDScript/11 native binding cases and 15 GDScript/10 native ownership cases also pass on both debug and release libraries. The actual addon paths are checked after installation. GDScript release-profile checks put the release binary at an isolated debug manifest entry; the normal editor executable selects that entry. The native manifest uses a release-named path for both profiles, and each isolated run installs the intended binary there. The real native project symlink is preserved.

Implementation keeps the incremental SmallVec/SmolSet traversal. Insertion validates the proposed child against its receiving parent's address before mutation. Manager traversal validates before applying bindings, then restarts the iterator and binds shallowly without recursive calls or a full-graph snapshot. Restart retains the pending stack's capacity; smolset 1.3.1's `clear()` resets a spilled visited set to inline storage, so large graphs can allocate visited-set storage again on the second pass. Failed registration warns and returns without removing an existing registration. Direct edits of hidden queue/inserts remain the caller's responsibility.

Files include baseline/candidate probe logs, focused profile and installed-path logs, build logs, Cargo results, process validation JSON, and installed binary/source hashes. `baseline-data.rs` and `baseline-index_map_impl.rs` preserve the reviewed implementation before integration. Other platform binaries and broader-suite release blockers remain outside this focused verification.
