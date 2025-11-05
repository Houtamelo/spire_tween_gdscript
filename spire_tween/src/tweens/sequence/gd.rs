use super::*;

/// A sequence allows you to chain and/or parallelize multiple tweens, callables, and intervals.
///
/// This is similar to the built-in [Tween]
/// (while other SpireTweens are closer to [Tweener]s).
///
/// [b]Note:[/b] This is a tween just like other Spire's tweens, it shares the same base properties and
/// methods.
///
/// ---
///
/// # Blocks
/// A sequence is essentially a queue of "blocks", where each block contains one or more tweens.
/// This is a sequence with 3 blocks:
/// (each tween is a letter)
///
/// ```ignore
/// █░Block1░█->█░Block2░█->█░Block3░█
/// █░░░A░░░░█->█░░░D░░░░█->█░░░F░░░░█
/// █░░░B░░░░█->█░░░E░░░░█->█░░░G░░░░█
/// █░░░C░░░░█->█░░░░░░░░█->█░░░H░░░░█
/// ```
///
/// When it starts, the sequence will play, in the following order:
/// - `A`, `B`, and `C` in parallel.
/// - `D` and `E` in parallel.
/// - `F`, `G`, and `H` in parallel.
///
/// The duration of each block is determined by the tween with the highest duration in that block,
/// the next block will not start until all tweens of the current block have completed.
///
/// Here's a visual example: the duration of each tween is indicated by how many consecutive letters it has.
/// (e.g. `AAA` has a duration of 3, `BB` has a duration of 2, etc.)
/// ```ignore
/// █░Block1░█->█░Block2░█->█░Block3░█
/// █░AAA░░░░█->█░D░░░░░░█->█░FFF░░░░█
/// █░BBBBBB░█->█░EEEE░░░█->█░GGG░░░░█
/// █░CCC░░░░█->█░░░░░░░░█->█░HHH░░░░█
/// ```
///
/// In this case, the duration of each block is:
/// - Block 1 => `BBBBBB` (6)
/// - Block 2 => `EEEE` (4)
/// - Block 3 => `FFF`, `GGG`, `HHH` (3)
///
/// So the total duration of the sequence is `6 + 4 + 3 = 13`.
///
/// You define your sequence's blocks by using [method append] and [method join], see their
/// documentation for more details.
///
/// [b]Note:[/b] The position/index of a tween within a block is irrelevant in most cases,
/// the only difference is that, within a block, tweens will be executed in the same order as they were added.
///
/// # Inserts
/// You can insert tweens and callables at specific time offsets from the start of the sequence
/// using the [method insert] and [method insert_call] methods. Inserted elements do not interact
/// with the block system, they will run independently of it.
///
/// # Loops
/// Sequences can be looped, although they only support the mode [constant Spire.LOOP_MODE_RESTART]
/// (the default mode), you can set other modes, but they will be ignored.
///
/// When a sequence loops, it restarts all of its child tweens.
///
/// # Notes regarding child tweens
/// - When a tween is added to the sequence, it will automatically be unregistered from the TweenManager, the sequence
///   takes the responsibility of "ticking" that tween.
/// - Calling `stop()` on a child tween will make the sequence skip it within the current loop.
/// - Calling `pause()` on a child tween won't do anything, the sequence will automatically call `play()` on it on
///   the next update.
/// - We recommend against adding the same tween to the sequence more than once, or the same tween to multiple
///   sequences, this will lead to undefined behavior (though it won't cause crashes or memory corruption).
/// - A child tween that never completes will make the entire sequence get stuck in that tween's block.
/// - If a child tween has loops enabled, the sequence will wait for it to complete all its loops
///   before moving to the next block.
/// - The [enum Spire.ProcessMode] of child tweens is ignored. Only the sequence's process mode matters.
/// - The [enum Spire.PauseMode] of child tweens works normally. Though keep in mind that a child tween that can't
///   process due to its `PauseMode` will keep the sequence from advancing to the next block, even if the sequence
///   itself is allowed to process.
/// - A child tween's effective `speed_scale` is calculated by `sequence.speed_scale * child.speed_scale`.
#[derive(GodotClass)]
#[class(base = RefCounted, no_init)]
pub struct SpireSequence {
    pub base:  Base<RefCounted>,
    pub inner: UnsafeCell<RcPtr<SpireTween<Sequence>>>,
}

