use godot::classes::PackedScene;
use godot::prelude::*;

use super::base_test_case::BaseTestCase;
use super::tween_setups::{self, Tweeners};

/// Disco-balls benchmark: tweens both global_position and modulate (color)
/// on each node simultaneously.
///
/// Port of `disco_balls.gd`.
#[derive(GodotClass)]
#[class(base = Node)]
pub struct DiscoBalls {
    base: Base<Node>,
}

#[godot_api]
impl INode for DiscoBalls {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let test_name =
            "| global-position & modulate | 16x16-circles | parallel | 1 tweener/property"
                .to_string();
        BaseTestCase::launch_default(&mut self.to_gd().upcast(), test_name, disco_balls_setup);
    }
}

/// Setup function for the disco-balls benchmark (reusing tweeners variant).
fn disco_balls_setup(root: &mut Gd<Node>, is_builtin: bool, duration: f64, amount: i64) {
    let prefab: Gd<PackedScene> = load("res://benchmarks/prefabs/circle_16x16.tscn");
    let nodes = tween_setups::spawn_nodes(&prefab, root, amount as usize);

    let mut tweeners = if is_builtin {
        Tweeners::Builtin(Vec::new())
    } else {
        Tweeners::Spire(Vec::new())
    };

    tween_setups::tween_global_positions(
        duration,
        is_builtin,
        &nodes,
        &mut tweeners,
        true,  // use_sequence
        None,
        None,
    );
    tween_setups::tween_modulates(
        duration,
        is_builtin,
        &nodes,
        &mut tweeners,
        true,  // use_sequence
        None,
        None,
    );
}
