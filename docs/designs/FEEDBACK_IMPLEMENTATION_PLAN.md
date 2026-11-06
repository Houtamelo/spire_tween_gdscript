# Feedback authoring implementation plan

Revision 2, 2026-11-04. **Planning only: no implementation unit, build, experiment, or integration test was executed for this revision.** This replaces the P00–P14 roadmap with twelve bounded units. No previous package was assigned or executed.

Inputs:

- [Runtime requirements](FEEDBACK_SYSTEM_DRAFT.md): restart, targets, atomic effects, and composition.
- [Editor requirements](FEEDBACK_EDITOR_TOOLING_DRAFT.md): artist workflows and EDT-01 through EDT-23.
- [Selected implementation contract](FEEDBACK_IMPLEMENTATION_CONTRACT.md): concrete names, fields, APIs, and behavior. Its engineering selections are proposals for review, not retroactively attributed user decisions.
- [Existing controller evidence](../../release_audit_evidence/editor_controller_2026_11_03/README.md): root/controller lifetime on Godot 4.7.2 only.

This applies the general writing-plans guidance. SDD lifecycle, roles, approval levels, timeline paths, and commit procedures are excluded. Developers consume selected contracts rather than choose product behavior inside a work unit.

## 1. Design selections and readiness

The user's binding choice remains fixed restart with no retrigger enum, queues, overlap policies, or inheritance controls. Implementation selections are recorded in the separate owning contract instead of being delegated to an unresolved P00:

| Topic | Selected planning baseline |
|---|---|
| Authoring model | Extend the in-progress TweenBlueprint; no parallel SpireFeedback hierarchy. |
| Discoverable roles | Four thin GDScript subclasses selecting Property, MethodCaller, ChainBlock, and ParallelBlock. |
| Storage | Preserve existing field names/ordinals and Variant payloads; read-model dictionaries are transient. |
| Reset/start | Built-ins do not rewind target values; implicit starts read the current value at native tween start. |
| Construction | Eager child play/build, as in the core draft; no unselected lazy-child semantic change. |
| Triggers | Ready, tree-entry, and mouse conventions in contract section 4; runtime only. Other signals use Godot's native scene connections. |
| Preview | One native session, declared reversible writes, retained final pose, pre-operation restoration. |
| Presets | Fade In and Scale Pop with fixed catalogue/defaults in contract section 7; a separate Method Caller runtime demo. |

The TweenBlueprint selection incorporates new source observed during this refinement and remains a planning proposal for user review. If the user selects the alternate model, revise the contract and dependent units before assignment. Do not silently treat it as an approved user decision or overwrite the active prototype writer's work.

One technical evidence gate remains: U02 must prove real editor edit/save/Undo boundary ordering. U07/U08 require that proof. Failure returns a reproduction for design review, not permission to weaken restoration or switch preview workflows. Other units may proceed according to dependencies.

Timeline/keyframes, scrubbing/seeking, reverse/baking, preview speed/loop controls, arbitrary editor method execution, a new preset browser/format, and seamless native-binary hot reload remain deferred.

## 2. Existing abstractions and data flow

| Existing authority | Consequence |
|---|---|
| [Blueprint prototype](../../spire_tween/src/blueprint/mod.rs), [crate entry](../../spire_tween/src/lib.rs) | Integrate existing authoring work and retain unrelated edits. |
| [Native property construction](../../spire_tween/src/tweens/property/do_property.rs), [Variant dispatch](../../spire_tween/src/tweens/lerping/variant.rs), [property timing](../../spire_tween/src/tweens/property/data.rs) | Reuse actual type support and delayed start evaluation. |
| [Sequence API](../../spire_tween/src/tweens/sequence/gd.rs), [base playback](../../spire_tween/src/tweens/base/spire_tween.rs) | Reuse adoption, elapsed time, pause/stop, and success/cancellation ordering. |
| [Controller](../../spire_tween/src/global/mod.rs) | Tool annotation already exists; no second ticker/controller. |
| [Native runner](../../spire_tween_rust/src/tests/test_runner.rs), [GDScript runner](../../spire_tween_gdscript/examples/tests/test_runner.gd) | Register groups and verify positive execution counts. |
| [Release tool](../../release/tools/src/main.rs), [configuration](../../release/config.json) | Preserve platform/profile/export gates and the 4.4/4.5.2/4.6.3/4.7.2 matrix. |

Authored fields → shared native resolution/validation → native sequence construction → existing controller is the runtime path. The Inspector consumes AuthoringModelV1 and writes the same fields through editor change handling. Preview adds preflight, canonical snapshots, and a session identity around that same native path. No preview state enters scene storage.

Two concrete delivery gaps remain: prepare builds class metadata before copying addon files, while package currently collects libraries without the new scripts/presets. U01/U11 own those fixes and retain cold-import mitigation.

## 3. Work order and whole-file ownership

| Unit | Outcome | Depends on |
|---|---|---|
| U01 | Reliable editor-test execution | None |
| U02 | Verified editor operation boundaries | U01 |
| U03 | Serializable authoring model and shared metadata | U01 |
| U04 | Native effects, composition, and custom hooks | U03 |
| U05 | Runtime event binding | U04 |
| U06 | Discoverable authoring and editable Inspector | U05 |
| U07 | Native preview sessions and restoration | U02, U05 |
| U08 | Preview controls and editor lifecycle integration | U06, U07 |
| U09 | Composition explanation and actionable diagnostics | U08 |
| U10 | Presets and GDScript examples | U09 |
| U11 | Complete customer package and enforced evidence | U10 |
| U12 | Independent delivered-feature acceptance | U11 |

