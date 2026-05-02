#[allow(unused_imports)]
use super::*;

/// Uniform playback interface implemented by every concrete tween type
/// (`SpireTween<LerpPropertyData<T>>`, `SpireTween<LerpMethodData<T>>`,
/// `SpireTween<Sequence>`, `SpireTween<Callable>`).
///
/// Routed through [`AnyTween`] so the global `TweenManager` can drive any tween
/// without knowing its concrete type. You usually don't call these methods directly
/// on a `SpireTween<T>` — use the same-named methods on the `Gd<Spire…>` handle
/// (which delegate here), or just rely on the manager to tick the tween for you.
pub trait SpireTweener {
    /// Resumes a paused tween, or starts a stopped one from the beginning.
    /// No-op on an already-playing tween.
    ///
    /// **Inside sequences:** sequences control their children's playback — calling
    /// this on a child tween of a sequence is not recommended.
    fn play(&mut self);
    /// Pauses a playing tween — it stops processing until [`play`](Self::play)
    /// resumes it. No-op on an already-paused or stopped tween.
    fn pause(&mut self);
    /// Stops a tween and resets its counters (`total_elapsed_time`, `loop_time`,
    /// `loop_counter`). The next [`play`](Self::play) will restart from frame 0.
    ///
    /// Connections registered with [`SpireFlags::ONE_SHOT`] that have already fired
    /// are *not* restored.
    fn stop(&mut self);
    /// Advances the tween by `delta_time` seconds. Called by the global
    /// `TweenManager` each frame; you only call it directly when implementing a
    /// custom processing loop or when [`ProcessMode::Manual`] is in effect.
    /// `is_tree_paused` should reflect `SceneTree::is_paused()`.
    fn process(&mut self, delta_time: f64, is_tree_paused: bool) -> AdvanceTimeResult;
    /// Jumps to the final state — sets the property to its end value, fires the
    /// `finished` event, transitions to [`State::Stopped`]. Skips any remaining
    /// loops without firing `loop_finished` for them.
    ///
    /// No-op on a stopped tween.
    fn force_complete(&mut self);
}

/// Glue between a native [`RcPtr<SpireTween<T>>`] and its `Gd<Spire…>` wrapper. An
/// implementation detail — implementations are emitted by `define_base_gd_methods!`
/// inside the macros that build the wrapper types. Pure-Rust users won't call this.
pub trait GdFromNativeTween: GodotClass {
    type Inner: ITweenable;
    fn gd_from_native_tween(tween: RcPtr<SpireTween<Self::Inner>>) -> Gd<Self>;
}

/// Free-function entry point to [`GdFromNativeTween::gd_from_native_tween`]. Used
/// internally by the bridge to wrap a native tween in its GDScript-facing handle.
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
