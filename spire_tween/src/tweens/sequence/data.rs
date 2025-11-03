use super::*;

#[derive(Default)]
pub struct Sequence {
    pub queue: Vec<Vec<BlockItem>>,
    pub inserts: Vec<(f64, InsertItem)>,
    pub default_ease: Option<Ease>,
}

pub enum BlockItem {
    Tween(AnyTween),
    Call {
        call: Callable,
        invoked: bool,
    },
    Interval {
        interval_time: f64,
        elapsed_time:  f64,
    },
}

pub enum InsertItem {
    Tween(AnyTween),
    Call { call: Callable, invoked: bool },
}

impl From<AnyTween> for InsertItem {
    fn from(tween: AnyTween) -> Self { Self::Tween(tween) }
}

impl From<Callable> for InsertItem {
    fn from(call: Callable) -> Self {
        Self::Call {
            call,
            invoked: false,
        }
    }
}

impl From<AnyTween> for BlockItem {
    fn from(tween: AnyTween) -> Self { Self::Tween(tween) }
}

impl From<Callable> for BlockItem {
    fn from(call: Callable) -> Self {
        Self::Call {
            call,
            invoked: false,
        }
    }
}

impl SpireTweener for SpireTween<Sequence> {
    #[inline]
    fn play(&mut self) {
        if self.is_stopped() {
            self.reset_counters();
            self.restart_inner_tweens();
        }

        self.state = State::Playing;
    }

    #[inline]
    fn pause(&mut self) {
        if self.state != State::Stopped {
            self.state = State::Paused;
        }
    }

    #[inline]
    fn stop(&mut self) { self.state = State::Stopped; }

    fn process(&mut self, delta_time: f64, is_tree_paused: bool) -> AdvanceTimeResult {
        let Some(actual_step) = self.handle_time_step(delta_time)
        else { return AdvanceTimeResult::Playing };

        self.t.inserts.retain_mut(|(ins_time, item)| {
            let past_insertion = self.loop_time - *ins_time;

            match item {
                InsertItem::Tween(tween) => {
                    match tween.get_state() {
                        State::Playing | State::Paused => {
                            if past_insertion < 0. {
                                return true;
                            }

                            // Harmless if already playing.
                            tween.play();

                            match tween.process(past_insertion, is_tree_paused) {
                                | AdvanceTimeResult::Playing
                                | AdvanceTimeResult::Paused
                                // Even if completed, preserve for future reuse.
                                | AdvanceTimeResult::Completed { .. } => true,
                                // Tween explicitly requested deletion
                                AdvanceTimeResult::ShouldDespawn => false,
                            }
                        }
                        State::Stopped => true,
                    }
                }
                InsertItem::Call { call, invoked } => {
                    if !*invoked && past_insertion >= 0. {
                        if !call.is_valid() {
                            return false;
                        }

                        call.call(&[]);
                        *invoked = true;
                    }

                    true
                }
            }
        });

        let mut remaining_step = actual_step;

        for block in self.t.queue.iter_mut() {
            let mut next_step = remaining_step;

            block.retain_mut(|item| {
                match item {
                    BlockItem::Tween(tween) => {
                        match tween.get_state() {
                            State::Playing => {}
                            State::Paused => tween.play(),
                            State::Stopped => return true,
                        }

                        match tween.process(remaining_step, is_tree_paused) {
                            | AdvanceTimeResult::Playing | AdvanceTimeResult::Paused => {
                                next_step = f64::min(next_step, 0.);
                                true
                            }
                            AdvanceTimeResult::Completed { excess_time } => {
                                next_step = f64::min(next_step, excess_time);
                                true
                            }
                            AdvanceTimeResult::ShouldDespawn => {
                                // Don't mess with next_step, let the sequence keep going.
                                false
                            }
                        }
                    }
                    BlockItem::Interval {
                        interval_time,
                        elapsed_time,
                    } => {
                        *elapsed_time += remaining_step;
                        next_step = f64::min(next_step, *elapsed_time - *interval_time);
                        true
                    }
                    BlockItem::Call { call, invoked } => {
                        if !*invoked {
                            if !call.is_valid() {
                                return false;
                            }

                            call.call(&[]);
                            *invoked = true;
                        }

                        true
                    }
                }
            });

            remaining_step = next_step;

            if remaining_step <= 0. {
                return AdvanceTimeResult::Playing;
            }
        }

        if let Some(excess_time) = self.handle_loop_finished(remaining_step) {
            AdvanceTimeResult::Completed { excess_time }
        } else {
            // If we reached here, this loop finished, all inner tweens are done,
            // so restart them before going to the next loop.

            self.restart_inner_tweens();
            AdvanceTimeResult::Playing
        }
    }

