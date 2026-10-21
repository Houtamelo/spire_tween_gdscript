#[allow(unused_imports)]
use super::*;

/// Type-specific hooks used by the common [`SpireTween`] playback pipeline.
/// Implemented by every concrete tween type
/// (`SpireTween<LerpPropertyData<T>>`, `SpireTween<LerpMethodData<T>>`,
/// `SpireTween<Sequence>`, `SpireTween<Callable>`).
///
/// Call [`SpireTween::play`], [`SpireTween::pause`], [`SpireTween::stop`], and
/// [`SpireTween::force_complete`] (also delegated by [`AnyTween`]) for state guards
/// and lifecycle events. These hooks do not update state or emit terminal events.
/// Use [`TweenPtr::custom_step`] for checked stepping in managers, sequences, and
/// manual processors. [`SpireTween::process`] handles the internal result uniformly.
pub trait SpireTweener {
    /// Internal polymorphic callback for [SpireTween::play].
    ///
    /// Called before the state changes. Restarts clocks and type-specific state
    /// when previously stopped; resuming from paused preserves animation position.
    #[doc(hidden)]
    fn play_internal(&mut self);

    /// Internal polymorphic callback for [SpireTween::pause].
    ///
    /// Called only from playing, before the state changes.
    #[doc(hidden)]
    fn pause_internal(&mut self);

    /// Internal polymorphic callback for [SpireTween::stop].
    ///
    /// Called only from playing or paused, before state change and stop events.
    /// Performs type-specific cleanup, including stopping sequence descendants.
    #[doc(hidden)]
    fn stop_internal(&mut self);

    /// Internal polymorphic callback for [SpireTween::process].
    ///
    /// Reports animation advancement or invalidation. [`SpireTween::process`]
    /// translates terminal results into lifecycle events; this hook does not.
    #[doc(hidden)]
    fn process_internal(&mut self, delta_time: f64, is_tree_paused: bool) -> AdvanceTimeResult;

    /// Internal polymorphic callback for [SpireTween::force_complete].
    ///
    /// Applies the final value or completes sequence contents without emitting
    /// this tween's terminal events. Called only while playing or paused.
    ///
    /// Returns `true` if completion was successful; `false` otherwise.
    #[doc(hidden)]
    #[must_use]
    fn force_complete_internal(&mut self) -> bool;
}

/// Glue between a native [`TweenPtr<T>`] and its `Gd<Spire…>` wrapper. An
/// implementation detail — implementations are emitted by `define_base_gd_methods!`
/// inside the macros that build the wrapper types. Pure-Rust users won't call this.
pub trait GdFromNativeTween: GodotClass {
    type Inner: ITweenable;
    fn gd_from_native_tween(tween: TweenPtr<Self::Inner>) -> Gd<Self>;
}

/// Free-function entry point to [`GdFromNativeTween::gd_from_native_tween`]. Used
/// internally by the bridge to wrap a native tween in its GDScript-facing handle.
pub fn gd_from_native_tween<T: GdFromNativeTween>(tween: TweenPtr<T::Inner>) -> Gd<T> { T::gd_from_native_tween(tween) }

#[allow(unused)]
#[doc(hidden)]
pub trait InnerTypeName {
    fn inner_type_name(&self) -> &'static str;
}

/// Type-specific outcome of a playing tween's internal update.
///
/// Returned inside [`TweenStepResult::Stepped`] by [`TweenPtr::custom_step`].
/// Explicit binding restrictions and existing paused/stopped states have their
/// own [`TweenStepResult`] variants instead. Container retention is a separate
/// decision: a sequence keeps a valid completed child for loops and replay.
#[must_use]
#[derive(Debug)]
pub enum AdvanceTimeResult {
    /// The tween remains active, including while waiting for its initial delay.
    /// There is no excess time available to the next sequence block.
    Playing,
    /// Type-specific owner/pause checks prevented animation advancement.
    /// This reports a blocked update; it does not change the playback state.
    Paused,
    /// The tween completed its final loop.
    Completed {
        /// Leftover time in the units of the `delta_time` supplied to
        /// [`SpireTween::process`], with this tween's `speed_scale` removed.
        /// Parent sequences can pass it to the next block without conversion.
        /// Internal animation and loop counters still use scaled time.
        ///
        /// When `abs(speed_scale) < 0.00001`, excess is reported as zero instead
        /// of dividing by a near-zero speed. This deliberately discards any
        /// remainder; a magnitude exactly equal to the cutoff is still divided.
        unscaled_excess_time: f64,
    },
    /// An owner or callable is invalid, or the tween otherwise needs removal
    /// from its processor. This is cancellation, not successful completion.
    ShouldDespawn,
}