#[godot_api]
impl SpireSequence {
    /// Emitted when this tween completes a loop.
    ///
    /// [b]Note:[/b] On the last loop of a tween, this is emitted before [signal finished].
    #[signal]
    pub fn loop_finished();

    /// Emitted when this tween finishes playing.
    /// This happens when the tween completes all its loops, or when [method force_complete] is called.
    ///
    /// [b]Note:[/b] This will never be emitted if the tween is set to loop infinitely
    /// (e.g. calling [method set_loops] with `-1`).
    #[signal]
    pub fn finished();

    /// Documentation: see [method Spire.sequence].
    #[func]
    pub fn build() -> Gd<Self> {
        let inner = UnsafeCell::new(SpireTween::<Sequence>::new().register());
        let handle = Gd::from_init_fn(|base| Self { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }

    /// [b]Behavior:[/b] Appends a tween to the end of the sequence, creating a new block
    /// containing only the given tween.
    ///
    /// # Example
    /// If a sequence looks like this:
    /// ```ignore
    /// █░Block1░█->█░Block2░█
    /// █░░░A░░░░█->█░░░D░░░░█
    /// █░░░B░░░░█->█░░░E░░░░█
    /// █░░░C░░░░█->█░░░░░░░░█
    /// ```
    ///
    /// Appending the tween `F` will result in:
    /// ```ignore
    /// █░Block1░█->█░Block2░█->█░Block3░█
    /// █░░░A░░░░█->█░░░D░░░░█->█░░░F░░░░█
    /// █░░░B░░░░█->█░░░E░░░░█->█░░░░░░░░█
    /// █░░░C░░░░█->█░░░░░░░░█->█░░░░░░░░█
    /// ```
    #[func(gd_self)]
    pub fn append(this: Gd<Self>, tween: Option<Gd<RefCounted>>) -> Gd<Self> {
        if let Some(to_append) = tween.log_bad_spire_arg(|| "tween") {
            this.bind().to_mut().append_ptr(to_append);
        }

        this
    }

    /// [b]Behavior:[/b] Appends a callable to the end of the sequence, creating a new block containing only
    /// the given callable (A callable is analogous to a zero-duration tween that simply invokes a function once).
    ///
    /// See [method append] for a visual example of how appending interacts with the sequence's block system..
    #[func(gd_self)]
    pub fn append_call(this: Gd<Self>, func: Callable) -> Gd<Self> {
        this.bind().to_mut().append_call(func);
        this
    }

    /// [b]Behavior:[/b] Appends an interval (delay) to the end of the sequence, creating a new block containing only
    /// the given interval (An interval is analogous to a tween with delay = [param seconds] that immediately
    /// completes after the delay passes).
    ///
    /// See [method append] for a visual example of how appending interacts with the sequence's block system.
    #[func(gd_self)]
    pub fn append_interval(this: Gd<Self>, seconds: f64) -> Gd<Self> {
        this.bind().to_mut().append_interval(seconds);
        this
    }

    /// [b]Behavior:[/b] Appends multiple tweens, callables, and/or intervals to the end of the sequence.
    /// This will not place all the given items in the same block, instead, each item will be placed
    /// in its own block, as if you called [method append], [method append_call], or [method append_interval]
    /// for each item in succession.
    ///
    /// [b]Parameter [param tweens]:[/b] An untyped Array where each element can be one of:
    /// any SpireTween, [Callable], or interval([float]).
    ///
    /// See [method append] for a visual example of how appending interacts with the sequence's block system.
    #[func(gd_self)]
    pub fn append_many(this: Gd<Self>, tweens: VariantArray) -> Gd<Self> {
        let gd_ref = this.bind();
        let sequence = gd_ref.to_mut();

        let mut idx = 0;
        while let Some(item) = tweens.get(idx) {
            if let Ok(callable) = item.try_to::<Callable>() {
                sequence.append_call(callable);
            } else if let Ok(handle) = item.try_to::<Gd<RefCounted>>() {
                if let Some(tween) = handle.log_bad_spire_arg(|| format!("item at index {idx}")) {
                    sequence.append_ptr(tween);
                }
            } else if let Ok(seconds) = item.try_to_relaxed::<f64>() {
                sequence.append_interval(seconds);
            } else {
                godot_error!("Expected Callable, Spire type, or interval(float); got: {:?}", item.get_type());
            }

            idx += 1;
        }

        drop(gd_ref);
        this
    }

    /// [b]Behavior:[/b] Places a tween in the last block of the sequence (will not create a new block).
    /// Compared to the built-in [Tween], this is equivalent to parallelizing a tween (see [method Tween.parallel]),
    /// except that this only requires a single method call.
    ///
    /// [b]Note:[/b] If the sequence is empty, this is the same as calling [method append].
    ///
    /// # Example
    /// If a sequence looks like this:
    /// ```ignore
    /// █░Block1░█->█░Block2░█
    /// █░░░A░░░░█->█░░░D░░░░█
    /// █░░░B░░░░█->█░░░E░░░░█
    /// █░░░C░░░░█->█░░░░░░░░█
    /// ```
    ///
    /// Joining the tween `F` will result in:
    /// ```ignore
    /// █░Block1░█->█░Block2░█
    /// █░░░A░░░░█->█░░░D░░░░█
    /// █░░░B░░░░█->█░░░E░░░░█
    /// █░░░C░░░░█->█░░░F░░░░█
    /// ```
    #[func(gd_self)]
    pub fn join(this: Gd<Self>, tween: Option<Gd<RefCounted>>) -> Gd<Self> {
        if let Some(tween) = tween.log_bad_spire_arg(|| "tween") {
            this.bind().to_mut().join_ptr(tween);
        }

        this
    }

    /// [b]Behavior:[/b] Places an interval (a delay) in the last block of the sequence (will not create a new block).
    ///
    /// [b]Note:[/b] The length of a block is determined by the longest tween in that block, so this won't do anything
    /// unless [param seconds] is bigger than the duration of all other tweens in the block.
    ///
    /// [b]Note:[/b] If the sequence is empty, this is the same as calling [method append_interval].
    ///
    /// See [method join] for a visual example of how joining interacts with the sequence's block system.
    #[func(gd_self)]
    pub fn join_interval(this: Gd<Self>, seconds: f64) -> Gd<Self> {
        this.bind().to_mut().join_interval(seconds);
        this
    }

    /// [b]Behavior:[/b] Places a callable in the last block of the sequence (will not create a new block).
    ///
    /// [b]Note:[/b] If the sequence is empty, this is the same as calling [method append_call].
    ///
    /// See [method join] for a visual example of how joining interacts with the sequence's block system.
    #[func(gd_self)]
    pub fn join_call(this: Gd<Self>, func: Callable) -> Gd<Self> {
        this.bind().to_mut().join_call(func);
        this
    }

    /// [b]Behavior:[/b] Places multiple tweens, [Callable]s, and/or intervals([float]) in the last
    /// block of the sequence (will not create new blocks).
    ///
    /// [b]Parameter [param tweens]:[/b] An untyped Array where each element can be one of:
    /// any SpireTween, [Callable], or interval([float]).
    ///
    /// [b]Note:[/b] If the sequence is empty, this will create a new block and place all the given items in it.
    ///
    /// See [method join] for a visual example of how joining interacts with the sequence's block system.
    #[func(gd_self)]
    pub fn join_many(this: Gd<Self>, tweens: VariantArray) -> Gd<Self> {
        let gd_ref = this.bind();
        let sequence = gd_ref.to_mut();

        let mut idx = 0;
        while let Some(item) = tweens.get(idx) {
            if let Ok(callable) = item.try_to::<Callable>() {
                sequence.join_call(callable);
            } else if let Ok(handle) = item.try_to::<Gd<RefCounted>>() {
                if let Some(tween) = handle.log_bad_spire_arg(|| format!("item at index {idx}")) {
                    sequence.join_ptr(tween);
                }
            } else if let Ok(seconds) = item.try_to_relaxed::<f64>() {
                sequence.join_interval(seconds);
            } else {
                godot_error!("Expected Callable, Spire object, or interval(float); got: {:?}", item.get_type());
            }

            idx += 1;
        }

        drop(gd_ref);
        this
    }

    /// [b]Behavior:[/b] Inserts a tween at a specific time offset from the start of the sequence.
    ///
    /// [b]Note:[/b] Inserted elements do not interact with the block system, they will run independently of it.
    /// However, inserted tweens may still affect the total duration of the sequence if they last longer than
    /// all the blocks. The sequence won't complete until all inserted tweens have completed.
    ///
    /// [b]Note:[/b] If the sequence itself has non-zero delay (see [method set_delay]), the sequence's delay
    /// is "added" to the time offset of all inserted elements. Example: if the sequence has a delay of 2.0 seconds,
    /// and you insert a tween with [param time_offset] = 3.0 seconds, it will effectively
    /// start 2.0 + 3.0 = 5.0 seconds after the sequence begins playing.
    #[func(gd_self)]
    pub fn insert(this: Gd<Self>, time_offset: f64, tween: Option<Gd<RefCounted>>) -> Gd<Self> {
        if let Some(to_insert) = tween.log_bad_spire_arg(|| "tween") {
            this.bind().to_mut().insert_ptr(time_offset, to_insert);
        }

        this
    }

    /// [b]Behavior:[/b] Inserts a callable to be invoked at a specific time offset from the start of the sequence.
    ///
    /// [b]Note:[/b] Inserted elements do not interact with the block system, they will run independently of it.
    ///
    /// [b]Note:[/b] If the sequence itself has non-zero delay (see [method set_delay]), the sequence's delay
    /// is "added" to the time offset of all inserted elements. Example: if the sequence has a delay of 2.0 seconds,
    /// and you insert a [Callable] with [param time_offset] = 3.0 seconds, it will be invoked
    /// 2.0 + 3.0 = 5.0 seconds after the sequence begins playing.
    #[func(gd_self)]
    pub fn insert_call(this: Gd<Self>, time_offset: f64, func: Callable) -> Gd<Self> {
        this.bind().to_mut().insert_call(time_offset, func);
        this
    }

    /// [b]Behavior:[/b] Searches for [param handle] among the sequence's blocks and inserts, then removes it
    /// if found.
    ///
    /// [b]Note:[/b] This does not perform a recursive removal, which means that nested sequences won't
    /// be queried for the tween. You need to call this method on the exact [SpireSequence] that contains the tween
    /// you want to remove.
    ///
    /// [b]Returns:[/b] `true` if the tween was found; `false` otherwise.
    #[func]
    pub fn remove(&self, tween: Option<Gd<RefCounted>>) -> bool {
        if let Some(to_remove) = tween.log_bad_spire_arg(|| "tween") { self.to_mut().remove(&to_remove) } else { false }
    }

    /// [b]Behavior:[/b] Searches for [param func] among the sequence's blocks and inserts, then removes it
    /// if found.
    ///
    /// [b]Note:[/b] This does not perform a recursive removal, which means that nested sequences won't
    /// be queried for the callable. You need to call this method on the exact [SpireSequence] that contains
    /// the callable you want to remove.
    ///
    /// [b]Returns:[/b] `true` if the callable was found; `false` otherwise.
    #[func]
    pub fn remove_call(&self, func: Callable) -> bool { self.to_mut().remove_call(&func) }

    /// [b]Behavior:[/b] When adding child tweens to this sequence, override their easing function **IF**
    /// their easing function is currently set to [constant Spire.EASE_DEFAULT]. This method is analogous to
    /// the built-in [method Tween.set_ease].
    ///
    /// [b]Note:[/b] This takes priority over the global easing configuration
    /// (see [method SpireGlobalSettings.set_default_ease]).
    ///
    /// [b]Note:[/b] This only affects tweens added after calling this method.
    ///
    /// [b]Note:[/b] The following tweens use easing functions:
    /// [SpireProperty], [SpireMethod], and [SpireSequence] - (as well as their derivatives).
    #[func(gd_self)]
    pub fn set_default_children_ease(this: Gd<Self>, ease: Ease) -> Gd<Self> {
        this.bind().to_mut().set_default_ease(ease);
        this
    }
}

define_base_gd_methods! { SpireSequence => SpireTween<Sequence> }
