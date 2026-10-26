use super::*;
mod adapters;
mod motion;
mod color;
mod counter;
mod effect;
mod punch;
mod shake_variants;
mod orientation;
mod ui_normalized;
mod spiral_3d;
pub use self::{punch::*, shake_variants::*, orientation::*, ui_normalized::*, spiral_3d::*};
pub use self::{adapters::*, motion::*, color::*, counter::*};
// TODO: Check if integration tests cover all these.

#[allow(dead_code)]
mod do_bone;
#[allow(dead_code)]
mod do_contour_shape;
mod do_ellipsis;
#[allow(dead_code)]
mod do_ellipsis_3d;
mod do_follow;
mod do_follow_3d;
#[allow(dead_code)]
mod do_shake;
mod shake_path;
mod do_spiral;

#[allow(unused_imports)]
pub use self::{
    do_bone::*,
    do_contour_shape::*,
    do_ellipsis::*,
    do_ellipsis_3d::*,
    do_follow::*,
    do_follow_3d::*,
    do_shake::{DoShakeControl, DoShakeNode2D},
    do_spiral::*,
};

#[cfg(feature = "integration-tests")]
#[doc(hidden)]
pub use self::do_shake::do_shake_internal;
