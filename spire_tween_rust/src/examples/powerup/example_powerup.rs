use godot::classes::Sprite2D;
use godot::prelude::*;
use spire_tween::prelude::*;

/// Port of `examples/powerup/example_powerup.gd`.
///
/// Demonstrates spiraling energy balls around a character, combined with
/// sprite frame animation, color fades, and scaling effects -- all
/// orchestrated through sequences.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct ExamplePowerup {
    base: Base<Node>,

    #[export]
    #[init(val = 3.8)]
    duration: f64,

    #[export]
    #[init(val = 0.0)]
    shear: f32,

    #[export]
    #[init(val = 4.0 * std::f32::consts::PI)]
    from_angle: f32,

    #[export]
    #[init(val = 0.0)]
    to_angle: f32,

    #[export]
    #[init(val = Vector2::ONE * 64.0)]
    spiral_scale: Vector2,

    #[export]
    #[init(val = 4)]
    balls_count: i32,
}

#[godot_api]
impl INode for ExamplePowerup {
    fn ready(&mut self) {
        let mut actor: Gd<Sprite2D> = self.base().get_node_as("Actor");
        actor.set_modulate(Color::from_rgba(0.5, 0.5, 0.5, 1.0));

        let frame_time = 0.2;
        let speed = 1.0 / frame_time;
        let color_speed = (3.0_f64 * 0.5 * 0.5).sqrt() * 2.0;

        let duration = self.duration;
        let from_angle = self.from_angle;
        let to_angle = self.to_angle;
        let spiral_scale = self.spiral_scale;
        let shear = self.shear;
        let balls_count = self.balls_count;

        // Spawn energy balls.
        let ball_prefab: Gd<PackedScene> = load("res://examples/powerup/ball_prefab.tscn");
        let mut balls: Vec<Gd<Sprite2D>> = Vec::new();
        for _ in 0..balls_count {
            let spawned: Gd<Sprite2D> = ball_prefab.instantiate().unwrap().cast();
            self.base_mut().add_child(&spawned);
            balls.push(spawned);
        }

        let angle_interval = 2.0 / balls.len() as f32;
        let center = actor.get_global_position();

        // Ball sequence: spiral in + fade in + scale up.
        let mut ball_seq = SpireTween::<Sequence>::new();

        for (i, ball) in balls.iter().enumerate() {
            let rotation = std::f32::consts::PI * i as f32 * angle_interval;

            // Spiral the ball around the character.
            ball_seq.join(
                ball.do_spiral(
                    center,
                    from_angle,
                    to_angle,
                    spiral_scale,
                    duration,
                    rotation,
                    shear,
                    Spiral::Fermat,
                    Vector2::ONE,
                ),
            );

            // Fade-in the energy ball.
            ball_seq.join(
                ball.do_color_a(0.8, duration).begin_from(0.0),
            );

            // Scale the ball from 0 to 0.25.
            ball_seq.join(
                ball.do_scale(Vector2::new(0.25, 0.25), duration)
                    .begin_from(Vector2::ZERO),
            );
        }

        // After spiral finishes, fade out + scale up (explosion effect).
        ball_seq.append_interval(0.0);

        for ball in &balls {
            ball_seq.join(
                ball.do_color(Color::from_rgba(1.0, 1.0, 1.0, 0.0), 2.0)
                    .with_ease(EaseKind::Basic(Ease::OutExpo)),
            );
            ball_seq.join(
                ball.do_scale(Vector2::ONE * 3.0, 2.0)
                    .with_ease(EaseKind::Basic(Ease::OutExpo)),
            );
        }

        ball_seq.register();

        // Actor sequence: frame animation + color flash.
        let mut actor_seq = SpireTween::<Sequence>::new();

        // Frame 0 -> 6 (speed-based).
        actor_seq.append(
            actor.do_frame(6, speed).begin_from(0).as_speed_based(),
        );

        // Frame 6 -> 8 yoyo (5 loops).
        let mut frame_yoyo = actor.do_frame(8, speed).as_speed_based();
        frame_yoyo.set_loops(5, LoopMode::Yoyo);
        actor_seq.append(frame_yoyo);

        // Color flash in parallel with frame yoyo.
        let mut color_flash = actor
            .do_color(Color::WHITE, color_speed)
            .as_speed_based()
            .with_ease(EaseKind::Basic(Ease::InOutSine));
        color_flash.set_loops(5, LoopMode::Yoyo);
        actor_seq.join(color_flash);

        // Frame 8 -> 9.
        actor_seq.append(
            actor.do_frame(9, speed).as_speed_based(),
        );

        // Brief pause.
        actor_seq.append_interval(frame_time);

        // Reset frame to 0.
        let actor_reset = actor.clone();
        actor_seq.append_call(Callable::from_fn("reset_frame", move |_| {
            if actor_reset.is_instance_valid() {
                let mut a = actor_reset.clone();
                a.set_frame(0);
            }
            Variant::nil()
        }));

        actor_seq.register();
    }
}
