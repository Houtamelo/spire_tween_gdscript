use godot::prelude::*;

/// Port of `examples/screen_saver/screen_saver.gd`.
///
/// The parent scene that launches a `BouncyBall` RigidBody2D with a random
/// initial velocity. This is a simple scene controller with no tweens of
/// its own -- the tweening happens inside `BouncyBall` and `CollisionFx`.
#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct ScreenSaver {
    base: Base<Node2D>,

    #[export]
    #[init(val = 1000.0)]
    initial_speed: f64,
}

#[godot_api]
impl INode2D for ScreenSaver {
    fn ready(&mut self) {
        let mut ball: Gd<godot::classes::RigidBody2D> = self.base().get_node_as("BouncyBall");

        let angle = godot::global::randf() as f32 * 2.0 * std::f32::consts::PI;
        let velocity =
            Vector2::from_angle(angle) * self.initial_speed as f32;
        ball.set_linear_velocity(velocity);
    }
}
