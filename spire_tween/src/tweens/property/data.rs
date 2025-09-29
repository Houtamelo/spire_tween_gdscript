use replace_with::replace_with_or_abort;

use super::*;
use crate::{
    benchmarking::benchmark,
    cow_fn::{CowDistanceFn, CowRelativeFn, CowStepFn},
};

pub struct LerpPropertyData<T: PropertyType> {
    pub data: T::Data,
    pub lerp_mode: LerpMode<T>,
    pub ease: Ease,
    pub to: Evaluator<T>,
    pub lerp_fn: CowLerpFn<T>,
    pub relative_fn: CowRelativeFn<T>,
    pub step_fn: CowStepFn<T>,
    pub distance_fn: CowDistanceFn<T>,
}

impl<Val> SpireTweener for SpireTween<LerpPropertyData<Val>>
where Val: PropertyType
{
    #[inline]
    fn get_state(&self) -> TweenState { self.state }

    #[inline]
    fn set_state(&mut self, state: TweenState) {
        match state {
            TweenState::Stopped => {
                self.elapsed_time = 0.0;
                self.cycle_count = 0;
            }
            | TweenState::Playing | TweenState::Paused => {}
        }

        self.state = state;
    }
}

impl<T> SpireTween<LerpPropertyData<T>>
where T: PropertyType + Clone + Default + FromGodot
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
    pub fn get_ease(&self) -> Ease { self.t.ease.clone() }
    #[inline]
    pub fn set_ease(&mut self, ease: Ease) { self.t.ease = ease; }

    #[inline]
    pub fn get_final_value(&mut self) -> T
    where T: Clone + FromGodot + Default {
        self.t.to.eval()
    }

    #[inline]
    pub fn set_final_value(&mut self, to: T) { self.t.to = Evaluator::Static(to); }

    pub fn set_begin_value(&mut self, value: T) {
        match &mut self.t.lerp_mode {
            LerpMode::Absolute { from, .. } => {
                *from = Some(value);
            }
            LerpMode::SpeedBased { .. } => {
                godot_warn!("Starting value is not used in current lerp mode `SpeedBased`.");
            }
            LerpMode::Relative { origin, .. } => {
                *origin = value;
            }
        }
    }
}

impl<T> SpireTween<LerpPropertyData<T>>
where T: PropertyType + Clone + Default + FromGodot
{
    #[inline]
    pub fn set_relative(&mut self, relative_to: T) {
        replace_with_or_abort(&mut self.t.lerp_mode, |lerp_mode| {
            match lerp_mode {
                LerpMode::Absolute { duration, .. } => {
                    LerpMode::Relative {
                        duration,
                        origin: relative_to,
                    }
                }
                LerpMode::SpeedBased { speed, .. } => {
                    let distance = (self.t.distance_fn)(&relative_to, &self.t.to.eval());
                    let duration = distance / speed;

                    LerpMode::Relative {
                        duration,
                        origin: relative_to,
                    }
                }
                LerpMode::Relative { duration, .. } => {
                    LerpMode::Relative {
                        duration,
                        origin: relative_to,
                    }
                }
            }
        })
    }

    #[inline]
    pub fn set_absolute(&mut self) {
        replace_with_or_abort(&mut self.t.lerp_mode, |lerp_mode| {
            match lerp_mode {
                LerpMode::SpeedBased { speed, .. } => {
                    let val_at_obj = self.t.data.get_property_value();

                    let distance = (self.t.distance_fn)(&val_at_obj, &self.t.to.eval());
                    let duration = distance / speed;

                    LerpMode::Absolute {
                        duration,
                        from: None,
                    }
                }
                LerpMode::Relative { duration, .. } => {
                    LerpMode::Absolute {
                        duration,
                        from: None,
                    }
                }
                already_abs @ LerpMode::Absolute { .. } => already_abs,
            }
        })
    }

    #[inline]
    pub fn set_speed_based(&mut self, speed: f64) {
        self.t.lerp_mode = LerpMode::SpeedBased { speed, t_sum: 0. };
    }
}

