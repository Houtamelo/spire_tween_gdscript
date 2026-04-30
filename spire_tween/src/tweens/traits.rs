#[allow(unused_imports)]
use super::*;

/// Uniform interface for controlling tween playback.
pub trait SpireTweener {
    /// If stopped, resets counters before playing.
    fn play(&mut self);
    /// No effect if already stopped.
    fn pause(&mut self);
    /// Calling `play()` on a stopped tween restarts from the beginning.
    fn stop(&mut self);
    fn process(&mut self, delta_time: f64, is_tree_paused: bool) -> AdvanceTimeResult;
    /// Jumps to final state, emits `finished` signal if a GD handle is attached.
    fn force_complete(&mut self);
}

/// Bridges a native `RcPtr<SpireTween<T>>` to its GDScript wrapper type.
pub trait GdFromNativeTween: GodotClass {
    type Inner: ITweenable;
    fn gd_from_native_tween(tween: RcPtr<SpireTween<Self::Inner>>) -> Gd<Self>;
}

pub fn gd_from_native_tween<T: GdFromNativeTween>(tween: RcPtr<SpireTween<T::Inner>>) -> Gd<T> {
    T::gd_from_native_tween(tween)
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
    /// `excess_time` is leftover time beyond the tween's duration,
    /// used by parent sequences to feed into the next tween.
    Completed { excess_time: f64 },
    /// Owner was freed or tween otherwise needs removal.
    ShouldDespawn,
}