For scopes below, A means spire_tween_gdscript/addons/spire_tween; G means spire_tween_gdscript; E means spire_tween_editor_tests; N means spire_tween/src/blueprint. These are exact path aliases. Directory scopes grant exclusive ownership of new files beneath them. All retained files are owned whole for one writer cycle.

Shared integration files require explicit sequential ownership transfer:

| Path | Writer succession |
|---|---|
| release/tools/src/main.rs | U01 → U11 |
| release/tools/src/editor_test.rs | U01 → U11 |
| N/mod.rs | U03 → U04 → U05 → U07 |
| spire_tween_rust/src/tests/mod.rs and test_runner.rs | U03 → U04 → U05 |
| G/examples/tests/test_runner.gd | U03 → U04 → U05 → U10 |
| release/config.json | U03 → U04 → U05 → U10 → U11 |
| A/editor/plugin.gd and inspector.gd | U06 → U08 → U09 |
| docs/feedback_runtime.md | U03 → U04 → U05 |
| docs/feedback_authoring.md | U06 → U08 → U09 → U10 |

Every arrow requires the earlier unit's acceptance and review before the next writer receives the whole file. Concurrent writers to a shared path are forbidden. U07 depends on U05 for the N/mod.rs handoff as well as its native feature inputs. All other listed paths have one exclusive unit owner. Current user edits are not assumed to belong to this feature; unresolved writer conflicts stop the affected unit without reset/stash/revert.

## 4. Named checks and evidence

All commands below are future checks. U01 supplies isolated GD_FIXTURE/NATIVE_FIXTURE paths through the existing prepare path, recording exact addon/native-test artifacts. ADDON is a complete isolated addon directory, not a library silently installed into a working project. The new editor-test command is not presently implemented.

```yaml
checks:
  C-BUILD:
    commands:
      - "cargo check --locked -p spire_tween"
      - "cargo build --locked -p spire_tween --features include_gdext_lib,include_gdscript_bridge,api-4-4"
      - "cargo build --locked -p spire_tween_rust --features api-4-4"
    expected: "Successful builds; unchanged feature defaults/native-only usage."
  C-NATIVE:
    command: "$GODOT --headless --path $NATIVE_FIXTURE res://test_runner.tscn -- --test=$GROUP"
    expected: "Native result marker, positive selected execution, zero failures, exit 0."
  C-GDSCRIPT:
    command: "$GODOT --headless --path $GD_FIXTURE res://examples/tests/test_runner.tscn -- --test=$GROUP --no-cache"
    expected: "GDScript result marker, positive execution, zero failures/skips, exit 0."
  C-EDITOR:
    command: "spire-release editor-test --addon $ADDON --godot $GODOT --out $REPORT --mode $MODE --suite $SUITE"
    optional: "--case CASE_ID and --profile debug|release"
    available_after: U01
    expected: "Complete EditorValidationV1, positive selected/executed counts, all assertions pass, exit 0."
  C-RELEASE:
    command: "spire-release test --stage $STAGE --godot $GODOT --out $REPORT --templates $TEMPLATES"
    expected: "Existing aggregate/failure-control/both-profile/export gates pass."
  C-ARCHIVE:
    command: "spire-release test-archive --archive $ARCHIVE --godot $GODOT --out $REPORT --templates $TEMPLATES"
    expected: "Extracted file, checksum, install, editor, runtime, and export gates pass."
```

spire-release is the binary from release/tools. MODE is headless or rendered. Suites are harness, boundaries, model, effects, triggers, inspector, snapshots, preview, composition, presets, and acceptance. Cases use an EDT or RT claim prefix plus scenario suffix; reject unknown/duplicate IDs and empty selection. Deliberate self-test failures are explicitly selected controls, excluded from ordinary suites.

```yaml
EditorValidationV1:
  schema: 1
  source_identity: "revision and relevant working-tree file hashes"
  addon_files_sha256: "complete tested addon input set"
  library_sha256: "actual selected native library"
  engine: "exact version/hash"
  platform: "actual OS and architecture"
  profile: "debug | release"
  mode: "headless | rendered"
  suite: "selected suite"
  selected_count: "integer greater than zero"
  executed_count: "equals selected_count"
  cases: "Array[{id, passed, assertions, log, screenshots}]"
  passed: "true only when every required result is present and passing"
EditorBoundaryAdapterV1:
  before_edit: "(object: Object, properties: PackedStringArray) -> void"
  before_serialize: "(scene_root: Node) -> void"
  selection_changed: "(selection: Array[Node]) -> void"
  scene_changed: "(scene_root: Node = null) -> void"
  shutdown: "() -> void"
  evidence: "engine-specific hook mapping and operation-order traces from U02"
EditorCaseProviderV1:
  script: "cases/*_cases.gd, extends RefCounted, loaded only by the test driver"
  get_cases: "(context: Dictionary) -> Array[Dictionary{id: String, claim: String, run: Callable}]"
  run_signature: "(context: Dictionary) -> void; may await real editor operations"
  context:
    editor_plugin: "test-driver EditorPlugin"
    fixture_root: "Node for the active test fixture"
    check: "Callable(condition: bool, message: String); records assertions and failures"
  execution: "Driver awaits run.call(context), enforces a deadline, and requires at least one assertion."
```

