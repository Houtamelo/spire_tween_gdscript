use bitflags::bitflags;

use super::*;

/// Core tween container, generic over a data type `T` implementing `ITweenable`.
///
/// `T` determines what happens each tick (property interpolation, method call,
/// delayed callback, or sequence). Use extension traits like `DoProperty` to
/// construct, then `.register()` to submit to the global `TweenManager`.
#[must_use = "This is just a data type, use `.register()` to submit to the global `TweenManager`."]
pub struct SpireTween<T: ITweenable> {
    pub bound_nodes: SmolSet<[Gd<Node>; 1]>,
    pub delay: f64,
    pub speed_scale: f64,
    pub ignore_time_scale: bool,
    pub total_elapsed_time: f64,
    pub pause_mode: PauseMode,
    pub process_mode: ProcessMode,
    pub loop_time: f64,
    pub loop_max: i64,
    pub loop_mode: LoopMode,
    pub loop_counter: i64,
    pub loop_finished_connections: Vec<Connection>,
    pub finished_connections: Vec<Connection>,
    #[doc(hidden)]
    // Hidden because manually setting the state of a tween is not recommended.
    pub state: State,
    pub gd_handle: Option<T::GdHandle>,
    pub t: T,
}

/// One callback registered against a tween's `finished` or `loop_finished` event.
///
/// Stored inside [`SpireTween::finished_connections`] /
/// [`SpireTween::loop_finished_connections`]. Built indirectly by the
/// `*_connect` / `*_connect_callable` methods.
pub struct Connection {
    /// Invoked when the corresponding event fires.
    pub callable: Callable,
    /// Dispatch options — see [`SpireFlags`] (notably `DEFERRED` and `ONE_SHOT`).
    pub flags: SpireFlags,
}

bitflags! {
    /// Dispatch options for closure-event connections registered via
    /// [`SpireTween::finished_connect`] / [`SpireTween::loop_finished_connect`] (and
    /// their `_callable` siblings).
    ///
    /// Tweens emit two events: `loop_finished` (per loop) and `finished` (after the
    /// last loop, or after [`SpireTweener::force_complete`]). There are two parallel
    /// ways to subscribe:
    /// - **GDScript / gdext-signal consumers:** connect to the `loop_finished` /
    ///   `finished` Godot signals on the `Gd<Spire…>` handle returned by
    ///   [`SpireTween::register_with_gd_handle`].
    /// - **Pure Rust consumers:** use the `*_connect` / `*_connect_callable` methods
    ///   on [`SpireTween<T>`]. No `Gd` handle required —
    ///   [`SpireTween::register`] is enough.
    pub struct SpireFlags: u8 {
        /// Dispatch via `Callable::call_deferred` (end-of-frame) instead of `call`.
        /// I recommend always leaving this on since it avoids 95% of "double borrow"
        /// panic cases that gdext-rust users often come across. You **MUST** use this
        /// when the callback may edit running tweens (registering/unregistering
        /// operations are automatically deferred so those are fine), or otherwise
        /// mutate global state that the dispatch site is iterating.
        const DEFERRED = 1;
        /// Drop the connection after the first invocation. Without this flag,
        /// callbacks persist across loops, re-registrations, and `play()` cycles.
        ///
        /// Combine with [`Self::DEFERRED`] (`SpireFlags::DEFERRED | SpireFlags::ONE_SHOT`)
        /// for the safe-by-default choice in most situations.
        const ONE_SHOT = 2;
    }
}

impl<T: ITweenable> SpireTween<T> {
    pub fn new_with_data(t: T) -> Self {
        Self {
            bound_nodes: Default::default(),
            state: State::Playing,
            delay: 0.,
            speed_scale: 1.,
            ignore_time_scale: false,
            total_elapsed_time: 0.,
            loop_time: 0.,
            loop_counter: 0,
            pause_mode: Default::default(),
            process_mode: Default::default(),
            loop_max: 1,
            loop_mode: Default::default(),
            loop_finished_connections: Default::default(),
            finished_connections: Default::default(),
            gd_handle: None,
            t,
        }
    }

    /// Returns the current playback state. See [`State`] for details on each variant.
    #[inline]
    pub fn get_state(&self) -> State { self.state }

