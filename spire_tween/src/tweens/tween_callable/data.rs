use super::*;

pub struct CallableData {
    pub callable: Callable,
    pub owner: Option<ObjectOrNode>,
    pub from: Variant,
    pub to: Variant,
    pub duration: f64,
    pub ease: Ease,
    pub lerp_fn: Callable,
}

impl SpireTween<CallableData> {
    pub fn new(
        callable: Callable,
        from: Variant,
        to: Variant,
        duration: f64,
        auto_play: AutoPlay,
        lerp_fn: Callable,
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
            t: CallableData {
                owner: callable.object().map(ObjectOrNode::from_unchecked_object),
                callable,
                duration,
                ease: Default::default(),
                from,
                to,
                lerp_fn,
            },
            calls_on_finish: Vec::new(),
        }
    }

    pub fn new_registered(
        callable: Callable,
        from: Variant,
        to: Variant,
        duration: f64,
        auto_play: AutoPlay,
        lerp_fn: Callable,
    ) -> SpireHandle<CallableData> {
        Self::new(callable, from, to, duration, auto_play, lerp_fn).register()
    }
}

impl SpireTween<CallableData> {
    pub fn get_callable(&self) -> Callable { self.t.callable.clone() }
    pub fn set_callable(&mut self, callable: Callable) { self.t.callable = callable; }

    pub fn get_start_value(&self) -> Variant { self.t.from.clone() }
    pub fn set_start_value(&mut self, from: Variant) { self.t.from = from; }

    pub fn get_final_value(&self) -> Variant { self.t.to.clone() }
    pub fn set_final_value(&mut self, to: Variant) { self.t.to = to; }

    pub fn get_duration(&self) -> f64 { self.t.duration }
    pub fn set_duration(&mut self, duration: f64) { self.t.duration = duration; }

    pub fn get_ease(&self) -> Ease { self.t.ease.clone() }
    pub fn set_ease(&mut self, ease: Ease) { self.t.ease = ease; }

    pub fn get_lerp_fn(&self) -> Callable { self.t.lerp_fn.clone() }
    pub fn set_lerp_fn(&mut self, lerp_func: Callable) { self.t.lerp_fn = lerp_func; }
}

impl SpireTweener for SpireTween<CallableData> {
    #[inline]
    fn get_state(&self) -> TweenState { self.state }

    #[inline]
    fn set_state(&mut self, state: TweenState) { self.state = state; }
}

impl AdvanceTime for SpireTween<CallableData> {
    fn advance_time(&mut self, delta_time: f64) -> f64 {
        if let Some(owner) = &self.t.owner {
            match owner {
                ObjectOrNode::Object(obj) => {
                    let instance_id = obj.instance_id_unchecked().to_i64();
                    if !is_instance_id_valid(instance_id) {
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
        }

        self.elapsed_time += delta_time * self.speed_scale;

        if self.elapsed_time < self.delay {
            return -1.0;
        }

        let target_value = {
            let eased_ratio = {
                let elapsed_ratio =
                    ratio_with_delay_duration(self.delay, self.t.duration, self.elapsed_time);

                self.t.ease.sample(elapsed_ratio)
            };

            self.t.lerp_fn.call(&[
                self.t.from.clone(),
                self.t.to.clone(),
                eased_ratio.to_variant(),
            ])
        };

        let maybe_excess_time = {
            let total_duration = self.delay + self.t.duration;
            let excess = self.elapsed_time - total_duration;
            if excess > 0. {
                Some(excess)
            } else {
                None
            }
        };

        self.t.callable.call(&[target_value]);

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

    fn complete(&mut self) {
        match self.state {
            | TweenState::Playing | TweenState::Paused => {
                self.seek_end();
            }
            TweenState::Stopped => {}
        }
    }
}

impl SpireTween<CallableData> {
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
