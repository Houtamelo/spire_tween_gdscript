use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;
use spire_tween::prelude::*;

const FADE_DURATION: f64 = 8.0;
const MIN_DISTANCE: f32 = 2.0;

/// Port of `trail_spawner.gd`.
/// Spawns fading trail copies when the ball moves more than `MIN_DISTANCE`
/// from the last spawn position.
#[derive(GodotClass)]
#[class(init, base = Sprite2D)]
pub struct TrailSpawner {
    base: Base<Sprite2D>,
    last_spawn_pos: Vector2,
    initialized: bool,
}

#[godot_api]
impl ISprite2D for TrailSpawner {
    fn process(&mut self, _delta: f64) {
        let pos = self.base().get_global_position();

        if !self.initialized {
            self.last_spawn_pos = pos;
            self.initialized = true;
            return;
        }

        if pos.distance_to(self.last_spawn_pos) < MIN_DISTANCE {
            return;
        }
        self.last_spawn_pos = pos;

        let mut copy = Sprite2D::new_alloc();
        copy.set_texture(&self.base().get_texture().unwrap());
        copy.set_modulate(self.base().get_modulate());
        copy.set_global_position(pos);

        if let Some(mut parent) = self.base().get_parent() {
            parent.add_child(&copy);
        }

        let copy_node: Gd<Node2D> = copy.clone().upcast();
        let mut tween = copy_node.do_color_a(0.0, FADE_DURATION);
        tween.set_begin_value(0.75);
        let handle = tween.register();

        let mut copy_for_free = copy;
        handle.to_mut().gd_handle.as_ref().unwrap()
            .signals()
            .finished()
            .connect(move || {
                copy_for_free.queue_free();
            });
    }
}
