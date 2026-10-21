use super::*;

impl SpireTweener for SpireTween<Callable> {
    fn play_internal(&mut self) {
        match self.state {
            State::Stopped => {
                self.reset_counters();
            }
            State::Paused | State::Playing => {}
        }
    }

    fn pause_internal(&mut self) {}

    fn stop_internal(&mut self) {}

    fn force_complete_internal(&mut self) -> bool {
        let is_valid = self.t.is_valid();
        if is_valid {
            self.loop_time = self.delay;
            self.t.call(&[]);
        }

        is_valid
    }

    fn process_internal(&mut self, delta_time: f64, _is_tree_paused: bool) -> AdvanceTimeResult {
        if !self.t.is_valid() {
            return AdvanceTimeResult::ShouldDespawn;
        }

        if let Some(step) = self.handle_time_step(delta_time) {
            self.t.call(&[]);

            if let Some(excess_time) = self.handle_loop_finished(step) {
                let unscaled_excess_time = unscale_time(excess_time, self.speed_scale);
                AdvanceTimeResult::Completed { unscaled_excess_time }
            } else {
                AdvanceTimeResult::Playing
            }
        } else {
            AdvanceTimeResult::Playing
        }
    }
}

impl SpireTween<Callable> {
    /// Constructs a delayed-call tween: invokes `callable` once after `delay` seconds
    /// have elapsed, then auto-stops. The callable receives no arguments — bind any
    /// extra args via `Callable::from_fn` or `Callable::bind` first.
    /// Invalid callables stop on the next processing step, or immediately when
    /// forced to complete, without invocation or finish/loop-finish events.
    /// Processing reports [`AdvanceTimeResult::ShouldDespawn`] to its consumer.
    /// Manual callers choose when to unregister or release their handles;
    /// forced stopping preserves registration while external owners remain.
    ///
    /// You usually don't call this directly — use [`DoDelayedCall::do_delayed_call`]
    /// (closure form) or [`DoDelayedCallable::do_delayed_callable`] (existing
    /// `Callable` form), which auto-bind the tween to the calling node so it
    /// auto-stops if the node is freed.
    pub fn new(callable: Callable, delay: f64) -> Self { Self::new_with_data(callable).with_delay(delay) }

    /// Convenience: [`new`](Self::new) followed by [`register`](SpireTween::register).
    /// Submits the tween to the global `TweenManager` and returns the [`TweenPtr`] handle.
    pub fn new_registered(callable: Callable, delay: f64) -> TweenPtr<Callable> {
        Self::new(callable, delay).register()
    }
}
