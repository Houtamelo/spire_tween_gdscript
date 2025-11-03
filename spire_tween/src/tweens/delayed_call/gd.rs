use super::*;

#[derive(GodotClass)]
#[class(base = RefCounted, no_init)]
/// A simple tween that invokes a function (only once) after a delay.
pub struct SpireDelayedCall {
    pub base:  Base<RefCounted>,
    pub inner: UnsafeCell<RcPtr<SpireTween<Callable>>>,
}

#[godot_api]
impl SpireDelayedCall {
    /// Emitted when this tween completes a loop.
    ///
    /// [b]Note:[/b] On the last loop of a tween, this is emitted before [signal finished].
    #[signal]
    pub fn loop_finished();

    /// Emitted when this tween finishes playing.
    /// This happens when the tween completes all its loops, or when [method force_complete] is called.
    ///
    /// [b]Note:[/b] This will never be emitted if the tween is set to loop infinitely
    /// (e.g. calling [method set_loops] with `-1`).
    #[signal]
    pub fn finished();

    /// See [method Spire.do_delayed_call].
    #[func]
    pub fn build(func: Callable, delay: f64) -> Gd<Self> {
        let inner = UnsafeCell::new(SpireTween::<Callable>::new(func, delay).register());
        let handle = Gd::from_init_fn(|base| Self { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }

    /// Returns the callable that will be invoked after the delay time.
    #[func]
    pub fn get_callable(&self) -> Callable { self.to_ref().t.clone() }
}

define_base_gd_methods! { SpireDelayedCall => SpireTween<Callable> }