    /// Delegates to [`play`](SpireTweener::play), [`pause`](SpireTweener::pause), or
    /// [`stop`](SpireTweener::stop) based on the given state.
    #[inline]
    pub fn set_state(&mut self, state: State)
    where Self: SpireTweener {
        match state {
            State::Stopped => self.stop(),
            State::Playing => self.play(),
            State::Paused => self.pause(),
        }
    }

    /// Returns `true` if the state is [`State::Playing`]; `false` otherwise.
    #[inline]
    pub fn is_playing(&self) -> bool { self.state == State::Playing }
    /// Returns `true` if the state is [`State::Paused`]; `false` otherwise.
    ///
    /// A paused tween works like a node outside the SceneTree — it doesn't interact
    /// with the game in any way.
    #[inline]
    pub fn is_paused(&self) -> bool { self.state == State::Paused }
    /// Returns `true` if the state is [`State::Stopped`]; `false` otherwise.
    ///
    /// A tween automatically stops once it finishes playing, but you can forcibly
    /// stop it with [`SpireTweener::stop`].
    #[inline]
    pub fn is_stopped(&self) -> bool { self.state == State::Stopped }

    /// Direct mutable access to the bound-nodes set. Prefer [`bound_to`](Self::bound_to)
    /// or [`maybe_bound`](Self::maybe_bound) for chained construction.
    #[inline]
    pub(crate) fn bound_nodes_mut(&mut self) -> &mut SmolSet<[Gd<Node>; 1]> { &mut self.bound_nodes }
    /// Returns the set of nodes this tween is currently bound to. See [`bound_to`](Self::bound_to)
    /// for the semantics of binding.
    ///
    /// **Note:** Property/Method tweens are forcibly bound to the node they animate;
    /// that node is *not* included in this set.
    #[inline]
    pub fn get_bound_nodes(&self) -> &SmolSet<[Gd<Node>; 1]> { &self.bound_nodes }
    /// Unbinds this tween from all nodes it is currently bound to.
    /// See [`bound_to`](Self::bound_to) for the semantics of binding.
    ///
    /// **Note:** Property/Method tweens are forcibly bound to the node they animate;
    /// this method won't unbind them from that node, and there is no way to do so.
    #[inline]
    pub fn clear_bound_nodes(&mut self) { self.bound_nodes.clear(); }

    /// Returns the total delay (in seconds) before the tween starts playing.
    /// See [`set_delay`](Self::set_delay) for full semantics.
    ///
    /// **Note:** This returns the *initial* delay, not the remaining delay. Remaining
    /// delay can be computed as `get_delay() - get_total_elapsed_time()`.
    #[inline]
    pub fn get_delay(&self) -> f64 { self.delay }
    /// Sets the delay (in seconds) before the tween starts playing. Default is `0.0`.
    ///
    /// While in the delay window the tween is effectively paused.
    ///
    /// **Note:** Delay is affected by [`set_speed_scale`](Self::set_speed_scale) — a
    /// speed scale of `3.0` makes delay pass three times faster.
    ///
    /// **Note:** When the tween is configured to loop, delay only applies before the
    /// first loop; subsequent loops start immediately.
    ///
    /// **Note:** [`get_animation_position`](Self::get_animation_position) only starts
    /// increasing once the delay has elapsed.
    ///
    /// **Note:** Delay does not elapse if the tween isn't processing (due to
    /// [`PauseMode`]) or if it is paused/stopped.
    #[inline]
    pub fn set_delay(&mut self, delay: f64) { self.delay = delay; }

    /// Returns this tween's speed-scale multiplier. See [`set_speed_scale`](Self::set_speed_scale).
    #[inline]
    pub fn get_speed_scale(&self) -> f64 { self.speed_scale }
    /// Multiplier applied to `delta_time` when processing the tween.
    /// `0.0` effectively pauses the tween; `2.0` makes it run twice as fast. Default is `1.0`.
    #[inline]
    pub fn set_speed_scale(&mut self, speed_scale: f64) { self.speed_scale = speed_scale; }