The unit implementer runs focused checks and records evidence. U12's integration reviewer owns final candidate evidence. A failed final check returns to its owning unit; it does not grant arbitrary product-edit scope to the reviewer. Each unit includes source, tests, registration/count updates, and relevant documentation in one writer cycle.

The acceptance suite aggregates all non-self-test providers and requires cases for every EDT-01 through EDT-23 claim. It fails on a missing claim rather than requiring U12 to add a new test implementation. Parse errors, missing completions, or failed assertions cannot be hidden by a successful editor process exit.

## 5. Unit U01: Reliable editor-test execution

Outcome: A maintained runner executes real editor cases in isolated projects and reliably detects failure.

Claims: RT-HARNESS; fixture preparation for all EDT cases.

Depends on: None.

Owned paths:

- Add: release/tools/src/editor_test.rs and release/tools/tests/editor_cli.rs.
- Add: E/project.godot, E/addons/test_driver/, E/cases/harness_cases.gd, E/fixtures/base_scene.tscn, E/README.md.
- Modify: release/tools/src/main.rs; ordered handoff to U11.

Required context:

- release/tools/src/main.rs: prepare, run_logged, profile selection, command parsing, and asset isolation.
- Existing controller audit harness: use its observations without a dated audit directory becoming a product dependency.
- Implementation contract sections 1–2: actual class/field availability.

Interfaces and invariants:

- Consumes: Engine binary, complete addon directory, and exact native-test artifacts where native fixtures are requested.
- Produces: C-EDITOR; EditorValidationV1; isolated runtime/editor fixture paths; discovery of case files with unique IDs.
- Preserves: Cold-import mitigation, artifact-specific class availability, user projects/settings, and existing release subcommands.

Execution:

1. Implement editor-test with the arguments in C-EDITOR; reject malformed/duplicate options and existing report directories.
2. Copy addon scripts/assets before deriving fixture script-class metadata. Prepare native-only fixtures without bridge-dependent scripts; prove addon class discovery with a real class.
3. Launch real editors with deadlines and isolated settings/cache. Drive rendered Controls through input events and verify geometry/focus; do not substitute direct handler calls.
4. Discover case files through a fixed interface returning IDs/callables. Require positive selected/executed counts and complete result records.
5. Add explicit assertion-failure, timeout, invalid/duplicate-case, and missing-result controls; clean up editor/display subprocesses on all exits.

Focused preflight:

- Inspect current prepare behavior and run existing release CLI checks; report unrelated baseline failures before editing the runner.

Evidence:

- Implementer: C-EDITOR(harness, headless/rendered) on 4.4 and 4.7.2 plus real subprocess negative controls.
- Record engine/artifact identities, fixture paths, and absence of writes to the normal user editor-settings profile.

Stop and return:

- Import requires weakened artifact identity, required classes are unavailable, or a rendered check would be replaced with a mocked UI call.

Rollback: Remove the editor-test command/module and fixtures; reverse this unit’s prepare changes together while keeping existing release commands usable.

Complete when: A rendered control case and controller-registration case pass; each deliberate failure control returns nonzero.

## 6. Unit U02: Verified editor operation boundaries

Outcome: A versioned adapter proves when temporary values can be restored before edits, history operations, and serialization.

Claims: EDT-13–16; technical evidence gate for U07/U08.

Depends on: U01.

Owned paths:

- Add: E/cases/boundary_cases.gd, E/fixtures/boundaries/, E/adapters/editor_boundaries.gd.
- Add: docs/designs/FEEDBACK_EDITOR_BOUNDARY_EVIDENCE.md and evidence below release_audit_evidence/feedback_boundaries/.

Required context:

- Editor specification sections 8, 10, and 12; implementation contract section 6.
- Actual Godot 4.4/4.7.2 save, Inspector, scene, launch, and Undo/Redo hook ordering; inspect engine source where needed.

Interfaces and invariants:

- Consumes: U01 runner, existing native property tweens, and fixture-only transient snapshots.
- Produces: EditorBoundaryAdapterV1 and a report mapping its callbacks to actual engine hooks and tested versions.
- Preserves: Authored file/history values, pre-existing dirty state, and intentional edits by another editor tool.

Execution:

1. Animate Node2D, Node3D, and Control fixtures with existing tweens; exercise playing, paused, and completed poses without building product preview UI.
2. Drive Save, Save All, project/current-scene launch, target edits, Undo/Redo, selection, tab/scene closure, and plugin shutdown through real operations.
3. Apply pre-operation restoration, then inspect saved/reopened files and undo baselines. A notification firing alone is not acceptance.
4. Include an external editor edit to a preview-owned property; verify the edit survives and its undo history contains authored values.
5. Record the working mapping in the adapter; preserve the callback contract and product semantics.

Focused preflight:

- C-EDITOR(harness, rendered); characterize unanimated dirty-state/save behavior first.

Evidence:

- Implementer: C-EDITOR(boundaries, headless/rendered) on 4.4 and 4.7.2, saved-file comparisons, history assertions, and hook-order traces.
- The evidence note must explicitly mark unsupported hook/version combinations.

Stop and return:

- A required operation saves an animated pose, records an animated undo baseline, loses an intentional edit, or lacks reliable interception. Return the smallest reproduction; do not choose a different preview workflow or weaken EDT-14/15.

