use godot::classes::Sprite2D;
use godot::prelude::*;
use spire_tween::prelude::*;

/// Port of `examples/readme/example_common_property.gd`.
///
/// Demonstrates the most basic property tween usage:
/// animating a Sprite2D's `global_position` and `modulate:r` simultaneously.
#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct ExampleCommonProperty {
    base: Base<Node2D>,

    #[export]
    #[init(val = Vector2::new(1200.0, 300.0))]
    destination: Vector2,

    #[export]
    #[init(val = 4.0)]
    duration: f64,
}

#[godot_api]
impl INode2D for ExampleCommonProperty {
    fn ready(&mut self) {
        let destination = self.destination;
        let duration = self.duration;

        // Grab a reference to the Circle child (Sprite2D).
        let circle: Gd<Sprite2D> = self.base().get_node_as("Circle");

        // Delay 2 seconds so the tween doesn't start before we focus the game window.
        let circle_pos = circle.clone();
        let circle_mod = circle.clone();

        self.to_gd().do_delayed_call(
            move || {
                // DoNode2D.global_position(circle, destination, duration)
                circle_pos
                    .do_global_position(destination, duration)
                    .register();

                // DoCanvasItem.modulate_r(circle, 1.0, duration)
                circle_mod.do_modulate_r(1.0, duration).register();
            },
            2.0,
        )
        .register();
    }
}
