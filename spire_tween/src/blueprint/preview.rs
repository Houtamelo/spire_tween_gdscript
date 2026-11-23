use std::cell::Cell;

use anyhow::{Context, Result, ensure};

use super::{tweenable_property_validation::property_descriptor, *};

thread_local! {
    // Copyable identity; resolve a live handle when stopping the active preview.
    static ACTIVE_PREVIEW: Cell<Option<InstanceId>> = const { Cell::new(None) };
}

pub(super) struct PreviewRun {
    restore: Callable,
    pub(super) sequence: Option<InstanceId>,
}

#[godot_api(secondary)]
impl TweenBlueprint {
    /// Starts an editor-only preview. An existing preview is stopped and restored
    /// synchronously before capturing this composition. Parent-managed children
    /// cannot start independent previews.
    #[func(gd_self)]
    pub fn play_animation_preview(mut this: Gd<Self>) {
        if !Engine::singleton().is_editor_hint() {
            godot_warn!("[TweenBlueprint] `play_animation_preview()` only works in Editor mode.");
            return;
        } else if !this.is_inside_tree() {
            godot_warn!(
                "[TweenBlueprint] `play_animation_preview()` only works if the blueprint is inside the SceneTree."
            );
            return;
        } else if this.bind().is_managed_by_parent() {
            godot_warn!(
                "[TweenBlueprint] `play_animation_preview()` only works if the blueprint is independent (not managed \
                 by a parent)."
            );
            return;
        }

        if let Some(active) = ACTIVE_PREVIEW.with(Cell::take)
            && let Ok(active) = Gd::<Self>::try_from_instance_id(active)
        {
            Self::stop(active);
        }
        Self::stop(this.clone());
        let restore = Self::capture_rest_state(this.clone());
        this.bind_mut().preview = Some(PreviewRun {
            restore,
            sequence: None,
        });
        ACTIVE_PREVIEW.set(Some(this.instance_id()));
        Self::play(this.clone());
        this.notify_property_list_changed();
    }

    /// Stops this blueprint's preview and restores its captured state immediately.
    /// Does not stop a different blueprint's preview or a normal runtime run.
    #[func(gd_self)]
    pub fn stop_animation_preview(this: Gd<Self>) {
        if this.bind().preview.is_some() {
            Self::stop(this);
        }
    }

    /// Captures rest state without changing targets. The returned Callable must
    /// restore synchronously, without deferred calls or awaiting signals.
    /// Scripted blueprints may override this hook to capture their custom state;
    /// their default is an invalid Callable, meaning nothing needs restoration.
    /// Containers capture all direct blueprint children before playback and invoke
    /// valid restoration Callables recursively in reverse child order.
    #[func(virtual, gd_self)]
    pub fn capture_rest_state(this: Gd<Self>) -> Callable {
        match capture_native(&this) {
            Ok(restore) => restore,
            Err(error) => {
                godot_warn!("TweenBlueprint `{}`: {error:#}", this.get_name());
                Callable::invalid()
            }
        }
    }

    pub(super) fn preview_play_button(&mut self) {
        let this_gd = self.to_gd();
        let _reborrow_guard = self.base_mut();
        Self::play_animation_preview(this_gd);
    }

    pub(super) fn preview_stop_button(&mut self) {
        let this_gd = self.to_gd();
        let _reborrow_guard = self.base_mut();
        Self::stop_animation_preview(this_gd);
    }
}

/// Called after the native sequence has stopped all its children. Taking the
/// callback before invoking user code makes repeated stop/exit/save harmless.
pub(super) fn restore(mut this: Gd<TweenBlueprint>) {
    let Some(run) = this.bind_mut().preview.take() else { return };
    ACTIVE_PREVIEW.with(|active| {
        if active.get() == Some(this.instance_id()) {
            active.set(None);
        }
    });
    if run.restore.is_valid() {
        run.restore.call(&[]);
    }
    if this.is_instance_valid() {
        this.notify_property_list_changed();
    }
}

pub(super) fn sequence_stopped(this: Gd<TweenBlueprint>, sequence: InstanceId) {
    let matches = this
        .bind()
        .preview
        .as_ref()
        .is_some_and(|run| run.sequence == Some(sequence));
    if matches {
        restore(this);
    }
}

