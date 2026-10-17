# Commercial release review — 2026-10-11

**The current candidate still needs runtime fixes and a verified distribution before sale.** This review starts from `cc35c19` (SpireTween 1.4.0). The earlier wrapper-cycle, explicit-binding,
supported-API sequence-cycle, delayed-call, and Restart-only shake repairs remain in place. The remaining problems below come from current source and fresh real-engine checks.

Evidence is in [roundtrip_2026_10_11](release_audit_evidence/roundtrip_2026_10_11/). Source references below identify the reviewed commit, before the separately requested constructor
bridge/documentation work. No production runtime source or installed libraries were changed during this review.

## Current verification

| Check                                                                       | Result                                                                                                                           |
|-----------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
| Workspace/all-target compilation with existing lockfile                     | Pass; six existing native-demo warnings                                                                                          |
| Core Cargo tests, addon features                                            | Eight benchmark smoke tests and one compile-fail doctest pass; compile-only target has zero runtime tests; 78 doctests ignored   |
| Persistent Bash/license checks                                              | 8/8 pass                                                                                                                         |
| Fresh debug and release GDScript suites, all tests, uncached, fixed 120 FPS | Both start/finish 97 cases; both log the invalid-enum script error and exit 0                                                    |
| Fresh debug native harness, fixed 120 FPS                                   | All 45 focused lifecycle/API/binding/cycle cases pass; older suite starts 68 and finishes 66, then exits 0 despite failures      |
| Deterministic lifecycle/timing probes                                       | Confirm findings 12, 15, and 16 in both profiles; ordinary valid method tween and immediately freed property-owner controls work |
| Ellipse probes                                                              | Confirm freed-owner panics and equal-angle NaN positions in both profiles                                                        |
| Manager-count probe                                                         | Confirms growing dead-node and expired-binding records; observation-only instrumentation in a copied crate                       |
| Installed Linux debug binary versus fresh source                            | Installed binary completes an insert-only parent before its future child starts; fresh source keeps it playing                   |

Godot is **4.7.2**, godot-rust **0.5.5**, ordinary API **4.6**, Rust **1.99.0-nightly (2026-08-10)**. Only headless runtime checks were run. Cached projects and libraries were copied into isolated
`/tmp` fixtures; no editor import or graphical run was performed. A real-time capped full GDScript run exceeded its 240-second budget; it is recorded as incomplete, and the fixed-FPS runs above are
separate completed runs. Fixed FPS is unsuitable for certifying wall-clock/unscaled-time assertions.

The native fixture initially lacked the library selected by its manifest. Its `linux.debug` entry points to the **release** path. The completed native run deliberately places the freshly built debug
library at that selected path and records its hash; initialization reports strict safeguards. This is actual debug-library evidence, despite the filename directory.

## New confirmed findings

### 12. Child speed scaling passes excess time in the wrong units

**High priority; ordinary supported sequence behavior.** [sequence/data.rs:443](spire_tween/src/tweens/sequence/data.rs) passes the child's `Completed.excess_time` directly to subsequent blocks. The
child multiplied its input by its own speed scale in [base/spire_tween.rs:506](spire_tween/src/tweens/base/spire_tween.rs), so its reported excess is in child-scaled seconds, while the next block
expects parent seconds.

For a one-second first child followed by an ordinary second child:

| First-child speed | Parent step | Expected second-child elapsed | Actual   |
|-------------------|-------------|-------------------------------|----------|
| 1                 | 1.1         | 0.1                           | 0.1      |
| 2                 | 0.6         | 0.1                           | **0.2**  |
| 0.5               | 2.1         | 0.1                           | **0.05** |

Both addon profiles reproduce this. Normalize returned excess at the parent/child boundary and cover faster/slower children, parallel blocks, inserted children, nesting, and parent scaling.
Zero/invalid speed needs an explicit boundary rule when designing that conversion.

