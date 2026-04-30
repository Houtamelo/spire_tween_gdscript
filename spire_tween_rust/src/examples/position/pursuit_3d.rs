use godot::prelude::*;
use spire_tween::prelude::*;

/// Port of `examples/position/pursuit_3d.gd`.
///
/// Demonstrates `do_follow` (3D): a chaser node continuously follows a target
/// in 3D space. The target moves through 8 corners of a bounding box. A marker
/// is spawned at each corner when the target arrives.
#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct Pursuit3D {
    base: Base<Node3D>,

    #[export]
    #[init(val = 500.0)]
    chaser_speed: f64,

    #[export]
    #[init(val = 400.0)]
    target_speed: f64,

    #[export]
    #[init(val = Vector3::ZERO)]
    bounds_min: Vector3,

    #[export]
    #[init(val = Vector3::new(1920.0, 1080.0, 1080.0))]
    bounds_max: Vector3,
}

#[godot_api]
impl INode3D for Pursuit3D {
    fn ready(&mut self) {
        let chaser: Gd<Node3D> = self.base().get_node_as("Chaser");
        let target: Gd<Node3D> = self.base().get_node_as("Target");

        let bmin = self.bounds_min;
        let bmax = self.bounds_max;

        let corners = [
            bmin,
            Vector3::new(bmin.x, bmin.y, bmax.z),
            Vector3::new(bmin.x, bmax.y, bmax.z),
            bmax,
            Vector3::new(bmax.x, bmin.y, bmax.z),
            Vector3::new(bmax.x, bmin.y, bmin.z),
            Vector3::new(bmax.x, bmax.y, bmin.z),
            Vector3::new(bmin.x, bmax.y, bmin.z),
        ];

        // DoNode3D.follow(chaser, target, chaser_speed).set_ease(EASE_IN_CIRC)
        chaser
            .do_follow(target.clone(), self.chaser_speed)
            .with_ease(EaseKind::Basic(Ease::InCirc))
            .register();

        // Build a sequence that moves the target through corners.
        let mut sequence = SpireTween::<Sequence>::new();
        let self_gd = self.to_gd();

        for corner in &corners {
            let tween = target.do_move(*corner, self.target_speed).as_speed_based();

            // Spawn a marker when the target arrives at each corner.
            let corner_pos = *corner;
            let self_ref = self_gd.clone();

            // We use append_call after append to spawn the marker when a block finishes.
            sequence.append(tween);

            let spawn_callable = Callable::from_fn("spawn_marker", move |_args| {
                if !self_ref.is_instance_valid() {
                    return Variant::nil();
                }

                let prefab: Gd<PackedScene> = load("res://examples/position/marker.tscn");
                let mut instance: Gd<Node3D> = prefab.instantiate().unwrap().cast();
                let mut parent: Gd<Node> = self_ref.clone().upcast();
                parent.add_child(&instance);
                instance.set_global_position(corner_pos);
                Variant::nil()
            });

            // Insert a call at the end of this block.
            sequence.join_call(spawn_callable);
        }

        sequence.set_loops(-1, LoopMode::Restart);
        sequence.register();
    }
}
