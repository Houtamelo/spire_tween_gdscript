# Commercial release review — 2026-11-23

**Follow-up complete:** template packaging is repaired and verified on Linux with
Godot 4.4 and 4.7.2, including debug/release exports. The shared-pointer design is
accepted architecture. The remaining desktop CI work is a release validation
checklist. The documentation BBCode is intentional and required for Godot; no
formatting changes are needed. Current runtime checks pass, including Blueprint
authoring and preview.

Reviewed source: `d3d993e754e48630150624a4f4a76675e18a0305`, initially clean.
This report supersedes the readiness verdict in [round 2](release_audit_round2.md).
Small test, workflow, and documentation repairs made during this review remain
uncommitted. The follow-up changes release tooling and its smoke scene; no addon
runtime Rust code or installed addon libraries were changed.
Evidence and reproduction instructions are in
[review_2026_11_23](release_audit_evidence/review_2026_11_23/README.md).

## Findings and disposition

### R1 — Resolved: include the supplied templates in customer archives

The reviewed builder staged native libraries, the extension manifest, and
notices. Its package allowlist omitted the source addon's `templates/` directory.

**Reproduced against the actual current CI artifacts.** A Linux candidate built
from a clean checkout contains 449 files, zero template files, zero GDScript
files, and zero scenes. The source addon contains 33 template scenes plus their
shared scripts, UID sidecars, README, and catalog. Customers following the
installation instructions therefore receive none of those templates.

Both engines nevertheless reported `passed: true` for that original archive,
including debug and release exports: the smoke scene only exercised native
classes. Source-project template checks could not detect missing package content.

**Implemented:** the [release tool](release/tools/src/main.rs) stages and hashes
all 55 authored template files, verifies them against its source inventory, and
includes the shared payload once across platforms. The
[smoke scene](release/smoke.gd) loads, plays, and restores every catalogued scene.
The [integration tests](release/tools/tests/pipeline.rs) check exact file content,
reject incomplete stages, and remove a shared script from an archive while
keeping its checksums valid; that archive must fail runtime validation.

A fresh Linux build and candidate now deliver all 33 scenes and their scripts,
UID sidecars, catalog, and README. Installation and both exported profiles pass
on both engines, and the three real-artifact integration tests pass per engine.
See [follow-up evidence](release_audit_evidence/review_2026_11_23/template_packaging_followup/README.md)
and the updated [packaging report](docs/reports/EFFECT_TEMPLATE_RELEASE_PACKAGING.md).
The example gallery and third-party art remain outside the customer archive.

### R2 — Withdrawn: accepted shared-pointer architecture

The pointer API deliberately relies on the crate's borrowing discipline rather
than enforcing it through runtime guards. This was already discussed and accepted
by the owner, and the architecture has been exercised by the integration suites.
The review did not demonstrate a new execution path violating that discipline.

The [compile-only witness](release_audit_evidence/review_2026_11_23/alias_compile_probe.rs)
shows that a caller can deliberately request overlapping references. That
capability alone does not establish a new defect in the intended architecture.
It is retained as a bounded observation, not a release blocker or a request to
redesign the pointers. No pointer implementation was changed.

### R3 — Validation checklist: complete the configured desktop pipeline

