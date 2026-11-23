# Feedback editor tooling

Status: **draft specification for review**. This document describes the editor experience SpireTween should provide so artists can configure, compose, reuse, and test feedback animations without writing code. Custom effects may use GDScript; using the supplied effects must not require Rust or a local compilation step.

Preview update (2026-11-20): [Blueprint editor preview](BLUEPRINT_EDITOR_PREVIEW.md) supersedes this draft's preview API, controls and lifecycle rules. The approved implementation uses synchronous restoration Callables, native Play/Stop tool buttons, automatic restoration on completion/save/scene exit, and no interception of edits or Undo/Redo during playback.

This is a sibling of [the feedback-system draft](FEEDBACK_SYSTEM_DRAFT.md). That document defines runtime behavior. This document defines editor controls, persistence, preview behavior, and acceptance criteria. Requirements below are proposed editor scope, rather than claims about functionality already implemented.

The latest runtime decision is binding: **there is no retrigger-mode enum or setting in the first implementation**. Every feedback `play()` stops its previous run, invokes its reset hook, and builds a fresh sequence. Editor tooling must not reintroduce alternative policies or inheritance controls.

Concrete engineering selections for the refined plan are recorded in the [implementation contract](FEEDBACK_IMPLEMENTATION_CONTRACT.md). They include the proposed reuse of the current native `TweenBlueprint` authoring model, real field names, helper/preview interfaces, reset and trigger semantics, and two initial presets. These selections remain planning proposals for review; the user-approved restart-only scope is unchanged.

## 1. First-release scope

The first release uses Godot's Scene dock, Inspector, scene instances, and FileSystem dock as its authoring environment. Add custom controls where they remove scripting or make configuration understandable.

| Area | Required for the first release | Deferred |
|---|---|---|
| Discoverability | Feedback types in Create Node, recognizable icons, descriptions, documented example scenes. | A separate animation workspace or onboarding wizard. |
| Atomic effects | Generic Property Tweener and Method Caller configuration. | Additional atomic effects require their own behavior specification. |
| Composition | Serial sequences and parallel blocks authored through the Scene dock. | Timeline tracks, keyframe editing, graphical node connections. |
| Inspector | Target resolution, typed value editors, property/method pickers, delay, duration, easing, trigger configuration, contextual warnings. | Expression editors, arbitrary method-argument editors, bulk retargeting. |
| Preview | Explicit property-animation preview, Play/Restart, Pause/Resume, Stop & Restore, state and elapsed-time display. | Scrubbing, seeking, reverse playback, baking, speed controls, automatic looping controls. |
| Reuse | Shipped preset scenes, normal scene instancing, duplication, and saving user presets as scenes. | A custom searchable preset browser or a new preset Resource format. |
| Customization | GDScript custom-effect authoring and a documented preview-capability contract. | Additional scripting languages or user-authored Rust extensions. |
| Reliability | Undo/Redo for authored edits, retained configuration, scene-lifecycle cleanup, and rendered editor integration tests. | Seamless live replacement of native extension binaries during an active preview. |

Method Caller is a first-release runtime effect, but arbitrary method execution is not part of in-editor preview. Section 9 specifies how its tooling remains useful without invoking gameplay code in the editor.

## 2. Artist workflows

### 2.1 Use a supplied preset

1. Open a supplied example or instance a preset scene under the scene object to animate.
2. Select the preset root. Its Inspector explains the effect and exposes its useful parameters.
3. Accept the default target or assign a scene node with the standard node picker.
4. Choose an automatic trigger, or leave playback manual for the game's existing integration.
5. Press Play in the Inspector, adjust a parameter, and preview again.
6. Save the scene. Only authored configuration is saved.

An artist must not need to edit the preset's internal child nodes for its normal advertised use. The preset catalogue and its individual parameter sets remain a separate content decision; this specification does not commit the release to an additional effect catalogue.

### 2.2 Build an animation from primitives

1. Add a sequence or block to the Scene dock.
2. Add Property Tweener or Method Caller children using Create Node.
3. Configure their targets and effect parameters in the Inspector.
4. Drag siblings into order or nest another container to combine serial and parallel sections.
5. Preview a supported leaf or the complete supported composition.