fn capture_native(this: &Gd<TweenBlueprint>) -> Result<Callable> {
    let (kind, target) = {
        let this_ref = this.bind();
        if this_ref.base().get_script().is_some() {
            return Ok(Callable::invalid());
        }
        (this_ref.kind, this_ref.resolve_target())
    };

    match kind {
        BlueprintKind::ChainBlock | BlueprintKind::ParallelBlock => {
            let mut restorers = Vec::new();
            for child in this
                .get_children()
                .iter_shared()
                .filter_map(|c| c.try_cast::<TweenBlueprint>().ok())
            {
                let restore = TweenBlueprint::capture_rest_state(child.clone());
                if restore.is_valid() {
                    restorers.push(restore);
                }
            }

            Ok(Callable::from_fn("restore_blueprint_children", move |_| {
                for restore in restorers.iter().rev() {
                    if restore.is_valid() {
                        restore.call(&[]);
                    }
                }
            }))
        }
        BlueprintKind::Property => {
            let target = target.context("`target` could not be resolved.")?;
            let info = property_descriptor(&target, &this.bind().property_to_animate)?;
            let name = info.property_name.to_string();
            // Restoring text must also recover its derived character state. This
            // callback runs after the string is restored, in either child order.
            let restore_characters = if name == "text" {
                target
                    .clone()
                    .try_cast::<SpireRichTextLabel>()
                    .ok()
                    .map(|label| SpireRichTextLabel::capture_character_rest_state(label, 0, -1))
            } else {
                None
            };

            let properties = rest_properties(&target, &name);
            let values: Vec<_> = properties
                .into_iter()
                .map(|name| {
                    let path = NodePath::from(name);
                    let value = target.get_indexed(&path);
                    (path, value)
                })
                .collect();
            let id = target.instance_id();
            Ok(Callable::from_fn("restore_blueprint_properties", move |_| {
                let Ok(mut target) = Gd::<Node>::try_from_instance_id(id) else { return };

                for (path, value) in &values {
                    target.set_indexed(path, value);
                }

                let Some(restore) = &restore_characters else { return };

                if restore.is_valid() {
                    restore.call(&[]);
                }
            }))
        }
        BlueprintKind::Text => {
            let (label, first, last) = {
                let this_ref = this.bind();
                let (_, label) = this_ref.resolve_effect_configuration()?;
                (label, this_ref.text_start_index, this_ref.text_end_index)
            };

            let restore = SpireRichTextLabel::capture_character_rest_state(label, first, last);
            ensure!(
                restore.is_valid(),
                "Character state is not initialized; allow the label to draw before previewing."
            );
            Ok(restore)
        }
    }
}

/// Restore aliases through local state, so parent/child transform restoration
/// does not depend on the temporary global pose of either node.
fn rest_properties<'a>(target: &Gd<Node>, name: &'a str) -> Vec<&'a str> {
    let root = name.split(':').next().unwrap_or(name);
    if target.is_class("Node2D")
        && matches!(
            root,
            "position"
                | "global_position"
                | "rotation"
                | "rotation_degrees"
                | "global_rotation"
                | "global_rotation_degrees"
                | "scale"
                | "global_scale"
                | "skew"
                | "global_skew"
                | "transform"
                | "global_transform"
        )
    {
        return vec!["transform"];
    }
    if target.is_class("Node3D")
        && matches!(
            root,
            "position"
                | "global_position"
                | "rotation"
                | "rotation_degrees"
                | "global_rotation"
                | "global_rotation_degrees"
                | "scale"
                | "quaternion"
                | "transform"
                | "global_transform"
                | "basis"
                | "global_basis"
        )
    {
        return vec!["transform"];
    }
    if target.is_class("Control")
        && matches!(
            root,
            "position"
                | "global_position"
                | "size"
                | "rotation"
                | "rotation_degrees"
                | "scale"
                | "pivot_offset"
                | "anchor_left"
                | "anchor_top"
                | "anchor_right"
                | "anchor_bottom"
                | "offset_left"
                | "offset_top"
                | "offset_right"
                | "offset_bottom"
        )
    {
        return vec![
            "anchor_left",
            "anchor_top",
            "anchor_right",
            "anchor_bottom",
            "offset_left",
            "offset_top",
            "offset_right",
            "offset_bottom",
            "pivot_offset",
            "rotation",
            "scale",
        ];
    }
    vec![name]
}
