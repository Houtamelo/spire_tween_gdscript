#![feature(arbitrary_self_types)]

use godot::prelude::*;

mod benchmarks;
mod examples;
mod tests;

struct SpireTweenRustExamples;

#[gdextension]
unsafe impl ExtensionLibrary for SpireTweenRustExamples {}