**Resolved in the current working tree on 2026-10-12.** All four tween implementations now report excess in their caller's input units after scaled loop bookkeeping. Speeds with magnitude below
`0.00001` deliberately report zero excess. The contract is documented, and nine tests registered in the regular GDScript catalog cover 121 parameter cases, passing in both debug and release with the
refreshed installed Linux libraries. The focused runner also rejects the previous implementation and exits nonzero for failed checks and timeouts. See
the [integration evidence](release_audit_evidence/child_speed_excess_review_2026_10_12/integration.md).

### 13. Ellipse/circle templates have lifetime and geometry defects

**High priority for lifetime; medium priority for geometry.** These defects are independently reproducible through the GDScript API:

- **Freed owners:** [do_ellipsis.rs:45](spire_tween/src/tweens/templates/do_ellipsis.rs) and [do_ellipsis_3d.rs:44](spire_tween/src/tweens/templates/do_ellipsis_3d.rs) capture a Node in a Rust custom
  Callable. Method construction derives its lifetime owner from `callable.object()`, which is absent for these custom callables. After `queue_free()`, both tweens remain registered/playing and
  repeatedly panic when the closure writes the freed Node. The engine catches the panic; the tween is still live. This explains the native suite's freed-Node3D shutdown error, and affects both 2D and
  3D ordinary gameplay.
- **Equal angles:** both templates calculate `(theta - from_angle) / (to_angle - from_angle)`. An allowed `0 → 0` angle with radii `1 → 2` produces `(nan, nan)` in 2D. There is no documented rejection
  or fallback. Use a time/progress parameter or define and validate equal-angle semantics; do not let the result corrupt a transform.
- **3D circle:** [generated/node_3_d.rs:398](spire_tween/src/gdscript_bridge/generated/node_3_d.rs) forwards `Vector3::splat(radius)` to the ellipse, including its axial-displacement component. A
  radius-2 circle around `UP`, centered at zero, ends at **`(2, 2, 0)`**, rather than returning to its starting plane. The third radius component is documented as helical displacement. Correct the
  generator/template source as well as the generated adapter.

Bind the template's receiver explicitly, verify ordinary completion and both immediate/deferred owner deletion, and add deterministic geometry probes that do not depend on asynchronous frame midpoint
sampling.

**Resolved for the audited playback and geometry cases in the current working tree on 2026-10-12.** Ellipses animate normalized progress, so equal angles still interpolate radii. Both templates bind
the receiver and set it as the method owner: binding alone fixed standalone deletion, but immediate deletion of a sequence child's receiver still panicked until its owner check was enabled. The 3D
circle generator and generated wrapper now pass `(radius, radius, 0)` at both radius endpoints. Eleven registered integration tests cover 119 parameter cases, passing with installed debug and release
Linux libraries and no native panics. Explicit helical ellipse motion remains supported. See the [verification evidence](release_audit_evidence/ellipsis_regressions_2026_10_12/README.md).
Invalid-method forced-completion semantics remain the separate finding #15.

**Sequence child-binding follow-up resolved on 2026-10-13:** `process_tween` now validates bindings before the stopped-state return. Dead bindings remove children even when stopped; live unprocessable
bindings pause active children while completed children can pass. Six new parameterized binding tests cover 192 configurations across tween types, placements, nesting, states, detach/re-entry, pause
modes, and lazy construction. Testing reproduced the companion manual-step bug: `custom_step` passed `true` for tree-paused despite its documented unpaused override. It now passes `false`. All seven
focused GDScript suites pass in both addon profiles (87 test methods per profile); see the [verification evidence](release_audit_evidence/sequence_child_bindings_2026_10_13/README.md). Broader direct
Rust/manual-root binding validation remains outside this sequence-specific fix.

