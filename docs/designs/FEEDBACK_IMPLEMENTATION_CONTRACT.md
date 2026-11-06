# Feedback implementation contract

Status: **selected planning baseline for review, not implementation approval**. This document makes the interfaces consumed by the [refined implementation plan](FEEDBACK_IMPLEMENTATION_PLAN.md) concrete. The user's fixed-restart decision remains authoritative. Other selections below are engineering/design proposals recorded explicitly so an implementer does not have to choose them inside a work unit.

Implementation update (2026-11-05): the authorized handoff covers the current TweenBlueprint runtime and built-in Inspector. Its implemented API is documented in [TweenBlueprint authoring](../tween_blueprint.md). The broader preview/preset interfaces below remain proposals. The current model uses attached GDScript to override `_play_internal`, provides parameterless `play()`, and does not require a Custom kind or thin wrapper scripts. Each blueprint independently resolves its configured target or first non-blueprint ancestor; target overrides and propagation have been removed by user decision. Synchronous same-instance reentry is explicitly unsupported; generation tracking is not required.

## 1. One native authoring model

Extend the existing native `TweenBlueprint` in `spire_tween/src/blueprint/mod.rs`. Do not add a competing `SpireFeedback` native hierarchy. The conceptual Feedback, Property, Method, Sequence, and Block names in the runtime draft describe roles implemented by this model.

This selection incorporates the in-progress authoring node observed on 2026-11-04; it does not claim that its unfinished implementation has been validated. Its whole-file changes must be reconciled with the current writer before implementation. If the user selects a different model, revise this contract and dependent units before assigning them.

Use these thin addon GDScript classes for Create Node discoverability:

| Script class | Native kind | Purpose |
|---|---|---|
| `SpirePropertyFeedback` | `Property` | Generic direct-property animation. |
| `SpireMethodFeedback` | `MethodCaller` | Zero-required-argument method callback. |
| `SpireFeedbackSequence` | `ChainBlock` | Serial direct children. |
| `SpireFeedbackBlock` | `ParallelBlock` | Parallel direct children. |

These scripts extend `TweenBlueprint`; they do not implement interpolation, an alternate trigger system, or duplicated serialized effect data. Keep a custom `EditorPlugin` entry in GDScript and native preview state in `BlueprintPreviewService : RefCounted`. Do not register a second native EditorPlugin alongside that entry.

Preserve `BlueprintKind` values `ChainBlock=0`, `ParallelBlock=1`, and `Property=2`; append `MethodCaller=3` and `Custom=4`. Custom GDScript effects select Custom and override the custom hooks below. Built-in kind construction stays native.

## 2. Serialized configuration and defaults

Keep the prototype's existing serialized field names and enum ordinals. In particular, the Property field remains `animated_property`; `property_name` in the conceptual draft maps to that field. Do not introduce a second persistent dictionary mirroring the same values.

| Field or field group | Contract |
|---|---|
| `target` | Optional scene-node reference; null enables fallback resolution. Replace reliance on an invalid `OnEditor` value with explicit optional access. Saved references use Godot's scene mechanism. |
| `delay` | Finite, non-negative seconds; default 0. |
| `kind` | Existing native kind; the four supplied scripts choose their fixed kind. |
| `animated_property` | Direct property StringName, initially empty. |
| `start_enabled` | New Boolean, default false. |
| `start_value`, `end_value` | Sole stored Variant payloads; retain their actual types even when hidden, disabled, unresolved, or incompatible. |
| `speed_or_duration`, `lerp_mode`, `ease_basic`, `ease_curve` | Retain the prototype fields. The first-release authoring roles use Absolute mode and duration semantics; unsupported stored authoring modes diagnose and skip without changing the saved value. Existing native tween APIs keep all their modes. |
| `method_name` | New StringName, initially empty; relevant to MethodCaller. |
| `trigger_source` | Optional mouse-event source node; otherwise use the resolved target. |
| Existing native speed/process/pause/loop fields | Preserve and forward to native tween configuration. No additional preview speed/loop controls are introduced. |

The updated prototype uses `TriggerCondition` ordinals `Manual=0`, `AfterReady=1`, `AfterEnterTree=2`, mouse enter/exit 3/4, and left/right click 5/6. Preserve the prototype's Manual default and these ordinals. Other signal-driven playback uses Godot's native scene connections to `play()` in Manual mode; no addon signal-trigger mode is provided.

New instances use native operational defaults: speed_scale 1, loops 1, ignore_time_scale false, and native default pause/process/loop modes. Supplied Property feedbacks default to duration 1 and Linear easing, with no explicit start. Preserve explicitly serialized values, including zero speed, instead of rewriting existing data to those defaults. Basic easing applies unless a configured valid ease_curve selects native curve easing; editing its graph is not a new custom curve editor in this release.

