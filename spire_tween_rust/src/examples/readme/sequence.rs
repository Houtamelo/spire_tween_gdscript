use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;
use spire_tween::prelude::*;

/// Port of `examples/readme/example_sequence.gd`.
///
/// Demonstrates sequences: the sprite visits four vertices in order,
/// flashing red at each vertex. The sequence loops infinitely.
#[derive(GodotClass)]
#[class(init, base = Sprite2D)]
pub struct ExampleSequence {
    base: Base<Sprite2D>,

    #[var]
    #[init(val = 200.0)]
    speed: f64,

    #[var]
    #[init(val = 0.5)]
    flash_duration: f64,
}

#[godot_api]
impl ISprite2D for ExampleSequence {
    fn ready(&mut self) {
        let vertices = [
            Vector2::new(100.0, 100.0),
            Vector2::new(500.0, 100.0),
            Vector2::new(500.0, 400.0),
            Vector2::new(100.0, 400.0),
        ];

        let speed = self.speed;
        let flash_duration = self.flash_duration;

        // Start at the last vertex.
        self.base_mut()
            .set_global_position(vertices[3]);

        let mut seq = SpireTween::<Sequence>::new();
        seq.set_loops(-1, LoopMode::Restart);

        let self_gd: Gd<Sprite2D> = self.to_gd().upcast();

        for vert in &vertices {
            // `append` creates a new step: move to the vertex (speed-based).
            seq.append(
                self_gd
                    .do_move(*vert, speed)
                    .as_speed_based(),
            );

            // `join` makes the color flash play in parallel with the movement.
            // yoyo loop (2 loops) goes red then back to original.
            let mut flash_tween = self_gd.do_modulate(Color::RED, flash_duration);
            flash_tween.set_loops(2, LoopMode::Yoyo);
            seq.join(flash_tween);
        }

        seq.register();
    }
}
