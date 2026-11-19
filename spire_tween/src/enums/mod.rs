#![allow(deprecated)]
use super::*;

mod blueprint_kind;
#[allow(dead_code)]
pub mod docs;
mod ease;
mod evaluator;
mod lerp_mode_kind;
mod loop_mode;
mod macros;
mod path_text;
mod pause_mode;
mod process_mode;
mod rotation;
mod shake_randomness_mode;
mod spiral;
mod state;
mod text_effect_kind;
mod trigger_condition;

#[cfg(feature = "include_gdscript_bridge")]
#[allow(unused_imports)]
use gdscript_bridge::*;
#[allow(unused_imports)]
use godot::register::private::constant::*;
#[cfg(feature = "include_gdscript_bridge")]
#[allow(unused_imports)]
use godot::sys::{shard_add, shard_execute_pre_main};
pub(crate) use macros::register_enum;

#[allow(unused_imports)]
pub use self::{
    blueprint_kind::*,
    ease::*,
    evaluator::*,
    lerp_mode_kind::*,
    loop_mode::*,
    path_text::*,
    pause_mode::*,
    process_mode::*,
    rotation::*,
    shake_randomness_mode::*,
    spiral::*,
    state::*,
    text_effect_kind::*,
    trigger_condition::*,
};
