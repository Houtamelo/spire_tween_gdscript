use super::*;

pub trait CompleteBoundTweens {
    fn complete_bound_tweens(&mut self);
}

impl<T: Inherits<Node>> CompleteBoundTweens for Gd<T> {
    fn complete_bound_tweens(&mut self) {
        let node_id = self.instance_id_unchecked().to_i64();
        TWEENS.object_remove_and_edit_bound_tweens(node_id, |tween| tween.complete());
    }
}

pub trait KillBoundTweens {
    fn kill_bound_tweens(&mut self);
}

impl<T: Inherits<Node>> KillBoundTweens for Gd<T> {
    fn kill_bound_tweens(&mut self) {
        let node_id = self.instance_id_unchecked().to_i64();
        TWEENS.object_remove_and_edit_bound_tweens(node_id, |tween| tween.stop());
    }
}