impl<Val> SpireTween<LerpPropertyData<Val>>
where Val: PropertyType
{
    #[inline]
    pub fn is_absolute(&self) -> bool { matches!(self.t.lerp_mode, LerpMode::Absolute { .. }) }

    #[inline]
    pub fn is_relative(&self) -> bool { matches!(self.t.lerp_mode, LerpMode::Relative { .. }) }

    #[inline]
    pub fn is_speed_based(&self) -> bool { matches!(self.t.lerp_mode, LerpMode::SpeedBased { .. }) }

    #[inline]
    pub fn get_duration(&mut self) -> f64
    where Val: Clone + Default + FromGodot {
        match self.t.lerp_mode {
            LerpMode::Absolute { duration, .. } => duration,
            LerpMode::SpeedBased { speed, .. } => {
                (self.t.distance_fn)(&self.t.data.get_property_value(), &self.t.to.eval()) / speed
            }
            LerpMode::Relative { duration, .. } => duration,
        }
    }

    #[inline]
    pub fn set_duration(&mut self, new_duration: f64)
    where Val: Clone + Default + FromGodot {
        match &mut self.t.lerp_mode {
            LerpMode::Absolute { duration, .. } => *duration = new_duration,
            LerpMode::Relative { duration, .. } => *duration = new_duration,
            LerpMode::SpeedBased { speed, .. } => {
                godot_warn!(
                    "Setting duration on a speed-based tween is not recommended since it requires recalculating speed."
                );

                let distance =
                    (self.t.distance_fn)(&self.t.data.get_property_value(), &self.t.to.eval());
                if new_duration > 0.0 {
                    *speed = distance / new_duration;
                } else {
                    godot_error!(
                        "Expected duration to be greater than zero, got: `{new_duration:.2}`."
                    );
                }
            }
        }
    }
}

impl<Val> SpireTween<LerpPropertyData<Val>>
where Val: PropertyType + Clone + Default + FromGodot + Debug
{
    fn do_step(&mut self, delta_time: f64) -> Option<(Val, Option<f64>)> {
        match &mut self.t.lerp_mode {
            LerpMode::Absolute { duration, from } => {
                let start_val = match &from {
                    Some(val) => val,
                    None => {
                        let val_at_obj = self.t.data.get_property_value();
                        from.replace(val_at_obj);
                        from.as_ref().unwrap()
                    }
                };

                let target_value = {
                    let elapsed_ratio =
                        ratio_with_delay_duration(self.delay, *duration, self.elapsed_time);

                    let eased_ratio = self.t.ease.sample(elapsed_ratio.min(1.));

                    (self.t.lerp_fn)(start_val, &self.t.to.eval(), eased_ratio)
                };

                let excess_time = {
                    let total_duration = self.delay + *duration;
                    let excess = self.elapsed_time - total_duration;
                    if excess > 0. {
                        Some(excess)
                    } else {
                        None
                    }
                };

                Some((target_value, excess_time))
            }
            LerpMode::SpeedBased { speed, t_sum } => {
                let (target_value, step_result) = {
                    let val_at_obj = self.t.data.get_property_value();

                    (self.t.step_fn)(&val_at_obj, &self.t.to.eval(), *speed, delta_time + *t_sum)
                };

                let excess_time = match step_result {
                    StepResult::Unfinished {
                        accumulated_time: accumulated_t,
                    } => {
                        *t_sum = accumulated_t;
                        None
                    }
                    StepResult::Finished { excess_time } => {
                        *t_sum = 0.;
                        Some(excess_time)
                    }
                };

                Some((target_value, excess_time))
            }
            LerpMode::Relative { duration, origin } => {
                let target_value = {
                    let val_at_obj = self.t.data.get_property_value();

                    let previous_eased_ratio = {
                        let previous_ratio = ratio_with_delay_duration(
                            self.delay,
                            *duration,
                            self.elapsed_time - delta_time,
                        );

                        self.t.ease.sample(previous_ratio)
                    };

                    let next_eased_ratio = {
                        let elapsed_ratio =
                            ratio_with_delay_duration(self.delay, *duration, self.elapsed_time);

                        self.t.ease.sample(elapsed_ratio)
                    };

                    let end = self.t.to.eval();

                    let previous_relative = (self.t.lerp_fn)(origin, &end, previous_eased_ratio);

                    let next_relative = (self.t.lerp_fn)(origin, &end, next_eased_ratio);

                    (self.t.relative_fn)(&val_at_obj, &previous_relative, &next_relative)
                };

                let excess_time = {
                    let total_duration = self.delay + *duration;
                    let excess = self.elapsed_time - total_duration;

                    if excess > 0. {
                        Some(excess)
                    } else {
                        None
                    }
                };

                Some((target_value, excess_time))
            }
        }
    }
}

