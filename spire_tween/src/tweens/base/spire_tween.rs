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

pub struct Connection {
    pub callable: Callable,
    pub flags: SpireFlags,
}

bitflags! {
    pub struct SpireFlags: u8 {
        const DEFERRED = 1;
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

    #[inline]
    pub fn get_state(&self) -> State { self.state }

    /// Delegates to `play()`, `pause()`, or `stop()` based on the given state.
    #[inline]
    pub fn set_state(&mut self, state: State)
    where Self: SpireTweener {
        match state {
            State::Stopped => self.stop(),
            State::Playing => self.play(),
            State::Paused => self.pause(),
        }
    }

    #[inline]
    pub fn is_playing(&self) -> bool { self.state == State::Playing }
    #[inline]
    pub fn is_paused(&self) -> bool { self.state == State::Paused }
    #[inline]
    pub fn is_stopped(&self) -> bool { self.state == State::Stopped }

    #[inline]
    pub fn bound_nodes_mut(&mut self) -> &mut SmolSet<[Gd<Node>; 1]> { &mut self.bound_nodes }
    #[inline]
    pub fn get_bound_nodes(&self) -> &SmolSet<[Gd<Node>; 1]> { &self.bound_nodes }
    #[inline]
    pub fn clear_bound_nodes(&mut self) { self.bound_nodes.clear(); }

    #[inline]
    pub fn get_delay(&self) -> f64 { self.delay }
    #[inline]
    pub fn set_delay(&mut self, delay: f64) { self.delay = delay; }

    #[inline]
    pub fn get_speed_scale(&self) -> f64 { self.speed_scale }
    #[inline]
    pub fn set_speed_scale(&mut self, speed_scale: f64) { self.speed_scale = speed_scale; }

    // TODO: Add integration test for ignore_time_scale
    #[inline]
    pub fn get_ignore_time_scale(&self) -> bool { self.ignore_time_scale }
    #[inline]
    pub fn set_ignore_time_scale(&mut self, ignore: bool) { self.ignore_time_scale = ignore; }

    #[inline]
    pub fn get_pause_mode(&self) -> PauseMode { self.pause_mode }
    #[inline]
    pub fn set_pause_mode(&mut self, pause_mode: PauseMode) { self.pause_mode = pause_mode; }

    #[inline]
    pub fn get_process_mode(&self) -> ProcessMode { self.process_mode }
    #[inline]
    pub fn set_process_mode(&mut self, process_mode: ProcessMode) { self.process_mode = process_mode; }

    #[inline]
    pub fn get_animation_position(&self) -> f64 { self.loop_time }
    #[inline]
    pub fn get_total_elapsed_time(&self) -> f64 { self.total_elapsed_time }
    #[inline]
    pub fn get_loops_finished(&self) -> i64 { self.loop_counter }

    #[inline]
    pub fn get_loops(&self) -> i64 { self.loop_max }
    /// Negative values mean infinite looping. A value of 0 is clamped to 1.
    #[inline]
    pub fn set_loops(&mut self, loops: i64, loop_mode: LoopMode) {
        if loops == 0 {
            self.loop_max = 1;
        } else {
            self.loop_max = loops;
        }

        self.loop_mode = loop_mode;
    }

    #[inline]
    pub fn get_loop_mode(&self) -> LoopMode { self.loop_mode }
    #[inline]
    pub fn set_loop_mode(&mut self, loop_mode: LoopMode) { self.loop_mode = loop_mode; }

    #[inline]
    pub fn loop_finished_connect<F: FnMut() + 'static>(&mut self, mut f: F, flags: SpireFlags) {
        let callable = Callable::from_fn(type_name::<F>(), move |_| f());
        self.loop_finished_connect_callable(callable, flags);
    }

    #[inline]
    pub fn loop_finished_connect_callable(&mut self, callable: Callable, flags: SpireFlags) {
        self.loop_finished_connections.push(Connection { callable, flags });
    }

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

    #[inline]
    pub fn loop_finished_clear_connections(&mut self) { self.loop_finished_connections.clear(); }

    #[inline]
    pub fn finished_connect<F: FnMut() + 'static>(&mut self, mut f: F, flags: SpireFlags) {
        let callable = Callable::from_fn(type_name::<F>(), move |_| f());
        self.finished_connections.push(Connection { callable, flags });
    }

    #[inline]
    pub fn finished_connect_callable(&mut self, callable: Callable, flags: SpireFlags) {
        self.finished_connections.push(Connection { callable, flags });
    }

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