    /// Returns whether this tween ignores the global time scale.
    /// See [`set_ignore_time_scale`](Self::set_ignore_time_scale).
    // TODO: Add integration test for ignore_time_scale
    #[inline]
    pub fn get_ignore_time_scale(&self) -> bool { self.ignore_time_scale }
    /// If `ignore` is `true`, the tween ignores `Engine::time_scale` and updates with
    /// real elapsed time. This affects all child tweens (in a [`Sequence`]) and their delays.
    /// Default is `false`.
    #[inline]
    pub fn set_ignore_time_scale(&mut self, ignore: bool) { self.ignore_time_scale = ignore; }

    /// Returns the [`PauseMode`] assigned to this tween.
    /// See [`set_pause_mode`](Self::set_pause_mode) for behavior of each variant.
    #[inline]
    pub fn get_pause_mode(&self) -> PauseMode { self.pause_mode }
    /// Determines the tween's behavior when the SceneTree is paused.
    ///
    /// - [`PauseMode::Bound`]: only processes when all bound nodes can process
    ///   (checked via `Node::can_process`). Falls back to [`PauseMode::Stop`] if
    ///   there are no bound nodes.
    /// - [`PauseMode::Stop`]: only processes when the SceneTree is not paused.
    /// - [`PauseMode::Process`]: processes regardless of the SceneTree's pause state.
    ///
    /// Default is [`PauseMode::Bound`]. Most tweens created via the `Do*` traits are
    /// automatically bound to the node they animate.
    #[inline]
    pub fn set_pause_mode(&mut self, pause_mode: PauseMode) { self.pause_mode = pause_mode; }

    /// Returns the [`ProcessMode`] assigned to this tween.
    /// See [`set_process_mode`](Self::set_process_mode) for behavior of each variant.
    #[inline]
    pub fn get_process_mode(&self) -> ProcessMode { self.process_mode }
    /// Determines whether the tween updates in `_process`, `_physics_process`, or only
    /// when stepped manually.
    ///
    /// Default is [`ProcessMode::Idle`]. [`ProcessMode::Manual`] disables automatic
    /// processing entirely; advance the tween via the `process` method on the
    /// [`SpireTweener`] trait, or via [`AnyTween::process`] if you've type-erased it.
    ///
    /// **Inside sequences:** the [`ProcessMode`] of child tweens is ignored — only the
    /// root sequence's process mode matters.
    #[inline]
    pub fn set_process_mode(&mut self, process_mode: ProcessMode) { self.process_mode = process_mode; }

    /// Returns the current position within the active loop, in seconds (i.e. the time
    /// elapsed since the start of the current loop, after any delay).
    ///
    /// **Note:** This excludes the initial delay set via [`set_delay`](Self::set_delay) — it
    /// returns `0.0` during the delay window.
    ///
    /// **Note:** This does not take easing into account.
    ///
    /// **Note:** This is *not* real time — it is affected by [`set_speed_scale`](Self::set_speed_scale)
    /// and does not increase while the tween is paused or unable to process.
    #[inline]
    pub fn get_animation_position(&self) -> f64 { self.loop_time }
    /// Total time the tween has been animating (across all loops, since it started).
    /// Affected by [`set_speed_scale`](Self::set_speed_scale); reset to 0 by [`SpireTweener::stop`].
    #[inline]
    pub fn get_total_elapsed_time(&self) -> f64 { self.total_elapsed_time }
    /// Number of loops completed since this tween began playing. Resets to 0 when the
    /// tween is stopped.
    #[inline]
    pub fn get_loops_finished(&self) -> i64 { self.loop_counter }

    /// Returns the configured loop count (see [`set_loops`](Self::set_loops)).
    /// `-1` indicates an infinitely-looping tween.
    #[inline]
    pub fn get_loops(&self) -> i64 { self.loop_max }
    /// Sets how many times the tween will run, plus the [`LoopMode`] used between loops.
    ///
    /// Parameter `loops`:
    /// - Any positive number runs the tween that many times. `set_loops(1, _)` is the default behavior.
    /// - Tweens always run at least once, so `loops == 0` is treated as `1`.
    /// - Any negative number (conventionally `-1`) loops infinitely.
    ///
    /// Parameter `loop_mode` controls the start of each new loop — see [`LoopMode`] for the
    /// `Restart` / `Yoyo` / `Incremental` semantics.
    ///
    /// **Note:** Unlike Godot's built-in `Tween`, Spire never loops more than once per
    /// frame, even if a loop's duration is `0.0`.
    #[inline]
    pub fn set_loops(&mut self, loops: i64, loop_mode: LoopMode) {
        if loops == 0 {
            self.loop_max = 1;
        } else {
            self.loop_max = loops;
        }

        self.loop_mode = loop_mode;
    }

