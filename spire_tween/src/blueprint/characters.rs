use super::*;

impl CharacterAnimation {
    pub(super) fn is_punch(self) -> bool { matches!(self, Self::PunchOffset | Self::PunchRotation | Self::PunchScale) }
    pub(super) fn is_shake(self) -> bool { matches!(self, Self::ShakeOffset | Self::ShakeRotation | Self::ShakeScale) }
    pub(super) fn has_pivot(self) -> bool {
        matches!(
            self,
            Self::Rotation
                | Self::Scale
                | Self::PunchRotation
                | Self::PunchScale
                | Self::ShakeRotation
                | Self::ShakeScale
        )
    }
    pub(super) fn value_type(self) -> VariantType {
        match self {
            Self::Fade => VariantType::FLOAT,
            Self::Color => VariantType::COLOR,
            Self::Rotation | Self::PunchRotation | Self::ShakeRotation => VariantType::VECTOR3,
            _ => VariantType::VECTOR2,
        }
    }
    pub(super) fn defaults(self) -> (Variant, Variant) {
        match self {
            Self::Fade => (1.0_f64.to_variant(), 1.0_f64.to_variant()),
            Self::Color => (Color::WHITE.to_variant(), Color::WHITE.to_variant()),
            Self::Rotation => (Vector3::ZERO.to_variant(), Vector3::ZERO.to_variant()),
            Self::Scale => (Vector2::ONE.to_variant(), Vector2::ONE.to_variant()),
            Self::PunchOffset => (Vector2::ZERO.to_variant(), Vector2::new(0.0, -20.0).to_variant()),
            Self::PunchRotation | Self::ShakeRotation => {
                (Vector3::ZERO.to_variant(), Vector3::new(0.0, 0.0, 0.3).to_variant())
            }
            Self::PunchScale | Self::ShakeScale => (Vector2::ONE.to_variant(), Vector2::new(0.2, 0.2).to_variant()),
            Self::ShakeOffset => (Vector2::ZERO.to_variant(), Vector2::new(10.0, 10.0).to_variant()),
            Self::Offset => (Vector2::ZERO.to_variant(), Vector2::ZERO.to_variant()),
        }
    }
}

impl TweenBlueprint {
    pub(super) fn character_warnings(&self) -> PackedStringArray {
        let mut warnings = PackedStringArray::new();
        let label = self
            .resolve_target()
            .and_then(|node| node.try_cast::<SpireRichTextLabel>().ok());
        if label.is_none() {
            warnings.push("`target` must resolve to a SpireRichTextLabel for `kind` Characters.");
        }
        if !self.speed_or_duration.is_finite() || self.speed_or_duration <= 0.0 {
            warnings.push("`speed_or_duration` must be finite and greater than 0.0 for character animations.");
        }
        if self.character_first < 0
            || self.character_last < -1
            || (self.character_last != -1 && self.character_last < self.character_first)
        {
            warnings.push(
                "`character_first` must be non-negative; `character_last` must be -1 or at least `character_first`.",
            );
        } else if let Some(label) = label {
            let count = label.get_total_character_count() as i64;
            if self.character_first >= count || self.character_last >= count {
                warnings.push(
                    "`character_first` and `character_last` must select existing logical character indices; use -1 \
                     for `character_last` to follow the text length.",
                );
            }
        }
        if !self.character_stagger.is_finite() || self.character_stagger < 0.0 {
            warnings.push("`character_stagger` must be finite and non-negative.");
        }
        if !self.character_pivot.is_finite() {
            warnings.push("`character_pivot` must be finite.");
        }
        if self.character_loops < 1 {
            warnings.push("`character_loops` must be at least 1.");
        }
        let animation = self.character_animation;
        let ty = animation.value_type();
        if !tweenable_property_validation::value_matches(&self.end_value, ty) {
            warnings.push(&format!("`end_value` must match `character_animation`: expected {}.", ty.godot_type_name()));
        }
        if self.start_enabled && !tweenable_property_validation::value_matches(&self.start_value, ty) {
            warnings
                .push(&format!("`start_value` must match `character_animation`: expected {}.", ty.godot_type_name()));
        }
        if animation.is_punch() || animation.is_shake() {
            if self.character_vibrato < 1 {
                warnings.push("`character_vibrato` must be at least 1.");
            }
        }
        if animation.is_punch()
            && (!self.character_elasticity.is_finite() || !(0.0..=1.0).contains(&self.character_elasticity))
        {
            warnings.push("`character_elasticity` must be finite and between 0.0 and 1.0.");
        }
        if animation.is_shake()
            && (!self.character_randomness.is_finite() || !(0.0..=180.0).contains(&self.character_randomness))
        {
            warnings.push("`character_randomness` must be finite and between 0.0 and 180.0.");
        }
        warnings
    }

