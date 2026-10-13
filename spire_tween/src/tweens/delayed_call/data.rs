use super::*;

impl SpireTweener for SpireTween<Callable> {
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
            }
            State::Stopped => {}
        }
    }

    fn process_internal(&mut self, delta_time: f64, _is_tree_paused: bool) -> AdvanceTimeResult {
        if !self.check_callable_validity() {
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
    fn check_callable_validity(&mut self) -> bool {
        if self.t.is_valid() {
            return true;
        }

        self.stop();
        // TweenPtr's Rc<UnsafeCell<SpireTween>> points at this same allocation.
        // Trait methods only have &mut self, so enqueue removal by its address.
        TM.tween_unregister_address(std::ptr::from_ref(self).cast());
        false
    }

    fn seek_end(&mut self) {
        if !self.check_callable_validity() {
            return;
        }

        self.loop_time = self.delay;
        self.t.call(&[]);
        self.handle_finished();
    }
}

impl SpireTween<Callable> {
    /// Constructs a delayed-call tween: invokes `callable` once after `delay` seconds
    /// have elapsed, then auto-stops. The callable receives no arguments — bind any
    /// extra args via `Callable::from_fn` or `Callable::bind` first.
    /// Invalid callables stop and unregister on the next processing step, or
    /// immediately when forced to complete. Cancellation does not emit finish
    /// or loop-finish events.
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
