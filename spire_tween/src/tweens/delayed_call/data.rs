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

    fn process(&mut self, delta_time: f64, _is_tree_paused: bool) -> AdvanceTimeResult {
        if let Some(step) = self.handle_time_step(delta_time) {
            self.t.call(&[]);

            if let Some(excess_time) = self.handle_loop_finished(step) {
                AdvanceTimeResult::Completed { excess_time }
            } else {
                AdvanceTimeResult::Playing
            }
        } else {
            AdvanceTimeResult::Playing
        }
    }
}

impl SpireTween<Callable> {
    fn seek_end(&mut self) {
        self.loop_time = self.delay;
        self.t.call(&[]);
        self.handle_finished();
    }
}

impl SpireTween<Callable> {
    pub fn new(callable: Callable, delay: f64) -> Self {
        Self::new_with_data(callable).with_delay(delay)
    }

    pub fn new_registered(callable: Callable, delay: f64) -> RcPtr<Self> {
        Self::new(callable, delay).register()
    }
}