    pub(super) fn build_characters(this: Gd<Self>, sequence: Gd<SpireSequence>) {
        let state = this.bind();
        let label = state
            .resolve_target()
            .and_then(|node| node.try_cast::<SpireRichTextLabel>().ok());
        // Range selection needs the current text. Synchronize explicitly here,
        // without resetting unchanged state when replaying a blueprint.
        if let Some(label) = &label {
            SpireRichTextLabel::refresh_characters(label.clone(), false);
        }
        let warnings = state.character_warnings();
        if !warnings.is_empty() {
            for warning in warnings.as_slice() {
                godot_warn!("[TweenBlueprint] {warning}");
            }
            return;
        }
        let label = label.expect("validated character target");
        let last = if state.character_last == -1 {
            SpireRichTextLabel::get_character_count(label.clone()) - 1
        } else {
            state.character_last
        };
        let ease = state
            .ease_curve
            .as_ref()
            .filter(|curve| curve.is_instance_valid())
            .map(|curve| EaseKind::GodotCurve(curve.clone()))
            .unwrap_or(EaseKind::Basic(state.ease_basic));
        let animation = state.character_animation;
        let start = state.start_value.clone();
        let explicit = state.start_enabled;
        let pivot = state.character_pivot;
        let initialize = label.clone();
        let first = state.character_first;
        // Initialize the whole selection when this Blueprint begins, before staggering
        // its animation. This supports a hidden reveal without touching later serial steps.
        sequence.bind().to_mut().append_fn(move || {
            if !initialize.is_instance_valid() {
                return;
            }
            for index in first..=last {
                if !SpireRichTextLabel::is_valid_char(initialize.clone(), index) {
                    continue;
                }
                if animation.has_pivot() {
                    SpireRichTextLabel::set_char_pivot(initialize.clone(), index, pivot);
                }
                if explicit {
                    match animation {
                        CharacterAnimation::Offset
                        | CharacterAnimation::PunchOffset
                        | CharacterAnimation::ShakeOffset => SpireRichTextLabel::set_char_offset(
                            initialize.clone(),
                            index,
                            start.try_to_relaxed().expect("validated character value"),
                        ),
                        CharacterAnimation::Fade => SpireRichTextLabel::set_char_alpha(
                            initialize.clone(),
                            index,
                            start.try_to_relaxed().expect("validated character value"),
                        ),
                        CharacterAnimation::Color => SpireRichTextLabel::set_char_color(
                            initialize.clone(),
                            index,
                            start.try_to_relaxed().expect("validated character value"),
                        ),
                        CharacterAnimation::Rotation
                        | CharacterAnimation::PunchRotation
                        | CharacterAnimation::ShakeRotation => SpireRichTextLabel::set_char_rotation(
                            initialize.clone(),
                            index,
                            start.try_to_relaxed().expect("validated character value"),
                        ),
                        CharacterAnimation::Scale | CharacterAnimation::PunchScale | CharacterAnimation::ShakeScale => {
                            SpireRichTextLabel::set_char_scale(
                                initialize.clone(),
                                index,
                                start.try_to_relaxed().expect("validated character value"),
                            )
                        }
                    }
                }
            }
        });
        for index in first..=last {
            let mut child = TweenSequence::new()
                .bound_to(label.clone())
                .with_delay((index - first) as f64 * state.character_stagger);
            let end = &state.end_value;
            let duration = state.speed_or_duration;
            let vibrato = state.character_vibrato;
            let blendable = state.character_blendable;
            let elasticity = state.character_elasticity;
            let randomness = state.character_randomness;
            let fade = state.character_fade_out;
            let mode = state.character_randomness_mode;
            let tween = match animation {
                CharacterAnimation::Offset => {
                    label.do_offset_char(index, end.try_to_relaxed().expect("validated character value"), duration)
                }
                CharacterAnimation::Fade => {
                    label.do_fade_char(index, end.try_to_relaxed().expect("validated character value"), duration)
                }
                CharacterAnimation::Color => {
                    label.do_color_char(index, end.try_to_relaxed().expect("validated character value"), duration)
                }
                CharacterAnimation::Rotation => label.do_rotate_char(
                    index,
                    end.try_to_relaxed().expect("validated character value"),
                    duration,
                    state.character_rotate_mode,
                ),
                CharacterAnimation::Scale => {
                    label.do_scale_char(index, end.try_to_relaxed().expect("validated character value"), duration)
                }
                CharacterAnimation::PunchOffset => label.do_punch_char_offset(
                    index,
                    end.try_to_relaxed().expect("validated character value"),
                    duration,
                    vibrato,
                    elasticity,
                    blendable,
                ),
                CharacterAnimation::PunchRotation => label.do_punch_char_rotation(
                    index,
                    end.try_to_relaxed().expect("validated character value"),
                    duration,
                    vibrato,
                    elasticity,
                    blendable,
                ),
                CharacterAnimation::PunchScale => label.do_punch_char_scale(
                    index,
                    end.try_to_relaxed().expect("validated character value"),
                    duration,
                    vibrato,
                    elasticity,
                    blendable,
                ),
                CharacterAnimation::ShakeOffset => label.do_shake_char_offset(
                    index,
                    duration,
                    end.try_to_relaxed().expect("validated character value"),
                    vibrato,
                    randomness,
                    fade,
                    mode,
                    blendable,
                ),
                CharacterAnimation::ShakeRotation => label.do_shake_char_rotation(
                    index,
                    duration,
                    end.try_to_relaxed().expect("validated character value"),
                    vibrato,
                    randomness,
                    fade,
                    mode,
                    blendable,
                ),
                CharacterAnimation::ShakeScale => label.do_shake_char_scale(
                    index,
                    duration,
                    end.try_to_relaxed().expect("validated character value"),
                    vibrato,
                    randomness,
                    fade,
                    mode,
                    blendable,
                ),
            }
            .with_ease(ease.clone())
            .with_loops(state.character_loops, state.character_loop_mode);
            child.append(tween);
            if index == first {
                sequence.bind().to_mut().append(child);
            } else {
                sequence.bind().to_mut().join(child);
            }
        }
    }
}
