use super::*;
mod adapters;
mod color;
mod counter;
mod effect;
mod margins;
mod motion;
mod orientation;
mod punch;
mod shake_variants;
mod spiral_3d;
mod text_animator;
mod ui_normalized;
mod waypoint_path;
pub(crate) use text_animator::resolve_char_start_delays;
pub use text_animator::{
    CharTweenData,
    SpireCharRenderer,
    SpireCharTween,
    SpireDoRichTextCharacters,
    SpireRichTextLabel,
    presets::*,
};

pub use self::{
    adapters::*,
    color::*,
    counter::*,
    margins::*,
    motion::*,
    orientation::*,
    punch::*,
    shake_variants::*,
    spiral_3d::*,
    ui_normalized::*,
    waypoint_path::*,
};
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
mod do_spiral;
mod shake_path;

#[cfg(feature = "integration-tests")]
#[doc(hidden)]
pub use self::do_shake::do_shake_internal;
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
