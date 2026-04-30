use godot::classes::Sprite2D;
use godot::prelude::*;
use spire_tween::prelude::*;

/// Port of `examples/readme/example_method.gd`.
///
/// Demonstrates method tweens: interpolating a `Vector2i` from → to over
/// a duration, calling `create_dot` each tick with the interpolated value.
/// Uses physics process mode.
#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct ExampleMethod {
    base: Base<Node2D>,

    #[export]
    #[init(val = Vector2i::new(100, 100))]
    from: Vector2i,

    #[export]
    #[init(val = Vector2i::new(900, 600))]
    to: Vector2i,

    #[var]
    #[init(val = 10.0)]
    duration: f64,
}

#[godot_api]
impl INode2D for ExampleMethod {
    fn ready(&mut self) {
        let from = self.from;
        let to = self.to;
        let duration = self.duration;
        let self_gd = self.to_gd();

        // Delay 2 seconds, then start the method tween.
        self.to_gd().do_delayed_call(
            move || {
                // Spire.do_call_vector2i(create_dot, from, to, duration)
                //   → obj.do_method::<Vector2i, _>("create_dot", from, to, duration)
                self_gd
                    .do_method("create_dot", from, to, duration)
                    .with_process_mode(ProcessMode::Physics)
                    .register();
            },
            2.0,
        )
        .register();
    }
}

#[godot_api]
impl ExampleMethod {
    #[func]
    fn create_dot(&mut self, at: Vector2i) {
        let dot_scene: Gd<PackedScene> = load("res://examples/readme/dot_prefab.tscn");
        let mut dot: Gd<Sprite2D> = dot_scene.instantiate().unwrap().cast();
        self.base_mut().add_child(&dot);
        dot.set_global_position(Vector2::new(at.x as f32, at.y as f32));
    }
}
