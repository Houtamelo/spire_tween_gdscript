use super::{tweenable_property_validation::property_descriptor, *};

impl TweenBlueprint {
    pub(super) fn configured_target(&self) -> Option<Gd<Node>> {
        self.target
            .as_ref()
            .filter(|target| target.is_instance_valid())
            .cloned()
    }

    pub(super) fn manages_children(&self) -> bool {
        self.base().get_script().is_some()
            || matches!(self.kind, BlueprintKind::ChainBlock | BlueprintKind::ParallelBlock)
    }

    pub(super) fn is_managed_by_parent(&self) -> bool {
        self.base()
            .get_parent()
            .and_then(|node| node.try_cast::<Self>().ok())
            .is_some_and(|parent| parent.bind().manages_children())
    }

    pub(super) fn resolve_target(&self) -> Option<Gd<Node>> {
        if let Some(target) = self.configured_target() {
            return Some(target);
        }
        let mut ancestor = self.base().get_parent();
        while let Some(node) = ancestor {
            if !node.is_class("TweenBlueprint") {
                return Some(node);
            }
            ancestor = node.get_parent();
        }
        None
    }

    pub(super) fn setup_sequence_and_play(mut this: Gd<Self>) -> Gd<SpireSequence> {
        let sequence = {
            let state = this.bind();
            let mut sequence = TweenSequence::new()
                .bound_to(this.clone())
                .with_delay(normalize_non_negative(state.delay, "delay", 0.0))
                .with_speed_scale(normalize_non_negative(state.speed_scale, "speed_scale", 1.0))
                .with_pause_mode(state.tween_pause_mode)
                .with_process_mode(state.tween_process_mode)
                .with_loops(state.loops, state.loop_mode);
            sequence.ignore_time_scale = state.ignore_time_scale;
            // Built-in containers do not animate their own target; their children
            // own independent target bindings.
            if matches!(state.kind, BlueprintKind::Property | BlueprintKind::Characters)
                || state.base().get_script().is_some()
            {
                if let Some(target) = state.resolve_target() {
                    sequence = sequence.bound_to(target);
                }
            }
            sequence
        };
        // Register after configuration, before the builder, so script-created children can
        // use the normal append/join adoption path. No ticks run during this synchronous call.
        let (_, handle) = sequence.register_with_gd_handle();
        Self::forward_lifecycle(&this, &handle);
        this.signals().tween_spawned().emit(&handle);
        Self::play_internal(this.clone(), handle.clone());
        {
            let mut state = this.bind_mut();
            state.current_tween = Some(handle.clone());
        }
        handle
    }

    fn forward_lifecycle(this: &Gd<Self>, sequence: &Gd<SpireSequence>) {
        // Capturing a strong sequence in a callback owned by that sequence would leak it.
        // Instance IDs also allow retained handles to outlive their authoring node.
        let node_id = this.instance_id();
        let sequence_id = sequence.instance_id();
        sequence.signals().stopped().connect(move || {
            if let (Ok(node), Ok(sequence)) =
                (Gd::<Self>::try_from_instance_id(node_id), Gd::<SpireSequence>::try_from_instance_id(sequence_id))
            {
                node.signals().tween_stopped().emit(&sequence);
            }
        });
        sequence.signals().loop_finished().connect(move || {
            if let (Ok(node), Ok(sequence)) =
                (Gd::<Self>::try_from_instance_id(node_id), Gd::<SpireSequence>::try_from_instance_id(sequence_id))
            {
                node.signals().tween_loop_finished().emit(&sequence);
            }
        });
        sequence.signals().finished().connect(move || {
            if let (Ok(node), Ok(sequence)) =
                (Gd::<Self>::try_from_instance_id(node_id), Gd::<SpireSequence>::try_from_instance_id(sequence_id))
            {
                node.signals().tween_finished().emit(&sequence);
            }
        });
    }

