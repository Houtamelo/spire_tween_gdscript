use control::MouseFilter;

use super::*;

pub(super) struct TriggerConnection {
    source:   InstanceId,
    signal:   StringName,
    callable: Callable,
}

pub(super) fn disconnect_connection(connection: Option<TriggerConnection>) {
    if let Some(connection) = connection
        && let Ok(mut source) = Gd::<Node>::try_from_instance_id(connection.source)
    {
        if source.is_connected(&connection.signal, &connection.callable) {
            source.disconnect(&connection.signal, &connection.callable);
        }
    }
}

#[godot_api(secondary)]
impl TweenBlueprint {
    #[func(gd_self)]
    fn refresh_blueprint_configuration(this: Gd<Self>) {
        Self::refresh_blueprint_triggers(this.clone());
        for node in this.get_children().iter_shared() {
            if let Ok(child) = node.try_cast::<Self>() {
                Self::initialize_blueprint_values(child.clone());
                Self::refresh_blueprint_configuration(child);
            }
        }
    }

    /// Rebinds automatic events after a runtime target or trigger change.
    #[func(gd_self)]
    pub(super) fn refresh_blueprint_triggers(mut this: Gd<Self>) {
        Self::disconnect_triggers(this.clone());
        if !this.is_inside_tree() || Engine::singleton().is_editor_hint() {
            this.notify_property_list_changed();
            this.update_configuration_warnings();
            return;
        }

        let (condition, source, pending, managed) = {
            let mut state = this.bind_mut();
            let pending = std::mem::take(&mut state.enter_trigger_pending);
            (
                state.trigger_condition,
                state
                    .trigger_source
                    .as_ref()
                    .filter(|source| source.is_instance_valid())
                    .cloned()
                    .or_else(|| state.resolve_target()),
                pending,
                state.is_managed_by_parent(),
            )
        };

        if managed {
            return;
        }

        match condition {
            TriggerCondition::Manual | TriggerCondition::AfterReady => return,
            TriggerCondition::AfterEnterTree => {
                if pending {
                    Self::play(this);
                }
                return;
            }
            _ => {}
        }

        let Some(source) = source else {
            godot_warn!(
                "TweenBlueprint: `trigger_source` or the resolved `target` must reference a valid node for mouse \
                 triggers."
            );
            return;
        };

        let event_index = if source.is_class("Control") { 0 } else { 1 };
        let signal = match condition {
            TriggerCondition::MouseEnter => StringName::from("mouse_entered"),
            TriggerCondition::MouseExit => StringName::from("mouse_exited"),
            TriggerCondition::MouseClickLeft | TriggerCondition::MouseClickRight => {
                StringName::from(if source.is_class("Control") { "gui_input" } else { "input_event" })
            }
            _ => unreachable!(),
        };

        if let Some(warning) = this.bind().trigger_warning() {
            godot_warn!("TweenBlueprint: {warning}");
            return;
        }

        let node_id = this.instance_id();
        let callable = Callable::from_fn("blueprint_automatic_trigger", move |args| {
            if Engine::singleton().is_editor_hint() {
                return;
            }

            let Ok(node) = Gd::<Self>::try_from_instance_id(node_id) else { return };
            if !node.is_inside_tree() || node.bind().is_managed_by_parent() {
                return;
            }

            if matches!(condition, TriggerCondition::MouseClickLeft | TriggerCondition::MouseClickRight) {
                let Some(event) = args
                    .get(event_index)
                    .and_then(|value| value.try_to::<Gd<InputEventMouseButton>>().ok())
                else {
                    return;
                };

                if !event.is_pressed() {
                    return;
                }

                let expected = if condition == TriggerCondition::MouseClickLeft {
                    godot::global::MouseButton::LEFT
                } else {
                    godot::global::MouseButton::RIGHT
                };

                if event.get_button_index() != expected {
                    return;
                }
            }

            Self::play(node);
        });

        let mut source = source;
        if source.connect(&signal, &callable) == godot::global::Error::OK {
            this.bind_mut().trigger_connection = Some(TriggerConnection {
                source: source.instance_id(),
                signal,
                callable,
            });
        }
    }
}

impl TweenBlueprint {
    pub(super) fn disconnect_triggers(mut this: Gd<Self>) {
        let connection = this.bind_mut().trigger_connection.take();
        disconnect_connection(connection);
    }

    pub(super) fn trigger_warning(&self) -> Option<String> {
        if let TriggerCondition::Manual | TriggerCondition::AfterReady | TriggerCondition::AfterEnterTree =
            self.trigger_condition
        {
            return None;
        }

        let Some(source) = self
            .trigger_source
            .as_ref()
            .filter(|source| source.is_instance_valid())
            .cloned()
            .or_else(|| self.resolve_target())
        else {
            return Some(
                "`trigger_source` or the resolved `target` must reference a valid node for mouse triggers.".into(),
            );
        };

        if source.is_class("Control") {
            return if source.clone().cast::<Control>().get_mouse_filter() == MouseFilter::IGNORE {
                Some(
                    "Mouse triggers require `mouse_filter` to be `Control.MOUSE_FILTER_STOP` or \
                     `Control.MOUSE_FILTER_PASS` on the resolved `trigger_source`."
                        .into(),
                )
            } else {
                None
            };
        }

        if let Ok(collision) = source.clone().try_cast::<CollisionObject2D>() {
            return if collision.is_pickable() && collision.get_collision_layer() != 0 {
                None
            } else {
                Some(
                    "Mouse triggers require `input_pickable` to be enabled and `collision_layer` to be nonzero on the \
                     resolved `trigger_source`."
                        .into(),
                )
            };
        }

        if let Ok(collision) = source.try_cast::<CollisionObject3D>() {
            return if collision.is_ray_pickable() && collision.get_collision_layer() != 0 {
                None
            } else {
                Some(
                    "Mouse triggers require `input_ray_pickable` to be enabled and `collision_layer` to be nonzero on \
                     the resolved `trigger_source`."
                        .into(),
                )
            };
        }

        Some(
            "`trigger_source` must resolve to a Control, CollisionObject2D, or CollisionObject3D for mouse triggers."
                .into(),
        )
    }
}
