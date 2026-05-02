use super::*;

/// Chains and/or parallelizes multiple tweens, callables, and intervals.
///
/// Comparable to Godot's built-in `Tween` — other Spire tween types are closer to
/// individual `Tweener`s. A `Sequence` is itself an [`ITweenable`], so sequences can
/// nest inside other sequences.
///
/// # Blocks
///
/// A sequence is a queue of "blocks", where each block contains one or more items
/// that run in parallel. Below is a sequence with three blocks; each tween is a letter:
///
/// ```text
/// █░Block1░█->█░Block2░█->█░Block3░█
/// █░░░A░░░░█->█░░░D░░░░█->█░░░F░░░░█
/// █░░░B░░░░█->█░░░E░░░░█->█░░░G░░░░█
/// █░░░C░░░░█->█░░░░░░░░█->█░░░H░░░░█
/// ```
///
/// Playback order: `A,B,C` in parallel → `D,E` in parallel → `F,G,H` in parallel.
///
/// A block's duration equals the longest item in the block. Each letter below
/// represents one second of duration:
///
/// ```text
/// █░Block1░█->█░Block2░█->█░Block3░█
/// █░AAA░░░░█->█░D░░░░░░█->█░FFF░░░░█
/// █░BBBBBB░█->█░EEEE░░░█->█░GGG░░░░█
/// █░CCC░░░░█->█░░░░░░░░█->█░HHH░░░░█
/// ```
///
/// Block 1 = 6s (`B`), block 2 = 4s (`E`), block 3 = 3s. Total = 13s.
///
/// Build the queue with [`append`](SpireTween::<Sequence>::append) (new block) and
/// [`join`](SpireTween::<Sequence>::join) (add to current block). The position of an
/// item *within* a block is irrelevant beyond per-block insertion order.
///
/// # Inserts
///
/// [`insert`](SpireTween::<Sequence>::insert) and
/// [`insert_call`](SpireTween::<Sequence>::insert_call) place items at absolute time
/// offsets, independent of the block queue. Inserted items still affect total
/// duration: the sequence won't complete until all inserted items have completed.
///
/// # Loops
///
/// Sequences loop, but only [`LoopMode::Restart`] has effect — other modes are
/// silently ignored. Looping restarts all child tweens.
///
/// # Child tween rules
///
/// - Adding a tween to the sequence unregisters it from the global `TweenManager`;
///   the sequence becomes responsible for ticking it.
/// - Calling `stop()` on a child makes the sequence skip it for the current loop.
/// - Calling `pause()` on a child has no effect — the sequence calls `play()` on the
///   next update.
/// - Adding the same tween to multiple sequences (or twice to the same sequence)
///   results in undefined behavior — but won't crash.
/// - A child that never completes will hang the sequence on that block forever.
/// - A child with its own loops must finish all of them before the sequence advances.
/// - A child's [`ProcessMode`] is ignored; only the sequence's matters.
/// - A child's [`PauseMode`] is honored — a paused child stalls the block.
/// - Effective `speed_scale` of a child = `sequence.speed_scale * child.speed_scale`.
#[derive(Default)]
pub struct Sequence {
    pub queue: Vec<Vec<BlockItem>>,
    pub inserts: Vec<(f64, InsertItem)>,
    pub default_ease: Option<Ease>,
}

/// One slot inside a [`Sequence`] block: a child tween, a one-shot callable, or an
/// interval (delay).
pub enum BlockItem {
    Tween(AnyTween),
    Call { call: Callable, invoked: bool },
    Interval { interval_time: f64, elapsed_time: f64 },
}

/// One slot inserted into a [`Sequence`] at an absolute time offset, running
/// independently of the block queue.
pub enum InsertItem {
    Tween(AnyTween),
    Call { call: Callable, invoked: bool },
}

impl From<AnyTween> for InsertItem {
    fn from(tween: AnyTween) -> Self { Self::Tween(tween) }
}

impl From<Callable> for InsertItem {
    fn from(call: Callable) -> Self { Self::Call { call, invoked: false } }
}