    /// Returns the [`LoopMode`] assigned to this tween. See [`set_loops`](Self::set_loops).
    /// Default is [`LoopMode::Restart`].
    #[inline]
    pub fn get_loop_mode(&self) -> LoopMode { self.loop_mode }
    /// Changes the [`LoopMode`] without altering the loop count. Use [`set_loops`](Self::set_loops)
    /// to set both at once.
    #[inline]
    pub fn set_loop_mode(&mut self, loop_mode: LoopMode) { self.loop_mode = loop_mode; }

    /// Registers a Rust closure to run after each loop completes (and once more
    /// after the final loop, just before [`finished_connect`](Self::finished_connect)
    /// fires).
    ///
    /// `f` is wrapped in a `Callable::from_fn` internally — to disconnect later you'd
    /// need [`loop_finished_clear_connections`](Self::loop_finished_clear_connections)
    /// (you don't get back a comparable handle). If you want disconnect-by-callable,
    /// use [`loop_finished_connect_callable`](Self::loop_finished_connect_callable)
    /// and keep a clone of the `Callable`.
    ///
    /// **Recommended `flags`:** `SpireFlags::DEFERRED | SpireFlags::ONE_SHOT` for the
    /// safe-by-default choice (see [`SpireFlags::DEFERRED`] for why).
    ///
    /// # Example
    ///
    /// ```ignore
    /// use spire_tween::prelude::*;
    /// let handle = my_node.do_position(target, 1.0).register();
    /// handle.to_mut().loop_finished_connect(
    ///     || godot_print!("loop finished"),
    ///     SpireFlags::DEFERRED,
    /// );
    /// ```
    #[inline]
    pub fn loop_finished_connect<F: FnMut() + 'static>(&mut self, mut f: F, flags: SpireFlags) {
        let callable = Callable::from_fn(type_name::<F>(), move |_| f());
        self.loop_finished_connect_callable(callable, flags);
    }

    /// Same as [`loop_finished_connect`](Self::loop_finished_connect) but takes an
    /// existing [`Callable`]. Hold onto a clone if you intend to call
    /// [`loop_finished_disconnect_callable`](Self::loop_finished_disconnect_callable)
    /// later — gdext callable equality is reference-counted, so a freshly-built
    /// equivalent won't compare equal.
    #[inline]
    pub fn loop_finished_connect_callable(&mut self, callable: Callable, flags: SpireFlags) {
        self.loop_finished_connections.push(Connection { callable, flags });
    }

    /// Removes connections whose callable compares equal to `callable`. Pass a clone
    /// of the original handle (callable equality is reference-counted). Emits a
    /// `godot_warn!` if no match was found.
    #[inline]
    pub fn loop_finished_disconnect_callable(&mut self, callable: &Callable) {
        let extracted = self
            .loop_finished_connections
            .extract_if(.., |conn| conn.callable == *callable)
            .count();

        if extracted == 0 {
            godot_warn!(
                "[SpireTween::loop_finished_disconnect_callable] No connections found for the given callable \
                 `{callable:?}`."
            );
        }
    }

    /// Drops all loop-finished connections registered via this API. Does not affect
    /// Godot signal subscribers attached to the `Gd<Spire…>` handle.
    #[inline]
    pub fn loop_finished_clear_connections(&mut self) { self.loop_finished_connections.clear(); }

    /// Registers a Rust closure to run when this tween completes its last loop, or
    /// when [`SpireTweener::force_complete`] is called.
    ///
    /// **Never fires** for infinite-loop tweens (`set_loops(-1, _)`). On the last
    /// loop this fires *after* [`loop_finished_connect`](Self::loop_finished_connect)
    /// callbacks for that loop.
    ///
    /// `f` is wrapped in a `Callable::from_fn` internally — to disconnect later you'd
    /// need [`finished_clear_connections`](Self::finished_clear_connections). For
    /// disconnect-by-callable use
    /// [`finished_connect_callable`](Self::finished_connect_callable) and keep a
    /// clone of the `Callable`.
    ///
    /// **Recommended `flags`:** `SpireFlags::DEFERRED | SpireFlags::ONE_SHOT` for the
    /// safe-by-default choice (see [`SpireFlags::DEFERRED`] for why).
    ///
    /// # Example
    ///
    /// ```ignore
    /// use spire_tween::prelude::*;
    /// let handle = my_node.do_position(target, 1.0).register();
    /// handle.to_mut().finished_connect(
    ///     || godot_print!("done!"),
    ///     SpireFlags::DEFERRED | SpireFlags::ONE_SHOT,
    /// );
    /// ```
    #[inline]
    pub fn finished_connect<F: FnMut() + 'static>(&mut self, mut f: F, flags: SpireFlags) {
        let callable = Callable::from_fn(type_name::<F>(), move |_| f());
        self.finished_connections.push(Connection { callable, flags });
    }

    /// Same as [`finished_connect`](Self::finished_connect) but takes an existing
    /// [`Callable`]. Hold onto a clone if you intend to call
    /// [`finished_disconnect_callable`](Self::finished_disconnect_callable) later
    /// (callable equality is reference-counted).
    #[inline]
    pub fn finished_connect_callable(&mut self, callable: Callable, flags: SpireFlags) {
        self.finished_connections.push(Connection { callable, flags });
    }

    /// Removes connections whose callable compares equal to `callable`. Pass a clone
    /// of the original handle. Emits a `godot_warn!` if no match was found.
    #[inline]
    pub fn finished_disconnect_callable(&mut self, callable: &Callable) {
        let extracted = self
            .finished_connections
            .extract_if(.., |conn| conn.callable == *callable)
            .count();

        if extracted == 0 {
            godot_warn!(
                "[SpireTween::finished_disconnect_callable] No connections found for the given callable \
                 `{callable:?}`."
            );
        }
    }

    /// Drops all finished connections registered via this API. Does not affect
    /// Godot signal subscribers attached to the `Gd<Spire…>` handle.
    #[inline]
    pub fn finished_clear_connections(&mut self) { self.finished_connections.clear(); }
}

