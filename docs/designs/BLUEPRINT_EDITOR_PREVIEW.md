# Blueprint editor preview

Approved design, 2026-11-20. This supersedes the preview API and lifecycle proposals in the older feedback tooling draft and implementation contract.

`TweenBlueprint._capture_rest_state() -> Callable` captures the selected animation's rest state without changing targets. Its returned Callable restores synchronously. Custom GDScript implementations own what they capture and how they restore, including mutable Resources. An invalid Callable means there is nothing to restore; it neither refuses preview nor excludes a child from playback. A custom script without a capture override uses that default. There is no Dictionary-based restoration API.

Native Property steps capture their target property, with class-specific handling for overlapping transform/layout properties. Native Text steps capture exact character-channel state. Sequence and Block steps capture all participating direct children recursively, before playback, and invoke valid restoration callbacks in reverse child order. Children returning invalid Callables still play. A composition containing only invalid child callbacks also plays normally. Only the root's stop ends the preview; intermediate child completion does not restore anything.

Native Property animation of a SpireRichTextLabel's `text` also captures its derived character state. Restoration sets the original source and invokes the label's callback, which owns synchronization and channel restoration, including when composed with Text steps in either child order. Explicit character ranges refuse unsynchronized input. Whole-label capture also supports empty/uninitialized data through a callback that rebuilds default state during restoration. Capture itself does not refresh or mutate the target.

The native Inspector uses export tool buttons named **Play Animation Preview** and **Stop Animation Preview**. Parent-managed blueprints cannot start independent previews. Stop is available while that root owns a preview.

One preview is active across the editor. Replacement is synchronous: stop the previous run, restore it, capture the next run, then play. There are no frame delays. Manual stop, natural completion, editor pre-save and scene-tree exit all restore exactly once. Preview uses the existing native tween controller and only stops its own sequence.

Supplied GDScript templates implement the capture hook. Cleanup clears their lazy runtime snapshots so a later reset cannot reapply an intermediate preview pose. Character snapshots retain the distinction between inherited colors and explicit overrides.

Editing animated properties during preview, intercepting Undo/Redo, arbitrary script side effects, deferred restoration and native binary hot reload are outside this implementation's scope. Scene switching must stop the outgoing preview; saving must restore before packing the scene.

Validation uses real rendered Godot 4.4 and 4.7.2 editors on private virtual displays. Cover native properties/text/composition, custom callbacks and previews with no restoration action, all supplied templates, actual Inspector buttons, save/reload, scene switching/closing, interruption, replacement, deletion, invalid configuration and runtime isolation.
