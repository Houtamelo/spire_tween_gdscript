use super::*;

pub struct LerpCallableData {
    pub callable: Callable,
    pub owner: Option<ObjectOrNode>,
    pub from: Variant,
    pub to: Variant,
    pub duration: f64,
    pub ease: EaseKind,
    pub lerp_fn: Callable,
}

impl SpireTween<LerpCallableData> {
    pub fn new(
        callable: Callable,
        from: Variant,
        to: Variant,
        duration: f64,
        lerp_fn: Callable,
    ) -> Self {
        Self::new_with_data(LerpCallableData {
            owner: callable.object().map(ObjectOrNode::from_unchecked_object),
            callable,
            duration,
            ease: Default::default(),
            from,
            to,
            lerp_fn,
        })
    }

    pub fn new_registered(
        callable: Callable,
        from: Variant,
        to: Variant,
        duration: f64,
        lerp_fn: Callable,
    ) -> SpireHandle<LerpCallableData> {
        Self::new(callable, from, to, duration, lerp_fn).register()
    }
}

impl SpireTween<LerpCallableData> {
    pub fn get_callable(&self) -> Callable { self.t.callable.clone() }
    pub fn set_callable(&mut self, callable: Callable) { self.t.callable = callable; }

    pub fn get_start_value(&self) -> Variant { self.t.from.clone() }
    pub fn set_start_value(&mut self, from: Variant) { self.t.from = from; }

    pub fn get_final_value(&self) -> Variant { self.t.to.clone() }
    pub fn set_final_value(&mut self, to: Variant) { self.t.to = to; }

    pub fn get_duration(&self) -> f64 { self.t.duration }
    pub fn set_duration(&mut self, duration: f64) { self.t.duration = duration; }

    pub fn get_ease(&self) -> EaseKind { self.t.ease.clone() }
    pub fn set_ease(&mut self, ease: EaseKind) { self.t.ease = ease; }

    pub fn get_lerper(&self) -> Callable { self.t.lerp_fn.clone() }
    pub fn set_lerper(&mut self, lerp_func: Callable) { self.t.lerp_fn = lerp_func; }
}

impl SpireTweener for SpireTween<LerpCallableData> {
    #[inline]
    fn play(&mut self) { self.state = State::Playing; }

    #[inline]
    fn pause(&mut self) {
        if !self.is_stopped() {
            self.state = State::Paused;
        }
    }

    #[inline]
    fn stop(&mut self) {
        self.reset_counters();
        self.state = State::Stopped;
    }
}

impl AdvanceTime for SpireTween<LerpCallableData> {
    fn process(&mut self, delta_time: f64, _is_tree_paused: bool) -> AdvanceTimeResult {
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

        let actual_step = self.handle_time_step(delta_time);
        if actual_step < 0. {
            return AdvanceTimeResult::Playing;
        }

        let target_value = {
            let anim_pos = calc_animation_position(
                self.t.duration,
                self.loop_time,
                self.loop_counter,
                self.loop_mode,
                &self.t.ease,
            );

            self.t.lerp_fn.call(&[
                self.t.from.clone(),
                self.t.to.clone(),
                anim_pos.to_variant(),
            ])
        };

        self.t.callable.call(&[target_value]);

        let step_excess = self.loop_time - self.t.duration;
        if step_excess < 0. {
            AdvanceTimeResult::Playing
        } else {
            let excess_time = self.handle_loop_finished(step_excess);
            if excess_time >= 0. {
                AdvanceTimeResult::Completed { excess_time }
            } else {
                AdvanceTimeResult::Playing
            }
        }
    }

    fn force_complete(&mut self) {
        match self.state {
            | State::Playing | State::Paused => {
                self.seek_end();
            }
            State::Stopped => {}
        }
    }
}

impl SpireTween<LerpCallableData> {
    fn seek_end(&mut self) {
        let target_value = {
            let eased_ratio = self.t.ease.sample(1.);

            self.t.lerp_fn.call(&[
                self.t.from.clone(),
                self.t.to.clone(),
                eased_ratio.to_variant(),
            ])
        };

        self.t.callable.call(&[target_value]);

        self.handle_finished();
    }
}