Only feedback children participate in composition, using the runtime draft's child-discovery rules. A non-feedback node must not silently become a sequence step. The Inspector must make the actual participating children visible.

### 2.3 Save and share a custom preset

Artists use Godot's existing scene-saving and instancing workflow. A saved preset contains configuration, composition, and any authored GDScript references. It does not contain runtime tween handles, preview snapshots, or object instance IDs.

A preset scene may have no target while opened alone. Show this as an unresolved configuration and explain that instancing it under an appropriate target or assigning a target will resolve it. Ship example scenes with usable targets so standalone preset authoring does not require inventing a test harness.

## 3. Scene-dock integration

Provide discoverable entries for the two atomic effects and two composition types. Their names and descriptions must distinguish a serial sequence from a parallel block. Use recognizable Godot-themed icons; recognizing a feedback node must not depend on its user-assigned name.

Reuse native scene operations for adding, renaming, moving, duplicating, deleting, and instancing feedbacks. A separate structural editor is unnecessary for the first release.

| Operation | Required result |
|---|---|
| Reorder children | Serial playback follows the updated scene-tree order. |
| Nest or reparent feedbacks | Target resolution, parent-managed trigger presentation, warnings, and composition summaries refresh. |
| Duplicate a feedback or subtree | Configuration is copied; runtime and preview state are not shared. |
| Instance a preset twice | Each instance resolves its own targets and has independent playback state. |
| Delete a selected or playing feedback | Its preview ends, with surviving target properties restored. |
| Inspect an inherited or instanced scene | Respect Godot's existing editability and override rules. |

Do not automatically make scene children editable, change scene ownership, flatten a preset, or rewrite scene structure to remove a warning. Offer an explanation and a relevant navigation action instead.

## 4. Common Inspector

Keep the Inspector organized around the artist's decisions. The proposed order is:

| Section | Contents |
|---|---|
| Identity and status | Effect description, configuration summary, errors or warnings. |
| Preview | Playback controls, state, and elapsed time; present even when preview is unavailable, with a reason. |
| Target | This feedback's configured target, resolved target, and resolution source. |
| Effect | Property or method selection and effect-specific values. |
| Timing | Delay, duration where relevant, and easing where relevant. |
| Automatic trigger | Trigger condition and its condition-specific fields. |
| Composition | Participating children and their serial/parallel relationship, for containers. |

Use readable labels such as **Target**, **Delay**, and **End Value** rather than requiring artists to understand internal field names. Native Inspector conventions for reverting values, resource selection, keyboard focus, and tooltips should continue to work.

Custom GDScript exports remain visible. The editor must not replace their Inspector with a fixed list that hides parameters supplied by a custom effect.

### 4.1 Target selection and explanation

The configured target uses Godot's normal scene-node picker and drag-and-drop behavior. A read-only resolved-target row displays the target's name, class, and scene path. A navigation action selects that node, ending any active preview before selection changes.

The resolution-source label must distinguish:

- This feedback's configured target.
- The first non-feedback ancestor used as a fallback.
- An unresolved target.

Editor inspection, preview, and runtime playback use each feedback's own configured target or ancestor fallback. Containers do not propagate their targets to descendants.

Editor reflection and runtime playback must share target-resolution logic. Duplicating that logic in a property picker would allow the Inspector to describe one object while the animation changes another.

### 4.2 Timing

Delay and duration fields use seconds. Reject non-finite input and prevent negative values. Duration-zero behavior must match the native runtime; the Inspector must not invent a separate interpretation.

Show duration and easing only on effects to which they apply. A Method Caller exposes delay but has no artificial tween duration. Container delay is additional to child delays.

The easing selector uses SpireTween's supported names and values. Preserve the configured value across refreshes. Explain overshoot where applicable; do not clamp a supported easing curve into a different animation merely to simplify the editor display.

An easing-curve visualization is an enhancement, not a dependency of the first release.

### 4.3 Automatic triggers