Rollback: Remove fixture-only probes/adapter while retaining failed evidence; no product UI or persistent product state was introduced.

Complete when: Every required boundary case passes and its hook mapping delivers the contract callbacks without a new product decision.

## 7. Unit U03: Serializable authoring model and shared metadata

Outcome: TweenBlueprint is one durable model with optional targets, retained typed values, and authoritative Inspector metadata.

Claims: RT-MODEL; EDT-02–04 and EDT-07.

Depends on: U01.

Owned paths:

- Modify: N/mod.rs and spire_tween/src/lib.rs.
- Add: N/target.rs, N/reflection.rs, N/diagnostics.rs, docs/feedback_runtime.md.
- Add: spire_tween_rust/src/tests/blueprint_core.rs; G/examples/tests/blueprint_core.gd and .tscn; E/cases/model_cases.gd and E/fixtures/model/.
- Modify: spire_tween_rust/src/tests/mod.rs, spire_tween_rust/src/tests/test_runner.rs, G/examples/tests/test_runner.gd, release/config.json through the integration handoffs.

Required context:

- Current N/mod.rs and its writer’s changes; native property/Variant dispatch; Godot dynamic property/storage behavior.
- Implementation contract sections 1–2 and 4–5, including existing field names and enum ordinals.

Interfaces and invariants:

- Consumes: Contract fields/enums, native reflection, and scene node-reference serialization.
- Produces: AuthoringModelV1, DiagnosticV1, shared target resolution, and persistent Variant payloads independent of Inspector visibility.
- Preserves: Prototype fields/ordinals, native-only compilation, consumer values, and unrelated configuration.

Execution:

1. Reconcile the current prototype with its writer; retain agreed fields/ordinals and add the contract fields. Do not introduce a competing authoring hierarchy.
2. Fix property usage so hiding a field removes only editor visibility and retains storage. Unresolved/incompatible values keep their actual types.
3. Implement independent target resolution (configured target, then first non-blueprint ancestor), direct child discovery, property/method descriptors, and non-mutating diagnostics. Containers do not propagate targets.
4. Implement first-selection initialization and explicit repair semantics, providing metadata for later undoable UI actions.
5. Register real native/GDScript groups, verify they execute, update aggregate expected counts from results, and document model/storage behavior.

Focused preflight:

- Inspect saved consumers of TweenBlueprint fields and current user changes; C-BUILD baseline; verify test fixture/artifact identities.

Evidence:

- Implementer: C-NATIVE(blueprint_core), C-GDSCRIPT(blueprint_core), C-EDITOR(model, headless) on minimum/development engines.
- Cover independent child targets and ancestor fallback, unresolved/recreated references, supported/unsupported/read-only properties, duplicate/instance independence, and valid/unresolved/incompatible save/reload with tooling disabled.

Stop and return:

- A consumer needs destructive conversion, names collide, metadata disagrees with native dispatch, or integration requires discarding user work. Return a migration/design proposal.

Rollback: Reverse this unit’s model/registration additions while restoring pre-unit fields; retain evidence and do not mass-rewrite consumer scenes.

Complete when: Storage round trips and shared metadata pass without custom Inspector code or duplicate persistent configuration.

## 8. Unit U04: Native effects, composition, and custom hooks

Outcome: All four built-in roles and the Custom hook contract build real native sequences with fixed restart.

Claims: RT-RESTART, RT-EFFECTS, RT-COMPOSITION, RT-CUSTOM; EDT-05–07, EDT-10, EDT-18.

Depends on: U03.

Owned paths:

- Modify: N/mod.rs and docs/feedback_runtime.md through their handoffs.
- Add: N/playback.rs, N/effects.rs, N/composition.rs, A/feedback/.
- Add: spire_tween_rust/src/tests/blueprint_effects.rs; G/examples/tests/blueprint_effects.gd and .tscn; E/cases/effect_cases.gd and E/fixtures/effects/.
- Modify: spire_tween_rust/src/tests/mod.rs, spire_tween_rust/src/tests/test_runner.rs, G/examples/tests/test_runner.gd, release/config.json through their handoffs.

Required context:

- Implementation contract sections 1 and 3; native constructors, sequence adoption, elapsed-time getters, and stop/finish semantics.

Interfaces and invariants:

- Consumes: U03 target resolution, configuration, diagnostics, and AuthoringModelV1.
- Produces: Contract play/stop, append_property_step/append_method_step, four custom virtuals, and the four named GDScript subclasses.
- Preserves: Usable handles for invalid input, implicit-start timing, native loops/replay, and one existing native tick path.

Execution:

1. Introduce failing restart/start-timing/method-order behavioral cases before their product edits.
2. Implement stop → reset → configure/build → publish; release Rust borrows before script calls. Same-instance reentry before the current play call returns is unsupported and requires no generation tracking. Emit the contract’s existing tween_spawned signal before building so synchronous completion is observable; preserve exact-handle lifecycle forwarding.
3. Use existing typed property builders; apply explicit start only when enabled. Validate and schedule zero-required-argument methods; ignore returns and never implicitly await them.
4. Eagerly construct direct children, append/join their exact fresh handles, and preserve original-parent continuation when a child is replaced.
5. Implement Custom dispatch and ordinary reusable native helpers; supplied scripts select fixed kinds and Manual without duplicated effect state.
6. Integrate registrations/counts and document native-handle replay boundaries and custom hooks.

Focused preflight:

