use super::*;

#[godot_api(secondary)]
impl Spire {
    /// [b]Behavior:[/b] Invokes [param call] after [param delay] seconds have passed.
    ///
    /// [b]Usage:[/b] This is similar to using a timer:
    /// ```gdscript
    /// get_tree().create_timer(delay).timeout.connect(call)
    /// ```
    /// except that this can be called from anywhere since it doesn't require accessing the [SceneTree].
    ///
    /// [b]Returns:[/b] A handle that can be used to further customize the tween.
    ///
    /// [b]Note:[/b] No arguments will be passed when invoking [param call]. If it
    /// requires arguments, then either they should have default values, or you can
    /// use [method Callable.bind] to bind fixed arguments to the Callable.
    ///
    /// [b]Note:[/b] If [param call] becomes invalid, including when its target
    /// object is freed, the tween stops and unregisters on its next processing
    /// step or when forced to complete. Cancellation does not emit completion signals.
    #[func]
    pub fn do_delayed_call(callable: Callable, delay: f64) -> Gd<SpireDelayedCall> {
        SpireDelayedCall::build(callable, delay)
    }
}