impl<Val: PropertyType> SpireTween<LerpPropertyData<Val>> {
    fn try_seek_end(&mut self) -> anyhow::Result<()>
    where Val: Clone + Default + FromGodot {
        if !self.t.data.get_owner().is_instance_valid() {
            let prop_name = &self.t.data.get_property_path();
            bail!(
                "Cannot set property `{prop_name}` on Object, owner does not point to a valid instance."
            );
        }

        let target_value = match &mut self.t.lerp_mode {
            LerpMode::Absolute { .. } => self.t.to.eval(),
            LerpMode::SpeedBased { t_sum, .. } => {
                *t_sum = 0.;
                self.t.to.eval()
            }
            LerpMode::Relative { duration, origin } => {
                let val_at_obj = self.t.data.get_property_value();

                let end = self.t.to.eval();

                let previous_relative = {
                    let previous_eased_ratio = {
                        let previous_ratio =
                            ratio_with_delay_duration(self.delay, *duration, self.elapsed_time);

                        self.t.ease.sample(previous_ratio)
                    };

                    (self.t.lerp_fn)(origin, &end, previous_eased_ratio)
                };

                (self.t.relative_fn)(&val_at_obj, &previous_relative, &end)
            }
        };

        self.t.data.set_property_value(target_value);

        Ok(())
    }
}

impl<Val> AdvanceTime for SpireTween<LerpPropertyData<Val>>
where Val: PropertyType + FromGodot + ToGodot + Clone + Default + Debug
{
    fn complete(&mut self) {
        match self.state {
            | TweenState::Playing | TweenState::Paused => {
                match self.try_seek_end() {
                    Ok(_) => {}
                    Err(err) => {
                        godot_error!("{err}");
                    }
                }

                self.handle_finished();
            }
            TweenState::Stopped => {}
        }
    }

    fn advance_time(&mut self, delta_time: f64) -> f64 {
        match self.t.data.get_owner() {
            ObjectOrNode::Object(obj) => {
                let id = obj.instance_id_unchecked().to_i64();
                if !is_instance_id_valid(id) {
                    self.stop();
                    return -1.0;
                }
            }
            ObjectOrNode::Node(node) => {
                match TWEENS.node_get_status_fresh(*node) {
                    NodeStatus::InsideTree => {
                        if let SpirePauseMode::Bound = self.pause_mode
                            && !node.is_processing()
                        {
                            return -1.0;
                        }
                    }
                    NodeStatus::OutsideTreeMaybeDead => {
                        if let SpirePauseMode::Bound = self.pause_mode {
                            return -1.0;
                        }
                    }
                    NodeStatus::Dead => {
                        self.stop();
                        return -1.0;
                    }
                }
            }
        }

        self.elapsed_time += delta_time * self.speed_scale;

        if self.elapsed_time < self.delay {
            return -1.0;
        }

        let Some((target_value, maybe_excess_time)) =
            (benchmark! { Self::do_step[self, delta_time] })
        else {
            return -1.0
        };
        /*
        let Some((target_value, maybe_excess_time)) = self.do_step(delta_time)
        else { return -1.0 };
        */

        //self.t.data.set_property_value(target_value);

        let f = <Val as PropertyType>::Data::set_property_value;
        benchmark! { f[&mut self.t.data, target_value] }

        let Some(excess_time) = maybe_excess_time
        else { return -1.0 };

        self.cycle_count += 1;

        match &mut self.loop_mode {
            LoopMode::Infinite => {
                self.elapsed_time = self.delay + excess_time;
                -1.0
            }
            LoopMode::Finite(loop_count) => {
                if self.cycle_count < *loop_count {
                    self.elapsed_time = self.delay + excess_time;
                    -1.0
                } else {
                    self.elapsed_time -= excess_time;
                    self.handle_finished();
                    excess_time
                }
            }
        }
    }
}