#[derive(Debug)]
pub(crate) enum ObjectValidityResult {
    CanProcess,
    DontProcess,
    SomeObjectsDead,
}

impl<T: ITweenable> SpireTween<T> {
    pub(crate) fn handle_bound_nodes_validity(&mut self, is_tree_paused: bool) -> ObjectValidityResult
    where Self: SpireTweener {
        use BoundInstancesState::*;
        use PauseMode::*;

        let bound_objects_status = eval_bound_objects_status(self.bound_nodes.iter());
        match (self.pause_mode, bound_objects_status) {
            (Bound, AllAliveOrInsideTree) => {
                if self.bound_nodes.iter().any(|node| !node.can_process()) {
                    return ObjectValidityResult::DontProcess;
                }
            }
            (Bound, SomeOutsideTree) => {
                return ObjectValidityResult::DontProcess;
            }
            (Process, AllAliveOrInsideTree | SomeOutsideTree) => {}
            (_, SomeDead) => {
                self.stop();
                return ObjectValidityResult::SomeObjectsDead;
            }
            (Stop, _) => {
                if is_tree_paused {
                    return ObjectValidityResult::DontProcess;
                }
            }
        }

        ObjectValidityResult::CanProcess
    }

    #[inline]
    pub(crate) fn reset_counters(&mut self) {
        self.total_elapsed_time = 0.;
        self.loop_time = 0.;
        self.loop_counter = 0;
    }

