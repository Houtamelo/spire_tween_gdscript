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
mod spiral;
mod state;

#[cfg(feature = "standalone")]
#[allow(unused_imports)]
use gdscript_bridge::*;
#[allow(unused_imports)]
use godot::register::private::constant::*;
#[cfg(feature = "standalone")]
#[allow(unused_imports)]
use godot::sys::{plugin_add, plugin_execute_pre_main};
use macros::*;

#[allow(unused_imports)]
pub use self::{ease::*, evaluator::*, loop_mode::*, pause_mode::*, process_mode::*, spiral::*, state::*};
