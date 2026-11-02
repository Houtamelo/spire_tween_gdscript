#![allow(deprecated)]
use super::*;

#[allow(dead_code)]
pub mod docs;
mod ease;
mod evaluator;
mod loop_mode;
mod macros;
mod pause_mode;
mod process_mode;
mod shake_randomness_mode;
mod rotation;
mod path_text;
mod spiral;
mod state;

#[cfg(feature = "include_gdscript_bridge")]
#[allow(unused_imports)]
use gdscript_bridge::*;
#[allow(unused_imports)]
use godot::register::private::constant::*;
#[cfg(feature = "include_gdscript_bridge")]
#[allow(unused_imports)]
use godot::sys::{shard_add, shard_execute_pre_main};
use macros::*;

#[allow(unused_imports)]
pub use self::{
    ease::*,
    evaluator::*,
    loop_mode::*,
    pause_mode::*,
    process_mode::*,
    shake_randomness_mode::*,
    rotation::*,
    path_text::*,
    spiral::*,
    state::*,
};
