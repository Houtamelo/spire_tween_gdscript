#[allow(unused_imports)]
use super::*;

/// Uniform playback interface implemented by every concrete tween type
/// (`SpireTween<LerpPropertyData<T>>`, `SpireTween<LerpMethodData<T>>`,
/// `SpireTween<Sequence>`, `SpireTween<Callable>`).
///
/// Routed through [`AnyTween`] so playback controls work without knowing the
/// concrete type. Use [`TweenPtr::custom_step`] for checked native stepping;
/// the manager and sequences use that same path. GDScript wrapper methods
/// delegate playback controls here and expose their own `custom_step` result.
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

    /// Internal animation update after the explicit-binding and playback checks
    /// in [`TweenPtr::custom_step`]. Custom/manual processors must use that checked
    /// entry point rather than call this method directly. Implementations still
    /// perform their type-specific owner and callable validation here.
    ///
    /// On completion, `unscaled_excess_time` uses the same time units as the input
    /// `delta_time`, so parent sequences can pass the remainder to their next block.
    /// When `abs(speed_scale) < 0.00001`, this remainder is reported as zero;
    /// a magnitude exactly equal to the cutoff is still divided by `speed_scale`.
    #[doc(hidden)]
    fn process_internal(&mut self, delta_time: f64, is_tree_paused: bool) -> AdvanceTimeResult;

    /// Jumps to the final state — sets the property to its end value, fires the
    /// `finished` event, transitions to [`State::Stopped`]. Skips any remaining
    /// loops without firing `loop_finished` for them.
    ///
    /// No-op on a stopped tween.
    fn force_complete(&mut self);
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
        /// [`SpireTweener::process_internal`], with this tween's `speed_scale` removed.
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