**Checked stepping follow-up on 2026-10-13:** the manager, sequences, and native/GDScript manual callers now share `TweenPtr::custom_step`. Explicit binding lifetime validation also runs for paused/stopped manual roots. Exported step-result variants distinguish playback state, temporary binding restrictions, and cancellation; retention remains a caller decision. GDScript reports completed/stopped/cancelled tweens as not wanting to live, including stopped tweens with disabled/detached live bindings. The contract is documented in rustdoc, Godot method docs, and both READMEs. Four additional registered integration tests cover 42 standalone return-value configurations. All seven focused suites pass in both profiles (91 test methods per profile), and the refreshed installed Linux libraries pass the 23-method binding suite. See [checked-stepping evidence](release_audit_evidence/checked_stepping_2026_10_13/README.md). Findings #14–17 and the broader release/distribution decisions remain open.

### 14. Manager bookkeeping grows after objects and tweens are released

**Medium priority; accumulating native memory during long sessions.** [global/tweens_map.rs](spire_tween/src/global/tweens_map.rs) adds node entries permanently; the exit callback marks
entries dead but does not remove them. Living nodes also retain expired `WeakAnyTween` entries in `bound_tweens` after completed handles disappear. Weak references keep their Rust allocation control
blocks allocated.

A copied crate with an additional observation-only class reports:

| Stage                                       | Tracked nodes | Dead records | Binding weak entries | Root tweens |
|---------------------------------------------|---------------|--------------|----------------------|-------------|
| Warm                                        | 0             | 0            | 0                    | 0           |
| 1,000 nodes freed                           | 1,000         | 1,000        | 0                    | 0           |
| Another 1,000 nodes freed                   | 2,000         | 2,000        | 0                    | 0           |
| 1,000 completed bindings on one living node | 2,001         | 2,000        | 1,000                | 0           |
| Another 1,000 completed bindings            | 2,001         | 2,000        | 2,000                | 0           |

Godot's object count returns to baseline after the node batches. These are retained native records, separate from the repaired wrapper cycle. Freeing the bound node clears its weak entries, but leaves
its dead node record. Introduce bounded pruning that preserves bindings for still-live handles and node re-entry semantics; verify repeated batches plateau.

**2026-10-13 pruning review: partially resolved.** The new metadata lifetime tracker and two-loop pruning return ordinary freed-node records to baseline after three 1,000-node batches for each direct/queued and inside/outside-tree deletion combination. The new native integration suite also verifies the two-loop barrier, retained live bindings, re-entry, and cancellation across idle/physics/manual and playing/paused/stopped modes. Two gaps remain: expired binding weak references on one living or detached node still accumulate to 1,000, 2,000, and 3,000; and `Node.duplicate()` shares the metadata tracker, delaying its deletion notification after the original node is freed. The original node record and its manual tween registration remain until the duplicate is freed. The new suite reports 8/11 passing in both profiles and exits nonzero for these three failing cases. See [review evidence](release_audit_evidence/bookkeeping_review_2026_10_13/README.md). This finding remains open.

**2026-10-13 follow-up: runtime regressions resolved.** The author now prunes expired weak bindings and holds the lifetime tracker in a non-persistent signal closure rather than metadata. All 11 bookkeeping methods pass in debug and release; the duplication method covers both deletion orders for sources inside/outside the tree. The 45 existing focused native and 91 focused GDScript methods also pass per profile. Editor execution remains outside this verification: godot-rust's editor connection registry retains Rust callable clones, which can delay release of a closure-captured tracker. See [follow-up evidence](release_audit_evidence/bookkeeping_followup_2026_10_13/README.md).

### 15. Invalid method tween cancellation leaves misleading state

**Medium priority; violates the documented auto-stop behavior.** [method/data.rs:126](spire_tween/src/tweens/method/data.rs) returns `AdvanceTimeResult::ShouldDespawn` for an invalid Callable without stopping the tween.
A freed method target yields `is_registered() == false` while `is_playing() == true` and `is_stopped() == false`, in both profiles.

Forced completion is also inconsistent with delayed calls: `seek_end()` skips an invalid Callable, but `force_complete()` still emits `finished`. The probe observes one finish signal. Natural
cancellation should stop as documented; decide whether forced invalid-target completion should follow delayed-call cancellation semantics or retain an explicitly documented distinct contract.

