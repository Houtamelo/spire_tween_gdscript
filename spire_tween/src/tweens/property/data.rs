use super::*;

pub struct LerpPropertyData<T: PropertyType> {
    pub data: T::Data,
    pub lerp_mode: LerpMode<T>,
    pub ease: EaseKind,
    pub to: Evaluator<T>,
    pub lerper: T::Lerper,
}

impl<T> SpireTween<LerpPropertyData<T>>
where
    T: PropertyType + Clone + Default + FromGodot,
    LerpPropertyData<T>: ITweenable,
{
    #[inline]
    pub fn get_property_path(&self) -> NodePath { self.t.data.get_property_path() }

    #[inline]
    pub fn get_owner(&self) -> &ObjectOrNode { self.t.data.get_owner() }

    #[inline]
    pub fn set_owner(&mut self, owner: ObjectOrNode) {
        if !self.t.data.try_set_owner(owner) {
            godot_error!(
                "Failed to set owner, object does not match the property's expect owner type `{}`.",
                self.t.data.owner_class()
            );
        }
    }

    #[inline]
    pub fn get_ease(&self) -> &EaseKind { &self.t.ease }
    #[inline]
    pub fn set_ease(&mut self, ease: EaseKind) { self.t.ease = ease; }

    #[inline]
    pub fn get_final_value(&mut self) -> T
    where T: Clone + FromGodot + Default {
        self.t.to.eval()
    }

    #[inline]
    pub fn set_final_value(&mut self, to: T) { self.t.to = Evaluator::Static(to); }

    #[inline]
    pub fn set_dynamic_target(&mut self, evaluator: Callable) {
        self.t.to = Evaluator::Callable(evaluator);
    }

    pub fn set_begin_value(&mut self, value: T) {
        match &mut self.t.lerp_mode {
            LerpMode::Absolute { from, .. } => {
                *from = FromValue::Explicit(value);
            }
            LerpMode::SpeedBased { .. } => {
                godot_warn!(
                    "[b]Warning:[/b] Starting value(set by calling `from(value)`) is ignored in \
                    Speed-Based tweens (created with `as_speed_based`)."
                );
            }
            LerpMode::Relative { .. } => {
                godot_warn!(
                    "[b]Warning:[/b] Starting value is ignored in Relative tweens (created with `as_relative`)."
                );
            }
        }
    }
}

impl<T> SpireTween<LerpPropertyData<T>>
where
    T: PropertyType + Clone + Default + FromGodot,
    LerpPropertyData<T>: ITweenable,
{
    #[inline]
    pub fn set_relative(&mut self, relative_to_value: T) {
        match &mut self.t.lerp_mode {
            | &mut LerpMode::Absolute { duration, .. }
            | &mut LerpMode::SpeedBased {
                speed: duration, ..
            } => {
                self.t.lerp_mode = LerpMode::relative(duration, relative_to_value);
            }
            LerpMode::Relative { relative_to, .. } => {
                *relative_to = relative_to_value;
            }
        }
    }

    #[inline]
    pub fn set_absolute(&mut self) {
        match &mut self.t.lerp_mode {
            | &mut LerpMode::Relative { duration, .. }
            | &mut LerpMode::SpeedBased {
                speed: duration, ..
            } => {
                self.t.lerp_mode = LerpMode::absolute(duration);
            }
            LerpMode::Absolute { .. } => {}
        }
    }

    #[inline]
    pub fn set_speed_based(&mut self) {
        match &mut self.t.lerp_mode {
            LerpMode::Absolute { duration, .. } | LerpMode::Relative { duration, .. } => {
                self.t.lerp_mode = LerpMode::speed_based(*duration);
            }
            LerpMode::SpeedBased { .. } => {}
        }
    }
}

impl<T> SpireTween<LerpPropertyData<T>>
where
    T: PropertyType,
    LerpPropertyData<T>: ITweenable,
{
    #[inline]
    pub fn is_absolute(&self) -> bool { matches!(self.t.lerp_mode, LerpMode::Absolute { .. }) }

    #[inline]
    pub fn is_relative(&self) -> bool { matches!(self.t.lerp_mode, LerpMode::Relative { .. }) }

    #[inline]
    pub fn is_speed_based(&self) -> bool { matches!(self.t.lerp_mode, LerpMode::SpeedBased { .. }) }

    #[inline]
    pub fn get_duration(&mut self) -> f64
    where T: Clone + Default + FromGodot {
        match self.t.lerp_mode {
            LerpMode::Absolute { duration, .. } => duration,
            LerpMode::SpeedBased { speed, .. } => {
                let distance = self
                    .t
                    .lerper
                    .spire_distance(&self.t.data.get_property_value(), &self.t.to.eval());

                distance / speed
            }
            LerpMode::Relative { duration, .. } => duration,
        }
    }

    #[inline]
    pub fn set_duration(&mut self, new_duration: f64)
    where T: Clone + Default + FromGodot {
        match &mut self.t.lerp_mode {
            | LerpMode::Absolute { duration, .. }
            | LerpMode::Relative { duration, .. }
            | LerpMode::SpeedBased {
                speed: duration, ..
            } => {
                *duration = new_duration;
            }
        }
    }
}