[The current CI run](https://github.com/Houtamelo/spire_tween_gdscript/actions/runs/45291109254)
built Linux and Windows successfully. The macOS build was cancelled before any
steps ran, with the annotation:

```text
The configuration 'macos-14-arm64-us-default' is not supported
```

Validation, packaging, extracted-archive checks, and delivery were consequently
skipped. No failing runtime test was established by that CI run. GitHub's
[runner image inventory](https://github.com/actions/runner-images/blob/main/README.md)
confirms that macOS 14 images were retired and lists `macos-15` for ARM64 and
`macos-15-intel` for Intel.

The three ARM64 runner selections are repaired locally to use `macos-15`;
the Intel selections remain explicit. Workflow syntax passes actionlint.
This does not establish that a new macOS build or runtime run has succeeded.

Before release, obtain the complete configured engine/platform/architecture
matrix from the final source revision and verify the exact assembled archive.
This means all four configured engine versions on Linux x86_64, Windows x86_64,
macOS ARM64, and macOS Intel, with debug/release native and GDScript suites,
runner failure controls, installation, and exports. The assembled archive also
has separate extracted installation/export checks on Godot 4.4 and 4.7.2.
Local checks here cover Linux on Godot 4.4 and 4.7.2; they do not certify Windows,
macOS, Godot 4.5.2/4.6.3, downloaded-package quarantine behavior, or the macOS 11
deployment minimum. Android/Web/iOS remain outside the desktop archive's claim.

### R4 — Withdrawn: intentional Godot documentation markup

The BBCode tags are necessary for Godot's documentation renderer. Their use is
intentional and requires no formatting changes or follow-up work.

The extra audit command, using
`RUSTDOCFLAGS='-D rustdoc::broken_intra_doc_links'`, interpreted that BBCode as
Markdown links and promoted its warnings to errors. That result does not
establish a documentation defect. Ordinary
`cargo doc --locked -p spire_tween --no-deps` succeeds and generates documentation.
The logs remain as observations of the audit command; the finding is withdrawn.
No documentation markup was changed.

## Repairs made during this review

| Finding on the reviewed revision | Local repair and verification |
|---|---|
| Production libraries cannot parse `text_animator.gd`: a catalog test infers types from the integration-only `render_sample_for_test` method | Moved the glyph-observation test into the instrumented rendered suite. The ordinary catalog loads against customer libraries; the rendered check still executes. |
| Release validation expects 184 native and 291 GDScript cases | Updated the totals to the observed 189 and 343. Original runs correctly failed rather than approving incomplete discovery or accepting a stale total. |
| macOS 14 runner labels prevent the desktop pipeline from reaching validation | Replaced all three ARM64 selections with `macos-15`; actionlint passes. A remote run remains required. |
| The dedicated rendered preset/character suite is absent from CI | Added it to Linux validation using the instrumented native test extension and retained its logs. Customer libraries still exclude observation helpers. The same command passes locally on both tested engines. |
| README advertises a placeholder purchase URL and an already available demo; GitHub has no published releases | Replaced those claims with the current pre-release installation route and clarified the desktop package scope. Final paid/demo contents remain an owner decision. |
| README says GLIBC 2.34, but the actual CI libraries require 2.35 | Corrected the claim after inspecting both profiles with `readelf`; `build.json` also records `glibc_minimum: "2.35"`. |
| Release instructions still describe the repaired historical GDScript shutdown warnings as open | Updated the text and documented the graphical validation suites. Current ordinary suite logs are checked separately for shutdown diagnostics. |

## Verification

The table below records the initial audit; the packaging follow-up has its own
fresh stage and validation metadata linked under R1. Initial Linux libraries
came from this revision's GitHub Actions build, not the example
projects' existing binaries. Their manifest targets API 4.4. Customer debug and
release libraries were tested separately; rendered observation tests use the
separately built native integration extension. Graphical checks used private
Xvfb displays, software rendering, isolated projects, and isolated user data.

| Check | Result |
|---|---|
| Workspace/all-target Cargo check | Pass; 11 warnings in the demo/test crate |
| Core Cargo tests | 16 tests and 3 compile-fail doctests pass; 84 doctests ignored |
| Release CLI tests | 5 pass |
| Real-artifact release-tool integration tests | 3 pass, including missing-library, tampering, extra-file, and missing-file controls |
| Production GDScript catalog, 4.4 and 4.7.2, both profiles | 343 pass per run after the test-only repair |
| Native catalog and focused lifecycle suites | 189 catalog cases plus 67 focused checks pass per engine/profile. Debug aggregates complete in one process; release aggregates were interrupted by the Codex server restart, and their remaining 41/45 cases pass individually. |
| Runner deliberate pass/fail controls | All 16 processes produce the expected markers and exit codes across both engines, languages, and profiles |
| Source template scenes, 4.4 and 4.7.2 | 33 runtime and 33 editor cases pass on each engine |
| Blueprint preview, 4.4 and 4.7.2 | 228 editor plus 2 runtime checks pass on each engine |
| Instrumented rendered text/preset suites, both engines | 66 pass per engine, plus typed-array rejection and editor checks |
| Original extracted Linux candidate, both engines | Native checksums/install/export checks pass despite absent templates; this omission is fixed in the follow-up candidate |
| Follow-up extracted Linux candidate, both engines | All 33 installed templates load, animate and restore in debug/release library and exported-game profiles; all 3 real-artifact integration tests pass per engine |
| Additional native-feature archive probe, both engines | 12 checks pass in each source run and both exported profiles: property/text Blueprints, range timing, preset callbacks, code-after-preset ordering, and absence of test helpers |
| Editor object-lifetime observation, 4.7.2 | Three 500-node tweened batches plateau; matched node-only control is stable. This is an object-count observation, not a manager-record census. |
| Extra strict native/bridge rustdoc link checks | Reject intentional Godot markup; withdrawn as a finding. Ordinary `cargo doc` succeeds. |
| Edited workflow / whitespace checks | actionlint and `git diff --check` pass |
| Complete desktop release matrix / verified deliverable | Unavailable; R3 remains open |

The original Linux candidate SHA-256 is
`8e1f0f3c0f1b9d4dbf79c8a926c6f9ff0a177eef09354ee2eb030790e40bf8d0`.
It is explicitly a partial development candidate, not a commercial desktop
package. Its successful native checks did not detect R1's missing assets.

## Status of earlier concerns

Current real-engine suites exercise the earlier ownership, node-binding,
sequence-cycle, delayed/method-call validity, bookkeeping, sequence timing, and
ellipse repairs. No new failure in those checks was found. The release catalog
coverage spans the interrupted aggregate and resumed cases; it is not evidence
of a complete uninterrupted release aggregate or a full desktop CI pass. The paired editor
object-count observation also did not reproduce growing retained objects.

The archive includes project/dependency notices, Rust notices, and MPL dependency
sources. Its allowlist excludes native test libraries and Girl Knight example
art. This confirms package content, not a new legal opinion or permission to
bundle the complete example repository in a paid source download.

Previously accepted behavior remains accepted: loops cross at most one boundary
per update; manager initialization requires an installed SceneTree; custom
preview restorers run synchronously; invalid restoration Callables mean nothing
to restore. Editing animated properties during preview was not added to scope.

The cold headless GDExtension-import workaround remains in the harness. Graphical
source-template/preview imports succeed here; this review does not assert that
upstream cold headless import is fixed. Benchmark numbers, C# interop, hot reload,
mobile/device behavior, and every generated property adapter were not re-certified.

## Remaining release decisions

Choose the release version and changelog after the recent breaking API/serialized
field changes, define paid/demo/source payloads, and provide the intended support
contact and update policy. Those are product decisions; this review does not
invent terms or publish a storefront.

R1's packaged-template workflow is fixed and verified locally. The remaining
release step is the desktop pipeline on the final clean revision.
No commit or push was made as part of this review.