Only the outermost composed feedback subscribes to automatic events. Automatic event subscriptions must remain inactive in the editor, including mouse events and entering the edited scene tree. Preview runs only when the artist explicitly starts it.

| Placement | Inspector presentation |
|---|---|
| Outermost feedback | Trigger condition is visible and editable. |
| Feedback managed by a containing feedback | Its stored trigger condition stays visible and read-only, with **Automatic triggers are managed by the parent**. |

Moving a child out of a composition restores editability and its previously stored setting. Hide condition-specific controls that are irrelevant to the selected trigger; preserve their stored values rather than clearing them when hidden.

| Trigger condition | Required authoring support |
|---|---|
| Manual | Explain that gameplay code or another feedback calls `play()`. No extra binding fields. |
| Enter Tree | Explain the configured runtime event. Editor tree entry never fires the animation automatically. |
| Mouse enter/exit | Identify the event source and validate that it can supply the selected event. |
| Left/right click | Identify the event source and explain which button event the runtime handles. |

Other signal-driven playback uses Godot's built-in Signals dock to connect a source signal to `play()` in Manual mode. Provide no addon signal picker or automatic arbitrary-signal binding logic. Mouse source and input-event conventions follow the shared runtime contract.

There are no retrigger controls, parent-policy selectors, or inheritance labels.

## 5. Generic Property Tweener Inspector

### 5.1 Property picker

List writable target properties with types supported by SpireTween's generic property-tween path. Derive support from the runtime's actual dispatch capabilities rather than presenting every Godot Variant type as tweenable.

Group properties by their normal Inspector category when metadata permits. Allow searching by property name. Display the selected property's exact name and type, even if the picker uses a friendlier label.

The first-release picker covers direct properties named by the runtime draft's `property_name`. Subproperties, resource traversal, array entries, dictionary entries, and shader uniforms are not automatically included. They require an explicit runtime path/type contract before editor support is promised.

When an existing selection becomes invalid, retain its name and values, show the reason, and offer the picker to repair it. Do not silently select a different property.

### 5.2 Typed values

| Field | Required behavior |
|---|---|
| Use Start Value | Disabled by default. Toggling it preserves the stored start value. |
| Start Value | Uses the selected property's type and applicable hints; visible when enabled. |
| End Value | Uses the same type and applicable hints. |
| Duration | Seconds, using the native effect's duration semantics. |
| Easing | A supported SpireTween easing value. |

Retain relevant hints such as numeric units, ranges, enum labels, and color editing behavior when they describe the selected property correctly. Do not copy hints that would change the meaning of a different dynamic field.

Provide **Copy Current to Start** and **Copy Current to End** actions for readable, compatible values. Copying to Start also enables Use Start Value as one undoable edit. These actions read the authored target value; while a preview pose exists, end the preview and restore its baseline before reading.

When Use Start Value is disabled, display **Read when the tween starts**. Do not materialize a captured start value in scene data merely by inspecting or previewing the feedback.

### 5.3 Refresh and serialization

Changing the target, property name, hierarchy, or relevant script/property metadata refreshes the dynamic fields and warnings.

Compatible values survive a refresh. If the new property is incompatible, retain the previous typed payload and show an explicit mismatch. Offer **Reset Values for Selected Property** as an undoable repair action; choose an initialization value from the readable target or the selected type's default and show what will be used. Do not overwrite an incompatible payload simply because the Inspector was rebuilt.

