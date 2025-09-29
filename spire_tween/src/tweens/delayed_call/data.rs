use super::*;

pub enum DelayedCallData {
    Callable(Callable),
    Closure(Box<dyn FnMut()>),
}

impl DelayedCallData {
    pub fn invoke(&mut self) {
        match self {
            DelayedCallData::Callable(callable) => {
                if callable.is_valid() {
                    callable.callv(&VariantArray::new());
                } else {
                    godot_warn!(
                        "Cannot invoke callable: {:?}, it is invalid",
                        callable.method_name()
                    );
                }
            }
            DelayedCallData::Closure(closure) => {
                closure();
            }
        }
    }
}

impl<T: FnMut() + 'static> From<T> for DelayedCallData {
    fn from(value: T) -> Self { Self::Closure(Box::new(value)) }
}

impl From<Callable> for DelayedCallData {
    fn from(value: Callable) -> Self { Self::Callable(value) }
}

impl SpireTweener for SpireTween<DelayedCallData> {
    #[inline]
    fn get_state(&self) -> TweenState { self.state }

    #[inline]
    fn set_state(&mut self, state: TweenState) { self.state = state; }
}

impl AdvanceTime for SpireTween<DelayedCallData> {
    fn complete(&mut self) {
        match self.state {
            | TweenState::Playing | TweenState::Paused => {
                self.seek_end();
            }
            TweenState::Stopped => {}
        }
    }

    fn advance_time(&mut self, delta_time: f64) -> f64 {
        self.elapsed_time += delta_time * self.speed_scale;

        let excess = self.elapsed_time - self.delay;
        if excess <= 0. {
            return -1.0;
        }

        self.t.invoke();

        self.cycle_count += 1;
        self.elapsed_time = excess;

        match &mut self.loop_mode {
            LoopMode::Infinite => -1.0,
            LoopMode::Finite(loop_count) => {
                if self.cycle_count < *loop_count {
                    -1.0
                } else {
                    self.elapsed_time = self.delay;
                    self.handle_finished();
                    excess
                }
            }
        }
    }
}

impl SpireTween<DelayedCallData> {
    fn seek_end(&mut self) {
        self.elapsed_time = self.delay;
        self.t.invoke();
        self.handle_finished();
    }
}

impl SpireTween<DelayedCallData> {
    pub fn new(f: impl FnMut() + 'static, delay: f64, auto_play: AutoPlay) -> Self {
        Self {
            bound_nodes: Default::default(),
            state: match auto_play.0 {
                true => TweenState::Playing,
                false => TweenState::Paused,
            },
            delay,
            speed_scale: 1.,
            elapsed_time: 0.,
            cycle_count: 0,
            pause_mode: SpirePauseMode::Stop,
            process_mode: SpireProcessMode::Idle,
            loop_mode: LoopMode::Finite(0),
            calls_on_finish: Vec::new(),
            t: f.into(),
        }
    }

    pub fn new_registered(
        f: impl FnMut() + 'static,
        delay: f64,
        auto_play: AutoPlay,
    ) -> SpireHandle<DelayedCallData> {
        Self::new(f, delay, auto_play).register()
    }

    pub fn new_callable(callable: Callable, delay: f64, auto_play: AutoPlay) -> Self {
        Self {
            bound_nodes: Default::default(),
            state: match auto_play.0 {
                true => TweenState::Playing,
                false => TweenState::Paused,
            },
            delay,
            speed_scale: 1.,
            elapsed_time: 0.,
            cycle_count: 0,
            pause_mode: SpirePauseMode::Stop,
            process_mode: SpireProcessMode::Idle,
            loop_mode: LoopMode::Finite(0),
            calls_on_finish: Vec::new(),
            t: callable.into(),
        }
    }

    pub fn new_callable_registered(
        callable: Callable,
        delay: f64,
        auto_play: AutoPlay,
    ) -> SpireHandle<DelayedCallData> {
        Self::new_callable(callable, delay, auto_play).register()
    }
}
