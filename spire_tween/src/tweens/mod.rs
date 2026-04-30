use super::*;

mod any_tween;
mod base;
mod bound_tweens;
mod delayed_call;
mod handle_enum;
mod lerping;
mod macros;
mod method;
mod property;
mod sequence;
mod templates;
mod traits;
mod tweenables;

use godot::builtin::math::ApproxEq;
pub(crate) use macros::*;

#[allow(unused_imports)]
pub use self::{
    any_tween::*,
    base::*,
    bound_tweens::*,
    delayed_call::*,
    handle_enum::*,
    lerping::*,
    method::*,
    property::*,
    sequence::*,
    templates::*,
    traits::*,
    tweenables::*,
};

/// Returns position in current animation, normally ranging from 0.0 to 1.0.
/// However, values outside this range are possible if tween is in "Incremental" loop mode,
/// or if the easing function produces such values.
pub(crate) fn calc_animation_position(
    duration: f64,
    loop_time: f64,
    loop_count: i64,
    loop_mode: LoopMode,
    ease: &EaseKind,
) -> f64 {
    if duration.approx_eq(&0.0) {
        godot_error!("Duration cannot be zero, returning animation position `1.0`.");
        return 1.;
    }

    let raw_position = if loop_time >= duration { 1. } else { loop_time / duration };

    match loop_mode {
        LoopMode::Restart => ease.sample(raw_position),
        LoopMode::Yoyo => {
            if loop_count % 2 == 0 {
                ease.sample(raw_position)
            } else {
                ease.sample(1. - raw_position)
            }
        }
        LoopMode::Incremental => loop_count as f64 + ease.sample(raw_position),
    }
}