**2026-10-14 follow-up: resolved.** The chosen contract keeps stopping separate from finishing and leaves removal to the owning processor. Method/delayed-call validity guards stop invalid callables without invocation, completion events, or manager mutation. Automatic processing consumes `ShouldDespawn` and removes invalid tweens. Manual stepping and forced stopping preserve registration while external handles remain; owners may unregister or release those handles. Real-Godot tests cover retained stopped handles, collection after the final owner drops, idle/physics/manual roots, playing/paused forced stopping, raw builders without manager side effects, custom interpolation, sequence children, and valid completion/replay. All focused suites pass: 66 native and 98 GDScript methods per profile. See the [initial method-validity evidence](release_audit_evidence/method_validity_2026_10_13/README.md) and [ownership follow-up](release_audit_evidence/callable_retention_2026_10_14/README.md).

### 16. An early failed manager initialization disables subsequent playback

**2026-10-14 decision: accepted startup restriction; not a release blocker.** SpireTween explicitly requires an installed `SceneTree` main loop with an available root before the manager's first use.
Initialization errors explain an unavailable tree or unsupported main-loop type and expose incorrect startup ordering during testing. The installation and Rust quickstart documentation now state this
requirement and the absence of automatic retry. No runtime change is required under the accepted contract.

**Original reproduction; restricted startup phase, permanent effect.** [global/tweens_map.rs](spire_tween/src/global/tweens_map.rs) logs failure from `try_init_singleton()` but still commits the
`LazyLock` value. There is no retry.

A `SceneTree` script creates a delayed call in a member initializer, before `Engine.get_main_loop()` is set. Later, after the tree is running, a second ordinary delayed call still never fires: both
call counts remain zero, registration reports true, and the root has no controller. Both profiles reproduce this.

This reproduction calls Spire while the custom `SceneTree` itself is being constructed, before Godot installs it as the main loop. It exercises unsupported first use, rather than an ordinary scene
or autoload startup defect. A deferred first-use control works in the other probes. After an initialization error, the user must correct the startup call and restart the application.

### 17. Follow behavior contradicts its public Rust documentation

**2026-10-14 decision: resolved by documentation correction.** The intended template completes once it reaches the target; continuous following requires `set_loops(-1, Restart)`. A freed target
deliberately leaves the tween chasing its last known position with the configured loop setting. Rust trait docs, generated Godot method docs and their generator, and both READMEs now state this
contract. Runtime behavior is unchanged.

**Original documentation mismatch.** [do_follow.rs:27](spire_tween/src/tweens/templates/do_follow.rs) incorrectly promised continuous following until stopped or the target was freed. Its builder
keeps the normal one-loop limit: the audit's follower one unit from its target, speed 1, stopped after a 1.1-second step with `get_loops() == 1`. Moving the target afterward does not automatically
resume that completed tween. The last-known-position fallback also contradicted the immediate auto-stop claim. Both behaviors are intended, and the 3D implementation uses the same construction pattern.

## Earlier release items still open

### 8. Test success still cannot certify a release

**2026-10-14 follow-up: audited harness defects resolved.** GDScript assertion helpers record failures independently of returned Booleans; ordinary tests use these helpers rather than built-in `assert`.
Aggregate runners report failures and exit nonzero. Native cases have individual names and explicit setup/async/callback assertion outcomes. Per-test wall-clock deadlines and always-processing,
time-scale-independent suite watchdogs bound execution, including paused-tree cases. GDScript cache identity includes the selected addon binary and engine executable hashes, engine version, shared
scripts/scenes, project/extension configuration, and clock settings; old or mismatching cache data is rejected, and `--no-cache` bypasses reads and writes.