impl<T: PropertyType> SpireTween<LerpPropertyData<T>>
where LerpPropertyData<T>: ITweenable
{
    fn try_seek_end(&mut self) -> anyhow::Result<()>
    where T: Clone + Default + FromGodot {
        if !self.t.data.get_owner().is_instance_valid() {
            let prop_name = &self.t.data.get_property_path();
            bail!(
                "Cannot set property `{prop_name}` on Object, owner does not point to a valid instance."
            );
        }

        let target_value = match &mut self.t.lerp_mode {
            LerpMode::Absolute { .. } => self.t.to.eval(),
            LerpMode::SpeedBased { step_sum, .. } => {
                *step_sum = 0.;
                self.t.to.eval()
            }
            LerpMode::Relative {
                duration: _,
                relative_to,
                previous_anim_pos,
            } => {
                let val_at_obj = self.t.data.get_property_value();
                let end = self.t.to.eval();
                let previous_relative =
                    self.t
                        .lerper
                        .spire_lerp(relative_to, &end, *previous_anim_pos);
                self.t
                    .lerper
                    .add_relative(&val_at_obj, &previous_relative, &end)
            }
        };

        self.t.data.set_property_value(target_value);
        Ok(())
    }
}

impl<T> SpireTweener for SpireTween<LerpPropertyData<T>>
where
    T: PropertyType + FromGodot + ToGodot + Clone + Default + Debug,
    LerpPropertyData<T>: ITweenable,
{
    #[inline]
    fn play(&mut self) {
        if self.is_stopped() {
            self.reset_counters();

            if let LoopMode::Restart = self.loop_mode {
                self.t.lerp_mode.reset_state();
            }
        }

        self.state = State::Playing;
    }

    #[inline]
    fn pause(&mut self) {
        if !self.is_stopped() {
            self.state = State::Paused;
        }
    }

    #[inline]
    fn stop(&mut self) {
        self.t.lerp_mode.reset_state();
        self.state = State::Stopped;
    }

    fn force_complete(&mut self) {
        match self.state {
            | State::Playing | State::Paused => {
                match self.try_seek_end() {
                    Ok(_) => {}
                    Err(err) => {
                        godot_error!("{err}");
                    }
                }

                self.handle_finished();
            }
            State::Stopped => {}
        }
    }

    fn process(&mut self, delta_time: f64, _is_tree_paused: bool) -> AdvanceTimeResult {
        match self.check_owner_validity_and_pause_mode(self.t.data.get_owner()) {
            ObjectValidityResult::CanProcess => {}
            ObjectValidityResult::DontProcess => return AdvanceTimeResult::Paused,
            ObjectValidityResult::SomeObjectsDead => {
                self.stop();
                return AdvanceTimeResult::ShouldDespawn;
            }
        }

        let Some(step) = self.handle_time_step(delta_time)
        else { return AdvanceTimeResult::Playing };

        let set_fn = <T as PropertyType>::Data::set_property_value;

        let step_excess = match &mut self.t.lerp_mode {
            LerpMode::Absolute { duration, from } => {
                let start_val = match &from {
                    FromValue::AlreadyEvaluated(val) | FromValue::Explicit(val) => val,
                    FromValue::PendingEvaluation => {
                        let val_at_obj = self.t.data.get_property_value();
                        from.set_evaluated(val_at_obj)
                    }
                };

                let anim_pos = calc_animation_position(
                    *duration,
                    self.loop_time,
                    self.loop_counter,
                    self.loop_mode,
                    &self.t.ease,
                );

                let target_val = self
                    .t
                    .lerper
                    .spire_lerp(start_val, &self.t.to.eval(), anim_pos);

                set_fn(&mut self.t.data, target_val);
                self.loop_time - *duration
            }
            LerpMode::SpeedBased {
                speed,
                from,
                start_distance,
                step_sum,
            } => {
                let curr_val = self.t.data.get_property_value();

                let end_val = self.t.to.eval();

                let start_distance_val = *start_distance.get_or_insert_with(|| {
                    let start_val = from.get_or_evaluate(|| curr_val.clone());
                    self.t.lerper.spire_distance(start_val, &end_val)
                });

                let remaining_distance = self.t.lerper.spire_distance(&curr_val, &end_val);

                if start_distance_val.is_zero_approx() {
                    *step_sum = 0.;
                    set_fn(&mut self.t.data, end_val);
                    step
                } else {
                    const EPS: f64 = 0.01;
                    let distance_ratio =
                        f64::max(0., 1. - (remaining_distance / start_distance_val));
                    let gain = (EPS + self.t.ease.sample(distance_ratio))
                        / (EPS + f64::abs(distance_ratio));

                    let actual_speed = *speed * gain;

                    let (target_val, step_result) = self.t.lerper.spire_step(
                        &curr_val,
                        &end_val,
                        actual_speed,
                        step + *step_sum,
                    );

                    set_fn(&mut self.t.data, target_val);

                    match step_result {
                        StepResult::Unfinished { accumulated_time } => {
                            *step_sum = accumulated_time;
                            -1.0
                        }
                        StepResult::Finished { excess_time } => {
                            *step_sum = 0.;
                            excess_time
                        }
                    }
                }
            }
            LerpMode::Relative {
                duration,
                relative_to,
                previous_anim_pos,
            } => {
                let curr_val = self.t.data.get_property_value();

                let next_anim_pos = calc_animation_position(
                    *duration,
                    self.loop_time,
                    self.loop_counter,
                    self.loop_mode,
                    &self.t.ease,
                );

                let end = self.t.to.eval();

                let previous_relative =
                    self.t
                        .lerper
                        .spire_lerp(relative_to, &end, *previous_anim_pos);
                let next_relative = self.t.lerper.spire_lerp(relative_to, &end, next_anim_pos);

                *previous_anim_pos = next_anim_pos;
                let target_val =
                    self.t
                        .lerper
                        .add_relative(&curr_val, &previous_relative, &next_relative);

                set_fn(&mut self.t.data, target_val);
                self.loop_time - *duration
            }
        };

        if step_excess < 0. {
            AdvanceTimeResult::Playing
        } else {
            if let LoopMode::Restart = self.loop_mode {
                self.t.lerp_mode.reset_state();
            }

            if let Some(excess_time) = self.handle_loop_finished(step_excess) {
                AdvanceTimeResult::Completed { excess_time }
            } else {
                AdvanceTimeResult::Playing
            }
        }
    }
}

