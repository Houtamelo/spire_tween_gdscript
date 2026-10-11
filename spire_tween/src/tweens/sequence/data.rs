use smolset::SmolSet;

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
/// - Adding the same immediate child twice is rejected. Sequence children that
///   would create a cycle, or contain repeated tweens, are rejected with a warning
///   before changing child playback, ease, registration, or bindings.
/// - Reusing a tween across multiple sequences is unsupported.
/// - A child that never completes will hang the sequence on that block forever.
/// - A child with its own loops must finish all of them before the sequence advances.
/// - A child's [`ProcessMode`] is ignored; only the sequence's matters.
/// - A child's [`PauseMode`] is honored — a paused child stalls the block.
/// - Effective `speed_scale` of a child = `sequence.speed_scale * child.speed_scale`.
#[derive(Default)]
pub struct Sequence {
    /// Manual mutation bypasses insertion checks and child playback, registration,
    /// and binding setup. It can create ownership cycles, leaks, or infinite
    /// recursion that crashes the engine. Callers must preserve the invariants.
    #[doc(hidden)]
    pub queue: Vec<Vec<BlockItem>>,
    /// Manual mutation bypasses insertion checks and child playback, registration,
    /// and binding setup. It can create ownership cycles, leaks, or infinite
    /// recursion that crashes the engine. Callers must preserve the invariants.
    #[doc(hidden)]
    pub inserts: Vec<(f64, InsertItem)>,
    pub default_ease: Option<Ease>,
}

pub enum TweenCtor {
    RustFn(Box<dyn FnOnce() -> AnyTween>),
    Callable(Callable),
    Empty,
}

/// One slot inside a [`Sequence`] block: a child tween, a one-shot callable, or an
/// interval (delay).
pub enum BlockItem {
    Tween(AnyTween),
    Ctor(TweenCtor),
    RustFn { f: Box<dyn FnMut()>, invoked: bool },
    Callable { callable: Callable, invoked: bool },
    Interval { interval_time: f64, elapsed_time: f64 },
}

/// One slot inserted into a [`Sequence`] at an absolute time offset, running
/// independently of the block queue.
pub enum InsertItem {
    Ctor(TweenCtor),
    Tween(AnyTween),
    RustFn { f: Box<dyn FnMut()>, invoked: bool },
    Callable { callable: Callable, invoked: bool },
}

impl From<AnyTween> for InsertItem {
    fn from(tween: AnyTween) -> Self { Self::Tween(tween) }
}

impl From<Callable> for InsertItem {
    fn from(call: Callable) -> Self {
        Self::Callable {
            callable: call,
            invoked:  false,
        }
    }
}

impl From<AnyTween> for BlockItem {
    fn from(tween: AnyTween) -> Self { Self::Tween(tween) }
}

