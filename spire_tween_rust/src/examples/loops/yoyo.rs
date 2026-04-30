use godot::classes::Sprite2D;
use godot::prelude::*;
use spire_tween::prelude::*;

/// Port of `examples/loops/yoyo.gd`.
///
/// Demonstrates `LoopMode::Yoyo`: the ball moves from x = -512 to x = 512
/// and back, infinitely, with `InOutSine` easing for a smooth bounce.
#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Yoyo {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Yoyo {
    fn ready(&mut self) {
        let ball: Gd<Sprite2D> = self.base().get_node_as("Ball");

        // DoNode2D.move_x(ball, 512, 1.0).from(-512).set_loops(-1, LOOP_MODE_YOYO).set_ease(EASE_IN_OUT_SINE)
        let mut tween = ball
            .do_move_x(512.0, 1.0)
            .begin_from(-512.0)
            .with_ease(EaseKind::Basic(Ease::InOutSine));
        tween.set_loops(-1, LoopMode::Yoyo);
        tween.register();
    }
}