    pub(crate) fn check_owner_validity_and_pause_mode(&self, owner: &ObjectOrNode) -> ObjectValidityResult {
        match owner {
            ObjectOrNode::Object(obj) => {
                let id = obj.instance_id_unchecked().to_i64();
                if !is_instance_id_valid(id) {
                    return ObjectValidityResult::SomeObjectsDead;
                }
            }
            ObjectOrNode::Node(node) => {
                if !is_instance_id_valid(node.instance_id_unchecked().to_i64()) {
                    return ObjectValidityResult::SomeObjectsDead;
                }
                match TM.node_get_status_fresh(node.clone()) {
                    NodeStatus::InsideTree => {
                        if let PauseMode::Bound = self.pause_mode
                            && !node.can_process()
                        {
                            return ObjectValidityResult::DontProcess;
                        }
                    }
                    // When using get_status_fresh this is guaranteed to be OutsideTree + NOT dead.
                    NodeStatus::OutsideTreeMaybeDead => {
                        if let PauseMode::Bound = self.pause_mode {
                            return ObjectValidityResult::DontProcess;
                        }
                    }
                    NodeStatus::Dead => {
                        return ObjectValidityResult::SomeObjectsDead;
                    }
                }
            }
        }

        ObjectValidityResult::CanProcess
    }

    pub(crate) fn handle_time_step(&mut self, delta_time: f64) -> Option<f64> {
        let step = delta_time * self.speed_scale;
        self.total_elapsed_time += step;

        let past_delay = self.total_elapsed_time - self.delay;

        if past_delay <= 0. {
            return None;
        }

        let actual_step = if past_delay >= step { step } else { past_delay };

        self.loop_time += actual_step;
        Some(actual_step)
    }

    pub(crate) fn handle_loop_finished(&mut self, excess_time: f64) -> Option<f64>
    where Self: SpireTweener {
        self.loop_counter += 1;
        self.emit_loop_finished();

        match self.loop_max {
            ..0 => {
                self.loop_time = excess_time;
                None
            }
            loop_max => {
                if self.loop_counter < loop_max {
                    self.loop_time = excess_time;
                    None
                } else {
                    self.loop_time -= excess_time;
                    self.handle_finished();
                    Some(excess_time)
                }
            }
        }
    }

    fn emit_loop_finished(&mut self) {
        self.gd_handle.as_ref().map(Signaler::emit_loop_finished);

        self.loop_finished_connections.retain_mut(|conn| {
            if conn.flags.contains(SpireFlags::DEFERRED) {
                conn.callable.call_deferred(&[]);
            } else {
                conn.callable.call(&[]);
            }

            !conn.flags.contains(SpireFlags::ONE_SHOT)
        });
    }

    #[inline]
    pub(crate) fn handle_finished(&mut self)
    where Self: SpireTweener {
        self.stop();
        self.emit_finished();
    }

    fn emit_finished(&mut self) {
        self.gd_handle.as_ref().map(Signaler::emit_finished);

        self.finished_connections.retain_mut(|conn| {
            if conn.flags.contains(SpireFlags::DEFERRED) {
                conn.callable.call_deferred(&[]);
            } else {
                conn.callable.call(&[]);
            }

            !conn.flags.contains(SpireFlags::ONE_SHOT)
        });
    }
}

impl<T: ITweenable> InnerTypeName for SpireTween<T> {
    fn inner_type_name(&self) -> &'static str { type_name::<T>() }
}

enum BoundInstancesState {
    AllAliveOrInsideTree,
    SomeOutsideTree,
    SomeDead,
}

#[inline]
fn eval_bound_objects_status<'a>(bound_nodes: impl Iterator<Item = &'a Gd<Node>> + 'a) -> BoundInstancesState {
    let mut any_outside = false;

    for node in bound_nodes {
        // Official gdext panics on clone of freed instances, check ID first.
        if !is_instance_id_valid(node.instance_id_unchecked().to_i64()) {
            return BoundInstancesState::SomeDead;
        }

        match TM.node_get_status_fresh(node.clone()) {
            NodeStatus::InsideTree => {}
            NodeStatus::OutsideTreeMaybeDead => {
                any_outside = true;
            }
            NodeStatus::Dead => return BoundInstancesState::SomeDead,
        }
    }

    if any_outside { BoundInstancesState::SomeOutsideTree } else { BoundInstancesState::AllAliveOrInsideTree }
}