impl From<AnyTween> for BlockItem {
    fn from(tween: AnyTween) -> Self { Self::Tween(tween) }
}

impl From<Callable> for BlockItem {
    fn from(call: Callable) -> Self { Self::Call { call, invoked: false } }
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
        let Some(actual_step) = self.handle_time_step(delta_time) else { return AdvanceTimeResult::Playing };

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
            block.iter_mut().for_each(|item| match item {
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
            })
        });

        self.t.inserts.iter_mut().for_each(|(_, item)| match item {
            InsertItem::Tween(tween) => tween.force_complete(),
            InsertItem::Call { call, invoked } => {
                if !*invoked && call.is_valid() {
                    call.call(&[]);
                    *invoked = true;
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
    /// Constructs an empty sequence. Equivalent to `SpireTween::<Sequence>::new_with_data(Sequence::default())`.
    pub fn new() -> Self { Self::new_with_data(Sequence::default()) }

    /// When adding child tweens to this sequence, override their easing if (and only if)
    /// the child's easing is currently [`Ease::Default`].
    ///
    /// This takes priority over the global default set via `SpireGlobalSettings::set_default_ease`.
    /// Only affects tweens added *after* this call. Eligible child types: tweens whose `T` is
    /// [`LerpPropertyData<U>`], [`LerpMethodData<U>`], or [`Sequence`].
    pub fn set_default_ease(&mut self, ease: Ease) { self.t.default_ease = Some(ease); }

    /// Appends a tween in a new block at the end of the queue.
    ///
    /// Given a sequence with two blocks:
    /// ```text
    /// █░Block1░█->█░Block2░█
    /// █░░░A░░░░█->█░░░D░░░░█
    /// █░░░B░░░░█->█░░░E░░░░█
    /// █░░░C░░░░█->█░░░░░░░░█
    /// ```
    /// `append(F)` produces:
    /// ```text
    /// █░Block1░█->█░Block2░█->█░Block3░█
    /// █░░░A░░░░█->█░░░D░░░░█->█░░░F░░░░█
    /// █░░░B░░░░█->█░░░E░░░░█->█░░░░░░░░█
    /// █░░░C░░░░█->█░░░░░░░░█->█░░░░░░░░█
    /// ```
    pub fn append<T: ITweenable>(&mut self, tween: SpireTween<T>)
    where AnyTween: From<RcPtr<SpireTween<T>>> {
        let tween = AnyTween::from(RcPtr::new(tween));
        self.append_ptr(tween);
    }

    /// Like [`append`](Self::append) but accepts an already-erased [`AnyTween`] (or
    /// anything `Into<AnyTween>`, e.g. `RcPtr<SpireTween<T>>`). Use when you've kept
    /// a handle to a previously-built tween.
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

    /// Appends a one-shot [`Callable`] in a new block. Equivalent to a zero-duration
    /// tween that just invokes a function once.
    pub fn append_call(&mut self, call: Callable) { self.t.queue.push(vec![call.into()]); }

    /// Appends an interval (delay) in a new block. Equivalent to a tween with
    /// `delay = time` that completes immediately afterward.
    pub fn append_interval(&mut self, time: f64) {
        let item = BlockItem::Interval {
            interval_time: time,
            elapsed_time:  0.,
        };
        self.t.queue.push(vec![item]);
    }

    /// Adds a tween to the **last** block, running it in parallel with the block's
    /// existing items. Equivalent to Godot's `Tween.parallel()`, but in one call.
    ///
    /// If the sequence is empty, behaves like [`append`](Self::append).
    ///
    /// Given:
    /// ```text
    /// █░Block1░█->█░Block2░█
    /// █░░░A░░░░█->█░░░D░░░░█
    /// █░░░B░░░░█->█░░░E░░░░█
    /// █░░░C░░░░█->█░░░░░░░░█
    /// ```
    /// `join(F)` produces:
    /// ```text
    /// █░Block1░█->█░Block2░█
    /// █░░░A░░░░█->█░░░D░░░░█
    /// █░░░B░░░░█->█░░░E░░░░█
    /// █░░░C░░░░█->█░░░F░░░░█
    /// ```
    pub fn join<T: ITweenable>(&mut self, tween: SpireTween<T>)
    where AnyTween: From<RcPtr<SpireTween<T>>> {
        let tween = AnyTween::from(RcPtr::new(tween));
        self.join_ptr(tween);
    }

    /// Like [`join`](Self::join) but accepts an already-erased [`AnyTween`].
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

    /// Adds a one-shot [`Callable`] to the last block (parallel). Falls back to
    /// [`append_call`](Self::append_call) if the sequence is empty.
    pub fn join_call(&mut self, call: Callable) {
        if let Some(last_block) = self.t.queue.last_mut() {
            last_block.push(call.into());
        } else {
            self.append_call(call);
        }
    }

    /// Adds an interval to the last block. Since block duration is determined by the
    /// longest item, this only changes the block's length if `time` exceeds every
    /// other item in the block. Falls back to [`append_interval`](Self::append_interval)
    /// if the sequence is empty.
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

    /// Inserts a tween at an absolute time offset from the start of the sequence,
    /// independent of the block queue.
    ///
    /// Inserted items still affect total duration — the sequence won't complete until
    /// all inserted tweens have completed.
    ///
    /// **Note:** If the sequence has a non-zero delay, the delay is *added* to the time
    /// offset. Example: a sequence with `delay = 2.0` and `insert(3.0, t)` will start
    /// `t` at the 5.0s mark from when the sequence begins playing.
    pub fn insert<T: ITweenable>(&mut self, time: f64, tween: SpireTween<T>)
    where AnyTween: From<RcPtr<SpireTween<T>>> {
        let tween = AnyTween::from(RcPtr::new(tween));
        self.insert_ptr(time, tween);
    }

    /// Like [`insert`](Self::insert) but accepts an already-erased [`AnyTween`].
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

    /// Inserts a one-shot [`Callable`] at an absolute time offset, independent of the
    /// block queue. The sequence's delay is added to the offset (see [`insert`](Self::insert)).
    pub fn insert_call(&mut self, time: f64, call: Callable) { self.t.inserts.push((time, call.into())); }

    /// Iterator over child tweens in the immediate queue and inserts (does not descend
    /// into nested sequences). Skips `BlockItem::Call` and `BlockItem::Interval`.
    pub fn iter_inner_tweens_non_recursive(&self) -> impl Iterator<Item = AnyTween> {
        self.t
            .queue
            .iter()
            .flat_map(|blocks| {
                blocks.iter().filter_map(
                    |item| {
                        if let BlockItem::Tween(tween) = item { Some(tween.clone()) } else { None }
                    },
                )
            })
            .chain(
                self.t.inserts.iter().filter_map(|(_, item)| {
                    if let InsertItem::Tween(tween) = item { Some(tween.clone()) } else { None }
                }),
            )
    }

    /// Iterator that recursively descends into child sequences.
    pub fn iter_inner_tweens_recursive(&self) -> SequenceIter { SequenceIter::new(self) }

    /// Searches the queue and inserts for `to_remove`, removes it if found.
    /// Returns `true` if the tween was found and removed.
    ///
    /// Does **not** descend into nested sequences — call `remove` on the exact
    /// [`Sequence`] that contains the target.
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

    /// Searches the queue and inserts for a [`Callable`] equal to `to_remove`, removes
    /// it if found. Returns `true` if removed.
    ///
    /// Does **not** descend into nested sequences. Note that [`Callable`] equality
    /// uses reference counting, so you must pass the same handle that was inserted.
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
        let Some(ease) = self.t.default_ease else { return };

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
            .for_each(|item| match item {
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
            });

        self.t.inserts.iter_mut().for_each(|(_, item)| match item {
            InsertItem::Tween(tween) => {
                tween.stop();
                tween.play();
            }
            InsertItem::Call { call: _, invoked } => {
                *invoked = false;
            }
        });
    }

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