impl From<Callable> for BlockItem {
    fn from(call: Callable) -> Self {
        Self::Callable {
            callable: call,
            invoked:  false,
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
        let Some(actual_step) = self.handle_time_step(delta_time) else { return AdvanceTimeResult::Playing };

        let mut shared_excess_inserts = actual_step;
        self.process_inserts(actual_step, &mut shared_excess_inserts, is_tree_paused);

        let mut shared_excess_queue = actual_step;
        for queue_idx in 0..self.t.queue.len() {
            self.process_block(queue_idx, shared_excess_queue, &mut shared_excess_queue, is_tree_paused);

            if shared_excess_queue <= 0. {
                return AdvanceTimeResult::Playing;
            }
        }

        if shared_excess_inserts <= 0.0 {
            return AdvanceTimeResult::Playing;
        }

        let shared_excess = f64::min(shared_excess_queue, shared_excess_inserts);

        if let Some(excess_time) = self.handle_loop_finished(shared_excess) {
            AdvanceTimeResult::Completed { excess_time }
        } else {
            // If we reached here, this loop finished, all inner tweens are done,
            // so restart them before going to the next loop.

            self.restart_inner_tweens();
            AdvanceTimeResult::Playing
        }
    }

    fn force_complete(&mut self) {
        self.force_complete_inserts();

        for queue_idx in 0..self.t.queue.len() {
            self.force_complete_block(queue_idx);
        }

        self.handle_finished();
    }
}

impl SpireTween<Sequence> {
    fn process_inserts(&mut self, step: f64, shared_excess: &mut f64, is_tree_paused: bool) {
        let mut pending_deletion = SmallVec::<[usize; 16]>::new();

        for idx in 0..self.t.inserts.len() {
            let (ins_time, item) = &mut self.t.inserts[idx];

            let ins_time = *ins_time;
            let past_insertion = self.loop_time - ins_time;

            let actual_step = f64::min(past_insertion, step);

            match item {
                InsertItem::Ctor(ctor) => {
                    if past_insertion < 0. {
                        *shared_excess = 0.0;
                        continue;
                    }

                    if let Some(mut tween) = run_tween_ctor(ctor)
                        && self.sanitize_tween_before_inclusion(&mut tween)
                        && process_tween(&mut tween, actual_step, shared_excess, is_tree_paused)
                    {
                        if !tween.is_stopped() {
                            *shared_excess = 0.0;
                        }
                        self.t.inserts[idx] = (ins_time, InsertItem::Tween(tween));
                    } else {
                        pending_deletion.push(idx);
                    }
                }
                InsertItem::Tween(tween) => {
                    if past_insertion < 0. {
                        if !tween.is_stopped() {
                            *shared_excess = 0.0;
                        }

                        continue;
                    }

                    if process_tween(tween, actual_step, shared_excess, is_tree_paused) {
                        if !tween.is_stopped() {
                            *shared_excess = 0.0;
                        }
                    } else {
                        pending_deletion.push(idx);
                    }
                }
                InsertItem::RustFn { f, invoked } => {
                    if past_insertion < 0. {
                        *shared_excess = 0.0;
                        continue;
                    }

                    if !*invoked {
                        f();
                        *invoked = true;
                        *shared_excess = f64::min(*shared_excess, past_insertion);
                    }
                }
                InsertItem::Callable { callable, invoked } => {
                    if past_insertion < 0. {
                        *shared_excess = 0.0;
                        continue;
                    }

                    if *invoked {
                        continue;
                    }

                    if callable.is_valid() {
                        callable.call(&[]);
                        *invoked = true;
                        *shared_excess = f64::min(*shared_excess, past_insertion);
                    } else {
                        pending_deletion.push(idx);
                    }
                }
            }
        }

        // rev() avoids index shifting the issues.
        for &idx in pending_deletion.iter().rev() {
            self.t.inserts.remove(idx);
        }
    }

    fn process_block(&mut self, queue_idx: usize, step: f64, shared_excess: &mut f64, is_tree_paused: bool) {
        let mut block = &mut self.t.queue[queue_idx];
        let mut pending_deletion = SmallVec::<[usize; 16]>::new();

        for block_idx in 0..block.len() {
            match &mut block[block_idx] {
                BlockItem::Ctor(ctor) => {
                    if let Some(mut tween) = run_tween_ctor(ctor)
                        && self.sanitize_tween_before_inclusion(&mut tween)
                        && process_tween(&mut tween, step, shared_excess, is_tree_paused)
                    {
                        block = &mut self.t.queue[queue_idx];
                        block[block_idx] = BlockItem::Tween(tween);
                    } else {
                        block = &mut self.t.queue[queue_idx];
                        pending_deletion.push(block_idx);
                    }
                }
                BlockItem::Tween(tween) => {
                    if !process_tween(tween, step, shared_excess, is_tree_paused) {
                        pending_deletion.push(block_idx);
                    }
                }
                BlockItem::Interval {
                    interval_time,
                    elapsed_time,
                } => {
                    *elapsed_time += step;
                    *shared_excess = f64::min(*shared_excess, *elapsed_time - *interval_time);
                }
                BlockItem::RustFn { f, invoked } => {
                    if !*invoked {
                        f();
                        *invoked = true;
                    }
                }
                BlockItem::Callable { callable, invoked } => {
                    if *invoked {
                        continue;
                    }

                    if callable.is_valid() {
                        callable.call(&[]);
                        *invoked = true;
                    } else {
                        pending_deletion.push(block_idx);
                    }
                }
            }
        }

        // rev() avoids index shifting the issues.
        for &block_idx in pending_deletion.iter().rev() {
            block.remove(block_idx);
        }
    }