Every configurable property retains STORAGE usage independently of its EDITOR visibility. A refresh never erases typed payloads. If both stored values are nil on first property selection, that one authored action initializes them from a readable selected value or its supported type default. Changing an already configured incompatible type retains the old payload and requires the explicit repair action specified in the editor draft.

Existing source declarations are not evidence of a released serialization schema. Before editing them, search saved scenes/resources for their consumers. If consumers need a conversion that cannot preserve names/types/values, return a migration proposal rather than silently rewriting their assets.

## 3. Runtime interface and execution

The following signatures are Godot-facing contracts; native helpers may use idiomatic internal Rust types. Godot-facing methods do not return Rust Result values.

```yaml
TweenBlueprint:
  methods:
    play: "() -> SpireSequence"
    stop: "() -> void"
    get_authoring_model: "() -> Dictionary"
    get_preview_description: "() -> Dictionary"
    append_property_step: "(sequence: SpireSequence, target: Node, property: StringName, start_enabled: bool, start: Variant, end: Variant, duration: float, ease: int) -> void"
    append_method_step: "(sequence: SpireSequence, target: Node, method: StringName) -> void"
  script_virtuals:
    _reset_state: "() -> void; default no-op"
    _build_custom: "(sequence: SpireSequence, context: Dictionary) -> void; Custom kind only"
    _validate_custom: "(context: Dictionary) -> PackedStringArray; default empty"
    _describe_custom_preview: "(context: Dictionary) -> Dictionary; default runtime_only"
  existing_signal_payloads:
    tween_spawned: "Variant containing the fresh SpireSequence"
    tween_stopped: "Variant containing the exact stopped sequence"
    tween_loop_finished: "Variant containing the exact looped sequence"
    tween_finished: "Variant containing the exact successfully completed sequence"
  runtime_context:
    target: "Node or null"
```

Virtual dispatch uses `#[func(virtual, gd_self)]` with native borrow guards released before calling a script. Regular helper methods provide reusable native behavior; scripts must not depend on calling a Rust virtual default through `super`.

Fixed restart applies to manual, triggered, and composed calls. Take the old handle out of node state, stop that handle, invoke reset, construct/configure a new sequence, populate it, then publish the new handle. Calling play again on the same instance from a hook or callback before the current play call returns is unsupported; no generation tracking is required. Stop takes and stops the current handle without forcing completion.

Emit tween_spawned once after the fresh handle is configured and before its builder runs. This permits preview/lifecycle observers to attach before a custom builder can complete or stop it synchronously. Forward the native lifecycle signals with that exact handle; observers filter by handle/session identity rather than treating every node signal as the current run. Native success ordering remains stopped before finished. Forwarding must not retain stale node callbacks after teardown.

The built-in Property, Method, and container reset implementations are no-ops. They do not rewind a runtime target or apply an editor snapshot. Property animation without explicit start therefore continues from the value present after interruption, when the new tween actually begins.

Containers retain the core draft's **eager child construction**: enumerate direct TweenBlueprint children, call each child's play while constructing the parent, and immediately append/join its fresh sequence. This deliberately does not adopt the old plan's unselected lazy-child recommendation. Native pending-start evaluation still occurs after the child's delay/preceding steps. Runtime-added unowned children participate; non-blueprint nodes do not.

Native sequence loops/replay keep their existing meaning and reuse constructed children. They are not new authoring requests. Restart guarantees concern calls through TweenBlueprint.play; directly replaying a superseded returned native handle is outside the authoring-node contract. Do not redesign the native sequence engine to add independent child policies.

Invalid runtime effects warn and produce/retain a usable empty sequence or skipped step; they do not hang subsequent valid steps. Editor preflight is stricter: a configuration or capability failure rejects the entire selected preview before writes.

## 4. Target and trigger rules

Each blueprint resolves its own configured target, then its first non-blueprint ancestor. Containers call each child's parameterless play method and do not propagate targets. An ancestor blueprint's configured target does not affect a descendant's resolution, including during editor inspection.

Only outermost blueprints connect automatic triggers. Manual never connects. AfterReady runs once after runtime ready; AfterEnterTree runs after each runtime entry when references are available. Editor entry/ready and editor input never trigger playback.

For mouse modes, use `trigger_source` when configured, otherwise the resolved target. Support Control and CollisionObject2D/3D mouse sources using their native enter/exit and input signals. Left/right click means a pressed InputEventMouseButton with the matching button; ignore release and motion events. Diagnose missing input-picking prerequisites rather than enabling scene flags automatically. Exit/reparent cleanup prevents duplicate or stale mouse activations. Users connect other signals to `play()` through Godot's built-in editor; those connections are not managed by the addon.

## 5. Read models consumed by the Inspector

