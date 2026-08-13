use godot::{
    classes::{ISprite2D, Sprite2D},
    prelude::*,
};
use spire_tween::prelude::*;

/// Port of `examples/screen_saver/collision_fx.gd`.
///
/// A collision effect Sprite2D that scales up from zero while fading out,
/// then frees itself when the animation completes. Bound to a sequence so
/// the node is automatically freed.
#[derive(GodotClass)]
#[class(init, base = Sprite2D)]
pub struct CollisionFx {
    base: Base<Sprite2D>,

    #[export]
    #[init(val = 2.0)]
    duration: f64,

    #[export]
    #[init(val = 3.0)]
    max_scale: f64,

    // In GDScript this was `@export var ease_mode: Spire.Ease = Spire.EASE_IN_EXPO`.
    // Storing the ease variant ID as a simple integer for export purposes.
    #[var]
    #[init(val = 0)]
    ease_mode_id: i64,
}

#[godot_api]
impl ISprite2D for CollisionFx {
    fn ready(&mut self) {
        let duration = self.duration;
        let max_scale = self.max_scale;
        let ease = EaseKind::Basic(Ease::InExpo);
        let self_gd: Gd<Sprite2D> = self.to_gd().upcast();

        let mut seq = SpireTween::<Sequence>::new().bound_to(self_gd.clone().upcast());

        // Scale up from (0,0) to (max_scale, max_scale).
        seq.append(
            self_gd
                .do_scale(Vector2::new(max_scale as f32, max_scale as f32), duration)
                .begin_from(Vector2::ZERO)
                .with_ease(ease.clone()),
        );

        // In parallel: fade color from a random color to transparent.
        let rand_color = Color::from_rgba(
            godot::global::randf() as f32,
            godot::global::randf() as f32,
            godot::global::randf() as f32,
            1.0,
        );
        seq.join(
            self_gd
                .do_color(Color::from_rgba(1.0, 1.0, 1.0, 0.0), duration)
                .begin_from(rand_color)
                .with_ease(ease),
        );

        // Queue free when the sequence finishes.
        let self_for_free = self_gd.clone();
        seq.append_call(Callable::from_fn("queue_free", move |_| {
            if self_for_free.is_instance_valid() {
                let mut node: Gd<Node> = self_for_free.clone().upcast();
                node.queue_free();
            }
            Variant::nil()
        }));

        seq.register();
    }
}
