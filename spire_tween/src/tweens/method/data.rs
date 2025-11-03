use super::*;

pub struct LerpMethodData<T: ILerpable> {
    pub owner: Option<ObjectOrNode>,
    pub callable: Callable,
    pub from: T,
    pub to: T,
    pub duration: f64,
    pub ease: EaseKind,
    pub lerper: T::BasicLerper,
}

impl<T: ILerpable> SpireTween<LerpMethodData<T>>
where LerpMethodData<T>: ITweenable
{
    #[inline]
    pub fn get_callable(&self) -> &Callable { &self.t.callable }

    #[inline]
    pub fn get_owner(&self) -> &Option<ObjectOrNode> { &self.t.owner }

    #[inline]
    pub fn get_duration(&self) -> f64 { self.t.duration }
    #[inline]
    pub fn set_duration(&mut self, duration: f64) { self.t.duration = duration; }

    #[inline]
    pub fn get_ease(&self) -> &EaseKind { &self.t.ease }
    #[inline]
    pub fn set_ease(&mut self, ease: EaseKind) { self.t.ease = ease; }

    #[inline]
    pub fn get_start_value(&self) -> T
    where T: Clone {
        self.t.from.clone()
    }

    #[inline]
    pub fn set_start_value(&mut self, from: T) { self.t.from = from; }

    #[inline]
    pub fn get_final_value(&self) -> T
    where T: Clone {
        self.t.to.clone()
    }

    #[inline]
    pub fn set_final_value(&mut self, to: T) { self.t.to = to; }
}

impl<T: ILerpable + ToGodot> SpireTweener for SpireTween<LerpMethodData<T>>
where LerpMethodData<T>: ITweenable
{
    #[inline]
    fn play(&mut self) {
        if self.is_stopped() {
            self.reset_counters();
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
    fn stop(&mut self) { self.state = State::Stopped; }

    #[inline]
    fn force_complete(&mut self) {
        match self.state {
            | State::Playing | State::Paused => {
                self.seek_end();
                self.handle_finished();
            }
            State::Stopped => {}
        }
    }

    fn process(&mut self, delta_time: f64, _is_tree_paused: bool) -> AdvanceTimeResult {
        if !self.t.callable.is_valid() {
            return AdvanceTimeResult::ShouldDespawn;
        }

        if let Some(owner) = &self.t.owner {
            match self.check_owner_validity_and_pause_mode(owner) {
                ObjectValidityResult::CanProcess => {}
                ObjectValidityResult::DontProcess => return AdvanceTimeResult::Paused,
                ObjectValidityResult::SomeObjectsDead => {
                    self.stop();
                    return AdvanceTimeResult::ShouldDespawn;
                }
            }
        }

        if self.handle_time_step(delta_time).is_none() {
            return AdvanceTimeResult::Playing;
        }

        let anim_pos = calc_animation_position(
            self.t.duration,
            self.loop_time,
            self.loop_counter,
            self.loop_mode,
            &self.t.ease,
        );

        let target_value = self.t.lerper.spire_lerp(&self.t.from, &self.t.to, anim_pos);

        self.t.callable.call(&[target_value.to_variant()]);

        let excess_time = self.loop_time - self.t.duration;
        if excess_time < 0. {
            AdvanceTimeResult::Playing
        } else if let Some(excess_time) = self.handle_loop_finished(excess_time) {
            AdvanceTimeResult::Completed { excess_time }
        } else {
            AdvanceTimeResult::Playing
        }
    }
}

impl<T: ILerpable> SpireTween<LerpMethodData<T>>
where
    T: ToGodot,
    LerpMethodData<T>: ITweenable,
{
    fn seek_end(&mut self) {
        if !self.t.callable.is_valid() {
            return;
        }

        let target_value = {
            let eased_ratio = self.t.ease.sample(1.);
            self.t
                .lerper
                .spire_lerp(&self.t.from, &self.t.to, eased_ratio)
        };

        self.t.callable.call(&[target_value.to_variant()]);
    }
}

// ----------------------------------------------------------------
// Builder methods

impl<T: ILerpable> SpireTween<LerpMethodData<T>>
where LerpMethodData<T>: ITweenable
{
    pub fn with_duration(self, duration: f64) -> Self {
        Self {
            t: LerpMethodData { duration, ..self.t },
            ..self
        }
    }

    pub fn with_ease(self, ease: EaseKind) -> Self {
        Self {
            t: LerpMethodData { ease, ..self.t },
            ..self
        }
    }

    pub fn with_end(self, end: T) -> Self {
        Self {
            t: LerpMethodData { to: end, ..self.t },
            ..self
        }
    }

    pub fn with_begin(self, start: T) -> Self {
        Self {
            t: LerpMethodData {
                from: start,
                ..self.t
            },
            ..self
        }
    }
}

impl<T: ILerpable<BasicLerper: Default>> SpireTween<LerpMethodData<T>>
where
    AnyTween: From<RcPtr<Self>>,
    LerpMethodData<T>: ITweenable,
{
    pub fn new(callable: Callable, from: T, to: T, duration: f64) -> Self {
        let owner = callable.object().map(ObjectOrNode::from_unchecked_object);

        Self::new_with_data(LerpMethodData {
            owner,
            callable,
            duration,
            ease: Default::default(),
            from,
            to,
            lerper: Default::default(),
        })
    }

    pub fn new_registered(callable: Callable, from: T, to: T, duration: f64) -> RcPtr<Self>
    where AnyTween: From<RcPtr<Self>> {
        Self::new(callable, from, to, duration).register()
    }
}

impl SpireTween<LerpMethodData<Variant>> {
    pub fn new_custom(
        callable: Callable,
        from: Variant,
        to: Variant,
        duration: f64,
        lerper: Callable,
    ) -> Self {
        let owner = callable.object().map(ObjectOrNode::from_unchecked_object);

        Self::new_with_data(LerpMethodData {
            owner,
            callable,
            duration,
            ease: Default::default(),
            from,
            to,
            lerper: CustomBasicLerper::new(lerper),
        })
    }
}
