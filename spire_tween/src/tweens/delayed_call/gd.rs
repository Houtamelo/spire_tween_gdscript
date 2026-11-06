use super::*;

#[derive(GodotClass)]
#[class(base = RefCounted, no_init)]
/// A simple tween that invokes a function (only once) after a delay.
pub struct SpireDelayedCall {
    pub base:  Base<RefCounted>,
    pub inner: UnsafeCell<TweenPtr<Callable>>,
}

#[godot_api(secondary)]
impl SpireDelayedCall {
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
