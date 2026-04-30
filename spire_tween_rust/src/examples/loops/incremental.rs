use godot::classes::Label;
use godot::prelude::*;
use spire_tween::prelude::*;

/// Port of `examples/loops/incremental.gd`.
///
/// Demonstrates `LoopMode::Incremental`: each loop iteration adds the
/// tween's delta on top of the previous end value. Here, gravity_scale
/// increases by 2.0 each loop, 10 times.
#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Incremental {
    base: Base<Node2D>,

    #[export]
    #[init(val = 1000.0)]
    initial_speed: f64,
}

#[godot_api]
impl INode2D for Incremental {
    fn ready(&mut self) {
        let mut ball: Gd<godot::classes::RigidBody2D> = self.base().get_node_as("BouncyBall");

        // Launch ball in a random direction.
        let angle = godot::global::randf() as f32 * 2.0 * std::f32::consts::PI;
        let velocity = Vector2::from_angle(angle) * self.initial_speed as f32;
        ball.set_linear_velocity(velocity);

        // DoRigidBody2D.gravity_scale(ball, 2, 2).set_loops(10, LOOP_MODE_INCREMENTAL)
        let mut tween = ball.do_gravity_scale(2.0, 2.0);
        tween.set_loops(10, LoopMode::Incremental);
        tween.register();
    }

    fn process(&mut self, _delta: f64) {
        let ball: Gd<godot::classes::RigidBody2D> = self.base().get_node_as("BouncyBall");
        let label: Gd<Label> = self.base().get_node_as("Label");

        let gravity = ball.get_gravity_scale();
        let mut label_mut = label;
        label_mut.set_text(&format!("Gravity: {gravity}"));
    }
}
