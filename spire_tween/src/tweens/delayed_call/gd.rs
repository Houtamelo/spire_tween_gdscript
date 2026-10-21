use super::*;

#[derive(GodotClass)]
#[class(base = RefCounted, no_init)]
/// A simple tween that invokes a function (only once) after a delay.
pub struct SpireDelayedCall {
    pub base:  Base<RefCounted>,
    pub inner: UnsafeCell<TweenPtr<Callable>>,
}

#[godot_api]
impl SpireDelayedCall {
    /// Emitted once when playback changes from playing or paused to stopped,
    /// including successful completion, explicit [method stop], and cancellation
    /// caused by an invalid callable or freed binding. State is already stopped.
    /// Repeated stops and intermediate loops do not emit it; replay permits another emission.
    /// Unregistering or dropping a handle does not itself stop the tween.
    ///
    /// Successful completion emits [signal stopped] before [signal finished].
    /// Deferred handlers execute later. Cancellation emits no completion signals.
    #[signal]
    pub fn stopped();

    /// Emitted when this tween completes a loop.
    ///
    /// [b]Note:[/b] On the last loop of a tween, this is emitted before [signal finished].
    #[signal]
    pub fn loop_finished();

    /// Emitted when this tween successfully finishes playing, after [signal stopped].
    /// This happens when the tween completes all its loops, or when [method force_complete] is called.
    ///
    /// Invalid callable cancellation does not emit this signal.
    /// Infinite looping prevents natural completion; a valid [method force_complete]
    /// still emits this signal.
    #[signal]
    pub fn finished();

    /// See [method Spire.do_delayed_call].
    #[func]
    pub fn build(func: Callable, delay: f64) -> Gd<Self> {
        let tween = SpireTween::<Callable>::new(func, delay).register();
        gd_from_native_tween(tween)
    }

    /// Returns the callable that will be invoked after the delay time.
    #[func]
    pub fn get_callable(&self) -> Callable { self.to_ref().t.clone() }
}

define_base_gd_methods! { SpireDelayedCall, SpireTween<Callable>, Callable }