Saving with a temporarily unresolved target must preserve the selected name, value types, start-enabled state, and configured values. Inspector visibility and persistent storage are separate concerns. This uses the dynamic-property and storage mechanisms described by [Godot's Object API](https://docs.godotengine.org/en/4.4/classes/class_object.html#class-object-private-method-get-property-list).

## 6. Method Caller Inspector

Provide a searchable method picker showing each method's name and signature. A valid selection has zero required arguments: no parameters, all fixed parameters defaulted, or optional variadic arguments. Display invalid methods as unavailable with the reason, or provide a clearly labelled invalid-results filter; never make them look callable by this effect.

The selected method executes when its sequence step is reached. Its return value is ignored and the sequence does not wait for asynchronous work it starts. State those two facts next to the field or in its tooltip.

Reflection must not call methods to discover their signatures. Use available native/script metadata; do not instantiate gameplay behavior merely to populate a list. If editor metadata is incomplete, show **Could not validate in the editor** and preserve the selection for runtime validation.

There is no **Call Now** button in the first release. Method execution belongs to running the game or a dedicated example scene, as described in section 9.

## 7. Composition and preset presentation

### 7.1 Composition summary

A container's Inspector shows a read-only summary of participating feedback children in scene-tree order. Each entry shows the node name, effect type, delay, and target status. Indicate **Sequential** or **Parallel** prominently at the container level.

Selecting an entry navigates to the corresponding scene node. Reordering uses the Scene dock; the summary must not become a second independently stored ordering system.

Show parent and child delays separately. Do not display a calculated total duration as exact when lazy construction, custom behavior, or loops make it unknown. Elapsed preview time remains useful without an estimated total.

Warn when children write the same target property in parallel. This is an advisory conflict, not an automatic rewrite of valid runtime behavior. Identify the participating nodes so the artist can choose a serial arrangement or different targets.

### 7.2 Preset scenes

Each supplied preset should have a description, useful defaults, a compact parameter set, and an example target or demo scene. Use named intent-oriented controls where those are part of the preset's design; keep the lower-level composition available through normal scene inspection.

Preset controls and child configuration must have one source of truth. If a root parameter drives multiple child values, that relationship must be implemented deliberately, survive scene loading, and work with Undo/Redo. Inspector refresh must not silently regenerate and overwrite artist-edited children.

Provide documentation for making an independent copy or editable instance using Godot's existing workflow. Loading an example must not modify the distributed preset asset.

## 8. In-editor preview

### 8.1 Session ownership

The editor has at most one feedback preview session at a time. Starting a preview first stops and restores the previous session, including a previous preview of the same feedback. This is an editor-session rule, not a new runtime retrigger policy.

A session records the active edited scene, selected feedback subtree, resolved targets, properties it owns, baseline values, its runtime sequences, and an internal session identity. Late callbacks from an old session must not stop or restore a newer session.

Preview a selected container to run its composition. Preview a selected leaf in isolation using the same target-resolution context it has within its authored hierarchy. This does not also start its siblings or its automatic trigger subscriptions.

All preview targets must belong to the active edited scene. Refuse to preview editor UI objects, another scene tab, or objects outside that scene. This restriction applies to preview; it does not redefine the runtime target API.

### 8.2 Controls and states

| State | Play control | Pause control | Stop & Restore |
|---|---|---|---|
| Idle | **Play** validates, captures a baseline, and starts a fresh run. | Disabled. | Disabled. |
| Playing | **Restart** restores the baseline and starts a fresh run. | **Pause** freezes the preview. | Stops owned runs and restores the baseline. |
| Paused | **Restart** restores the baseline and starts a fresh run. | **Resume** continues the existing run. | Stops owned runs and restores the baseline. |
| Completed | **Play Again** restores the baseline and starts a fresh run. | Disabled. | Restores the baseline. |
| Unavailable | Disabled with a specific reason. | Disabled. | Available if a previous session still needs cleanup. |

Resume must use the existing runtime run's resume operation. Calling `Feedback.play()` would restart it and is therefore incorrect for this button.

Natural completion leaves the final pose visible until the artist restores it or another lifecycle event ends the session. Display **Preview pose — Stop & Restore to return to authored values** while retaining that pose. Completed sessions still own their baseline and require cleanup.

Show current state and elapsed playback time, including delay. Paused time does not advance the displayed playback time. A callback-only or zero-duration composition can complete immediately. Unknown or unbounded duration must not produce a misleading progress percentage.

### 8.3 Start procedure

1. End and restore an existing preview session.
2. Resolve the selected subtree and validate every participating effect and target.
3. Verify that every effect supports editor preview and declares all properties it may write.
4. Capture all baseline values before calling `play()`, reset hooks, builders, or tween constructors that may modify targets.
5. Create a fresh run through the runtime's normal feedback path, then expose the Playing state.

If validation fails, report all actionable failures without changing target values. If construction fails after a baseline has been captured, stop any partially constructed owned runs and restore all surviving targets.

### 8.4 Baselines and restoration

Snapshot each affected object/property pair once for the selected composition. Shared writes must restore one authored value, rather than several intermediate values captured after another child has already changed the property.

Account for direct properties that describe shared state, such as local/global position or a transform and its components. Baseline capture and restoration must use a consistent representation and dependency order, including when both a parent and child transform are animated. Independent snapshots of aliases must not restore conflicting poses. If a property's restoration behavior is not supported, report that limitation before starting its preview.

First-release preview is limited to supported direct Node properties with usable value snapshots. Arbitrary mutation of shared Resources, nested containers of data, or other external state is not covered by shallow Variant copies and must not be accepted as if it were restorable.

Stop & Restore stops owned sequences before writing baseline values. Pending callbacks must not execute after restoration. Cleanup is idempotent: natural completion followed by scene change, repeated stop requests, or invalidated targets must not double-restore or access freed objects.

Read fresh baselines for each new session. Do not cache them permanently in `_ready()`. Preview restoration is editor bookkeeping and does not decide whether a runtime effect's `_reset_state()` restores an original pose or retains an interrupted value.

### 8.5 Runtime integration

Use the existing tool-enabled `TweensController` and the native tween/sequence implementation. Do not introduce an independent GDScript interpolation engine or tick a registered run a second time from the preview UI.

Stop only sequences owned by the preview session. Other tools may use SpireTween in the editor; preview cleanup must not clear the global manager, pause the editor SceneTree, or change the global time scale.

Completion and cancellation are distinct. SpireTween emits `stopped` on successful completion before `finished`; a stopped notification alone must not classify a successful preview as cancellation or restore its final pose too early. The preview state machine must handle that signal ordering explicitly.

## 9. Method calls and custom GDScript effects

The first release previews the supported Property Tweener and containers composed entirely of preview-capable children. A Method Caller or unsupported custom effect makes preview of the selected composition unavailable. Identify the exact child that prevents preview; do not silently skip it and show a different animation as the complete result.

Offer **Run Current Scene** or explain Godot's normal run-scene workflow for runtime-only behavior. Running the scene does not automatically change a Manual feedback into an automatic trigger. Supplied runtime demo scenes should include a simple GDScript playback button so artists can test manual effects without writing a harness.

A custom GDScript effect can participate in editor preview only when it provides the documented preview contract:

| Capability | Required declaration or behavior |
|---|---|
| Editor execution | The script and editor dependencies are available in tool mode. |
| Validation | Configuration can be checked without starting gameplay or changing target state. |
| Affected properties | All writes from reset, construction, and playback can be enumerated before execution. |
| Playback | It builds native SpireTween operations through the same runtime effect hook used in-game. |
| Cleanup | Its declared changes can be restored, and any additional supported temporary state has explicit cleanup. |

The exact Rust/GDScript hook signatures remain an implementation design decision. No custom preview callback is required from an artist using shipped effects. Editor-only hooks must not be required to run the shipped effect in an exported game.

An arbitrary method becoming callable in a tool script does not automatically make its side effects restorable. First-release preview does not offer a blanket method-execution opt-in. See [Godot's tool-script documentation](https://docs.godotengine.org/en/4.4/tutorials/plugins/running_code_in_the_editor.html) for the underlying editor execution model.

## 10. Scene lifecycle and interrupted editing

The editor controller is attached to the editor SceneTree root, rather than the currently edited scene. The [local controller experiments](../../release_audit_evidence/editor_controller_2026_11_03/README.md) verified that this root and controller survive scene-tab operations on Godot 4.7.2. Switching tabs detaches the old scene, while closing its tab frees its nodes later.

Preview must end explicitly on tab changes; relying on node-bound tween pausing would allow an old preview to resume when the artist returns.

| Event | Required action |
|---|---|
| Select a different feedback or another scene object | Stop and restore the old preview before inspecting or editing the new selection. |
| Change a feedback parameter, target, property, or composition | End the old preview before committing the authored edit. Refresh validation; leave replay explicit. |
| Open or activate another scene tab | Stop and restore the outgoing scene's preview; do not restart it when the tab becomes active again. |
| Close the scene | Restore targets while they still exist, stop owned runs, and release references. Skip targets already freed. |
| Remove, reparent, or free a target | Invalidate the session, restore surviving targets where applicable, and explain which target changed. |
| Save, Save All, or an editor-triggered save | Restore before serialization; save authored state; leave preview stopped afterward. |
| Undo or Redo | End preview without overwriting the user's requested history operation. Preview values must not become history baselines. |
| Run the project or current scene | Restore editor values before saving/launching; keep the launched game's playback independent. |
| Disable editor tooling or close the editor | Restore surviving targets and disconnect/dispose preview callbacks and controls. |
| Relevant GDScript reload | End the old session without invoking stale script callbacks; rebuild metadata before another preview. |

Cleanup must also cover paused and completed sessions. Do not infer that a target has already been freed merely because a scene-closed signal arrived.

Preserve intentional edits made by the user or another editor tool. A restoration pass must not overwrite a later authored edit because it holds an older snapshot. The ordering of edit interception, snapshot restoration, and Undo/Redo is an implementation validation task, not something that can be assumed from a post-edit notification.

For saves and game launches, investigate the editor's pre-operation hooks, including [`EditorPlugin._apply_changes()`](https://docs.godotengine.org/en/4.4/classes/class_editorplugin.html#class-editorplugin-private-method-apply-changes). A scene-saved notification happens too late to prevent an animated pose from being serialized. Verify the complete path for each supported editor version.

Native extension hot reload during an active preview is deferred. Normal plugin disable/shutdown must still clean up its session; seamless recovery across replacement of the native binary is not a first-release feature.

## 11. Validation and diagnostics

Show scene-node configuration warnings and relevant inline Inspector messages. Keep the diagnosis close to the field that needs repair. Prefer a concrete explanation such as **Target has no writable property 'scale'** over a generic invalid-feedback message.

| Condition | Presentation and action |
|---|---|
| No effective target | Explain the resolution rule and offer target selection. Disable preview. |
| Selected property missing, read-only, or unsupported | Preserve configuration, identify the reason, and offer property selection. Disable preview. |
| Stored value incompatible with selected property | Preserve the payload and offer the explicit reset-values repair action. Disable preview. |
| Invalid timing | Highlight the field; do not start a preview with invalid data. |
| Method requires arguments or no longer exists | Explain the signature problem and preserve the selected name. Runtime revalidates independently. |
| Mouse trigger source unavailable | Identify the source problem. An otherwise valid manually started property preview can remain available. |
| Editor reflection incomplete | Distinguish unknown from confirmed invalid. Explain that runtime validation is still required. |
| Child prevents complete preview | Name the child and its limitation; disable preview of that selected composition. |
| Parallel writes to the same property | Show an advisory conflict with links to the involved feedback nodes. |
| Layout or physics owns the animated state | Explain the limitation and suggest an appropriate visual target or wrapper. |

For example, positioning a Control managed by a [Container](https://docs.godotengine.org/en/4.4/classes/class_container.html) can fight layout. A warning should point artists toward animating a visual wrapper, without changing their hierarchy automatically. Moving a physics body in the editor is not evidence that its physics interaction matches a running game; unsupported physics-dependent preview should direct users to runtime testing.

Warnings refresh on relevant edits, reparenting, target deletion/recreation, script changes, and explicit property-list notifications. Report a persistent problem once in the UI rather than logging it every frame. Runtime diagnostics remain available when a game supplies a different target from the one inspected.

## 12. Undo/Redo, saving, and scene ownership

Authored changes must use normal editor property-change handling or the [`EditorUndoRedoManager`](https://docs.godotengine.org/en/4.4/classes/class_editorundoredomanager.html). Custom Inspector fields should integrate through [`EditorProperty`](https://docs.godotengine.org/en/4.4/classes/class_editorproperty.html), rather than writing values directly and bypassing the editor's change history.

Treat combined actions as one edit: copying Current to Start plus enabling Start Value, or resetting both value payloads after changing property type, should undo coherently. Continuous numeric dragging should merge using native Inspector conventions.

| Persisted scene data | Transient editor data |
|---|---|
| Feedback hierarchy and child order. | Active sequences and session identities. |
| Authored node references. | Resolved object instance IDs and reflection caches. |
| Trigger configuration. | Editor event subscriptions for diagnostics and UI. |
| Property/method names and typed value payloads. | Preview snapshots and retained preview poses. |
| Delay, duration, easing, and preset parameters. | Elapsed time, state, selection highlights, and UI expansion state. |

Serialize node references using Godot's scene-reference mechanisms. Never save runtime IDs or machine-specific absolute node/asset paths as the target-binding format.

Preview does not create Undo/Redo actions, mark the scene dirty solely for animation, or clear a pre-existing dirty state. Metadata refresh is also not an authored edit. Saving or reloading with editor tooling disabled must retain the configuration, including dynamic values whose targets are unresolved.

The first release does not provide **Keep Preview Pose**, **Apply Preview**, or baking. Those operations would need explicit authoring and undo semantics separate from temporary playback.

## 13. Editor extension structure and performance

Use an [`EditorInspectorPlugin`](https://docs.godotengine.org/en/4.4/classes/class_editorinspectorplugin.html) for Inspector additions and property editors, with an editor integration component owning selection, scene, save, and preview-session lifecycle handling. Node-side metadata and configuration-warning hooks should remain usable without a custom dock.

Runtime configuration and target-resolution logic belong in the native feedback system. GDScript wrappers or custom effects can expose exported parameters and override supported hooks. This specification does not assume that one Rust Godot class can inherit another; the native/GDScript class arrangement remains a separate architecture decision.

Loading the addon, opening a scene, selecting a node, or rebuilding the Inspector must not play a feedback. Having a tool-enabled tween controller does not make automatic gameplay triggers active in the editor.

Cache reflection results and invalidate them on relevant changes. Do not scan every scene node or rebuild all dynamic properties on every editor frame. Update preview labels without re-parsing the entire Inspector each tick.

Refreshing the Inspector must not duplicate buttons, signal connections, or preview sessions. Enabling/disabling the editor integration repeatedly must not accumulate controls or subscribers. Multiple scene tabs must not create independent global preview managers.

Reuse Godot themes, scalable controls, tooltips, keyboard focus, and standard file/node pickers. First-release preview is single-selection only; multi-selection must disable the preview controls with an explanation rather than unexpectedly run several effects.

## 14. Acceptance tests

These are required implementation gates, not tests already completed by writing this document. Use real Godot editor integration tests. Runtime-only tests, mocked property lists, and headless editor checks cannot substitute for the rendered artist workflow.

Run the editor acceptance suite on the supported minimum Godot version and the development version. The existing controller-lifecycle evidence covers Godot 4.7.2; it does not establish the complete editor feature set on Godot 4.4 or any other version.

| ID | Exercise | Required result |
|---|---|---|
| EDT-01 | Create both atomic effects and both container types using the editor. | Discoverable types, usable labels, expected configuration sections. |
| EDT-02 | Select properties of several actually supported types; assign a missing/read-only/unsupported property. | Correct typed fields and hints; actionable failure states without data loss. |
| EDT-03 | Save/reload configured values with a valid target, then with an unresolved target. | Names, types, enabled state, and payloads survive both round trips. |
| EDT-04 | Change between compatible and incompatible property types; undo repairs. | Compatible values persist; incompatible payloads require explicit repair; undo restores configuration. |
| EDT-05 | Select no-argument, default-argument, variadic, missing, and required-argument methods. | Reflection matches runtime validity and never invokes the selected methods. |
| EDT-06 | Reorder, nest, duplicate, and reparent feedback subtrees. | Composition, target resolution, and trigger presentation update correctly; instances remain independent. |
| EDT-07 | Configure different targets on nested containers and children, including a child using ancestor fallback. | Each child resolves independently; Inspector reports the same targets the runtime actually changes. |
| EDT-08 | Trigger normal mouse/editor navigation while editing; separately run the scene and exercise configured triggers. | No automatic editor playback; runtime trigger bindings follow their specified contract. |
| EDT-09 | Play, restart during delay/playback, pause, resume, complete, and Stop & Restore. | Fresh runs on restart, continuation on resume, correct final-pose and restoration states. |
| EDT-10 | Preview serial and parallel compositions, including shared-property writes. | Runtime ordering is preserved; baseline capture precedes all writes; conflicts are explained. |
| EDT-11 | Preview a Method Caller, unsupported custom effect, or mixed container. | Whole selected preview is unavailable with the responsible node identified; no callback is executed or silently skipped. |
| EDT-12 | Fail validation or construction, and delete a target during preview. | No preflight mutation; partial runs are stopped; surviving values restore without accessing freed objects. |
| EDT-13 | Change selection, switch tabs, close a scene, and close every scene tab while playing/paused/completed. | Preview is cleaned up in all states; returning to a tab never resumes an old preview. |
| EDT-14 | Save, Save All, and launch the scene while a playing/paused/completed preview pose exists. | Files and the launched game receive authored values; preview stays stopped afterward. |
| EDT-15 | Edit parameters and target properties, then Undo/Redo around a preview. | Intentional edits survive; history contains authored values; preview adds no history entries. |
| EDT-16 | Preview an already-dirty scene; restore and save it. | Existing authored edits survive; preview does not clear dirty state or serialize transient fields. |
| EDT-17 | Instantiate a shipped preset twice, customize one instance, save/reload, and inspect another scene. | No shared playback state or modifications to the supplied asset. |
| EDT-18 | Exercise declared custom GDScript preview support, missing tool dependencies, and metadata refresh after script edits. | Supported hooks work; missing capabilities are explained without executing unsupported behavior. |
| EDT-19 | Rebuild the Inspector repeatedly; disable/re-enable tooling and close the editor during preview. | No duplicated UI/subscribers, retained preview writes, or stale callbacks. |
| EDT-20 | Keep an unrelated editor tween running while starting/stopping a feedback preview. | Cleanup affects only the preview-owned runs. |
| EDT-21 | Use narrow/wide Inspector widths, alternate editor themes/scales, and keyboard navigation. | Controls remain readable, reachable, and correctly labelled. |
| EDT-22 | Select several feedbacks or a feedback targeting an object outside the active edited scene. | Preview is unavailable with a clear explanation; no unexpected multi-object mutation. |
| EDT-23 | Animate parent/child transforms and local/global aliases; stop, save, and replay. | Restoration produces the authored pose regardless of child traversal order, or the unsupported property is rejected before mutation. |

Retain editor version, addon build identity, test scene, assertions, relevant logs, and rendered screenshots for acceptance. Test the actual custom controls for the artist workflow; API-driven probes remain useful for native lifetime and serialization checks.

## 15. Selected planning contracts and remaining evidence

The refined plan records the following decisions in their owning implementation contract, so a developer does not select product behavior while building controls:

| Topic | Selected planning contract or required evidence |
|---|---|
| Runtime reset state | Built-ins do not rewind runtime targets; implicit starts read the current value when the native tween starts. Editor baseline restoration remains separate. |
| Target resolution | Each feedback uses its own configured target or first non-feedback ancestor; containers do not propagate targets. |
| Trigger binding | Optional trigger_source defaults to the resolved target; mouse clicks use matching pressed events and addon bindings are runtime-only. Other signals use Godot's native scene connections to play. |
| Native/GDScript arrangement | Proposed single native TweenBlueprint with four thin discoverable role scripts and Custom hooks; see contract sections 1–3. |
| Save and Undo/Redo interception | Unit U02 must produce passing real-editor boundary evidence before U07/U08 can be accepted. This proof has not been run during planning. |
| Shipped presets | Fade In and Scale Pop use the exact parameters/defaults in contract section 7; a Method Caller demo runs in-game. |

Review the selected contract before implementation assignment. If a selection changes, update its owning contract and affected units together. A failed lifecycle proof returns to design review instead of weakening preview restoration or adding unrelated tooling.
