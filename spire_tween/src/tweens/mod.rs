pub use any_tween::*;
pub use base::*;
pub use bound_tweens::*;
pub use cfg::*;
pub use delayed_call::*;
use godot::builtin::math::ApproxEq;
pub use handle::*;
pub use lerping::*;
pub(crate) use macros::*;
pub use method::*;
pub use property::*;
pub use sequence::*;
pub use traits::*;
pub use tween_callable::*;

use super::*;

mod any_tween;
mod base;
mod bound_tweens;
mod cfg;
mod delayed_call;
mod handle;
mod lerping;
mod macros;
mod method;
mod property;
mod sequence;
mod traits;
mod tween_callable;

pub(crate) fn ratio_with_delay_duration(delay: f64, duration: f64, elapsed_time: f64) -> f64 {
    if duration.approx_eq(&0.0) {
        godot_warn!("Attempted division by zero, returning 1.0.");
        return 1.;
    }

    f64::max((elapsed_time - delay) / duration, 0.)
}