- C-BUILD and U03 model suites; inspect the current sequence API changes before relying on older audit layout.

Evidence:

- Implementer: C-NATIVE/C-GDSCRIPT(blueprint_effects), C-EDITOR(effects, headless).
- Cover every advertised type, delay/explicit/implicit starts, nested ordering, valid/defaulted/variadic/required arguments, no construction-time method invocation, invalid steps, deletion, paused/delayed restart, cancellation, reentrancy, synchronous custom completion, and GDScript access to native properties.

Stop and return:

- A second scheduler, sequence replay redesign, unsafe retained Rust borrow, or target mutation during reflection becomes necessary.

Rollback: Remove playback/effect modules and supplied scripts; reverse only this unit’s declarations/registrations. U03 remains independently usable.

Complete when: The four configured roles and one Custom fixture run from saved scenes with the contract’s restart/failure behavior.

## 9. Unit U05: Runtime event binding

Outcome: Automatic events use the same restart path and never activate during editor authoring.

Claims: RT-TRIGGERS; EDT-08.

Depends on: U04.

Owned paths:

- Add: N/triggers.rs, spire_tween_rust/src/tests/blueprint_triggers.rs, G/examples/tests/blueprint_triggers.gd and .tscn, E/cases/trigger_cases.gd, E/fixtures/triggers/.
- Modify: N/mod.rs, docs/feedback_runtime.md, spire_tween_rust/src/tests/mod.rs, spire_tween_rust/src/tests/test_runner.rs, G/examples/tests/test_runner.gd, release/config.json through their handoffs.

Required context:

- Implementation contract section 4; current Control/CollisionObject signals and node lifetime/connection handling.

Interfaces and invariants:

- Consumes: Play, optional source fields, shared target descriptors, and internal connection generations.
- Produces: AfterReady/AfterEnterTree/mouse bindings and shared mouse-source diagnostics.
- Preserves: Enum ordinals, no editor automatic subscriptions, manual child play, and stored parent-managed trigger settings.

Execution:

1. Connect only outermost blueprints. Filter matching pressed button events. Other signal-driven playback uses Godot's native persistent connections to play, with no addon signal-selection or subscription logic.
2. Validate sources and picking prerequisites without changing scene flags. Rebind on configuration/hierarchy change; disconnect on exit/deletion.
3. Reject stale callbacks using binding generation, preventing duplicate activation after reparenting or source replacement.
4. Add real-game and editor event cases and shared validation descriptors.

Focused preflight:

- U04 runtime suites; inspect actual supported-source signatures on 4.4 and 4.7.2.

Evidence:

- Implementer: C-NATIVE/C-GDSCRIPT(blueprint_triggers), C-EDITOR(triggers, headless/rendered).
- Cover valid/invalid mouse sources, press/release/motion, native persistent scene connections to play, repeated entry, promotion/demotion, duplicate connections, deletion, repeated restart, and zero plays from editor ready/input.

Stop and return:

- A new source type, input convention, or editor execution of gameplay subscriptions is required.

Rollback: Remove event connections and this unit’s registrations, retaining Manual playback and stored configuration.

Complete when: Each supported runtime event activates once and every editor event case produces zero automatic activations.

## 10. Unit U06: Discoverable authoring and editable Inspector

Outcome: Artists configure the four roles through themed, undoable controls without writing property/method names in code.

Claims: EDT-01–05, EDT-07, EDT-21–22.

Depends on: U05.

Owned paths:

- Add: A/plugin.cfg, A/editor/plugin.gd, A/editor/inspector.gd, A/editor/property_picker.gd, A/editor/typed_values.gd, A/editor/method_picker.gd, A/editor/trigger_editor.gd, A/icons/.
- Add: E/cases/inspector_cases.gd, E/fixtures/inspector/, docs/feedback_authoring.md.

Required context:

- Editor specification sections 3–6 and 12–13; AuthoringModelV1; native editor change/revert/Undo conventions.

Interfaces and invariants:

- Consumes: get_authoring_model and authored native fields; eventual temporary edits are ended using the host boundary contract.
- Produces: One plugin/Inspector registration, named testable Controls, searchable pickers, typed values, timing/easing, target navigation, and explicit repair actions.
- Preserves: Custom GDScript exports, scene editability/ownership, retained values, ordinary revert behavior, and plugin-disabled serialization.

Execution:

1. Create one GDScript editor entry with Inspector registration and descriptions/icons; do not register another native EditorPlugin.
2. Build the specified section order from native descriptors, including concrete invalid/unresolved/read-only states.
3. Use emit_changed/EditorUndoRedoManager for edits. Copy Current to Start plus enabling Start is one action; incompatible repair is explicit.
4. Refresh hierarchy-managed trigger presentation and dynamic fields without discarding payloads or regenerating child scenes.
5. Test narrow/scaled/theme variants, keyboard focus, and multi-selection behavior. U08 adds working preview controls; do not add a misleading nonfunctional preview placeholder.

Focused preflight:

- U03–U05 suites and C-EDITOR(harness, rendered); inspect the selected editor’s actual UI/theme.

Evidence:

- Implementer: C-EDITOR(inspector, rendered) on 4.4 and 4.7.2 using actual picker/value/revert/undo/navigation input and save/reload.
- Include invalid methods/properties, missing metadata/targets, incompatible payloads, instance restrictions, and repeated refresh/enable-disable.

Stop and return:

