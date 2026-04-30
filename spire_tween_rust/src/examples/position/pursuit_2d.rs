use godot::prelude::*;
use spire_tween::prelude::*;

/// Port of `examples/position/pursuit_2d.gd`.
///
/// Demonstrates `do_follow` (2D): a chaser node continuously follows a target
/// node using a speed-based tween with easing. The target moves through a
/// sequence of corners in a loop.
#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Pursuit2D {
    base: Base<Node2D>,

    #[export]
    #[init(val = 500.0)]
    chaser_speed: f64,

    #[export]
    #[init(val = 400.0)]
    target_speed: f64,

    #[export]
    #[init(val = Vector2::ZERO)]
    bounds_min: Vector2,

    #[export]
    #[init(val = Vector2::new(1920.0, 1080.0))]
    bounds_max: Vector2,
}

#[godot_api]
impl INode2D for Pursuit2D {
    fn ready(&mut self) {
        let chaser: Gd<Node2D> = self.base().get_node_as("Chaser");
        let target: Gd<Node2D> = self.base().get_node_as("Target");

        let bounds_min = self.bounds_min;
        let bounds_max = self.bounds_max;

        let corners = [
            bounds_min,
            Vector2::new(bounds_max.x, bounds_min.y),
            bounds_max,
            Vector2::new(bounds_min.x, bounds_max.y),
        ];

        // DoNode2D.follow(chaser, target, chaser_speed).set_ease(EASE_IN_CIRC)
        chaser
            .do_follow(target.clone(), self.chaser_speed)
            .with_ease(EaseKind::Basic(Ease::InCirc))
            .register();

        // Build a sequence that moves the target through corners, looping infinitely.
        let mut sequence = SpireTween::<Sequence>::new();

        for corner in &corners {
            sequence.append(
                target.do_move(*corner, self.target_speed).as_speed_based(),
            );
        }

        sequence.set_loops(-1, LoopMode::Restart);
        sequence.register();
    }
}