    fn force_complete_inserts(&mut self) {
        let mut pending_deletion = SmallVec::<[usize; 16]>::new();

        for idx in 0..self.t.inserts.len() {
            let (ins_time, item) = &mut self.t.inserts[idx];
            let ins_time = *ins_time;

            match item {
                InsertItem::Ctor(ctor) => {
                    if let Some(mut tween) = run_tween_ctor(ctor)
                        && self.sanitize_tween_before_inclusion(&mut tween)
                    {
                        tween.force_complete();
                        self.t.inserts[idx] = (ins_time, InsertItem::Tween(tween));
                    } else {
                        pending_deletion.push(idx);
                    }
                }
                InsertItem::Tween(tween) => {
                    tween.force_complete();
                }
                InsertItem::RustFn { f, invoked } => {
                    if !*invoked {
                        f();
                        *invoked = true;
                    }
                }
                InsertItem::Callable { callable, invoked } => {
                    if !*invoked && callable.is_valid() {
                        callable.call(&[]);
                        *invoked = true;
                    }
                }
            }
        }

        // rev() avoids index shifting the issues.
        for &idx in pending_deletion.iter().rev() {
            self.t.inserts.remove(idx);
        }
    }

    fn force_complete_block(&mut self, queue_idx: usize) {
        let mut block = &mut self.t.queue[queue_idx];
        let mut pending_deletion = SmallVec::<[usize; 16]>::new();

        for block_idx in 0..block.len() {
            match &mut block[block_idx] {
                BlockItem::Ctor(ctor) => {
                    if let Some(mut tween) = run_tween_ctor(ctor)
                        && self.sanitize_tween_before_inclusion(&mut tween)
                    {
                        tween.force_complete();
                        block = &mut self.t.queue[queue_idx];
                        block[block_idx] = BlockItem::Tween(tween);
                    } else {
                        block = &mut self.t.queue[queue_idx];
                        pending_deletion.push(block_idx);
                    }
                }
                BlockItem::Tween(tween) => {
                    tween.force_complete();
                }
                BlockItem::Interval {
                    interval_time,
                    elapsed_time,
                } => {
                    *elapsed_time = *interval_time;
                }
                BlockItem::RustFn { f, invoked } => {
                    if !*invoked {
                        f();
                        *invoked = true;
                    }
                }
                BlockItem::Callable { callable, invoked } => {
                    if !callable.is_valid() {
                        pending_deletion.push(block_idx);
                    } else if !*invoked {
                        callable.call(&[]);
                        *invoked = true;
                    }
                }
            }
        }

        // rev() avoids index shifting the issues.
        for &block_idx in pending_deletion.iter().rev() {
            block.remove(block_idx);
        }
    }
}

fn process_tween(tween: &mut AnyTween, step: f64, shared_excess: &mut f64, is_tree_paused: bool) -> bool {
    match tween.get_state() {
        State::Playing => {}
        State::Paused => tween.play(),
        State::Stopped => return true,
    }

    match tween.process(step, is_tree_paused) {
        | AdvanceTimeResult::Playing | AdvanceTimeResult::Paused => {
            *shared_excess = f64::min(*shared_excess, 0.);
            true
        }
        AdvanceTimeResult::Completed { excess_time } => {
            *shared_excess = f64::min(*shared_excess, excess_time);
            true
        }
        AdvanceTimeResult::ShouldDespawn => {
            // Don't mess with shared_excess, let the sequence keep going.
            false
        }
    }
}

fn run_tween_ctor(ctor: &mut TweenCtor) -> Option<AnyTween> {
    let ctor = std::mem::replace(ctor, TweenCtor::Empty);

    match ctor {
        TweenCtor::RustFn(f) => Some(f()),
        TweenCtor::Callable(callable) => match callable.call(&[]).try_to::<Gd<RefCounted>>() {
            Ok(handle) => match tween_from_gd_handle(handle) {
                Ok(tween) => Some(tween),
                Err(err) => {
                    godot_error!("Expected Tween, got: {}", err.get_class());
                    None
                }
            },
            Err(err) => {
                godot_error!("Expected TweenCtor result to be a Tween. Error: {err:?}");
                None
            }
        },
        TweenCtor::Empty => None,
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
    where AnyTween: From<TweenPtr<T>> {
        let tween = AnyTween::from(TweenPtr::new(tween));
        self.append_ptr(tween);
    }

    /// Like [`append`](Self::append) but accepts an already-erased [`AnyTween`] (or
    /// anything `Into<AnyTween>`, e.g. `TweenPtr<T>`). Use when you've kept
    /// a handle to a previously-built tween.
    pub fn append_ptr(&mut self, tween: impl Into<AnyTween>) {
        let mut tween = tween.into();
        if self.sanitize_tween_before_inclusion(&mut tween) {
            let item = BlockItem::Tween(tween);
            self.append_item(item);
        }
    }

    /// Appends a [`Callable`] in a new block. Equivalent to a zero-duration
    /// tween that just invokes a function once.
    ///
    /// We guarantee that the Callable is only ever invoked in the main thread.
    pub fn append_call(&mut self, callable: Callable) {
        let item = BlockItem::Callable {
            callable,
            invoked: false,
        };
        self.append_item(item);
    }

    /// Same as [`append_call`], but accepts any Rust closure.
    pub fn append_fn(&mut self, f: impl FnMut() + 'static) {
        let f = Box::new(f);
        let item = BlockItem::RustFn { f, invoked: false };
        self.append_item(item);
    }

    pub fn append_ctor(&mut self, f: impl FnOnce() -> AnyTween + 'static) {
        let item = BlockItem::Ctor(TweenCtor::RustFn(Box::new(f)));
        self.append_item(item);
    }

    pub fn append_ctor_callable(&mut self, callable: Callable) {
        let item = BlockItem::Ctor(TweenCtor::Callable(callable));
        self.append_item(item);
    }

    /// Appends an interval (delay) in a new block. Equivalent to a tween with
    /// `delay = time` that completes immediately afterward.
    pub fn append_interval(&mut self, time: f64) {
        let item = BlockItem::Interval {
            interval_time: time,
            elapsed_time:  0.,
        };
        self.append_item(item);
    }

    fn append_item(&mut self, item: BlockItem) { self.t.queue.push(vec![item]); }

    /// Closes the current block in the queue, if the current block is non-empty.
    ///
    /// Calling this method will finalize the current block by pushing an empty
    /// block onto the queue. This prevents modification to the current block
    /// and prepares a new block for future use.
    ///
    /// # Behavior
    /// - If the current block is non-empty, it will be finalized, and an empty
    ///   block is added to the queue.
    /// - If the current block is empty or no block exists, calling this method
    ///   has no effect. It is safe to call this method multiple times consecutively.
    pub fn close_current_block(&mut self) {
        // Closing only mutates when an non-empty block already exists.
        // This means that it is harmless to call `close_current_block` multiple times.
        if let Some(last_block) = self.t.queue.last_mut()
            && !last_block.is_empty()
        {
            self.t.queue.push(vec![]);
        }
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
    where AnyTween: From<TweenPtr<T>> {
        let tween = AnyTween::from(TweenPtr::new(tween));
        self.join_ptr(tween);
    }

    /// Like [`join`](Self::join) but accepts an already-erased [`AnyTween`].
    pub fn join_ptr(&mut self, tween: impl Into<AnyTween>) {
        let mut tween = tween.into();

        if self.sanitize_tween_before_inclusion(&mut tween) {
            let item = BlockItem::Tween(tween);
            self.join_item(item);
        }
    }

    /// Adds a [`Callable`] to the last block (parallel). Falls back to
    /// [`append_call`](Self::append_call) if the sequence is empty.
    pub fn join_call(&mut self, callable: Callable) {
        let item = BlockItem::Callable {
            callable,
            invoked: false,
        };
        self.join_item(item);
    }

    /// Same as [`join_call`], but accepts any Rust closure.
    pub fn join_fn(&mut self, f: impl FnMut() + 'static) {
        let f = Box::new(f);
        let item = BlockItem::RustFn { f, invoked: false };
        self.join_item(item);
    }

    /// Adds an interval to the last block. Since block duration is determined by the
    /// longest item, this only changes the block's length if `time` exceeds every
    /// other item in the block. Falls back to [`append_interval`](Self::append_interval)
    /// if the sequence is empty.
    pub fn join_interval(&mut self, time: f64) {
        let item = BlockItem::Interval {
            interval_time: time,
            elapsed_time:  0.,
        };
        self.join_item(item);
    }

    pub fn join_ctor_callable(&mut self, callable: Callable) {
        let item = BlockItem::Ctor(TweenCtor::Callable(callable));
        self.join_item(item);
    }

    pub fn join_ctor(&mut self, f: impl FnOnce() -> AnyTween + 'static) {
        let item = BlockItem::Ctor(TweenCtor::RustFn(Box::new(f)));
        self.join_item(item);
    }

    fn join_item(&mut self, item: BlockItem) {
        if let Some(last_block) = self.t.queue.last_mut() {
            last_block.push(item);
        } else {
            self.append_item(item);
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
    where AnyTween: From<TweenPtr<T>> {
        let tween = AnyTween::from(TweenPtr::new(tween));
        self.insert_ptr(time, tween);
    }

    /// Like [`insert`](Self::insert) but accepts an already-erased [`AnyTween`].
    pub fn insert_ptr(&mut self, time: f64, tween: impl Into<AnyTween>) {
        let mut tween = tween.into();

        if self.sanitize_tween_before_inclusion(&mut tween) {
            let item = InsertItem::Tween(tween);
            self.insert_item(time, item);
        }
    }

    /// Inserts a [`Callable`] at an absolute time offset, independent of the
    /// block queue. The sequence's delay is added to the offset (see [`insert`](Self::insert)).
    pub fn insert_call(&mut self, time: f64, callable: Callable) {
        let item = InsertItem::Callable {
            callable,
            invoked: false,
        };
        self.insert_item(time, item);
    }

    /// Same as [`insert_call`], but accepts any Rust closure.
    pub fn insert_fn(&mut self, time: f64, f: impl FnMut() + 'static) {
        let f = Box::new(f);
        let item = InsertItem::RustFn { f, invoked: false };
        self.insert_item(time, item);
    }

    pub fn insert_ctor_callable(&mut self, time: f64, callable: Callable) {
        let item = InsertItem::Ctor(TweenCtor::Callable(callable));
        self.insert_item(time, item);
    }

    pub fn insert_ctor(&mut self, time: f64, f: impl FnOnce() -> AnyTween + 'static) {
        let item = InsertItem::Ctor(TweenCtor::RustFn(Box::new(f)));
        self.insert_item(time, item);
    }

    fn insert_item(&mut self, time: f64, item: InsertItem) { self.t.inserts.push((time, item)); }

    /// Iterator over child tweens in the immediate queue and inserts (does not descend
    /// into nested sequences). Skips `BlockItem::Call` and `BlockItem::Interval`.
    #[doc(hidden)]
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
                if let BlockItem::Callable { callable, .. } = item
                    && callable == to_remove
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
            if let InsertItem::Callable { callable, .. } = item
                && callable == to_remove
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
                BlockItem::Ctor(_) => {} // Intentional, no reason to restart something that didn't even start yet.
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
                BlockItem::RustFn { f: _, invoked } => {
                    *invoked = false;
                }
                BlockItem::Callable { callable: _, invoked } => {
                    *invoked = false;
                }
            });

        self.t.inserts.iter_mut().for_each(|(_, item)| match item {
            InsertItem::Ctor(_) => {} // Intentional, no reason to restart something that didn't even start yet.
            InsertItem::Tween(tween) => {
                tween.stop();
                tween.play();
            }
            InsertItem::RustFn { f: _, invoked } => {
                *invoked = false;
            }
            InsertItem::Callable { callable: _, invoked } => {
                *invoked = false;
            }
        });
    }

    fn is_already_inside(&self, tween: &AnyTween) -> bool {
        let addr = tween.address();
        // This may look suspicious, but `tween.address()` returns the address of the `UnsafeCell<Tween>`, not of the RcPtr.
        // In a hypothetical scenario where `self` == `tween`, `self_address` would indeed be equal to `tween.address()`.
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

        if let AnyTween::Sequence(sequence) = tween {
            for inner in SequenceIter::new(sequence, Some(self_address)) {
                if let Err(error) = inner {
                    error.warn("Sequence insertion");
                    return true;
                }
            }
        }

        false
    }

    #[must_use]
    fn sanitize_tween_before_inclusion(&mut self, tween: &mut AnyTween) -> bool {
        if self.is_already_inside(&tween) {
            return false;
        }

        if !TM.tween_notify_sequence_child(&tween) {
            return false;
        }

        match self.state {
            State::Playing | State::Paused => tween.pause(),
            State::Stopped => tween.stop(),
        }

        self.check_ease(tween);
        true
    }
}

impl TweenPtr<Sequence> {
    /// Iterator that recursively descends into child sequences.
    #[doc(hidden)]
    pub fn iter_inner_tweens_recursive(&self) -> SequenceIter { SequenceIter::new(self, Some(self.address())) }
}

pub struct SequenceIter {
    stack:   SmallVec<[AnyTween; 8]>,
    visited: SmolSet<[AnyTween; 16]>,
    forbid:  Option<*const ()>,
}

impl SequenceIter {
    pub(crate) fn new(seq: &SpireTween<Sequence>, forbid: Option<*const ()>) -> Self {
        let mut this = Self {
            stack: SmallVec::new(),
            visited: SmolSet::new(),
            forbid,
        };
        this.queue_inner_tweens(seq);
        this
    }

    fn queue_inner_tweens(&mut self, seq: &SpireTween<Sequence>) {
        for blocks in &seq.t.queue {
            for item in blocks {
                match item {
                    BlockItem::Ctor(_) => {}
                    BlockItem::Tween(tween) => self.stack.push(AnyTween::clone(tween)),
                    BlockItem::Interval { .. } => {}
                    BlockItem::RustFn { .. } => {}
                    BlockItem::Callable { .. } => {}
                }
            }
        }

        for (_, item) in &seq.t.inserts {
            match item {
                InsertItem::Ctor(_) => {}
                InsertItem::Tween(tween) => self.stack.push(tween.clone()),
                InsertItem::RustFn { .. } => {}
                InsertItem::Callable { .. } => {}
            }
        }
    }
}

pub enum SequenceTraversalError {
    RepeatedTween(AnyTween),
    ForbiddenTweenFound,
}

impl SequenceTraversalError {
    pub(crate) fn warn(&self, operation: &str) {
        match self {
            Self::RepeatedTween(tween) => {
                godot_warn!(
                    "[SpireTween] {operation} rejected: tween {:?} is referenced more than once (cycle or unsupported \
                     reuse).",
                    tween.address(),
                );
            }
            Self::ForbiddenTweenFound => {
                godot_warn!(
                    "[SpireTween] {operation} rejected: sequence references a forbidden ancestor (cycle detected).",
                );
            }
        }
    }
}

impl Iterator for SequenceIter {
    type Item = Result<AnyTween, SequenceTraversalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let tween = self.stack.pop()?;

        if let Some(addr) = self.forbid
            && addr == tween.address()
        {
            self.stack.clear(); // Subsequent calls return None.
            return Some(Err(SequenceTraversalError::ForbiddenTweenFound));
        }

        if !self.visited.insert(tween.clone()) {
            self.stack.clear(); // Subsequent calls return None.
            return Some(Err(SequenceTraversalError::RepeatedTween(tween)));
        }

        if let AnyTween::Sequence(sequence) = &tween {
            self.queue_inner_tweens(sequence);
        }

        Some(Ok(tween))
    }
}