- The UI needs its own target/type rules, destructive payload conversion, hidden custom exports, or direct writes bypassing edit history.

Rollback: Remove custom Inspector registration/assets; retain native configuration, warnings, and storage.

Complete when: Every authored field and repair action works with Undo/Redo in rendered editors without duplicate controls/subscriptions.

## 11. Unit U07: Native preview sessions and restoration

Outcome: BlueprintPreviewService previews declared reversible changes and restores complete authored state.

Claims: EDT-09–12, EDT-15–16, EDT-18, EDT-20, EDT-22–23.

Depends on: U02, U05.

Owned paths:

- Add: N/preview/, E/cases/snapshot_cases.gd, E/fixtures/snapshots/, docs/feedback_preview_contract.md.
- Modify: N/mod.rs after its previous writer cycle.

Required context:

- Implementation contract sections 5–6; passing U02 report/adapter; native elapsed/pause/stop and stopped-before-finished semantics.

Interfaces and invariants:

- Consumes: PreviewDescriptionV1, U03 diagnostics, U04 construction, and verified edit/serialize boundaries.
- Produces: The complete BlueprintPreviewService/PreviewStatusV1 API, pure preflight, canonical snapshot adapters, and session ownership.
- Preserves: Capture before every write, one session, no serialized preview state, and unrelated editor tweens/global settings.

Execution:

1. Preflight every child/write; reject MethodCaller, incomplete capabilities, external-scene targets, unsupported setters/resources, and unknown restoration dependencies before play.
2. Implement canonical plain-value, Node2D/Node3D transform, and Control geometry adapters. Deduplicate aliases and restore hierarchy-dependent state in the documented order.
3. Implement start/pause/resume/restore/status with session generations, fresh baselines, partial-build rollback, and retained final pose. Read native get_total_elapsed_time; do not add a ticker.
4. Distinguish stopped-before-finished success from cancellation, invalidate old callbacks before cleanup, and stop only owned handles.
5. Implement before_edit/before_serialize using U02’s proven ordering and preserve external authored edits. Custom manifests cover reset/construction writes too.

Focused preflight:

- Require passing U02 evidence on both engines and U04 runtime suites. An incomplete boundary report fails preflight.

Evidence:

- Implementer: C-EDITOR(snapshots, headless/rendered), checking actual targets/files/history and zero writes on rejection.
- Cover shared writes, aliases, parent/child transforms, completed/paused sessions, dirty scenes, partial startup failure, freed nodes, external edits, custom manifests, idempotent cleanup, and unrelated tweens.

Stop and return:

- A new restore strategy is needed, an intentional edit is overwritten, a callback survives cancellation, or passing requires weakening preview requirements.

Rollback: Remove the service/module registration; stop/restore test-owned sessions. Runtime animation/configuration stays intact.

Complete when: All restoration invariants pass at the service boundary before its UI integration.

## 12. Unit U08: Preview controls and editor lifecycle integration

Outcome: The specified controls operate native sessions and clean up through actual editor operations.

Claims: EDT-09, EDT-11–16, EDT-19–22.

Depends on: U06, U07.

Owned paths:

- Add: A/editor/preview_controls.gd, A/editor/editor_boundaries.gd, E/cases/preview_cases.gd, E/fixtures/preview/.
- Modify: A/editor/plugin.gd, A/editor/inspector.gd, docs/feedback_authoring.md through their handoffs.

Required context:

- Editor specification sections 8–10; U02 adapter; BlueprintPreviewService and PreviewStatusV1.

Interfaces and invariants:

- Consumes: Native session methods and verified host boundary callbacks.
- Produces: Play/Restart, Pause/Resume, Stop & Restore, elapsed/state/final-pose UI, and plugin-owned lifecycle forwarding.
- Preserves: Explicit playback only, one selected session, native ticking, and correct authored edit/save ordering.

Execution:

1. Implement exact labels/enabled states. Resume never calls blueprint.play; runtime-only rejection names the blocking child.
2. Forward verified selection/edit/tab/close/save/Undo/launch/reload/shutdown boundaries to the service.
3. Clean playing, paused, and completed sessions; update status without reparsing the Inspector each tick.
4. Dispose callbacks/controls on disable and filter stale session delivery. Never clear global tweens, double-tick runs, or auto-resume an outgoing tab.
5. Offer Run Current Scene guidance/action for runtime-only effects without changing Manual activation.

Focused preflight:

- U02 boundary, U06 rendered Inspector, and U07 snapshot suites pass against the selected artifacts.

Evidence:

- Implementer: C-EDITOR(preview, rendered) for every specified state/event combination plus zero-duration completion and repeated stop/disable.
- Inspect reopened files and Undo/Redo values while operating actual Controls, rather than only calling service methods.

Stop and return:

- A callback is post-operation when pre-operation ordering is required, a saved pose is animated, disable cannot restore, or processing is duplicated.

Rollback: Remove controls and host forwarding after service shutdown/restoration; retain U06 editing.

Complete when: The complete preview/lifecycle matrix passes, including absence of background-tab auto-resume.

## 13. Unit U09: Composition explanation and actionable diagnostics

Outcome: The Inspector accurately explains compositions and refreshes after native scene editing.

Claims: EDT-06–07, EDT-10, EDT-19, EDT-21, EDT-23.

Depends on: U08.

Owned paths:

- Add: A/editor/composition_summary.gd, E/cases/composition_cases.gd, E/fixtures/composition/.
- Modify: A/editor/plugin.gd, A/editor/inspector.gd, docs/feedback_authoring.md through their handoffs.

