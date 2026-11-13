//! Resource-authored looping text effects. Rendering always precedes the code layer.

mod compile;
mod registry;
mod reserved_tags;
mod resources;
mod sample;
mod tween;

pub(crate) use compile::CompiledTextEffect;
use godot::classes::{CharFxTransform, IRichTextEffect, Resource, RichTextEffect};
pub(super) use registry::Registry;
pub use resources::*;

use super::*;

/// Internal native renderer. The bundled GDScript subclass supplies Godot's tag variable.
#[derive(GodotClass)]
#[class(init, tool, base = RichTextEffect)]
pub struct SpireRichTextEffect {
    base: Base<RichTextEffect>,
    entries: Vec<sample::Entry>,
    #[init(val = rand::random())]
    seed: u64,
}

#[godot_api]
impl IRichTextEffect for SpireRichTextEffect {
    #[func(gd_self)]
    fn process_custom_fx(mut this: Gd<Self>, char_fx: Option<Gd<CharFxTransform>>) -> bool {
        let Some(mut fx) = char_fx else { return true };

        let state = &mut *this.bind_mut();
        let frame = Engine::singleton().get_process_frames();
        for (index, entry) in state.entries.iter_mut().enumerate() {
            entry.apply(&mut fx, frame, state.seed.wrapping_add((index as u64).wrapping_mul(0x9e3779b97f4a7c15)));
        }

        true
    }
}

#[cfg(feature = "integration-tests")]
#[godot_api]
impl SpireRichTextEffect {
    #[func(gd_self)]
    fn sample_for_test(this: Gd<Self>, elapsed: f64, index: i32, outline: bool) -> VarDictionary {
        let mut fx = CharFxTransform::new_gd();
        fx.set_elapsed_time(elapsed);
        fx.set_range(Vector2i::new(index, index + 1));
        fx.set_relative_index(index);
        fx.set_glyph_count(1);
        fx.set_outline(outline);
        fx.set_color(Color::WHITE);
        Self::process_custom_fx(this, Some(fx.clone()));
        let mut result = VarDictionary::new();
        result.set("transform", fx.get_transform());
        result.set("color", fx.get_color());
        result
    }
}
