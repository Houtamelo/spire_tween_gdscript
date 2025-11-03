use super::*;

pub trait CompleteBoundTweens {
    fn complete_bound_tweens(&mut self);
}

impl<T: Inherits<Node>> CompleteBoundTweens for Gd<T> {
    fn complete_bound_tweens(&mut self) {
        TM.node_bound_tweens_force_complete(self.clone().upcast());
    }
}

pub trait KillBoundTweens {
    fn kill_bound_tweens(&mut self);
}

impl<T: Inherits<Node>> KillBoundTweens for Gd<T> {
    fn kill_bound_tweens(&mut self) { TM.node_bound_tweens_kill(self.clone().upcast()); }
}