    pub(super) fn build_native(this: Gd<Self>, sequence: Gd<SpireSequence>) {
        let (kind, target) = {
            let state = this.bind();
            (state.kind, state.get_resolved_target())
        };
        match kind {
            BlueprintKind::Characters => Self::build_characters(this, sequence),
            BlueprintKind::ChainBlock | BlueprintKind::ParallelBlock => {
                for node in this.get_children().iter_shared() {
                    let Ok(child) = node.try_cast::<Self>() else { continue };
                    let handle = Self::play(child);
                    // Release each Godot borrow before calling the next child's script.
                    let child = handle.bind().to_ref().clone();
                    if kind == BlueprintKind::ChainBlock {
                        sequence.bind().to_mut().append_ptr(child);
                    } else {
                        sequence.bind().to_mut().join_ptr(child);
                    }
                }
            }
            BlueprintKind::Property => {
                let Some(target) = target else {
                    godot_warn!("TweenBlueprint: `target` could not be resolved; skipping property step.");
                    return;
                };

                let (path, end, start, start_enabled, duration, mode, ease) = {
                    let state = this.bind();

                    let ease = state.ease_source.resolve(state.ease_enum, state.ease_curve.as_ref());

                    (
                        state.animated_property.clone(),
                        state.end_value.clone(),
                        state.start_value.clone(),
                        state.start_enabled,
                        state.speed_or_duration,
                        state.lerp_mode,
                        ease,
                    )
                };
                let info = match property_descriptor(&target, &path) {
                    Ok(ok) => ok,
                    Err(err) => {
                        godot_warn!("[TweenBlueprint] {err}");
                        return;
                    }
                };

                if !duration.is_finite() || duration < 0.0 || (mode == LerpModeKind::SpeedBased && duration == 0.0) {
                    godot_warn!(
                        "TweenBlueprint: `speed_or_duration` must be finite and non-negative; speed-based `lerp_mode` \
                         requires a positive value."
                    );
                    return;
                }

                let property_path = NodePath::from(&info.property_name);
                macro_rules! property_tween {
                    ($ty:ty) => {{
                        let Ok(end) = end.try_to_relaxed::<$ty>() else {
                            godot_warn!(
                                "TweenBlueprint: `end_value` is incompatible with `animated_property` (`{path}`)."
                            );
                            return;
                        };
                        let mut tween = target
                            .do_property(&property_path, Evaluator::Static(end), duration)
                            .with_ease(ease);
                        match mode {
                            LerpModeKind::Absolute if start_enabled => {
                                let Ok(start) = start.try_to_relaxed::<$ty>() else {
                                    godot_warn!(
                                        "TweenBlueprint: `start_value` is incompatible with `animated_property` \
                                         (`{path}`)."
                                    );
                                    return;
                                };
                                tween.set_begin_value(start);
                            }
                            LerpModeKind::Absolute => {}
                            LerpModeKind::SpeedBased => tween.set_speed_based(),
                            LerpModeKind::Relative => tween.set_relative(<$ty>::default()),
                        }
                        if duration == 0.0 {
                            let mut tween = TweenPtr::new(tween);
                            sequence.bind().to_mut().append_fn(move || {
                                tween.play();
                                tween.force_complete();
                            });
                        } else {
                            sequence.bind().to_mut().append(tween);
                        }
                    }};
                }
                match info.variant_type {
                    VariantType::INT => property_tween!(i64),
                    VariantType::FLOAT => property_tween!(f64),
                    VariantType::STRING => property_tween!(GString),
                    VariantType::COLOR => property_tween!(Color),
                    VariantType::VECTOR2 => property_tween!(Vector2),
                    VariantType::VECTOR2I => property_tween!(Vector2i),
                    VariantType::VECTOR3 => property_tween!(Vector3),
                    VariantType::VECTOR3I => property_tween!(Vector3i),
                    VariantType::VECTOR4 => property_tween!(Vector4),
                    VariantType::QUATERNION => property_tween!(Quaternion),
                    VariantType::RECT2 => property_tween!(Rect2),
                    VariantType::RECT2I => {
                        // Godot Rect2i properties use the native Rect2 lerper and convert at
                        // the accessor boundary, just as integer-vector adapters do.
                        Self::append_integer_rect(
                            &sequence,
                            target,
                            property_path,
                            end,
                            start,
                            start_enabled,
                            duration,
                            mode,
                            ease,
                        );
                    }
                    invalid_ty => {
                        godot_warn!(
                            "[TweenBlueprint] `animated_property` (`{path}`) has unsupported type `{}`.",
                            invalid_ty.godot_type_name()
                        );
                    }
                }
            }
        }
    }

    fn append_integer_rect(
        sequence: &Gd<SpireSequence>,
        target: Gd<Node>,
        path: NodePath,
        end: Variant,
        start: Variant,
        start_enabled: bool,
        duration: f64,
        mode: LerpModeKind,
        ease: EaseKind,
    ) {
        let Ok(end) = end.try_to::<Rect2i>() else {
            godot_warn!("TweenBlueprint: `end_value` must be a Rect2i to match `animated_property`.");
            return;
        };
        let getter_target = target.clone();
        let getter_path = path.clone();
        let getter = move || {
            getter_target
                .get_indexed(&getter_path)
                .try_to::<Rect2i>()
                .map(|value| Rect2::new(value.position.to_vector2(), value.size.to_vector2()))
                .unwrap_or_default()
        };
        let binding = target.clone();
        let setter = move |value: Rect2| {
            let mut target = target.clone();
            target.set_indexed(
                &path,
                &Rect2i::new(value.position.round().to_vector2i(), value.size.round().to_vector2i()).to_variant(),
            );
        };
        let mut tween =
            do_get_set(getter, setter, Rect2::new(end.position.to_vector2(), end.size.to_vector2()), duration)
                .bound_to(binding)
                .with_ease(ease);
        match mode {
            LerpModeKind::Absolute if start_enabled => {
                let Ok(start) = start.try_to::<Rect2i>() else {
                    godot_warn!("TweenBlueprint: `start_value` must be a Rect2i to match `animated_property`.");
                    return;
                };
                tween.set_begin_value(Rect2::new(start.position.to_vector2(), start.size.to_vector2()));
            }
            LerpModeKind::Absolute => {}
            LerpModeKind::SpeedBased => tween.set_speed_based(),
            LerpModeKind::Relative => tween.set_relative(Rect2::default()),
        }
        if duration == 0.0 {
            let mut tween = TweenPtr::new(tween);
            sequence.bind().to_mut().append_fn(move || {
                tween.play();
                tween.force_complete();
            });
        } else {
            sequence.bind().to_mut().append(tween);
        }
    }
}

impl Drop for TweenBlueprint {
    fn drop(&mut self) {
        self.disconnect_character_effect();
        Self::disconnect_configuration_watch(&mut self.character_delay_curve_connection);
        if let Some(sequence) = self.current_tween.take() {
            sequence.bind().to_mut().stop();
        }
        triggers::disconnect_connection(self.trigger_connection.take());
    }
}

fn normalize_non_negative(value: f64, field: &str, default: f64) -> f64 {
    if value.is_finite() && value >= 0.0 {
        value
    } else {
        godot_warn!("TweenBlueprint: `{field}` must be finite and non-negative; using {default}.");
        default
    }
}
