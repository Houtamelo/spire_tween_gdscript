# GDScript sequence constructors — 2026-10-11

The bridge exposes `append_ctor(Callable)`, `join_ctor(Callable)`, and
`insert_ctor(float, Callable)` as chainable `SpireSequence` methods. They delegate
to the existing native Callable constructor methods; the sequence processor and
reverse-index pruning algorithm are unchanged.

Documentation covers all six native constructor methods, the constructor type and
new item variants, ordinary Rust callback methods, relevant pointer methods, and
the three registered Godot methods. Both READMEs include the constructor contract
and examples: activation time, one-time invocation, loops/replay, forced completion,
adoption/registration/easing/bindings, invalid results, and synchronous mutation and
capture ownership restrictions.

## Verification

- All **18** focused real-Godot cases pass with debug and release addon libraries:
  **36 passing cases**, no unexpected script errors or native panics. Deliberately
  invalid fixtures emit expected diagnostics.
- Both freshly rebuilt Linux libraries are installed in the actual addon paths.
  The same 18-case suite passes through fixture symlinks to each installed artifact;
  `installed-results.json` records the paths and hashes. Backups remain in `/tmp`.
- Coverage includes method signatures/chaining, lazy/current-state construction,
  join/empty join, insertion delay/clipping/per-update deltas, loops/replay, force
  completion, nil/non-tween results, invalid/freed-target Callables, duplicate/self/
  ancestor rejection, late default easing/explicit override, bindings/deletion,
  bound arguments/signals/removal, unchanged bulk callback semantics, and pending
  weak-parent-capture release.
- An intentional failed check exits 1. An intentionally stalled async test exits 1
  via the focused runner's watchdog. These checks use modified **copied fixtures**;
  production tests remain intact. The runner accumulates failed Boolean checks
  rather than relying on the old assertion helpers.
- The expanded uncached aggregate runner starts and finishes **115** tests,
  including all 18 constructor cases. The old invalid-enum script error remains;
  exit 0 is still not a full-suite success claim.
- The extracted README example runs and finishes at position `(100, 0)`, scale
  `(2, 2)`, red modulation. All 11 runner catalog/CLI checks pass.
- Workspace/all-target compilation, opt-in API 4.4 compilation, eight Cargo smoke
  checks and the pointer compile-fail doctest pass. API 4.4 remains opt-in; runtime
  checks here use default API 4.6 on Godot 4.7.2.
- The native Linux debug manifest now selects its debug path. Native libraries are
  also rebuilt in both profiles, preserving the project's existing symlinks. Fresh
  real engine loads pass all 45 focused native cases in each profile (90 cases);
  the debug load reports strict safeguards. Its older
  suite still reports the known clock/midpoint/freed-owner failures; these remain
  separate release findings.
- Rustdoc generation completes. Repository-wide Godot-markup/non-Rust-fence
  warnings remain; this is not a warning-free documentation build. The newly added
  native constructor and pointer documentation had no diagnostic locations in the
  checked run. See `rustdoc-summary.json`.

All engine runs are headless runtime checks using copied existing caches. No editor
import, graphical run, other-platform runtime, or publishing action was performed.
The [fresh release review](../../release_audit_round2.md) remains the readiness
assessment; exposing constructors does not repair its independent runtime issues.

## Reuse

```sh
./build_scripts/build_linux.sh
godot --headless --path spire_tween_gdscript --script res://examples/tests/sequence_ctors_runner.gd
```

The focused suite is also included in `test_runner.gd`'s scene catalog. Runtime
commands/results and artifact hashes are in the JSON files, full engine logs are
adjacent, and `source-hashes.json` identifies the final reviewed implementation.