    fn force_complete(&mut self) {
        self.t.queue.iter_mut().for_each(|block| {
            block.iter_mut().for_each(|item| {
                match item {
                    BlockItem::Tween(tween) => {
                        tween.force_complete();
                    }
                    BlockItem::Interval {
                        interval_time,
                        elapsed_time,
                    } => {
                        *elapsed_time = *interval_time;
                    }
                    BlockItem::Call { call, invoked } => {
                        if !*invoked && call.is_valid() {
                            call.call(&[]);
                            *invoked = true;
                        }
                    }
                }
            })
        });

        self.t.inserts.iter_mut().for_each(|(_, item)| {
            match item {
                InsertItem::Tween(tween) => tween.force_complete(),
                InsertItem::Call { call, invoked } => {
                    if !*invoked && call.is_valid() {
                        call.call(&[]);
                        *invoked = true;
                    }
                }
            }
        });
        self.handle_finished();
    }
}

impl Default for SpireTween<Sequence> {
    fn default() -> Self { Self::new() }
}

impl SpireTween<Sequence> {
    pub fn new() -> Self { Self::new_with_data(Sequence::default()) }

    pub fn set_default_ease(&mut self, ease: Ease) { self.t.default_ease = Some(ease); }

    pub fn append<T: ITweenable>(&mut self, tween: SpireTween<T>)
    where AnyTween: From<RcPtr<SpireTween<T>>> {
        let tween = AnyTween::from(RcPtr::new(tween));
        self.append_ptr(tween);
    }

    pub fn append_ptr(&mut self, tween: impl Into<AnyTween>) {
        let mut tween = tween.into();

        if self.is_already_inside(&tween) {
            return;
        }

        match self.state {
            State::Playing | State::Paused => tween.pause(),
            State::Stopped => tween.stop(),
        }

        self.check_ease(&mut tween);
        TM.tween_notify_sequence_child(&tween);

        self.t.queue.push(vec![tween.into()]);
    }

    pub fn append_call(&mut self, call: Callable) { self.t.queue.push(vec![call.into()]); }

    pub fn append_interval(&mut self, time: f64) {
        let item = BlockItem::Interval {
            interval_time: time,
            elapsed_time:  0.,
        };
        self.t.queue.push(vec![item]);
    }

    pub fn join<T: ITweenable>(&mut self, tween: SpireTween<T>)
    where AnyTween: From<RcPtr<SpireTween<T>>> {
        let tween = AnyTween::from(RcPtr::new(tween));
        self.join_ptr(tween);
    }

    pub fn join_ptr(&mut self, tween: impl Into<AnyTween>) {
        let mut tween = tween.into();

        if self.is_already_inside(&tween) {
            return;
        }

        match self.state {
            State::Playing | State::Paused => tween.pause(),
            State::Stopped => tween.stop(),
        }

        self.check_ease(&mut tween);
        TM.tween_notify_sequence_child(&tween);

        if let Some(last_block) = self.t.queue.last_mut() {
            last_block.push(tween.into());
        } else {
            self.append_ptr(tween);
        }
    }

    pub fn join_call(&mut self, call: Callable) {
        if let Some(last_block) = self.t.queue.last_mut() {
            last_block.push(call.into());
        } else {
            self.append_call(call);
        }
    }

    pub fn join_interval(&mut self, time: f64) {
        if let Some(last_block) = self.t.queue.last_mut() {
            last_block.push(BlockItem::Interval {
                interval_time: time,
                elapsed_time:  0.,
            });
        } else {
            self.append_interval(time);
        }
    }

    pub fn insert<T: ITweenable>(&mut self, time: f64, tween: SpireTween<T>)
    where AnyTween: From<RcPtr<SpireTween<T>>> {
        let tween = AnyTween::from(RcPtr::new(tween));
        self.insert_ptr(time, tween);
    }

    pub fn insert_ptr(&mut self, time: f64, tween: impl Into<AnyTween>) {
        let mut tween = tween.into();

        if self.is_already_inside(&tween) {
            return;
        }

        match self.state {
            State::Playing | State::Paused => tween.pause(),
            State::Stopped => tween.stop(),
        }

        self.check_ease(&mut tween);
        TM.tween_notify_sequence_child(&tween);

        self.t.inserts.push((time, tween.into()));
    }

    pub fn insert_call(&mut self, time: f64, call: Callable) {
        self.t.inserts.push((time, call.into()));
    }

    pub fn iter_inner_tweens_non_recursive(&self) -> impl Iterator<Item = AnyTween> {
        self.t
            .queue
            .iter()
            .flat_map(|blocks| {
                blocks.iter().filter_map(|item| {
                    if let BlockItem::Tween(tween) = item {
                        Some(tween.clone())
                    } else {
                        None
                    }
                })
            })
            .chain(self.t.inserts.iter().filter_map(|(_, item)| {
                if let InsertItem::Tween(tween) = item {
                    Some(tween.clone())
                } else {
                    None
                }
            }))
    }

    pub fn iter_inner_tweens_recursive(&self) -> SequenceIter { SequenceIter::new(self) }