// Builder Methods

// ------------------------------------------------------------
impl<T> SpireTween<LerpPropertyData<T>>
where
    T: PropertyType + Clone + Default + FromGodot,
    LerpPropertyData<T>: ITweenable,
{
    #[inline]
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.set_duration(duration);
        self
    }

    #[inline]
    pub fn with_ease(mut self, ease: EaseKind) -> Self {
        self.set_ease(ease);
        self
    }

    #[inline]
    pub fn begin_from(mut self, value: T) -> Self {
        self.set_begin_value(value);
        self
    }

    #[inline]
    pub fn end_at(mut self, value: T) -> Self {
        self.set_final_value(value);
        self
    }

    #[inline]
    pub fn as_absolute(mut self) -> Self
    where T: Default {
        self.set_absolute();
        self
    }

    #[inline]
    pub fn as_speed_based(mut self) -> Self {
        self.set_speed_based();
        self
    }

    #[inline]
    pub fn as_relative(mut self, relative_to: T) -> Self
    where T: Default + Clone + FromGodot {
        self.set_relative(relative_to);
        self
    }
}

impl<T> SpireTween<LerpPropertyData<T>>
where
    T: PropertyType,
    <T as ILerpable>::Lerper: Default,
    AnyTween: From<RcPtr<Self>>,
    LerpPropertyData<T>: ITweenable,
{
    pub fn new(data: T::Data, to: Evaluator<T>, duration: f64) -> Self {
        Self::new_with_data(LerpPropertyData {
            data,
            lerp_mode: LerpMode::absolute(duration),
            ease: Default::default(),
            to,
            lerper: Default::default(),
        })
    }

    pub fn new_registered(data: T::Data, end: Evaluator<T>, duration: f64) -> RcPtr<Self>
    where AnyTween: From<RcPtr<Self>> {
        Self::new(data, end, duration).register()
    }
}

// Variant Builder
impl SpireTween<LerpPropertyData<Variant>> {
    pub fn new_custom(
        property_path: impl AsArg<NodePath>,
        owner: impl Into<ObjectOrNode>,
        to: Evaluator<Variant>,
        duration: f64,
        lerper: CustomLerper,
    ) -> Self {
        Self::new_with_data(LerpPropertyData {
            data: PropertyDataCustom {
                path:  property_path.into_arg().cow_into_owned(),
                owner: owner.into(),
            },
            lerp_mode: LerpMode::absolute(duration),
            ease: Default::default(),
            to: match to {
                Evaluator::Static(val) => Evaluator::Static(val.to_variant()),
                Evaluator::Dynamic(mut f) => Evaluator::Dynamic(Box::new(move || f().to_variant())),
                Evaluator::Callable(call) => Evaluator::Callable(call),
            },
            lerper,
        })
    }
}
