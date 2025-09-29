use super::*;

pub struct MethodData<T> {
    pub method_name: StringName,
    pub owner: ObjectOrNode,
    pub from: T,
    pub to: T,
    pub duration: f64,
    pub ease: Ease,
    pub lerp_fn: CowLerpFn<T>,
}

impl<T> SpireTween<MethodData<T>> {
    pub fn get_method_name(&self) -> StringName { self.t.method_name.clone() }
    pub fn set_method_name(&mut self, method_name: StringName) { self.t.method_name = method_name; }

    pub fn get_owner(&self) -> &ObjectOrNode { &self.t.owner }
    pub fn set_owner(&mut self, owner: impl Into<ObjectOrNode>) { self.t.owner = owner.into(); }

    pub fn get_duration(&self) -> f64 { self.t.duration }
    pub fn set_duration(&mut self, duration: f64) { self.t.duration = duration; }

    pub fn get_ease(&self) -> Ease { self.t.ease.clone() }
    pub fn set_ease(&mut self, ease: Ease) { self.t.ease = ease; }

    pub fn get_start_value(&self) -> T
    where T: Clone {
        self.t.from.clone()
    }

    pub fn set_start_value(&mut self, from: T) { self.t.from = from; }

    pub fn get_final_value(&self) -> T
    where T: Clone {
        self.t.to.clone()
    }

    pub fn set_final_value(&mut self, to: T) { self.t.to = to; }
}

impl<Val> SpireTweener for SpireTween<MethodData<Val>> {
    #[inline]
    fn get_state(&self) -> TweenState { self.state }

    #[inline]
    fn set_state(&mut self, state: TweenState) { self.state = state; }
}

impl<Val> AdvanceTime for SpireTween<MethodData<Val>>
where Val: ToGodot
{
    fn complete(&mut self) {
        match self.state {
            | TweenState::Playing | TweenState::Paused => {
                self.seek_end();
            }
            TweenState::Stopped => {}
        }
    }

    fn advance_time(&mut self, delta_time: f64) -> f64 {
        match &self.t.owner {
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

        /*
        if !self.t.owner.is_instance_valid() {
            self.stop();
            return -1.0;
        }
        */

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

            (self.t.lerp_fn)(&self.t.from, &self.t.to, eased_ratio)
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

        delegate_object_or_node! { &mut self.t.owner => |o| o.call(&self.t.method_name, &[target_value.to_variant()]); }

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

impl<Val> SpireTween<MethodData<Val>>
where Val: ToGodot
{
    fn seek_end(&mut self) {
        if self.t.owner.is_instance_valid() {
            let target_value = {
                let eased_ratio = self.t.ease.sample(1.);
                (self.t.lerp_fn)(&self.t.from, &self.t.to, eased_ratio)
            };

            delegate_object_or_node! { &mut self.t.owner => |o| o.call(&self.t.method_name, &[target_value.to_variant()]); }
        }

        self.handle_finished();
    }
}

// ----------------------------------------------------------------
// Builder methods

impl<T> SpireTween<MethodData<T>> {
    pub fn with_duration(self, duration: f64) -> Self {
        Self {
            t: MethodData { duration, ..self.t },
            ..self
        }
    }

    pub fn with_ease(self, ease: Ease) -> Self {
        Self {
            t: MethodData { ease, ..self.t },
            ..self
        }
    }

    pub fn with_end(self, end: T) -> Self {
        Self {
            t: MethodData { to: end, ..self.t },
            ..self
        }
    }

    pub fn with_begin(self, start: T) -> Self {
        Self {
            t: MethodData {
                from: start,
                ..self.t
            },
            ..self
        }
    }
}

impl<Val> SpireTween<MethodData<Val>>
where
    AnyTween: From<SpireTween<MethodData<Val>>>,
    Val: SpireLerp,
{
    pub fn new(
        method: impl Into<StringName>,
        owner: impl Into<ObjectOrNode>,
        start: Val,
        end: Val,
        duration: f64,
        auto_play: AutoPlay,
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
            t: MethodData {
                method_name: method.into(),
                owner: owner.into(),
                duration,
                ease: Default::default(),
                from: start,
                to: end,
                lerp_fn: CowLerpFn::from_static(<Val>::spire_lerp),
            },
            calls_on_finish: Vec::new(),
        }
    }

    pub fn new_registered(
        method: impl Into<StringName>,
        owner: impl Into<ObjectOrNode>,
        start: Val,
        end: Val,
        duration: f64,
        auto_play: AutoPlay,
    ) -> SpireHandle<MethodData<Val>> {
        Self::new(method, owner, start, end, duration, auto_play).register()
    }
}

// Variant Builder
impl SpireTween<MethodData<Variant>> {
    pub fn new(
        method: impl Into<StringName>,
        owner: impl Into<ObjectOrNode>,
        start: Variant,
        end: Variant,
        duration: f64,
        auto_play: AutoPlay,
        lerp_fn: impl Fn(&Variant, &Variant, f64) -> Variant + 'static,
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
            t: MethodData {
                method_name: method.into(),
                owner: owner.into(),
                duration,
                ease: Default::default(),
                from: start.to_variant(),
                to: end.to_variant(),
                lerp_fn: CowLerpFn::from_boxed(lerp_fn),
            },
            calls_on_finish: Vec::new(),
        }
    }

    pub fn new_registered(
        method: impl Into<StringName>,
        owner: impl Into<ObjectOrNode>,
        start: Variant,
        end: Variant,
        duration: f64,
        auto_play: AutoPlay,
        lerp_fn: impl Fn(&Variant, &Variant, f64) -> Variant + 'static,
    ) -> SpireHandle<MethodData<Variant>> {
        Self::new(method, owner, start, end, duration, auto_play, lerp_fn).register()
    }
}
