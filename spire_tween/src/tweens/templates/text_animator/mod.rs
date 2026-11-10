//! Per-character animation owned by a RichTextEffect and updated by ordinary tweens.

mod gd;
pub mod presets;
mod render;
#[cfg(feature = "integration-tests")]
mod testing;
mod tween;

use godot::classes::IRichTextLabel;
pub use render::SpireCharRenderer;
pub use tween::SpireDoRichTextCharacters;

pub use self::gd::*;
use super::{
    effect::*,
    punch::{clean_vec2, clean_vec3},
    *,
};

#[derive(Clone, Copy)]
enum Channel {
    Offset,
    Rotation,
    Scale,
}

fn clean_color(value: Color) -> Color {
    Color::from_rgba(
        finite(value.r as f64, 1.0, "red") as f32,
        finite(value.g as f64, 1.0, "green") as f32,
        finite(value.b as f64, 1.0, "blue") as f32,
        unit(value.a as f64, 1.0, "alpha") as f32,
    )
}

#[derive(Clone, Copy)]
struct Value<T: Copy> {
    value: T,
    stamp: u64,
}
impl<T: Copy> Value<T> {
    fn new(value: T) -> Self { Self { value, stamp: 0 } }
    fn set(&mut self, value: T, stamp: u64) { *self = Self { value, stamp }; }
    fn merge(&mut self, other: Self) {
        if other.stamp > self.stamp {
            *self = other;
        }
    }
}

#[derive(Clone, Copy)]
struct CharState {
    cluster: usize,
    offset: Value<Vector2>,
    rotation: Value<Vector3>,
    scale: Value<Vector2>,
    color: Value<Option<Color>>,
    alpha: Value<Option<f64>>,
    pivot: Value<Vector2>,
    baseline_color: Color,
    pen_origin: Option<(u64, Vector2)>,
}

impl CharState {
    fn new(cluster: usize, baseline_color: Color) -> Self {
        Self {
            cluster,
            offset: Value::new(Vector2::ZERO),
            rotation: Value::new(Vector3::ZERO),
            scale: Value::new(Vector2::ONE),
            color: Value::new(None),
            alpha: Value::new(None),
            pivot: Value::new(Vector2::ZERO),
            baseline_color,
            pen_origin: None,
        }
    }
    fn effective_color(&self) -> Color {
        let mut value = self.color.value.unwrap_or(self.baseline_color);
        if let Some(alpha) = self.alpha.value {
            value.a = alpha as f32;
        }
        value
    }
    fn set_color(&mut self, value: Color, stamp: u64) {
        self.color.set(Some(value), stamp);
        self.alpha.set(Some(value.a as f64), stamp);
    }
    fn merge(&mut self, other: Self) {
        self.offset.merge(other.offset);
        self.rotation.merge(other.rotation);
        self.scale.merge(other.scale);
        self.color.merge(other.color);
        self.alpha.merge(other.alpha);
        self.pivot.merge(other.pivot);
    }
}
