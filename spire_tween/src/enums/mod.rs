#![allow(deprecated)]

pub mod docs;
mod ease;
mod evaluator;
//mod flags;
mod loop_mode;
mod macros;
mod pause_mode;
mod process_mode;
mod state;

pub use ease::*;
pub use evaluator::*;
//pub use flags::*;
#[allow(unused_imports)]
use gdscript_bridge::*;
#[allow(unused_imports)]
use godot::register::private::constant::*;
#[allow(unused_imports)]
use godot::sys::{plugin_add, plugin_execute_pre_main};
pub use loop_mode::*;
use macros::*;
pub use pause_mode::*;
pub use process_mode::*;
pub use state::*;

use super::*;