```yaml
AuthoringModelV1:
  schema: 1
  kind: "BlueprintKind integer"
  configured_target: "Node or null"
  resolved_target: "Node or null"
  target_source: "configured | ancestor | unresolved"
  target_source_node: "Node or null; the node supplying the configured target or ancestor fallback"
  manages_children: "boolean"
  trigger_managed_by_parent: "boolean"
  property_descriptors: "Array[Dictionary{name: StringName, type: int, hint: int, hint_string: String, writable: bool, supported: bool}]"
  method_descriptors: "Array[Dictionary{name: StringName, required_arguments: int, signature: String, valid: bool}]"
  children: "Array[Dictionary{node: Node, kind: int, delay: float, resolved_target: Node or null}]"
  diagnostics: "Array[DiagnosticV1]"
DiagnosticV1:
  code: "stable StringName"
  severity: "error | warning | information"
  node: "Node"
  field: "StringName; empty for a node-level condition"
  message: "actionable String"
  blocks_runtime: "boolean"
  blocks_preview: "boolean"
```

These are transient transport dictionaries, not a second serialization format. Property-type support comes from the existing native property/Variant dispatch; no new independently maintained editor type allowlist. Read model construction does not call gameplay methods or change scene values. A missing property is different from editor metadata that cannot be inspected.

## 6. Preview interface and restoration

```yaml
BlueprintPreviewService:
  methods:
    inspect: "(blueprint: TweenBlueprint) -> Dictionary; pure preflight"
    start: "(blueprint: TweenBlueprint) -> bool; restores an old session first"
    pause: "() -> void"
    resume: "() -> void; resumes native handle, never calls blueprint.play"
    restore: "() -> void; idempotent stop-and-restore"
    before_edit: "(object: Object, properties: PackedStringArray) -> void"
    before_serialize: "(scene_root: Node) -> void"
    on_selection_changed: "(selection: Array[Node]) -> void"
    on_scene_changed: "(scene_root: Node = null) -> void"
    shutdown: "() -> void"
    get_status: "() -> PreviewStatusV1"
PreviewDescriptionV1:
  schema: 1
  status: "ready | runtime_only | invalid"
  writes: "Array[Dictionary{target: Node, property: StringName}]"
  diagnostics: "Array[DiagnosticV1]"
PreviewStatusV1:
  schema: 1
  session_id: "transient integer; 0 when idle"
  state: "idle | playing | paused | completed | unavailable"
  elapsed_seconds: "float sampled from the native sequence get_total_elapsed_time"
  diagnostics: "Array[DiagnosticV1]"
```

MethodCaller always describes runtime_only. A Custom effect defaults to runtime_only until its script declares a valid complete write set; reset and construction writes must be included. Incomplete manifests reject the whole selected preview rather than skipping that child.

Preflight validates manifest structure, target/property validity, and restoration-adapter availability; it does not infer arbitrary GDScript write sets. Manifest completeness is the custom author's contract, and the shipped Custom examples must demonstrate it through actual changed-value/restoration tests.

Capture the complete baseline before reset, construction, or playback. Canonical snapshot keys use an ephemeral object identity and a restore adapter, not independently conflicting property aliases. Implement plain known value restoration, Node2D/Node3D local-transform restoration, and Control geometry restoration with parent-before-child application where required. Refuse unsupported setters, shared-resource mutation, and unresolved restoration dependencies before starting. Unknown custom setters are not implicitly safe because they accept a numeric value.

Only the active edited scene can be previewed, with one session and one selected blueprint at a time. Native tween processing remains the existing controller's responsibility. Pause/resume, retained completed pose, late-callback filtering, and stopping only owned runs follow the editor specification. A stop notification alone is not proof of cancellation because native success emits stopped before finished.

**One technical evidence gate remains:** the editor must deliver the edit/save/undo/selection lifecycle boundaries to this service in an order that preserves authored values and history. Unit U02 characterizes the actual mechanism on Godot 4.4 and 4.7.2. It produces a versioned boundary report consumed by U07/U08. A failed boundary test blocks those units; it does not authorize a developer to weaken the restoration requirement or switch preview workflows. No such experiment was run during this planning revision.

## 7. Initial preset content

Select two small built-in-only presets for this plan:

| Preset | Native property and default | Exported artist parameters |
|---|---|---|
| Fade In | CanvasItem.modulate, explicit Color(1,1,1,0) to Color(1,1,1,1), duration 0.2, Linear. | Start Color, End Color, Duration, Delay, Target, Trigger. |
| Scale Pop | Node2D/Control.scale, explicit Vector2(0.8,0.8) to Vector2(1,1), duration 0.15, Linear. | Start Scale, End Scale, Duration, Easing, Delay, Target, Trigger. |

Names describe the starting presets, not an additional atomic effect family. Colors and end scales are explicit authored parameters; do not claim that these presets automatically preserve an arbitrary target's tint or scale. Both start with Manual triggers. A separate Method Caller demo supplies a GDScript receiver and runtime Play button; it is not an in-editor callback-execution feature.