Required context:

- AuthoringModelV1 children/diagnostics, U07 canonical aliases, and native Scene dock/instance behavior.

Interfaces and invariants:

- Consumes: Actual child order, delays, resolved targets, diagnostics, and preview-safe navigation.
- Produces: Serial/parallel summary, child navigation, shared-state conflicts, and relevant layout/physics explanations.
- Preserves: Scene-tree ordering authority, inherited/instanced editability, and runtime behavior of advisory conflicts.

Execution:

1. Display actual participating children and relationship; do not invent exact total duration for unknown/custom behavior.
2. End previous previews before navigation/structural edits; invalidate on rename, reorder, nesting, duplication, deletion, and metadata notifications.
3. Explain known parallel/alias writes with the responsible nodes, without rewriting the animation.
4. Suggest appropriate visual targets/runtime testing for layout/physics cases without automatically changing hierarchy, ownership, or input flags.

Focused preflight:

- U06 Inspector, U07 alias snapshots, and U08 lifecycle suites pass.

Evidence:

- Implementer: C-EDITOR(composition, rendered) with real Scene dock edits, non-blueprint children, unowned runtime children, instances, conflicts, and repeated refresh.
- Assert UI and native playback order, preserved values, and absence of per-frame diagnostic/subscription growth.

Stop and return:

- Another stored ordering model, automatic scene restructuring, or an unverified restore alias is needed.

Rollback: Remove summary/advisory registrations, retaining U08 preview and native composition.

Complete when: Structural operations produce matching Inspector/runtime state and preserve authored data.

## 14. Unit U10: Presets and GDScript examples

Outcome: Two presets and the supplied runtime/Custom demos demonstrate zero-code and GDScript-only workflows.

Claims: EDT-17–18; RT-PRESETS.

Depends on: U09.

Owned paths:

- Add: A/presets/fade_in.tscn, A/presets/scale_pop.tscn, A/examples/, E/cases/preset_cases.gd, E/fixtures/presets/.
- Add: G/examples/tests/blueprint_presets.gd and .tscn, docs/feedback_gdscript_extensions.md.
- Modify: G/examples/tests/test_runner.gd, release/config.json, docs/feedback_authoring.md through their handoffs.

Required context:

- Implementation contract section 7 and custom/preview interfaces; native scene instancing and asset ownership rules.

Interfaces and invariants:

- Consumes: The four supplied classes and Custom hook/manifest contracts.
- Produces: Exact Fade In/Scale Pop defaults, runtime Play button/receiver, one preview-capable Custom example, and one runtime-only Custom example.
- Preserves: Preset source assets, one parameter authority, independent instances/state, and only approved distributable assets.

Execution:

1. Build the two specified presets using approved primitives and document their explicit authored colors/scales.
2. Provide self-contained demo targets; exclude unrelated legacy example assets from the customer payload.
3. Add the Method Caller runtime demo without enabling arbitrary editor execution.
4. Demonstrate reset/build/validation/write-description hooks, tool dependencies, exported parameters, and reload invalidation.
5. Document instancing/save-as-copy/custom workflows and register real preset runtime tests/counts.

Focused preflight:

- U04 effect/Custom, U08 preview, and U09 instance/navigation checks pass.

Evidence:

- Implementer: C-GDSCRIPT(blueprint_presets), C-EDITOR(presets, rendered).
- Cover two differently configured instances, undoable edits, round trips, unresolved target, plugin disabled, missing tool dependency, mixed runtime-only rejection, and unchanged source preset hashes.

Stop and return:

- Content needs another atomic effect, duplicated serialized child data, unsafe preview side effects, or undocumented third-party assets.

Rollback: Remove new content/examples and registrations, restoring expected counts; retain native authoring and preview.

Complete when: Artists instance/configure/test the presets without writing code and extend the example using only GDScript.

## 15. Unit U11: Complete customer package and enforced evidence

Outcome: The staged/extracted customer addon contains and validates the complete feature.

Claims: RT-PACKAGE; installation prerequisites for all EDT cases.

Depends on: U10.

Owned paths:

- Modify: release/tools/src/main.rs, release/tools/src/editor_test.rs, release/config.json through their handoffs.
- Add: release/tools/tests/editor_package.rs.
- Modify: release/tools/tests/pipeline.rs, release/tools/tests/desktop.rs, release/README.md, release/INSTALL.md, .github/workflows/release.yml.

Required context:

- Existing stage/seal/verify/package/archive_test policies, artifact/report identities, platform/profile/export matrix, and U01 preparation/protocol.

Interfaces and invariants:

- Consumes: Complete addon sources, platform binaries, EditorValidationV1, existing runtime/export reports, and exact identities.
- Produces: Allowlisted common addon payload, checksummed archive, enforced editor records, and extracted-package validation.
- Preserves: Existing runtime/export gates, native-only fixtures, cold-import mitigation, shell/Rust CI, ad-hoc-only Mac signing, and candidate separation.

Execution:

1. Stage scripts/plugin/icons/presets/approved demos along with libraries and seal all files; reject missing required files or inconsistent common assets across platforms.
2. Apply U01 preparation to complete class inheritance, icon, and UID/path dependencies before loading; keep bridge-dependent scripts out of incompatible native fixtures.
3. Bind editor reports to complete addon-file hash, selected library/profile, source, engine, and platform. Missing, stale, partial, or headless-only evidence cannot satisfy a required rendered gate.
4. Test fresh enablement, disable/re-enable, editor workflows, and exported configured games from the extracted archive.
5. Update CI/documentation without removing existing suite/failure/export gates. Keep incomplete artifacts explicitly candidate-only.

