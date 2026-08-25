use godot::{
    classes::{PackedScene, RandomNumberGenerator, Tween},
    prelude::*,
};
use spire_tween::prelude::*;

/// Instantiate `amount` nodes from a packed scene and add them as children of `root`.
pub fn spawn_nodes(prefab: &Gd<PackedScene>, root: &mut Gd<Node>, amount: usize) -> Vec<Gd<Node2D>> {
    let mut nodes = Vec::with_capacity(amount);
    for _ in 0..amount {
        let node: Gd<Node2D> = prefab.instantiate().unwrap().cast();
        root.add_child(&node);
        nodes.push(node);
    }
    nodes
}

/// Generate `amount` random `Vector2` positions in [0, 1920] x [0, 1080].
pub fn generate_random_positions(amount: usize) -> Vec<Vector2> {
    let mut rng = RandomNumberGenerator::new_gd();
    (0..amount)
        .map(|_| Vector2::new(rng.randf_range(0.0, 1920.0) as f32, rng.randf_range(0.0, 1080.0) as f32))
        .collect()
}

/// Generate `amount` random `Color` values with each channel in [0, 1].
pub fn generate_random_colors(amount: usize) -> Vec<Color> {
    let mut rng = RandomNumberGenerator::new_gd();
    (0..amount)
        .map(|_| Color::from_rgba(rng.randf() as f32, rng.randf() as f32, rng.randf() as f32, rng.randf() as f32))
        .collect()
}

/// Ensure the `tweeners` vec has at least as many Godot built-in Tween entries as `nodes`.
/// Missing entries are created via `Node::create_tween().set_parallel(true)`.
fn ensure_builtin_tweeners(nodes: &[Gd<Node2D>], tweeners: &mut Vec<Gd<Tween>>) {
    let tweeners_len = tweeners.len();
    let nodes_len = nodes.len();
    if tweeners_len >= nodes_len {
        return;
    }
    for i in tweeners_len..nodes_len {
        let tween = nodes[i].clone().upcast::<Node>().create_tween().set_parallel();
        tweeners.push(tween);
    }
}

/// Ensure the `tweeners` vec has at least as many Spire sequences as `nodes`.
fn ensure_spire_tweeners(_nodes: &[Gd<Node2D>], tweeners: &mut Vec<SpireTween<Sequence>>) {
    let tweeners_len = tweeners.len();
    let nodes_len = _nodes.len();
    if tweeners_len >= nodes_len {
        return;
    }
    for _ in tweeners_len..nodes_len {
        tweeners.push(SpireTween::<Sequence>::new());
    }
}

/// Tween types for the `tweeners_untyped` parameter.
pub enum Tweeners {
    Builtin(Vec<Gd<Tween>>),
    Spire(Vec<SpireTween<Sequence>>),
    None,
}

/// Tween global_position on each node.
///
/// Mirrors `tween_setups.gd::tween_global_positions`.
///
/// - `is_builtin == true`: uses Godot's built-in Tween with `tween_property`.
/// - `is_builtin == false, use_sequence == true`: uses Spire sequences with `join`.
/// - `is_builtin == false, use_sequence == false`: standalone Spire tweens.
pub fn tween_global_positions(
    duration: f64,
    is_builtin: bool,
    nodes: &[Gd<Node2D>],
    tweeners: &mut Tweeners,
    use_sequence: bool,
    froms: Option<Vec<Vector2>>,
    tos: Option<Vec<Vector2>>,
) {
    let amount = nodes.len();
    let froms = froms.unwrap_or_else(|| generate_random_positions(amount));
    let tos = tos.unwrap_or_else(|| generate_random_positions(amount));

    if is_builtin {
        let tweener_vec = match tweeners {
            Tweeners::Builtin(v) => v,
            _ => return,
        };
        ensure_builtin_tweeners(nodes, tweener_vec);
        for i in 0..froms.len() {
            tweener_vec[i]
                .tween_property(&nodes[i], "global_position", &tos[i].to_variant(), duration)
                .from(&froms[i].to_variant());
        }
    } else if use_sequence {
        let tweener_vec = match tweeners {
            Tweeners::Spire(v) => v,
            _ => return,
        };
        ensure_spire_tweeners(nodes, tweener_vec);
        for i in 0..froms.len() {
            let tween = nodes[i].do_global_position(tos[i], duration).begin_from(froms[i]);
            tweener_vec[i].join(tween);
        }
    } else {
        for i in 0..froms.len() {
            nodes[i]
                .do_global_position(tos[i], duration)
                .begin_from(froms[i])
                .register();
        }
    }
}

/// Tween modulate (color) on each node.
///
/// Mirrors `tween_setups.gd::tween_modulates`.
///
/// - `is_builtin == true`: uses Godot's built-in Tween with `tween_property` on `"modulate"`.
/// - `is_builtin == false, use_sequence == true`: uses Spire sequences with `join` + `do_color`.
/// - `is_builtin == false, use_sequence == false`: standalone Spire `do_color` tweens.
pub fn tween_modulates(
    duration: f64,
    is_builtin: bool,
    nodes: &[Gd<Node2D>],
    tweeners: &mut Tweeners,
    use_sequence: bool,
    froms: Option<Vec<Color>>,
    tos: Option<Vec<Color>>,
) {
    let amount = nodes.len();
    let froms = froms.unwrap_or_else(|| generate_random_colors(amount));
    let tos = tos.unwrap_or_else(|| generate_random_colors(amount));

    if is_builtin {
        let tweener_vec = match tweeners {
            Tweeners::Builtin(v) => v,
            _ => return,
        };
        ensure_builtin_tweeners(nodes, tweener_vec);
        for i in 0..froms.len() {
            tweener_vec[i]
                .tween_property(&nodes[i], "modulate", &tos[i].to_variant(), duration)
                .from(&froms[i].to_variant());
        }
    } else if use_sequence {
        let tweener_vec = match tweeners {
            Tweeners::Spire(v) => v,
            _ => return,
        };
        ensure_spire_tweeners(nodes, tweener_vec);
        for i in 0..froms.len() {
            let tween = nodes[i].do_color(tos[i], duration).begin_from(froms[i]);
            tweener_vec[i].join(tween);
        }
    } else {
        for i in 0..froms.len() {
            nodes[i].do_color(tos[i], duration).begin_from(froms[i]).register();
        }
    }
}