    pub fn remove(&mut self, to_remove: &impl Address) -> bool {
        let address = to_remove.address();
        let mut found = false;
        self.t.queue.retain_mut(|block| {
            block.retain_mut(|item| {
                if let BlockItem::Tween(tween) = item
                    && addr_eq(tween.address(), address)
                {
                    found = true;
                    false
                } else {
                    true
                }
            });

            !block.is_empty()
        });

        if found {
            return true;
        }

        self.t.inserts.retain_mut(|(_, item)| {
            if let InsertItem::Tween(tween) = item
                && addr_eq(tween.address(), address)
            {
                found = true;
                false
            } else {
                true
            }
        });

        found
    }

    pub fn remove_call(&mut self, to_remove: &Callable) -> bool {
        let mut found = false;
        self.t.queue.retain_mut(|block| {
            block.retain_mut(|item| {
                if let BlockItem::Call { call, .. } = item
                    && call == to_remove
                {
                    found = true;
                    false
                } else {
                    true
                }
            });

            !block.is_empty()
        });

        if found {
            return true;
        }

        self.t.inserts.retain_mut(|(_, item)| {
            if let InsertItem::Call { call, .. } = item
                && call == to_remove
            {
                found = true;
                false
            } else {
                true
            }
        });

        found
    }

    fn check_ease(&mut self, tween: &mut AnyTween) {
        let Some(ease) = self.t.default_ease
        else { return };

        match tween {
            AnyTween::Property(tween) => {
                if let EaseKind::Basic(Ease::Default) = tween.get_ease() {
                    tween.set_ease(EaseKind::Basic(ease));
                }
            }
            AnyTween::Method(tween) => {
                if let EaseKind::Basic(Ease::Default) = tween.get_ease() {
                    tween.set_ease(EaseKind::Basic(ease));
                }
            }
            AnyTween::DelayedCall(_) => {}
            AnyTween::Sequence(tween) => {
                if tween.t.default_ease.is_none_or(|e| e == Ease::Default) {
                    tween.t.default_ease = Some(ease);
                }
            }
        }
    }

    fn restart_inner_tweens(&mut self) {
        self.t
            .queue
            .iter_mut()
            .flat_map(|vec| vec.iter_mut())
            .for_each(|item| {
                match item {
                    BlockItem::Tween(tween) => {
                        tween.stop();
                        tween.play();
                    }
                    BlockItem::Interval {
                        interval_time: _,
                        elapsed_time,
                    } => {
                        *elapsed_time = 0.;
                    }
                    BlockItem::Call { call: _, invoked } => {
                        *invoked = false;
                    }
                }
            });

        self.t.inserts.iter_mut().for_each(|(_, item)| {
            match item {
                InsertItem::Tween(tween) => {
                    tween.stop();
                    tween.play();
                }
                InsertItem::Call { call: _, invoked } => {
                    *invoked = false;
                }
            }
        });
    }

    /// Returns true if the given tween is already inside this sequence.
    fn is_already_inside(&self, tween: &AnyTween) -> bool {
        let addr = tween.address();
        let self_address = (&raw const *self) as *const ();
        if std::ptr::eq(addr, self_address) {
            godot_error!("Cannot append a sequence tween to itself.");
            return true;
        }

        for item in self.t.queue.iter().flatten() {
            if let BlockItem::Tween(inner) = item
                && std::ptr::eq(inner.address(), addr)
            {
                godot_error!("Cannot add the same tween to the sequence more than once.");
                return true;
            }
        }

        for (_, item) in self.t.inserts.iter() {
            if let InsertItem::Tween(inner) = item
                && std::ptr::eq(inner.address(), addr)
            {
                godot_error!("Cannot add the same tween to the sequence more than once.");
                return true;
            }
        }

        false
    }
}

pub struct SequenceIter {
    stack: Vec<AnyTween>,
}

impl SequenceIter {
    fn new(seq: &SpireTween<Sequence>) -> Self {
        let mut this = Self { stack: Vec::new() };
        this.queue_inner_tweens(seq);
        this
    }

    fn queue_inner_tweens(&mut self, seq: &SpireTween<Sequence>) {
        for blocks in &seq.t.queue {
            for item in blocks {
                match item {
                    BlockItem::Tween(tween) => self.stack.push(AnyTween::clone(tween)),
                    BlockItem::Interval { .. } => {}
                    BlockItem::Call { .. } => {}
                }
            }
        }

        for (_, item) in &seq.t.inserts {
            match item {
                InsertItem::Tween(tween) => self.stack.push(tween.clone()),
                InsertItem::Call { .. } => {}
            }
        }
    }
}

impl Iterator for SequenceIter {
    type Item = AnyTween;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().inspect(|tween| {
            if let AnyTween::Sequence(seq) = tween {
                self.queue_inner_tweens(seq);
            }
        })
    }
}
