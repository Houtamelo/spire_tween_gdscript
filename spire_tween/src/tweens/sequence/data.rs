use super::*;

pub struct Sequence {
    pub queue:   Vec<Vec<ForkElement>>,
    pub inserts: Vec<(f64, PtrTween)>,
}

pub enum ForkElement {
    Tween(PtrTween),
    Interval {
        total_time:   f64,
        elapsed_time: f64,
    },
}

impl From<PtrTween> for ForkElement {
    fn from(tween: PtrTween) -> Self { Self::Tween(tween) }
}

impl SpireTweener for SpireTween<Sequence> {
    #[inline]
    fn get_state(&self) -> TweenState { self.state }

    #[inline]
    fn set_state(&mut self, state: TweenState) {
        match state {
            TweenState::Playing => self.private_play(),
            TweenState::Paused => self.private_pause(),
            TweenState::Stopped => self.private_stop(),
        }
    }
}

impl SpireTween<Sequence> {
    fn private_play(&mut self) {
        if self.state == TweenState::Playing {
            return;
        }

        let from_begin = self.state == TweenState::Stopped;

        self.state = TweenState::Playing;

        self.t
            .queue
            .iter_mut()
            .flat_map(|vec| vec.iter_mut())
            .for_each(|fork_element| {
                match fork_element {
                    ForkElement::Tween(tween) => {
                        if from_begin {
                            tween.stop();
                            tween.pause();
                        } else {
                            tween.pause();
                        }
                    }
                    ForkElement::Interval { elapsed_time, .. } => {
                        if from_begin {
                            *elapsed_time = 0.;
                        }
                    }
                }
            });

        self.t.inserts.iter_mut().for_each(|(_, tween)| {
            if from_begin {
                tween.stop();
                tween.pause();
            } else {
                tween.pause();
            }
        });
    }

    fn private_pause(&mut self) { self.state = TweenState::Paused; }

    fn private_stop(&mut self) {
        if self.state == TweenState::Stopped {
            return;
        }

        self.state = TweenState::Stopped;
        self.elapsed_time = 0.0;

        self.t
            .queue
            .iter_mut()
            .flat_map(|vec| vec.iter_mut())
            .for_each(|fork_element| {
                match fork_element {
                    ForkElement::Tween(tween) => {
                        tween.stop();
                    }
                    ForkElement::Interval { elapsed_time, .. } => {
                        *elapsed_time = 0.;
                    }
                }
            });

        self.t
            .inserts
            .iter_mut()
            .for_each(|(_, tween)| tween.stop());
    }
}

impl AdvanceTime for SpireTween<Sequence> {
    fn advance_time(&mut self, delta_time: f64) -> f64 {
        let raw_delta_time = delta_time * self.speed_scale;
        let old_elapsed = self.elapsed_time;
        self.elapsed_time += delta_time;

        if self.elapsed_time < self.delay {
            return -1.0;
        }

        let total_after_delay = self.elapsed_time - self.delay;

        let delta_time = if old_elapsed > self.delay {
            raw_delta_time
        } else {
            total_after_delay
        };

        for (at, tween) in self.t.inserts.iter_mut() {
            match tween.get_state() {
                TweenState::Playing => {
                    tween.advance_time(delta_time);
                }
                TweenState::Paused => {
                    if *at <= total_after_delay {
                        let above_at = total_after_delay - *at;
                        tween.play();
                        tween.advance_time(above_at);
                    }
                }
                TweenState::Stopped => {}
            }
        }

        let mut remaining_delta = delta_time;
        let mut queue_iter = self.t.queue.iter_mut();

        while let Some(fork) = queue_iter.next()
            && remaining_delta > 0.
        {
            remaining_delta = fork
                .iter_mut()
                .map(|fork_element| {
                    match fork_element {
                        ForkElement::Tween(tween) => {
                            match tween.get_state() {
                                TweenState::Playing => tween.advance_time(remaining_delta),
                                TweenState::Paused => {
                                    tween.play();
                                    tween.advance_time(remaining_delta)
                                }
                                TweenState::Stopped => remaining_delta,
                            }
                        }
                        ForkElement::Interval {
                            total_time,
                            elapsed_time,
                        } => {
                            *elapsed_time += remaining_delta;

                            *elapsed_time - *total_time
                        }
                    }
                })
                .fold(remaining_delta, f64::min);
        }

        if remaining_delta > 0. {
            self.handle_finished();
        }

        remaining_delta
    }

    fn complete(&mut self) {
        self.t.queue.drain(..).for_each(|fork| {
            fork.into_iter().for_each(|fork_element| {
                match fork_element {
                    ForkElement::Tween(mut tween) => {
                        tween.complete();
                    }
                    ForkElement::Interval { .. } => {}
                }
            })
        });

        self.t
            .inserts
            .drain(..)
            .for_each(|(_, mut tween)| tween.complete());
        self.handle_finished();
    }
}

impl Default for SpireTween<Sequence> {
    fn default() -> Self { Self::new() }
}

// ----------------------------------------------------------------
// Builder methods

impl SpireTween<Sequence> {
    pub fn new() -> Self {
        Self {
            bound_nodes: Default::default(),
            state: TweenState::Playing,
            delay: 0.,
            speed_scale: 1.,
            elapsed_time: 0.,
            cycle_count: 0,
            pause_mode: SpirePauseMode::Stop,
            process_mode: SpireProcessMode::Idle,
            loop_mode: LoopMode::Finite(0),
            calls_on_finish: Vec::new(),
            t: Sequence {
                queue:   Vec::new(),
                inserts: Vec::new(),
            },
        }
    }