Focused preflight:

- Existing release CLI/package tests pass against the prior schema; inspect report/allowlist boundaries before editing.

Evidence:

- Implementer: real artifact/subprocess checks for complete/missing/corrupt files, class metadata, mismatched/stale/empty reports, and missing platform coverage.
- C-RELEASE and C-ARCHIVE on actual stages; debug/release runtime/export checks and actual target-platform editor evidence.

Stop and return:

- Acceptance requires weakening a gate, shipping caches/tests/unapproved examples, trusting stale reports, paid signing, or Python in the pipeline.

Rollback: Revert payload/report/CI policy together; do not label incomplete authoring payloads verified. Keep diagnostics as candidate output only.

Complete when: The exact extracted archive passes installation/editor/runtime/export checks and rejects deliberate failure controls.

## 16. Unit U12: Independent delivered-feature acceptance

Outcome: A reviewer traces every claimed feature to passing evidence from the immutable delivered archive.

Claims: EDT-01 through EDT-23 and RT-MODEL/RESTART/EFFECTS/COMPOSITION/CUSTOM/TRIGGERS/PRESETS/PACKAGE.

Depends on: U11.

Owned paths:

- Add: docs/designs/FEEDBACK_ACCEPTANCE_REPORT.md and evidence below release_audit_evidence/feedback_authoring/.
- No product/test implementation paths; defects return to their owning unit and require a new candidate identity.

Required context:

- The three specifications/contract, ownership/dependency table, accepted unit evidence, and exact archive/source hashes.

Interfaces and invariants:

- Consumes: Immutable customer candidate and artifact-bound unit/release reports.
- Produces: Per-claim passed/failed/not-run acceptance with explicit environment boundaries.
- Preserves: Distinctions between headless/rendered, source/package, Linux/Windows/macOS, and editor/game/export validation.

Execution:

1. Review delivered workflows/docs against the contract; verify removed/deferred features are not advertised.
2. Execute EDT-01–23 from extracted packages with success/failure cases and rendered input where required.
3. Run existing full runtime/release/export gates; early minimum/development Linux results do not replace the configured matrix.
4. Obtain actual Windows/macOS editor evidence. Mark unavailable required environments not-run and keep verified release incomplete.
5. Check isolation, transient state, unrelated tweens, fresh imports/startups, and negative controls; bind results to the accepted archive hash.

Focused preflight:

- All preceding units accepted, U02 evidence covers actual versions, and required artifact/report hashes match; reject a moving candidate.

Evidence:

- Integration reviewer: C-EDITOR(acceptance, rendered), C-RELEASE, C-ARCHIVE across required environments; include file/history assertions, logs, and relevant screenshots.
- Implementers may supply reproductions but earlier checkout runs do not substitute for reviewer candidate evidence.

Stop and return:

- A required case fails/lacks evidence, an environment is untested, a stale binary is selected, or delivery differs from the specification.

Rollback: Withdraw acceptance. Do not publish, submit, commit, or alter product behavior as a workaround.

Complete when: Every required claim has passing artifact-bound evidence and remaining limitations are explicitly outside accepted scope.

## 17. Coverage and handoff review

The editor specification owns each detailed acceptance definition. U12 repeats the complete matrix from the delivered archive. U11 additionally owns installation/archive/report failure cases outside EDT numbering.

| Claim | Implementation owners; final evidence U12 |
|---|---|
| EDT-01 | U04, U06, U12 |
| EDT-02 | U03, U06, U12 |
| EDT-03 | U03, U06, U12 |
| EDT-04 | U03, U06, U12 |
| EDT-05 | U03, U04, U06, U12 |
| EDT-06 | U03, U04, U09, U12 |
| EDT-07 | U03, U06, U09, U12 |
| EDT-08 | U05, U06, U12 |
| EDT-09 | U07, U08, U12 |
| EDT-10 | U04, U07, U09, U12 |
| EDT-11 | U07, U08, U10, U12 |
| EDT-12 | U03, U07, U08, U12 |
| EDT-13 | U02, U08, U12 |
| EDT-14 | U02, U07, U08, U12 |
| EDT-15 | U02, U06, U07, U08, U12 |
| EDT-16 | U02, U03, U07, U08, U12 |
| EDT-17 | U10, U12 |
| EDT-18 | U04, U07, U10, U12 |
| EDT-19 | U06, U08, U09, U12 |
| EDT-20 | U07, U08, U12 |
| EDT-21 | U06, U08, U09, U12 |
| EDT-22 | U06, U07, U08, U12 |
| EDT-23 | U02, U07, U09, U12 |

Every later assignment must carry this plan path, stable unit ID, inputs, whole-file ownership, consumed/produced contracts, focused checks, evidence owner, stop conditions, rollback, and acceptance unchanged. A different interface or product behavior first changes its owning contract and affected plan units. No unit includes a commit or publishing step.

Before implementation handoff, review the selected contract and confirm the authoring-model choice against the user’s current direction. U02 remains an explicit evidence-producing dependency, not an experiment claimed to have been run. This request refines planning/design documents only; implementation remains unstarted.
