use super::*;

mod any_tween;
mod base;
mod bound_tweens;
mod delayed_call;
mod handle;
mod handle_enum;
mod lerping;
mod macros;
mod method;
mod property;
mod sequence;
mod traits;
mod tweenables;

pub use any_tween::*;
pub use base::*;
pub use bound_tweens::*;
pub use delayed_call::*;
use godot::builtin::math::ApproxEq;
#[allow(unused_imports)]
pub use handle::*;
pub use handle_enum::*;
pub use lerping::*;
pub(crate) use macros::*;
pub use method::*;
pub use property::*;
pub use sequence::*;
pub use traits::*;
pub use tweenables::*;

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

    let raw_position = if loop_time >= duration {
        1.
    } else {
        loop_time / duration
    };

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
