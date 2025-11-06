#[allow(unused_imports)]
use super::*;

pub trait SpireTweener {
    fn play(&mut self);
    fn pause(&mut self);
    fn stop(&mut self);
    fn process(&mut self, delta_time: f64, is_tree_paused: bool) -> AdvanceTimeResult;
    fn force_complete(&mut self);
}

#[allow(unused)]
#[doc(hidden)]
pub trait InnerTypeName {
    fn inner_type_name(&self) -> &'static str;
}

#[must_use]
#[derive(Debug)]
pub enum AdvanceTimeResult {
    Playing,
    Paused,
    Completed { excess_time: f64 },
    ShouldDespawn,
}
