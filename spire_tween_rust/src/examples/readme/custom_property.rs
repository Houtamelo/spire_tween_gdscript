use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;
use spire_tween::prelude::*;

/// Port of `examples/readme/example_custom_property.gd`.
///
/// Demonstrates animating a custom property (`distance_to_repeller`) using
/// `do_property`, combined with a rotation tween and a trail effect.
///
/// The custom property is exposed via `#[func]` getter/setter so that
/// Godot's property system can read/write it from tweens.
#[derive(GodotClass)]
#[class(init, base = Sprite2D)]
pub struct ExampleCustomProperty {
    base: Base<Sprite2D>,
}

#[godot_api]
impl ExampleCustomProperty {
    /// Custom property getter: returns the distance between self and the
    /// repelled particle.
    #[func]
    fn get_distance_to_repeller(&self) -> f64 {
        let repelled: Gd<Sprite2D> = self.base().get_node_as("RepelledParticle");
        self.base()
            .get_global_position()
            .distance_to(repelled.get_global_position()) as f64
    }

    /// Custom property setter: moves the repelled particle along the
    /// direction vector to match the new distance.
    #[func]
    fn set_distance_to_repeller(&mut self, new_distance: f64) {
        let repelled: Gd<Sprite2D> = self.base().get_node_as("RepelledParticle");
        let old_distance = self
            .base()
            .get_global_position()
            .distance_to(repelled.get_global_position()) as f64;
        let distance_diff = new_distance - old_distance;
        let direction = self
            .base()
            .get_global_position()
            .direction_to(repelled.get_global_position());
        let new_pos = repelled.get_global_position() + direction * distance_diff as f32;
        let mut repelled_mut = repelled.clone();
        repelled_mut.set_global_position(new_pos);
    }
}

#[godot_api]
impl ISprite2D for ExampleCustomProperty {
    fn ready(&mut self) {
        let repelled_particle: Gd<Sprite2D> = self.base().get_node_as("RepelledParticle");
        let self_gd: Gd<Sprite2D> = self.to_gd().upcast();

        // Animate the custom property `distance_to_repeller` with yoyo looping.
        let mut prop_tween = self_gd
            .do_property(
                "distance_to_repeller",
                Evaluator::Static(400.0_f64),
                3.0,
            )
            .with_ease(EaseKind::Basic(Ease::InOutSine));
        prop_tween.set_loops(-1, LoopMode::Yoyo);
        prop_tween.register();

        // Rotate the sprite continuously: 0 -> 2*PI, infinite loops.
        let mut rot_tween = self_gd
            .do_rotation(std::f64::consts::PI * 2.0, 4.0)
            .begin_from(0.0);
        rot_tween.set_loops(-1, LoopMode::Restart);
        rot_tween.register();

        // Spawn a trail on every physics frame.
        let self_ref = self_gd.clone();
        let particle = repelled_particle.clone();

        let mut tree = self.base().get_tree();
        tree.connect(
            "physics_frame",
            &Callable::from_fn("spawn_trail", move |_args| {
                if !self_ref.is_instance_valid() || !particle.is_instance_valid() {
                    return Variant::nil();
                }

                let trail: Gd<Sprite2D> = particle.duplicate().unwrap().cast();
                let mut parent = self_ref.clone().upcast::<Node>();
                parent.add_child(&trail);

                let mut trail_mut = trail.clone();
                trail_mut.set_as_top_level(true);
                let pos = particle.get_global_position();
                trail_mut.set_global_position(pos);

                // Fade the trail out over 4 seconds, then free it.
                trail.do_color_a(0.0, 4.0).register();

                // Queue free after fade completes.
                let trail_for_free = trail.clone();
                trail.do_delayed_call(
                    move || {
                        if trail_for_free.is_instance_valid() {
                            let mut t = trail_for_free.clone().upcast::<Node>();
                            t.queue_free();
                        }
                    },
                    4.0,
                )
                .register();

                Variant::nil()
            }),
        );
    }
}