// Builder Methods

// ------------------------------------------------------------
impl<Val> SpireTween<LerpPropertyData<Val>>
where Val: PropertyType + Clone + Default + FromGodot
{
    pub fn with_duration(self, duration: f64) -> Self {
        match self.t.lerp_mode {
            LerpMode::Absolute { from, .. } => {
                Self {
                    t: LerpPropertyData {
                        lerp_mode: LerpMode::Absolute { duration, from },
                        ..self.t
                    },
                    ..self
                }
            }
            LerpMode::Relative { origin, .. } => {
                Self {
                    t: LerpPropertyData {
                        lerp_mode: LerpMode::Relative { duration, origin },
                        ..self.t
                    },
                    ..self
                }
            }
            LerpMode::SpeedBased { .. } => {
                godot_warn!("Duration is not used in current lerp mode (SpeedBased).");
                self
            }
        }
    }

    pub fn with_ease(self, ease: Ease) -> Self {
        Self {
            t: LerpPropertyData { ease, ..self.t },
            ..self
        }
    }

    pub fn begin_from(mut self, value: Val) -> Self {
        self.set_begin_value(value);
        self
    }

    pub fn end_at(self, value: Val) -> Self {
        Self {
            t: LerpPropertyData {
                to: Evaluator::Static(value),
                ..self.t
            },
            ..self
        }
    }

    pub fn absolute(mut self) -> Self
    where Val: SpireLerp + Default {
        match self.t.lerp_mode {
            LerpMode::SpeedBased { speed, .. } => {
                let val_at_obj = self.t.data.get_property_value();

                let distance = (self.t.distance_fn)(&val_at_obj, &self.t.to.eval());
                let duration = distance / speed;

                Self {
                    t: LerpPropertyData {
                        lerp_mode: LerpMode::Absolute {
                            duration,
                            from: None,
                        },
                        ..self.t
                    },
                    ..self
                }
            }
            LerpMode::Relative { duration, origin } => {
                Self {
                    t: LerpPropertyData {
                        lerp_mode: LerpMode::Absolute {
                            duration,
                            from: Some(origin),
                        },
                        ..self.t
                    },
                    ..self
                }
            }
            LerpMode::Absolute { .. } => self,
        }
    }

    pub fn speed_based(self, speed: f64) -> Self {
        Self {
            t: LerpPropertyData {
                lerp_mode: LerpMode::SpeedBased { speed, t_sum: 0. },
                ..self.t
            },
            ..self
        }
    }

    pub fn relative(mut self, to: Val) -> Self
    where Val: Default + Clone + FromGodot {
        match self.t.lerp_mode {
            LerpMode::Absolute { duration, .. } => {
                Self {
                    t: LerpPropertyData {
                        lerp_mode: LerpMode::Relative {
                            duration,
                            origin: to,
                        },
                        ..self.t
                    },
                    ..self
                }
            }
            LerpMode::SpeedBased { speed, .. } => {
                let distance = (self.t.distance_fn)(&to, &self.t.to.eval());
                let duration = distance / speed;

                Self {
                    t: LerpPropertyData {
                        lerp_mode: LerpMode::Relative {
                            duration,
                            origin: to,
                        },
                        ..self.t
                    },
                    ..self
                }
            }
            LerpMode::Relative { duration, .. } => {
                Self {
                    t: LerpPropertyData {
                        lerp_mode: LerpMode::Relative {
                            duration,
                            origin: to,
                        },
                        ..self.t
                    },
                    ..self
                }
            }
        }
    }
}

