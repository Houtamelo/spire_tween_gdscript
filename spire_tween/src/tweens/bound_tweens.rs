use super::*;

/// Force-completes every tween currently bound to this node.
///
/// Each affected tween jumps to its final value, fires its `finished` event, and
/// transitions to [`State::Stopped`]. Equivalent to calling
/// [`SpireTweener::force_complete`] on each one individually.
///
/// Useful when a node is about to enter a different state (e.g. transition out of
/// a menu) and you want any pending animations to settle synchronously.
///
/// Implemented for any `Gd<T: Inherits<Node>>`.
pub trait CompleteBoundTweens {
    fn complete_bound_tweens(&mut self);
}

impl<T: Inherits<Node>> CompleteBoundTweens for Gd<T> {
    fn complete_bound_tweens(&mut self) {
        TM.node_bound_tweens_force_complete(self.clone().upcast());
    }
}

/// Unregisters every tween currently bound to this node — they stop ticking
/// immediately and do *not* fire `finished`.
///
/// Use this when you want to abandon pending animations without applying their
/// end state (the opposite of [`CompleteBoundTweens`]).
///
/// Implemented for any `Gd<T: Inherits<Node>>`.
pub trait KillBoundTweens {
    fn kill_bound_tweens(&mut self);
}

impl<T: Inherits<Node>> KillBoundTweens for Gd<T> {
    fn kill_bound_tweens(&mut self) { TM.node_bound_tweens_kill(self.clone().upcast()); }
}
