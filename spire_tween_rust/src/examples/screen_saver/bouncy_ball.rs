use godot::classes::{IRigidBody2D, PackedScene, PhysicsDirectBodyState2D, RigidBody2D};
use godot::prelude::*;

/// Port of `examples/screen_saver/bouncy_ball.gd`.
///
/// A `RigidBody2D` that detects collisions, slightly rotates its velocity
/// on each impact, and spawns a visual effect at the collision point.
#[derive(GodotClass)]
#[class(init, base = RigidBody2D)]
pub struct BouncyBall {
    base: Base<RigidBody2D>,

    #[var]
    #[init(val = Vector2::ZERO)]
    collision_pos: Vector2,
}

#[godot_api]
impl IRigidBody2D for BouncyBall {
    fn ready(&mut self) {
        let self_gd = self.to_gd();

        let body_entered_callable = Callable::from_fn("on_body_entered", {
            let self_ref = self_gd.clone();
            move |_args| {
                if !self_ref.is_instance_valid() {
                    return Variant::nil();
                }

                let mut body = self_ref.clone();
                let vel = body.bind().base().get_linear_velocity();
                let rand_angle = godot::global::randf_range(
                    -std::f64::consts::FRAC_PI_8,
                    std::f64::consts::FRAC_PI_8,
                ) as f32;
                body.bind_mut()
                    .base_mut()
                    .set_linear_velocity(vel.rotated(rand_angle));

                let collision_pos = body.bind().collision_pos;

                let fx_scene: Gd<PackedScene> =
                    load("res://examples/screen_saver/collision_fx.tscn");
                let fx = fx_scene.instantiate().unwrap();

                if let Some(mut root) = body
                    .bind()
                    .base()
                    .get_tree()
                    .get_root()
                {
                    root.add_child(&fx);
                    let mut fx_2d: Gd<Node2D> = fx.cast();
                    fx_2d.set_global_position(collision_pos);
                }

                Variant::nil()
            }
        });

        self.base_mut()
            .connect("body_entered", &body_entered_callable);
    }

    fn integrate_forces(&mut self, state: Option<Gd<PhysicsDirectBodyState2D>>) {
        if let Some(state) = state {
            if state.get_contact_count() >= 1 {
                self.collision_pos = state.get_contact_local_position(0);
            }
        }
    }
}