All 70 real-Godot runner regression checks pass across debug/release. Complete suites pass per profile: 134 native cases (66 focused plus 68 other cases) with real time, and 152 GDScript cases with
fixed FPS. The invalid-enum test now expects rejected calls and verifies the original valid tween remains usable; expected binding/script diagnostics are delimited. The 3D ellipse geometry test uses
exact manual steps and a tighter spatial tolerance rather than delayed timer samples. Installed-project positive/negative controls also pass. See [runner evidence](release_audit_evidence/test_runners_2026_10_14/README.md).
The GDScript aggregate still reports shutdown object/resource warnings; their cause is not attributed to the addon and is recorded for separate investigation in [shutdown warnings](release_audit_evidence/test_runners_2026_10_14/shutdown_warnings.md).

**Original audit observation.** The aggregate GDScript runner ignored Boolean results when choosing its exit code. More seriously, a fresh real-Godot probe calls the actual `TestBase.assert_eq(1, 2)` helper, logs an
assertion failure, then **returns success and exits 0**. The assertion aborts only the helper, allowing the test body to reach `return true`. Specialized runners counting returned Booleans do not
solve this by themselves.

The full debug/release GDScript runs still log the invalid-enum null-call error while reporting all 97 cases finished. The native runner's 45 focused cases pass, but older failed async tasks are
treated as finished: it starts 68, completes 66, prints all tests finished, and exits 0. Its 3D ellipse midpoint sample has `z = 0.05235425`, just outside its `0.05` tolerance; sequential timers and
deferred task resumption make this a frame-sensitive assertion. Its real-clock time-scale check is invalid under fixed FPS. Neither is proof of a tween math defect. The freed-owner panic is
independently confirmed by finding 13.

The earlier assertion propagation, async outcome, timeout, failure-exit, and cache-identity requirements are covered by the follow-up above. The native debug manifest already selects its actual debug
artifact. Passing these functional checks does not certify the remaining distribution/support items or resolve the separately recorded shutdown warnings.

### 4 and 10. Current source is ahead of the files a buyer would receive

**2026-10-17 release automation follow-up:** the initial release scope is Godot 4.4+, Windows/Linux x86_64, and macOS ARM64/Intel. The shell/Rust workflow, pinned toolchain/lockfile, isolated API 4.4 builds, universal frameworks, signing hooks, checksum/provenance records, full-suite/export jobs, dependency notices/source copies, and allowlisted extracted-archive checks are implemented. Android ARM64/x86_64 is an optional experimental build and is excluded from the desktop archive. Remote platform results, Apple credentials/signing and downloaded-package validation must be recorded before closing this finding. See [release instructions](release/README.md). The observations below describe the pre-pipeline snapshot.

The installed Linux debug library reports an insert-only parent stopped at 0.25 seconds even though its child is scheduled for 0.5 seconds. Fresh source keeps the parent playing. The installed binary
has `close_current_block`, so the older method-surface test alone cannot detect this newer staleness.

