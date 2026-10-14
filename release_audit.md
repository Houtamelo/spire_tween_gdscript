# Commercial release audit — SpireTween 1.4.0

**2026-10-11 current review:** [the second round-trip review](release_audit_round2.md) supersedes the original readiness verdict below. It confirms new sequence time-unit, template lifetime/geometry,
native-bookkeeping, cancellation-state, and startup issues, while preserving the earlier focused repairs. Fresh builds and real-engine evidence are recorded separately from old artifacts and
unresolved distribution decisions.

**2026-10-14 startup decision:** early manager initialization is an accepted startup restriction, not a release blocker. First use requires an installed `SceneTree` main loop with an available root;
failure logs an explanatory error and is not retried. The installation and Rust quickstart documentation state the requirement. See [finding 16](release_audit_round2.md#16-an-early-failed-manager-initialization-disables-subsequent-playback).

**2026-10-14 follow decision:** the follow template intentionally completes on reaching its target, supports continuous following through infinite Restart loops, and chases the last known position
when the target is freed. The incorrect Rust documentation and missing Godot method descriptions are corrected. See [finding 17](release_audit_round2.md#17-follow-behavior-contradicts-its-public-rust-documentation).

**Hold the commercial release.** Normal tweening and a Linux release export work, but the current source and local addon binaries have reproducible ownership, binding, sequence, installation, and
release-packaging defects. This audit examines commit `f518d78` on 2026-10-07. The working tree was clean before the audit.

The findings below distinguish runtime reproductions from source-review concerns and packaging decisions. During the original audit, production source, licenses, and existing tests were left
unchanged. Logs and small reproduction scenes are saved in [release_audit_evidence](release_audit_evidence/).

**2026-10-08 follow-up:** the unconditional wrapper ownership cycle in finding 1, explicit-binding inconsistencies in finding 2, and indirect sequence cycles in finding 3 have been repaired in the
working tree. See [the ownership follow-up](#ownership-follow-up-2026-10-08), [the binding follow-up](#node-binding-follow-up-2026-10-08),
and [the sequence-cycle follow-up](#sequence-cycle-follow-up-2026-10-08). The original audit below remains a record of commit `f518d78`; the other release findings still require separate resolution.

**2026-10-09 API follow-up:** explicitly selecting API 4.4 passes the focused suites on Godot 4.4.1 and 4.7.2 in debug and release. API 4.4 remains opt-in; ordinary builds retain godot-rust's default
API selection. See [the API follow-up](#godot-api-follow-up-2026-10-09). Release API selection, other platform artifacts, and remaining compatibility checks still need resolution.

**2026-10-09 delayed-call follow-up:** finding 6 is repaired for normal processing and forced completion: invalid callables stop and unregister without invocation or completion signals.

**2026-10-14 ownership clarification:** validity checks now only stop and report invalidity. Automatic processors remove tweens reporting `ShouldDespawn`; manual callers and owners of forcibly stopped tweens choose when to unregister or release their handles. Retained stopped handles may remain registered. The earlier 2026-10-09 run below records the superseded helper-driven unregistration policy; [current ownership evidence](release_audit_evidence/callable_retention_2026_10_14/README.md) verifies the final contract.
See [the delayed-call follow-up](#delayed-call-follow-up-2026-10-09).

**2026-10-10 shake follow-up:** finding 7 is repaired in the working tree for the accepted Restart-only template contract; all 28 focused current-source checks pass in debug and release.
See [fixed-source evidence](release_audit_evidence/shake_fixed_2026_10_10/).

**2026-10-10 mechanical follow-up:** the mechanical portions of findings 8–11 are repaired: the missing ellipsis suite and uncached CLI entry, exact project license copies in addon directories,
portable/fail-fast build scripts and corrected Web profile/features, plus factual installation/build/benchmark/Linux-requirement documentation. Actual Linux and Windows cross-builds pass in both
profiles; 88 focused Godot runner checks and eight shell/license checks pass. The full uncached run invokes all 97 tests, with the pre-existing invalid-enum error still present. Android verification
was interrupted by concurrent sequence constructor edits; Web lacks its SDK. [Evidence and remaining boundaries](release_audit_evidence/mechanical_fixes_2026_10_10/). Whole findings 8–11 remain open
for their larger or policy-dependent work.

## Verification performed

| Check                                                                                     | Result                                                                                                                                  |
|-------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| Fresh build of `spire_tween`, both addon features enabled, current lockfile               | Passed                                                                                                                                  |
| `cargo test -p spire_tween --features include_gdext_lib,include_gdscript_bridge --locked` | 8 spiral benchmark functions passed when run by the test harness; compile-only target contains no executable tests; 78 doctests ignored |
| GDScript suite, fresh source binary, all mode, Godot 4.7.2, fixed 60 FPS                  | 49 tests completed; 48 returned true, 1 returned false; 6 additional assertion errors; exit status 0                                    |
| Same suite at fixed 120 FPS                                                               | Timing assertions disappeared; invalid-enum test still failed; exit status 0                                                            |
| GDScript suite with existing addon debug binary at fixed 60 FPS                           | Same invalid-enum failure and timing assertion failures                                                                                 |
| Rust integration harness, freshly built debug binary, fixed 60 FPS                        | Panics, followed by a stalled runner and timeout; not a passing suite                                                                   |
| Rust integration harness, freshly built release binary, fixed 120 FPS                     | 68 tests started, 65 finished; 3 test panics and a freed-Node3D access panic at shutdown; still printed all tests finished and exited 0 |
| Clean minimal addon import, Godot 4.6 and 4.7.2                                           | First import crashes during shutdown; subsequent import succeeds                                                                        |
| Clean addon import, declared minimum Godot 4.4.1                                          | Library rejects the older engine; addon classes unavailable                                                                             |
| Linux release export and execution, matching Godot 4.6 editor/templates                   | Export selected the release `.so`; executable ran and tween reached `(10, 20)`                                                          |
| Ownership, binding, freed callable, nested sequence probes                                | Confirmed defects described below                                                                                                       |
| Native shake completion, exact endpoint versus just-before-endpoint control               | Exact endpoint restores correctly; near-endpoint case restores twice and leaves a permanent offset                                      |
| Tracked-file scan for private-key, AWS access-key, and GitHub token patterns              | No matches; this was a bounded pattern scan, not a comprehensive secret audit                                                           |

All Godot checks used real engines and native extensions, without mocks. Test projects, caches, editor configuration, and exports were isolated under `/tmp/spire-release-audit-3lu0eh8z`. Fixed FPS
accelerates simulated time; the 60/120 FPS comparison makes timing-sensitive limitations explicit. Godot 4.4.1 and 4.6 Linux binaries came from official `godotengine/godot-builds` releases; 4.7.2 was
installed locally. Windows, Android, Web, macOS, iOS, interactive editor reload, and C# integration were not runtime-validated.

## Findings requiring resolution before release

### 1. Every GDScript tween wrapper creates a strong ownership cycle

**Confirmed; affects ordinary repeated use.** In `spire_tween/src/tweens/macros.rs:775`, the Godot wrapper owns an `RcPtr<SpireTween<_>>`. Line 779 then stores a strong clone of that same Godot
wrapper in the native tween's `gd_handle`. The resulting ownership graph is:

```text
Godot RefCounted wrapper -> RcPtr<native tween> -> Gd<same wrapper>
```

Neither reference-counting system can collect that cycle. The manager's completed/stopped cleanup also depends on `strong_count() > 1` (`spire_tween/src/global/mod.rs:35`, `:56`), so forgotten handles
can remain registered as well.

A minimal test warms up the manager, records `Performance.OBJECT_COUNT`, creates and force-completes 1,000 zero-duration sequences, drops every user handle, and waits ten frames. The current binary on
Godot 4.7.2 reported:

```text
LEAK_MEASURE before=1567.0 after=2567.0 delta=1000.0
WARNING: 1001 ObjectDB instances were leaked at exit
```

The existing addon binary on Godot 4.6 independently retained exactly 1,000 additional objects. Full GDScript runs reported thousands of leaked objects. This is more than shutdown noise: object
retention increases during the running process.

**Required outcome:** remove the strong cycle while preserving handles, signals, restart, and registration semantics. Add real Godot tests for natural completion, force completion, stop, unregister,
dropped handles, and native `register_with_gd_handle`; measure object/manager retention after repeated use. A weak signal handle or a carefully designed ownership split needs review; merely clearing
the handle on completion can break replayed signals.

### 2. Explicit node binding is inconsistent and panics in debug builds

**Confirmed.** `spire_tween/src/tweens/base/spire_tween.rs:146` clears only the tween's local node set. It leaves the manager's weak binding entries intact in `global/index_map_impl.rs`. This
reproduction uses a sequence, so automatic property-owner lifetime does not confound the result:

```gdscript
var node := Node.new()
add_child(node)
var seq := Spire.sequence().append_interval(10.0)
seq.bind_node(node)
seq.clear_bound_nodes()
print(seq.get_bound_nodes().size()) # 0
node.queue_free()
await get_tree().process_frame
await get_tree().process_frame
print(seq.is_registered(), seq.is_stopped()) # false, true
```

Freeing the supposedly unbound node still kills the sequence, on both the current binary and the existing addon binary.

Related debug-build failures were reproduced:

| Operation                                        | Observed failure                                | Source                                                         |
|--------------------------------------------------|-------------------------------------------------|----------------------------------------------------------------|
| `bind_node(node)` twice                          | Rust panic                                      | `global/index_map_impl.rs:31`, `tweens/base/ptr_methods.rs:15` |
| Explicitly bind, unregister, then register again | Rust panic; sequence stays unregistered         | `global/index_map_impl.rs:31`, `:118`                          |
| Unbind a node tracked for another tween          | Rust panic despite documented harmless behavior | `global/index_map_impl.rs:42`, `tweens/base/ptr_methods.rs:24` |

The panics are caught at the GDExtension boundary and logged; a process exit code of zero does not make the operations successful. Release builds compile out these assertions, which can mask the
discrepancy between editor and exported-game behavior.

**Required outcome:** synchronize the manager and tween binding records; make duplicate/no-op operations idempotent; preserve explicit bindings across unregister/register. Cover success and failure
cases with debug and release extensions, including completion followed by registration and sequence insertion of explicitly bound children.

### 3. Indirect sequence cycles can crash the engine

**Confirmed; current source binary terminated with SIGSEGV.** `spire_tween/src/tweens/sequence/data.rs:658` rejects a direct self-reference and duplicate immediate children, but does not reject
ancestor/descendant cycles. Two sequences can therefore contain each other. Registering either as a root recursively traverses bindings in `global/index_map_impl.rs:218`:

```gdscript
var a := Spire.sequence()
var b := Spire.sequence()
a.append(b)
b.append(a)
a.register() # process terminated with signal 11
```

The explicit registration matters: adding each sequence to the other unregisters both roots, so a reproduction that stops after the two appends can appear harmless.

**Required outcome:** reject cycles before modifying ownership/registration. Cover two-node and deeper cycles through `append`, `join`, and `insert`, along with valid nested sequences. The existing
direct-self-add test does not cover this case.

### 4. Declared Godot compatibility is false, and bundled binaries are stale

**Confirmed.** `spire_tween_gdscript/addons/spire_tween/spire_tween.gdextension:3` declares `compatibility_minimum = 4.4.1`; the crate description claims Godot 4.4+ and the README says 4.x. The
existing Linux binary is compiled against API 4.6. On a clean Godot 4.4.1 project it rejects initialization:

```text
godot-rust compiled against newer Godot version: v4.6.stable.official
but loaded by older Godot binary, with version:  v4.4.1.stable.official
ERROR: GDExtension initialization function 'gdext_rust_init' returned an error.
```

`spire_tween/Cargo.toml:18` does not select an `api-4-*` feature. Fresh builds currently also target API 4.6, and updating godot-rust can change the default. A permissive manifest cannot lower a
library's compiled API requirement. See [godot-rust compatibility](https://godot-rust.github.io/book/toolchain/compatibility.html)
and [API version selection](https://godot-rust.github.io/book/toolchain/godot-version.html).

The current source exposes `SpireSequence.close_current_block` (`tweens/sequence/gd.rs:205`), introduced by commit `385c46d`. A fresh source binary reports `has_method("close_current_block") == true`;
the existing Linux addon binary reports **false**. It therefore does not represent the current advertised 1.4.0 source surface. Other platform binaries were not assumed current.

**Required outcome:** choose a real minimum API, pin it explicitly, rebuild every distributed artifact from one source revision, align manifest/README/store listing, and independently test every
advertised Godot minor version. Either genuinely support 4.4.1 or narrow the claim to the verified range.

### 5. A fresh addon installation crashes the headless editor on first import

**2026-10-09 upstream identification:** Godot's confirmed issues [#111048](https://github.com/godotengine/godot/issues/111048) and [#111645](https://github.com/godotengine/godot/issues/111645)
describe the same fresh-cache headless GDExtension import/shutdown conditions. Pending engine [PR #123658](https://github.com/godotengine/godot/pull/123658) identifies deferred documentation callbacks
running after `EditorHelp::cleanup_doc()` deletes their documentation data. The corresponding godot-rust report [#1711](https://github.com/godot-rust/gdext/issues/1711), using Godot 4.7.2 and
godot-rust 0.5.5, is classified upstream and records successful ordinary graphical editor sessions. This is a strong match for the audit's crash; exact local stack attribution remains inferred because
the audited engine binary is stripped. Track this entry as a known upstream headless-import limitation. A normal graphical first-install failure was not established by the audit.

**2026-10-09 local graphical comparison:** With the current API 4.6 libraries on Godot 4.7.2, all six fresh graphical imports (three per debug/release profile) exit 0. All six matching fresh headless
imports abort with SIGABRT. Fresh graphical startup and reopening of the full example project also exit 0; those sessions ended before the planned automated close request. The graphical runs use
X11/OpenGL compatibility with Mesa llvmpipe and produce only the same unsupported V-Sync warning as the empty control. [Comparison evidence](release_audit_evidence/nonheadless_import/) records
commands, profiles, hashes, and logs. No further graphical runs were made after the user requested stopping.

**Original audit observation:** A minimal empty Godot 4.7.2 project imports successfully. Adding only the addon manifest and Linux library makes the first `--headless --editor --import` run terminate
with SIGABRT after editor initialization. A second import of the same project succeeds. Fresh source and existing addon binaries both trigger this behavior. Godot 4.6 also crashes on first import.

Under GDB, the Godot 4.7.2 first-import run stopped on SIGSEGV in engine code during shutdown. The installed engine is stripped, so this audit does not attribute the exact fault to addon code or
godot-rust. The one-variable empty-project comparison establishes that adding the extension triggers the crash in this environment.

**Required outcome:** isolate and resolve the crash or establish a supported engine/binding combination with a passing clean import. Verify a real interactive first install and editor exit/reopen as
well. Cached project imports are insufficient release evidence.

### 6. Delayed calls do not honor their documented target lifetime

**2026-10-09 post-ownership recheck:** This remains reproducible with both current debug and release libraries on Godot 4.7.2. The target Node is actually freed and its callable becomes invalid, while
the retained unbound tween reports registered/playing. Adding explicit `bind_node(target)` cancels registration and stops playback even though the caller still holds its wrapper. After explicit
unregister and releasing that wrapper, its WeakRef becomes null in both variants, confirming that the repaired wrapper ownership cycle is not the
cause. [Recheck evidence](release_audit_evidence/delayed_lifetime_recheck/) records the probe, logs, controls, and installed-library hashes.

**Confirmed.** `spire_tween/src/gdscript_bridge/do_delayed_call.rs:19` promises automatic deletion when a callable's object is freed. `tweens/delayed_call/gd.rs:29` registers the callable without
binding its owner or tracking validity. The Rust `DoDelayedCallable` path does bind a node owner, so the two surfaces differ.

```gdscript
var node := Node.new()
add_child(node)
var tween := Spire.do_delayed_call(Callable(node, "queue_free"), 100.0)
node.queue_free()
await get_tree().process_frame
await get_tree().process_frame
print(tween.is_registered(), tween.is_stopped()) # true, false
```

This behavior reproduced with the current source binary on Godot 4.7.2 and the existing addon binary on Godot 4.6.

**Required outcome:** implement the lifetime contract consistently, including non-Node objects where promised, and test valid calls, freed Node/RefCounted targets, anonymous callables, and invalid
callables. Alternatively revise the public contract deliberately; do not leave the documented guarantee false.

### 7. The Rust shake template can finish away from its origin

**2026-10-10 verified repair:** finding 7 is repaired in the current working tree under the accepted Restart-only, caller-managed template contract. Rebuilt current-source probes pass all 28 focused
checks in debug and release on Godot 4.7.2, including restoration, sampling cadence, replay, force completion, external movement, and restored state inside Godot finish handlers.
See [the fixed-source evidence](release_audit_evidence/shake_fixed_2026_10_10/). The earlier observations below are historical; installed addon binaries and the broader suite were not refreshed or
revalidated here.

**2026-10-10 template contract follow-up:** the user defines this as a Restart-only template whose callback connections are caller-managed; support for other loop modes and survival of connection
clearing are not required. Shared clock resets and native-before-Godot finish emission resolve the previous reset/notification findings. The current accumulator expression still retains catch-up debt
because it multiplies by a remainder. An isolated one-line `%=` replacement passes all 28 clarified-contract checks in debug and release; current source reports six cadence failures per profile.
See [the template follow-up evidence](release_audit_evidence/shake_template_followup_2026_10_10/). The production accumulator line was not changed by this review.

**2026-10-10 current implementation review:** the corrected applied-offset bookkeeping and synchronous finish cleanup pass all 44 focused restoration checks in debug and release, including
Restart/Yoyo/Incremental, replay, force completion, and external movement. Remaining findings concern reset/backlogged sampling clocks, Godot completion notification order, and public connection
clearing removing intrinsic cleanup. The 60-case probe reports 48 passes and 12 failures per profile; see [the current review evidence](release_audit_evidence/shake_current_review_2026_10_10/).
Production implementation was not edited by this review.

**2026-10-09 loop-mode verification:** Yoyo and Incremental have additional sampling/endpoint defects. Current-source and isolated two-line-proposal builds confirm decreasing Yoyo values suppress
sampling and completion at zero skips restoration; later Incremental endpoints exceed the original duration and also skip restoration. The proposed bookkeeping resets alone do not resolve these modes.
All 56 real-engine observations are recorded in [the loop-mode evidence](release_audit_evidence/shake_loop_modes/). Production code remains unchanged.

**2026-10-09 cause investigation:** the current working tree reproduces the stale-offset subtraction on Godot 4.7.2. Nine real-engine observations also confirm repeated near-endpoint cleanup, forced
completion after early cleanup, Restart-loop/replay state retention, preservation of external movement plus the erroneous subtraction, and the same defect for Control. The helper retains both
`prev_offset` after removing it and `prev_update` across a new loop/run. This is investigation evidence, not a repair; see [the shake investigation](release_audit_evidence/shake_investigation/).

**Confirmed with a deterministic real-Godot control/variant.** The release Rust suite's shake-restoration test failed. A separate native probe then used the public `DoShakeNode2D` and
`SpireTweener::process` APIs to compare exact completion with completion reached across two nearby ticks:

```rust
let mut tween = node.do_shake(5.0, 25.0, 0.5, 30.0, 1.0);
tween.process(0.5, false);
tween.process(0.499999999, false); // position restored, tween still playing
tween.process(0.01, false);        // restoration subtracts the old offset again
```

The exact control (`0.5 + 0.5`) ended at `(0, 0)`. The near-endpoint variant first returned to `(0, 0)` while still playing, then finished at `(19.850286, 4.196206)`. The random offset varies; the
duplicate-restoration mechanism is deterministic.

`spire_tween/src/tweens/templates/do_shake.rs:154` restores whenever time approximately equals duration, but leaves `prev_offset` unchanged. The method processor can invoke that branch just below
duration and again at the clamped final endpoint (`tweens/method/data.rs:146`). The existing regression only observes eventual completion and does not isolate this two-tick boundary.

**Required outcome:** make restoration idempotent and correct across endpoint rounding, force completion, and loops. Preserve externally applied movement, and add a real Godot regression for both
exact and near-endpoint completion. This finding concerns the public Rust template; a GDScript shake constructor was not found in the bridge.

### 8. The current test runners cannot reliably certify a release

**Confirmed.** The GDScript runner defaults to `FailedOnly` (`examples/tests/test_runner.gd:26`), hashes test source rather than the extension artifact, and unconditionally calls `quit()` without a
failure exit code (`:54`). Passing cached results can survive an addon rebuild.

In explicit all mode, `test_invalid_enum` fails because a fluent call returns null; line 15 then calls a method on null. At fixed 60 FPS, six timing assertions also fail inside helper functions, yet
their calling tests return true and are cached as passing. All 49 tests complete and the process exits zero. Repeating at 120 FPS removes those timing errors but leaves the invalid-enum failure. This
separates a frame-sensitive timing issue from the persistent failure; it does not make the harness reliable.

Five tests in `examples/tests/ellipsis.gd` are missing from the runner's `TESTS` list. The test files contain 54 tests; the advertised all run executes only 49.

The Rust debug harness likewise logs assertion panics and eventually a manager binding panic, then stalls until the audit timeout. Its `type_name_of_val(&test_fn)` reporting
(`spire_tween_rust/src/tests/util.rs:22`) prints a function-pointer type rather than the individual test name, making failures harder to identify. The release harness started 68 tests, finished 65,
logged three test panics and one freed-node panic at shutdown, still printed `=== All tests finished! ===`, and exited zero. The failures concerned shake restoration, ignore-time-scale timing, and a
follow-3D midpoint check. The time-scale check compares simulated time with real clock time, so the fixed-FPS run cannot establish a product time-scale defect; the midpoint failure is also
frame-sensitive. The shake-restoration failure was independently confirmed in finding 7. Cargo's eight successful benchmark smoke checks do not cover these engine behaviors, and the `must_compile`
target runs zero tests.

**Required outcome:** make assertion/async-task failures propagate to a result and nonzero exit, invalidate caches on library changes, provide an uncached release command, include all intended scenes,
enforce per-test/whole-run timeouts, and name tests individually. Add engine tests for findings 1–7 in both debug and release configurations. Resolve timing expectations without simply weakening
assertions.

### 9. Third-party redistribution and binary notices need explicit handling

**Packaging blocker when shipping the examples; confirmed missing notices in the addon-only directory.** Both example projects contain 36 raw JumpButton character sprite sheets. README attribution and
`examples/powerup/sprites/Read_me.txt` preserve the creator's terms. Those terms and the [creator's current asset page](https://jumpbutton.itch.io/girlknightasset) permit credited commercial use and
separately restrict resale/reproduction for profit. Shipping reusable sprite source inside a paid tooling package is a different use from displaying it in a game or showcase; the inspected repository
contains no additional permission resolving that distribution case. This is a release packaging question, not a declaration that existing promotional use infringes anything.

**Required outcome:** omit/replace the third-party sprite sheets in paid/source example downloads, or document permission covering this form of redistribution. Keep attribution and license boundaries
explicit. Repository MIT text should not appear to grant MIT rights to third-party art.

The extracted `addons/spire_tween` directory contains neither the project's MIT license nor dependency notices/source-availability information. Cargo metadata identifies MPL-2.0 godot-rust components
and dependencies with MIT/Apache/Unicode/Zlib terms. The actual binary dependency versions need their own release inventory; the shipped binary and current build use different godot-rust revisions.

**Required outcome:** include the project's license, applicable dependency notices/license texts, and source locations for the MPL-covered components in the actual delivered
archive. [Mozilla's MPL FAQ, Q8](https://www.mozilla.org/en-US/MPL/2.0/FAQ/#q8-i-want-to-distribute-outside-my-organization-executable-programs-or-libraries-that-i-have-compiled-from-someone-elses-unchanged-mpl-licensed-source-code-either-standalone-or-part-of-a-larger-work-what-do-i-have-to-do)
describes the recipient source-information requirement for compiled libraries.

Selling MIT software is allowed; MIT also permits recipients to redistribute and sell copies. That is a business-model decision to confirm when describing what purchase buys, rather than a technical
prohibition. See the repository [LICENSE](LICENSE) and [MIT license text](https://opensource.org/license/mit).

## Release preparation gaps

### 10. Build scripts are not a reproducible release pipeline

The scripts pass shell syntax checks, but the documented `./build_scripts/build_linux.sh` command fails with **permission denied**: all four files are tracked as mode `100644`. Linux/Android/Web
scripts also lack shebangs. None fails immediately when a build fails, so later commands can obscure errors or reuse stale artifacts.

Additional concrete issues:

| File                                    | Issue                                                                                        |
|-----------------------------------------|----------------------------------------------------------------------------------------------|
| `build_scripts/build_wasm.sh:15`        | Space splits `--features include_gdext_lib, include_gdscript_bridge` into separate arguments |
| `build_scripts/build_wasm.sh:27`, `:42` | Builds placed in `release/` omit `--release`                                                 |
| `build_scripts/build_wasm.sh:42`        | Nonthreaded release command omits `include_gdscript_bridge`                                  |
| `build_scripts/build_windows_64.sh:16`  | Hardcoded `/home/houtamelo/xwin` paths; documented Windows execution is not portable         |
| `build_scripts/build_android.sh:10`     | Overwrites `ANDROID_NDK_HOME` with a developer-specific directory                            |
| `build_scripts/build_android.sh:22`     | x86 builds are commented out as nonworking while README marks x86 compiled                   |
| `rust-toolchain.toml:2`                 | Floating `nightly`, despite README describing a pinned toolchain                             |
| `.gitignore:22`                         | `Cargo.lock` excluded from source control; no committed release dependency snapshot          |

The local threaded Web debug/release files are byte-identical; the nonthreaded pair is also byte-identical, consistent with the missing release flag. Web remains explicitly untested/unavailable in the
README, so it can be excluded from this launch, but should not be silently shipped as a working target.

The addon directory currently contains **4,780,803,379 bytes uncompressed**, including `.rlib`/`.pdb` files and huge debug/Web artifacts. No tracked archive assembly, archive smoke test, checksum
manifest, or CI release workflow was found. Merely zipping this directory would ship substantial build debris and omit the licenses/docs outside it.

**Required outcome:** portable executable scripts, fail-fast builds, explicit profiles/features/toolchains, a release lockfile, and an allowlisted archive assembly. Validate exactly the archive sold
to customers, including extracted directory layout, required binaries, version/source revision, docs, licenses, and checksums. Debug extension binaries needed by the editor should be packaged
intentionally, not removed indiscriminately.

### 11. Installation and support documentation need a final reconciliation

`README.md:76` instructs buyers to enable the addon in Project Settings → Plugins, but the addon contains no `plugin.cfg` or EditorPlugin. Its classes register through GDExtension loading. Give buyers
the actual extraction/reimport steps and a small working verification example.

Also update:

- `README.md:69`: placeholder itch.io purchase URL.
- `README.md:108`: nonexistent `spire_tween_plugin` and an incorrect feature-unification explanation. A plain `cargo build --release` selects the default `spire_tween` member with neither
  extension-entry nor GDScript-bridge feature enabled; use the explicit addon build command.
- Engine/platform support claims in findings 4 and 10, with versioned compiled/tested evidence.
- The Linux binary requires GLIBC **2.34**, observed using `readelf --version-info`; publish that requirement or build on an older supported baseline. No older Linux userspace was tested.
- The existing benchmark results identify Godot 4.5.1, whereas the local current addon needs API 4.6. Keep these clearly historical or rerun against the release candidate after fixing ownership.
  Benchmark scenarios are documented; their current numerical claims were not remeasured by this audit.
- Decide whether the source download, Windows/Linux demo, and paid archive are distinct artifacts, with a clear support/update/contact route and version/changelog. MIT rights remain the same unless
  the licensing model is deliberately changed.

Android/Web compilation or a dependency's platform support is not proof of addon runtime/export support. macOS/iOS need not delay a Windows/Linux launch if excluded explicitly from the listing and
release archive.

## Concerns to test after the confirmed defects are fixed

These are source-review concerns or unverified boundaries, rather than additional confirmed crashes:

- **Early manager initialization — accepted restriction, 2026-10-14:** first use requires an installed `SceneTree` with an available root. Failed initialization logs an error and is not retried;
  correct the startup call and restart the application. The requirement is documented. See [finding 16](release_audit_round2.md#16-an-early-failed-manager-initialization-disables-subsequent-playback) for the custom-main-loop constructor reproduction and accepted scope.
- **Reentrancy and aliasing:** synchronous callbacks run while mutable tween/connection data is borrowed (`tweens/base/spire_tween.rs:528`, `:550`); `RcPtr` uses unchecked mutable access. One GDScript
  synchronous restart probe worked, so a blanket claim that all synchronous callbacks fail would be false. Test same-tween callback mutation and native connection-list mutation explicitly; assess Rust
  aliasing separately from a successful runtime probe.
- **Invalid numeric inputs:** duration, delay, manual delta, and speed-scale setters generally accept negative/nonfinite values. Decide rejection/fallback semantics for `NaN`, infinity, negative
  speed, and zero-duration infinite loops; verify errors do not poison registrations or hang callers. Negative speed currently decreases elapsed time and can stall before the delay indefinitely.
- **Manager bookkeeping growth:** tracked node entries and old weak tween entries have no evident general pruning path (`global/index_map_impl.rs`). Profile repeated node/tween creation after fixing
  the confirmed strong cycle and stale explicit bindings.
- **Hot reload/editor lifetime:** the manifest is `reloadable=true`, but no explicit manager reset/unload protocol was found. Test reload, editor restart, scene changes, and reuse of node/controller
  names before advertising reload support.
- **Generated surface:** 56 class adapters are generated from `tweenable_properties.json`; Cargo validates their static engine calls, but current integration scenarios do not exhaust all property
  types, easing/templates, invalid inputs, or language integrations.

## Release acceptance sequence

1. Repair wrapper ownership, binding consistency, cycle detection, delayed-call lifetime, and shake restoration; add real engine regressions for success and failure paths.
2. Make the test runners fail reliably, include all intended tests, and obtain passing uncached debug/release runs.
3. Establish a supported API/engine combination and pass first import, editor reopen, ordinary tweening, and export on that combination.
4. Rebuild every advertised platform from the same tagged source and recorded dependencies; compare public method surfaces and artifact hashes.
5. Assemble lean demo/paid archives with correct docs, licenses/notices, and permitted example assets. Extract each into a clean project and run the install/editor/runtime/export checks.
6. Publish the verified engine/platform matrix, actual itch.io URL, purchase contents, and support/update terms.

The successful Linux export is evidence that the normal runtime/export path works. It does not remove the confirmed lifetime and installation defects or validate the other advertised platforms.

## Ownership follow-up 2026-10-08

The working tree now uses `TweenPtr<T>` / `WeakTweenPtr<T>` for tween ownership. `T` is the inner data (`Sequence`, `LerpPropertyData<V>`, etc.). Generic `RcPtr<T>` / `WeakPtr<T>` remain unchanged for
shared helper state.

The wrapper owns a native tween pointer. Its native back-reference is strong while another native owner exists, and weak when the wrapper is the sole native owner. Clone and weak upgrade promote the
handle on `1 → 2`; Drop demotes it for the effective `2 → 1` transition. A separate shared handle-control slot lets these operations run without borrowing an already-processing native payload, and
releases the strong handle only after installing the weak state. No Godot reference-count notifications or generic pointer callbacks are used.

Manager cleanup discounts the wrapper's internal native reference, while retaining caller-held native/Godot handles for restart. A synchronous finished callback that restarts playback also keeps its
registration. A separate emitter supplied through `with_handle()` is treated as an external emitter rather than a wrapper owning that native tween.

Real Godot regressions cover native counts, clone/weak upgrade, expired weak observers, both orders of releasing native and Godot references, forgotten-handle signals, completion/replay, sequence
child loops, cancellation, pause/manual ownership, all four tween categories, and pointer upgrade during a synchronous signal callback.

| Verification                                      | Outcome                                                                                                                                                    |
|---------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Original source binary, 1,000 completed sequences | 1,000 wrapper instance IDs remain live; ownership regression runner exits 1                                                                                |
| Candidate, same batch                             | **0 live wrapper instance IDs**                                                                                                                            |
| GDScript ownership regressions                    | 15/15 pass with debug and release libraries; fixed 60/120 FPS and uncapped debug run checked                                                               |
| Native ownership regressions                      | 10/10 pass with debug and release harnesses                                                                                                                |
| Cargo feature checks                              | Default, extension entry only, bridge only, and both features pass                                                                                         |
| Cargo tests                                       | Eight benchmark smoke checks and the compile-fail test rejecting `TweenPtr<SpireTween<Sequence>>` pass; the compile-only target still has no runtime tests |
| Existing full GDScript suite                      | Existing invalid-enum failure remains; no additional assertion/panic failures in the checked fixed-120-FPS run; shutdown leak warnings are gone            |
| Existing full native release suite                | Existing shake, fixed-FPS time-scale, follow-3D midpoint, and freed-node shutdown failures remain; ownership cases pass                                    |

The batch's overall `Performance.OBJECT_COUNT` delta includes one additional object in this test fixture. The regression checks every created wrapper's instance ID directly, establishing that none of
the 1,000 tween wrappers survives. It does not attribute the remaining unrelated object count to a particular engine object.

The local Linux addon debug/release libraries and native release harness library have been rebuilt. Other platform binaries have not been rebuilt or validated for this change. Source-level Rust
consumers now use `TweenPtr<T>` instead of `RcPtr<SpireTween<T>>`; the Godot/GDScript API remains the same. The former public `gd_handle` storage is encapsulated; `get_gd_handle()` provides access to
a live signal handle.

The shared handle-control cell adds one allocation per tween and a second pointer field to `TweenPtr`. Runtime/setup performance benchmarks have not been rerun; updating the benchmark evidence remains
part of the release preparation described above. The rebuilt installed Linux libraries also pass the focused suites, and the GDScript ownership cases pass on Godot 4.6.

Logs and installed Linux binary hashes are in [release_audit_evidence/tween_ptr](release_audit_evidence/tween_ptr/). These results resolve the unconditional ownership cycle; they do not establish
general cycle collection for user-created callback/sequence cycles, or resolve the other commercial-release blockers.

## Node binding follow-up 2026-10-08

Finding 2 is repaired using the existing binding storage. `clear_bound_nodes()` removes the manager's reverse weak entries before clearing the tween's set, using node IDs so previously freed nodes can
be cleared. It leaves the manager uninitialized when clearing an unregistered builder. Manager bind/unbind operations accept existing/absent membership, so repeated binding, re-registration, and
sequence insertion no longer trip debug assertions. GDScript bind/unbind now delegate to the native methods.

The pre-fix debug addon reproduced seven failed GDScript cases and seven manager assertion panics. The expanded focused regressions pass 13/13 GDScript and 11/11 native cases on debug and release
Linux libraries. They cover synchronized clearing, harmless duplicate/no-op operations, multiple nodes/tweens, unregister/register, completion and replay, bound sequence children, freed-node clearing,
automatic owner lifetime, pause behavior, builder bindings, and node-driven force-complete/kill. The earlier 15 GDScript and 10 native ownership regressions also pass.

The broad GDScript suite completes 77 cases with the existing invalid-enum script error. The native all-tests scene now instantiates its Rust `TestRunner` instead of a plain `Node`; its focused suites
pass, then the older tests reproduce the previously recorded shake, time-scale, follow-3D, and freed-node errors. Its reporting limitations remain unresolved.

The actual project Linux addon libraries have been refreshed and checked; other platform binaries remain outside this verification. [Binding evidence](release_audit_evidence/node_binding/) contains
commands, output gates, build results, and binary hashes. A separate existing insert-only sequence completion defect is recorded
in [this report](release_audit_evidence/node_binding/sequence_insert_lifetime.md); sequence playback was left unchanged by the binding fix.

## Sequence cycle follow-up 2026-10-08

Finding 3 is repaired through the supported insertion and registration APIs. The user's incremental `SequenceIter` keeps pending work separately from visited tweens and reports terminal
forbidden/repeated-reference errors. Append, join, and insert validate the incoming sequence against the receiving parent's native address before changing playback, ease, registration, or bindings.
Invalid additions emit a warning and leave the existing hierarchy usable. Manager binding traversal uses two iterative passes, without collecting the whole graph. Failed registration warns and
returns; it does not unregister an existing root. The hidden public storage carries explicit warnings about caller-managed invariants.

The original two-sequence reproduction crashes the baseline library with SIGSEGV and succeeds with the integrated library after logging the rejected cycle. The focused suite passes 6/6 GDScript and
10/10 native cases in debug and release. It covers two-node/deeper cycles through all three insertion methods, rejected-child invariants, repeated graphs, manager refusal, terminal iterator errors,
heap-spill behavior, valid nesting, playback, and replay. The 49 existing focused binding/ownership cases also pass in both profiles. Actual installed Linux library paths are checked and refreshed;
the native symlink is preserved.

[Sequence-cycle evidence](release_audit_evidence/sequence_cycles/) records the baseline failures, expected warning counts, build/profile/installed-path gates, and artifact hashes. These focused checks
do not resolve the separate insert-only playback defect, prior broader-suite failures, installation issues, compatibility declarations, or other release findings.

## Godot API follow-up 2026-10-09

The API 4.4 compatibility experiment succeeds. Both crates expose the opt-in `api-4-4` feature; it forwards to `godot/api-4-4`, and the native crate also forwards to the core crate. Nullable timer and
built-in tween results use `#[cfg(feature = "api-4-4")]` unwraps; the feature-disabled path retains the newer bindings' nonnullable types. The initial default-feature selection was removed at the
user's request: this experiment does not change ordinary builds' API selection or establish the release API policy.

The core addon and native demo/test libraries compile in debug and release against API 4.4. All 70 focused cases pass on both Godot 4.4.1 and 4.7.2 in both profiles: 65 existing
ownership/binding/cycle cases and five new timer/built-in benchmark cases. The new suite includes real Godot refusal of position/color tweeners after playback starts, with the resulting expected
panics caught and checked. Its five cases also pass with `api-4-4` disabled, using API 4.6 on Godot 4.7.2. Cargo's eight benchmark smoke checks and one compile-fail doctest pass.

The broader GDScript suite starts and finishes 83 cases on each tested engine; `test_invalid_enum` still fails with a null-call script error. Each broader native release run starts 68 older cases and
finishes 65, after passing the 36 focused cases. Shake restoration, the fixed-FPS real-clock time-scale assertion, the follow-3D midpoint assertion, and freed-node access at shutdown reproduce the
prior findings. Both older runners still exit zero despite failures, so these runs do not certify a release.

The experiment refreshed Linux libraries and preserved the native project's release-library symlink; all 70 focused cases passed through those installed API 4.4 libraries on Godot 4.4.1. The installed
libraries are subsequently restored to ordinary builds using godot-rust's default API. [API 4.4 evidence](release_audit_evidence/api_4_4/) records the experimental build commands, logs, expected
failure handling, historical source/artifact hashes, and broader-suite failures. Other platform binaries, Godot 4.5/4.6 runtime checks for the API 4.4 libraries, clean first import, and export remain
unverified by this follow-up; finding 4 is not fully closed.

After removing the default-feature selection, explicit API 4.4 checks still pass. Ordinary debug/release builds use the locked godot-rust version's API 4.6, and all 70 focused cases pass on Godot
4.7.2 in both profiles. The installed Linux libraries are restored and checked through their project paths, with the native symlink
preserved. [Restoration evidence](release_audit_evidence/api_4_4/default_api_restore/) records the feature graph, build/test results, and current artifact hashes.

## Delayed-call follow-up 2026-10-09

Delayed-call processing checks `Callable::is_valid()` before advancing the delay or invoking the callback. Forced completion checks the same predicate before seeking the end. Invalid calls stop,
enqueue manager unregistration, and return without callback invocation or completion signals. Processing returns `NoLongerValid`, allowing parent sequences to remove invalid children and continue. A
small internal manager entry point accepts the native allocation address, so cancellation also works through a bare native pointer or an unregistered builder without a Godot wrapper.

The real-engine baseline GDScript suite failed six cancellation cases and passed its three valid-call controls. The candidate passes all nine GDScript and nine native cases in both debug and release
on Godot 4.7.2. Coverage includes freed Node/RefCounted targets, empty/missing-method callables, active/paused forced completion, direct manual processing, anonymous/custom callables, delays, valid
completion events, replay, loop target invalidation, differing Rust receiver/callable targets, and sequence continuation. The 70 previous focused ownership/binding/cycle/API cases also pass in both
profiles, for 88 focused cases. Explicit opt-in API 4.4 compilation and Cargo's eight smoke checks plus one compile-fail doctest pass.

Linux libraries are refreshed, the native symlink is preserved, and both nine-case suites pass through the actual installed project
paths. [Delayed-call evidence](release_audit_evidence/delayed_call_validity/) records the baseline, profile/installed checks, commands, hashes, and expected results. All engine checks were headless
runtime checks; no graphical launches or fresh editor imports were performed.

Validity is evaluated when the tween processes or is forced to complete. A paused/manual tween that is never processed or resumed is not proactively polled by this change. The documentation now states
that timing explicitly. The prior unrelated broad-suite failures and other platform artifacts were not revalidated or resolved here.