    pub fn append(&mut self, tween: impl Into<AnyTween>) {
        let mut tween = tween.into();

        match self.state {
            | TweenState::Playing | TweenState::Paused => {
                tween.pause();
            }
            TweenState::Stopped => {
                tween.stop();
            }
        }

        self.t
            .queue
            .push(vec![TWEENS.register_shared(tween).into()]);
    }

    pub fn append_call(&mut self, f: impl FnMut() + 'static) {
        let tween = SpireTween::<DelayedCallData>::new(f, 0.0, AutoPlay(true));
        self.t
            .queue
            .push(vec![TWEENS.register_shared(tween.into()).into()]);
    }

    pub fn append_callable(&mut self, callable: Callable) {
        let tween = SpireTween::<DelayedCallData>::new_callable(callable, 0.0, AutoPlay(true));
        self.t
            .queue
            .push(vec![TWEENS.register_shared(tween.into()).into()]);
    }

    pub fn append_shared(&mut self, shared: PtrTween) { self.t.queue.push(vec![shared.into()]); }

    pub fn append_interval(&mut self, time: f64) {
        let element = ForkElement::Interval {
            total_time:   time,
            elapsed_time: 0.,
        };
        self.t.queue.push(vec![element]);
    }

    pub fn join(&mut self, tween: impl Into<AnyTween>) {
        let mut tween = tween.into();

        match self.state {
            | TweenState::Playing | TweenState::Paused => {
                tween.pause();
            }
            TweenState::Stopped => {
                tween.stop();
            }
        }

        let shared = TWEENS.register_shared(tween);

        if let Some(back) = self.t.queue.last_mut() {
            back.push(shared.into());
        } else {
            self.append_shared(shared);
        }
    }

    pub fn join_call(&mut self, f: impl FnMut() + 'static) {
        if let Some(back) = self.t.queue.last_mut() {
            let tween = SpireTween::<DelayedCallData>::new(f, 0.0, AutoPlay(true));
            back.push(TWEENS.register_shared(tween.into()).into());
        } else {
            self.append_call(f);
        }
    }

    pub fn join_callable(&mut self, callable: Callable) {
        if let Some(back) = self.t.queue.last_mut() {
            let tween = SpireTween::<DelayedCallData>::new_callable(callable, 0.0, AutoPlay(true));
            back.push(TWEENS.register_shared(tween.into()).into());
        } else {
            self.append_callable(callable);
        }
    }

    pub fn join_shared(&mut self, shared: PtrTween) {
        if let Some(back) = self.t.queue.last_mut() {
            back.push(shared.into());
        } else {
            self.append_shared(shared);
        }
    }

    pub fn insert(&mut self, time: f64, tween: impl Into<AnyTween>) {
        let mut tween = tween.into();

        match self.state {
            | TweenState::Playing | TweenState::Paused => {
                tween.pause();
            }
            TweenState::Stopped => {
                tween.stop();
            }
        }

        self.t.inserts.push((time, TWEENS.register_shared(tween)));
    }

    pub fn insert_call(&mut self, time: f64, f: impl FnMut() + 'static) {
        let delayed_call = SpireTween::<DelayedCallData>::new(f, 0.0, AutoPlay(true));
        let tween = AnyTween::from(delayed_call);
        self.t.inserts.push((time, TWEENS.register_shared(tween)));
    }

    pub fn insert_callable(&mut self, time: f64, callable: Callable) {
        let delayed_callable =
            SpireTween::<DelayedCallData>::new_callable(callable, 0.0, AutoPlay(true));
        let tween = AnyTween::from(delayed_callable);
        self.t.inserts.push((time, TWEENS.register_shared(tween)));
    }

    pub fn insert_shared(&mut self, time: f64, shared: PtrTween) {
        self.t.inserts.push((time, shared));
    }

    pub fn iter_inner_tweens_non_recursive(&self) -> impl Iterator<Item = PtrTween> {
        self.t
            .queue
            .iter()
            .flat_map(|forks| {
                forks.iter().filter_map(|fork| {
                    if let ForkElement::Tween(tween) = fork {
                        Some(tween.clone())
                    } else {
                        None
                    }
                })
            })
            .chain(self.t.inserts.iter().map(|(_, tween)| tween.clone()))
    }

    pub fn iter_inner_tweens_recursive(&self) -> SequenceIter { SequenceIter::new(self) }
}

pub struct SequenceIter {
    stack: Vec<PtrTween>,
}

impl SequenceIter {
    fn new(seq: &SpireSequence) -> Self {
        let mut this = Self { stack: Vec::new() };
        this.queue_inner_tweens(seq);
        this
    }

    fn queue_inner_tweens(&mut self, seq: &SpireSequence) {
        for forks in &seq.t.queue {
            for fork in forks {
                match fork {
                    ForkElement::Tween(tween) => {
                        self.stack.push(tween.clone());
                    }
                    ForkElement::Interval { .. } => {}
                }
            }
        }

        for (_, tween) in &seq.t.inserts {
            self.stack.push(tween.clone());
        }
    }
}

impl Iterator for SequenceIter {
    type Item = PtrTween;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().inspect(|tween| {
            if let AnyTween::Sequence(seq) = &**tween {
                self.queue_inner_tweens(seq);
            }
        })
    }
}