impl<Val> SpireTween<LerpPropertyData<Val>>
where
    Val: PropertyType + SpireLerp,
    AnyTween: From<Self>,
{
    pub fn new(data: Val::Data, to: Evaluator<Val>, duration: f64, auto_play: AutoPlay) -> Self {
        Self {
            bound_nodes: Default::default(),
            state: match auto_play.0 {
                true => TweenState::Playing,
                false => TweenState::Paused,
            },
            delay: 0.,
            speed_scale: 1.,
            elapsed_time: 0.,
            cycle_count: 0,
            pause_mode: SpirePauseMode::Stop,
            process_mode: SpireProcessMode::Idle,
            loop_mode: LoopMode::Finite(0),
            calls_on_finish: Vec::new(),
            t: LerpPropertyData {
                data,
                lerp_mode: LerpMode::Absolute {
                    duration,
                    from: None,
                },
                ease: Default::default(),
                to,
                lerp_fn: CowLerpFn::from_static(Val::spire_lerp),
                relative_fn: CowRelativeFn::from_static(Val::add_relative),
                step_fn: CowStepFn::from_static(Val::spire_step),
                distance_fn: CowDistanceFn::from_static(Val::spire_distance),
            },
        }
    }

    pub fn new_registered(
        data: Val::Data,
        end: Evaluator<Val>,
        duration: f64,
        auto_play: AutoPlay,
    ) -> SpireHandle<LerpPropertyData<Val>> {
        Self::new(data, end, duration, auto_play).register()
    }
}

// Variant Builder
impl SpireTween<LerpPropertyData<Variant>> {
    pub fn new(
        property_path: impl AsArg<NodePath>,
        owner: impl Into<ObjectOrNode>,
        to: Evaluator<Variant>,
        duration: f64,
        auto_play: AutoPlay,
        lerp_fn: CowLerpFn<Variant>,
        relative_fn: CowRelativeFn<Variant>,
        step_fn: CowStepFn<Variant>,
        distance_fn: CowDistanceFn<Variant>,
    ) -> Self {
        Self {
            bound_nodes: Default::default(),
            state: match auto_play.0 {
                true => TweenState::Playing,
                false => TweenState::Paused,
            },
            delay: 0.,
            speed_scale: 1.,
            elapsed_time: 0.,
            cycle_count: 0,
            pause_mode: SpirePauseMode::Stop,
            process_mode: SpireProcessMode::Idle,
            loop_mode: LoopMode::Finite(0),
            calls_on_finish: Vec::new(),
            t: LerpPropertyData {
                data: PropertyDataCustom {
                    path:  property_path.into_arg().cow_into_owned(),
                    owner: owner.into(),
                },
                lerp_mode: LerpMode::Absolute {
                    duration,
                    from: None,
                },
                ease: Default::default(),
                to: match to {
                    Evaluator::Static(val) => Evaluator::Static(val.to_variant()),
                    Evaluator::Dynamic(mut f) => {
                        Evaluator::Dynamic(Box::new(move || f().to_variant()))
                    }
                    Evaluator::Callable(call) => Evaluator::Callable(call),
                },
                lerp_fn,
                relative_fn,
                step_fn,
                distance_fn,
            },
        }
    }

    pub fn new_registered(
        property_path: impl AsArg<NodePath>,
        owner: impl Into<ObjectOrNode>,
        to: Evaluator<Variant>,
        duration: f64,
        auto_play: AutoPlay,
        lerp_fn: CowLerpFn<Variant>,
        relative_fn: CowRelativeFn<Variant>,
        step_fn: CowStepFn<Variant>,
        distance_fn: CowDistanceFn<Variant>,
    ) -> SpireHandle<LerpPropertyData<Variant>> {
        Self::new(
            property_path,
            owner,
            to,
            duration,
            auto_play,
            lerp_fn,
            relative_fn,
            step_fn,
            distance_fn,
        )
        .register()
    }
}