API 4.4 **works as an opt-in experiment**; that remains settled. Ordinary current builds use API 4.6, while both manifests advertise minimum 4.4.1 and the crate README claims 4.4+. Distribution
artifacts and claims still need to agree. This does not require changing the default feature: select the intended API in the release build command or narrow the shipped-artifact support
claim. [godot-rust documents the runtime/API ordering requirement](https://godot-rust.github.io/book/toolchain/compatibility.html).

The local addon library tree is **4,685,963,102 bytes**. Runtime `.so`/`.dll`/`.wasm` files account for **2,069,704,578 bytes**; the rest includes `.rlib`, PDB, and import-library output. The old Web
debug/release pairs are byte-identical despite the corrected script. The Android script builds only arm64/x86_64, while the manifest and README still include x86 artifacts. There is no recorded
current-source build/runtime/export matrix for all advertised platforms.

Produce an allowlisted release archive from one revision and recorded dependency/toolchain snapshot, rebuild every included platform, and test its extracted installation/export. Keep editor-required
debug runtime libraries; omit Rust build intermediates from the customer addon. Floating nightly, ignored Cargo.lock, absent release CI/checksum manifest, and lack of an assembled customer archive
remain unresolved. No additional platform was runtime-validated here.

### 9. Distribution contents and third-party notices remain unresolved

Addon directories contain the project MIT license but no dependency notice/source-availability package. Current resolved Godot dependencies declare MPL-2.0; dependency inventory covers the whole Linux
workspace and is **not** a final linked-binary notice list. Complete that distribution inventory against actual release artifacts and provide the applicable notices/source
route. [Mozilla's MPL FAQ, question 8](https://www.mozilla.org/en-US/MPL/2.0/FAQ/#q8-i-want-to-distribute-outside-my-organization-executable-programs-or-libraries-that-i-have-compiled-from-someone-elses-unchanged-mpl-licensed-source-code-either-standalone-or-part-of-a-larger-work-what-do-i-have-to-do)
explains the source-location notice for libraries compiled from unchanged covered code.

Both example projects retain the Girl Knight source sprites. The bundled terms and [the author's current page](https://jumpbutton.itch.io/girlknightasset) allow commercial use with attribution but
restrict resale/reproduction for profit. Do not assume those terms authorize bundling the editable sprites in a paid tool asset. Omit them from customer archives, replace them, or obtain permission
for the intended distribution. This is a content-permission boundary, not a finding that every showcase use is prohibited.

### 11. Storefront/support promises still need reconciliation

The README purchase URL is still a placeholder. Paid/demo/source contents, engine/platform support, updates/contact, and a version/changelog need to match the final archive and listing. Installation
layout, explicit addon build features, historical benchmark labeling, and GLIBC 2.34 documentation were already repaired.

The headless cold-import limitation remains upstream: [Godot PR 123658](https://github.com/godotengine/godot/pull/123658) is still open when checked on 2026-10-11. Prior graphical import success
stands; no graphical testing was repeated and this is not a new ordinary-editor failure claim.

## Source-only boundaries

- `TweenPtr::to_mut()` is a safe function returning unchecked mutable access from a shared reference. Synchronous callbacks and Godot signals can re-enter native payload/connection traversal. Existing
  deferred-callback guidance reduces supported-use risk, but successful engine probes do not establish Rust aliasing soundness. No new crash or memory-corruption claim is made here.
- Numeric inputs have no general rejection contract. The probes confirm negative speed decreases elapsed time and NaN/infinite speed contaminates clocks. Define supported domains before adding guards;
  zero-duration infinite loops already have documented once-per-frame semantics.
- Native callable-backed properties validate only their optional owner, not both getter/setter Callables. Invalid methods can fall back to default getter values and failed setter calls; this was
  source-reviewed, not runtime-certified here.
- Hot reload/unload, C# integration, and exhaustive generated-property coverage remain unverified. The manifest's `reloadable=true` is not evidence of successful reload behavior.
- Reverse-index sequence pruning is unchanged by request. Its performance was not benchmarked and is not a correctness blocker.

## Practical order

1. Repair sequence time-unit conversion and ellipse owner lifetime, then geometry and cancellation states.
2. Make the runners trustworthy and obtain uncached debug/release passing results with suitable clock modes.
3. Reconcile numeric contracts. Manager runtime pruning and follow documentation are resolved; first-use SceneTree availability is an accepted startup requirement, as recorded in findings 14, 16, and 17.
4. Freeze the source/API/platform scope, rebuild and assemble permitted archives, then verify extracted install/runtime/export and publish matching support promises.

## Work after the review

The separately requested GDScript constructor bridge is exposed as `append_ctor`,
`join_ctor`, and `insert_ctor`, with Rust/Godot API documentation, README examples,
and a focused integration suite. See [constructor evidence](release_audit_evidence/sequence_ctors_gdscript_2026_10_11/).
The native Linux debug manifest selector was also corrected to its debug-library
path. The Linux addon libraries were subsequently rebuilt, installed, and verified
with all 18 constructor cases in each profile; the local Linux binary staleness
observed above is therefore repaired. Other platform artifacts and the independent
runtime/distribution findings remain open. The review evidence and source hashes
remain the pre-change snapshot.
