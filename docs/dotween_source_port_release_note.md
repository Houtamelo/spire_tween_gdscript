# Shake sampler replacement and provenance

Status: the directly translated waypoint generator has been removed. Updated
2026-10-23 after the user required behavior compatibility without copied source.

The first, uncommitted shake draft directly adapted the generator in
[DOTween.cs, revision bfb02b0](https://github.com/Demigiant/dotween/blob/bfb02b05551a45204352dc43bdc15b882147891a/_DOTween.Assembly/DOTween/DOTween.cs#L626-L686).
The [official license](https://dotween.demigiant.com/license.php) restricts
distribution of modified versions. That prompted review of the draft's
provenance; the user chose replacement rather than retaining that source port.

The replacement is `spire_tween/src/tweens/templates/shake_path.rs`, initially
authored as a std-only Rust module from [the mathematical specification](designs/shake_sampler_math.md)
in a fresh agent context. Its initial private interface accepted scalar parameters
and unit random inputs. It uses planar equations, analytic knot times, and a reusable
offset buffer. It contains no Unity vector/rotation calls or translated clamp
chain. The preexisting Spire adapter handles the clock, owner validity, blending,
replay, and completion events.

The implementation context reported consulting only the supplied specification,
its temporary Cargo manifest, and its original empty Rust placeholder. It did
not read the source project, upstream code, prior generator, memory, or network.
An accidental `git status` attempt in that temporary directory failed because it
was not a repository and returned no source data. The implementer ran six
mathematical tests and formatting checks there before integration. A later lookup
refinement added a seventh test covering segment boundaries and midpoints.

The module initially integrated was byte-identical to the implementer's final output:
SHA-256 `7e8a4c3ce5868f71348ecb0d807b4380ce02b19a208bb1530b2fb4af552d158c`.

Subsequent local maintenance on 2026-10-24 changed the segment count and constructor
argument to `NonZeroUsize`, updated callers, and extended boundary tests to one
segment. Sampling mathematics is unchanged. That revision's SHA-256 was
`1dbd602c3b532d1dcf446991f6a4ba01842c07115a23e9dcc8de87937a694814`.

On 2026-10-25 the adapter contract changed to one path per playback and periodic
phase sampling. Restart/Incremental reuse that path, Yoyo retraces it, and replay
regenerates. Existing stopped/finished connections handle invalidation and
restoration. The shared `MethodTweenContext` and its method-processing hooks were
removed. This changes the adapter, not the sampler module or its recorded hash.

On 2026-10-26 the adapter's thread-local test-draw override was replaced by an
internal constructor accepting an owned `Xoshiro256PlusPlus`. The public constructor
creates an RNG and forwards to that implementation. Real-engine tests supply
seeded RNGs directly; expected numerical fixtures are recalculated from the
mathematical specification. The private sampler and its hash remain unchanged.
The refactor passed all 11 native shake cases and the full 200-case GDScript suite
on Godot 4.4 and 4.7.2 without shutdown leak diagnostics. The default API check,
rustdoc link check, and ordinary-build helper-visibility checks also passed.

Further review on 2026-10-26 removed the sampler's duplicate randomness enum and
draw-supplying closure: it now uses `ShakeRandomnessMode` and accepts `&mut impl Rng`
directly. The adapter uses `RcPtr` for template state and warns/clamps invalid
segment counts to `[2, 65536]` without `expect`. The sampler tests use seeded RNGs
and run without an engine process. These maintenance edits preserve the equations;
the module's current SHA-256 is
`458efe3d93a5578790e78e239a71957fa3b8977286c3637f3534b13e77febe32`.
All seven sampler unit tests pass with the shared enum and seeded RNGs. The 11
native shake cases and full 200-case GDScript suites pass on both engines,
including lower/upper segment bounds and correction warnings, with no shutdown
leak diagnostics. The default API and rustdoc link checks also pass.

The specification's author had read the upstream implementation. This workflow
therefore records separation of the implementation context, not a claim that
the whole project had no upstream exposure or a legal determination about
redistribution. Source provenance has not been hidden by renaming the draft.

After the initial replacement, the real-engine suites passed 108 native tests and 199
GDScript tests on each of Godot 4.4 and 4.7.2. The numerical fixtures exercise
the normal constructor and position writes on actual Godot nodes. The default
API build, native rustdoc link check, and formatting checks also passed.

The subsequent segment-lookup refinement preserves the mathematical results and
restores O(log N) lookup using calculated knot times. It passed seven mathematical
tests and the focused ten native/seven GDScript shake tests on each engine.

The 2026-10-25 adapter revision passed the full 109-test native and 200-test
GDScript suites on each engine, including new periodic-phase and finish-cleanup
regressions. Seven sampler tests, the default API check, rustdoc link checking,
formatting, and generator reproducibility also passed. Those GDScript runs still
reported the preexisting shutdown diagnostics. A subsequent constructor-test
repair removed the strong self/ancestor captures and made the factories execute;
both full 200-case GDScript runs then passed without shutdown leak diagnostics.
See the [repair evidence](../release_audit_evidence/shutdown_2026_10_25/README.md).

No release, publication, push, or commit is part of this migration task.
