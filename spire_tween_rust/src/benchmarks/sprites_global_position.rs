use godot::classes::PackedScene;
use godot::prelude::*;

use super::base_test_case::BaseTestCase;
use super::tween_setups::{self, Tweeners};

/// Sprites global-position benchmark: tweens only global_position on many
/// small black sprites.
///
/// Port of `sprites_global_position.gd`.
#[derive(GodotClass)]
#[class(base = Node)]
pub struct SpritesGlobalPosition {
    base: Base<Node>,
}

#[godot_api]
impl INode for SpritesGlobalPosition {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let test_name = "tween-global-position_black-4x4-dots".to_string();
        BaseTestCase::launch_default(&mut self.to_gd().upcast(), test_name, sprites_global_position_setup);
    }
}

/// Setup function for the sprites global-position benchmark.
///
/// The GDScript original passes an empty array `[]` as tweeners, which means:
/// - For builtin: `ensure_builtin_tweeners` populates new Tweens per node.
/// - For spire: `use_sequence` defaults to `false`, so standalone tweens are
///   created directly (tweeners array is unused).
fn sprites_global_position_setup(
    root: &mut Gd<Node>,
    is_builtin: bool,
    duration: f64,
    amount: i64,
) {
    let prefab: Gd<PackedScene> = load("res://benchmarks/prefabs/square_4x4.tscn");
    let nodes = tween_setups::spawn_nodes(&prefab, root, amount as usize);

    // Set all nodes to black
    for node in &nodes {
        let mut canvas_item = node.clone().upcast::<godot::classes::CanvasItem>();
        canvas_item.set_modulate(Color::from_rgb(0.0, 0.0, 0.0));
    }

    // The GDScript passes an empty `[]`, which in the builtin path gets
    // populated by `ensure_builtin_tweeners`. For spire, use_sequence=false
    // so standalone tweens are created and the tweeners vec is not used.
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
        false, // use_sequence
        None,
        None,
    );
}
