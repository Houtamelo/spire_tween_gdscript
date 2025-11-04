use super::*;

mod do_call;
mod do_delayed_call;
mod do_property;
mod generated;
mod sequence;

pub(crate) use generated::*;

#[derive(GodotClass)]
#[class(base = Object, no_init)]
/// This is "namespace" class that serves as the main entrypoint for using SpireTween within GDScript.
/// It contains all the enums used by Spire, as well as static constructors that allow you to
/// create all the different types of tweens provided by Spire.
/// See each function's documentation for more details.
pub struct Spire {}

#[godot_api]
impl Spire {
    /*
    /// [b]Behavior:[/b] Returns `true` if these conditions are met; false otherwise:
    /// 1. [param handle] != `null` (it is completely ok to pass a null handle to this function).
    /// 2. The tween data referenced by the handle must still exist in native memory.
    ///
    /// [b]Usage:[/b] As long as the tween is valid, it can always be played/paused/stopped,
    /// or edited in any other way.
    ///
    /// [b]Note:[/b] Tweens classes exposed to GDScript are merely handles to the actual tween data in
    /// native memory. This means that even if the handle's instance
    /// is valid (which can be checked by using [method Object.is_instance_valid]),
    /// the data it points to may have been removed already. That being said, tween handles
    /// inherit [RefCounted], so [method Object.is_instance_valid] will always return `true` since the
    /// very reference being used to call that method is keeping the handle alive.
    ///
    /// [b]Note:[/b] This function and the handle's `is_valid` are the only ways of checking
    /// if the actual tween data still exists in memory.
    #[func]
    pub fn is_valid(handle: Option<Gd<RefCounted>>) -> bool {
        handle.is_some_and(|h| TM.is_valid(gd_handle_id!(h)))
    }
    */

    /*
    /// [b]Behavior:[/b] Immediately stops and kills the tween referenced by [param handle], if it exists.
    ///
    /// [b]Usage:[/b] This function is shorthand for:
    /// ```gdscript
    /// if handle != null: handle.kill()
    /// ```
    /// which means that it's completely ok to pass a `null` [param handle] to this function.
    #[func]
    pub fn kill(handle: Option<Gd<RefCounted>>) {
        if let Some(h) = handle {
            TM.tween_unregister(gd_handle_id!(h));
        }
    }
    */

    /// [b]Behavior:[/b] Registers [param tween] with the TweenManager, which makes the tween tick each frame.
    ///
    /// [b]Note:[/b] Tweens automatically register themselves once created, this method is only useful
    /// if the tween was previously unregistered somehow.
    ///
    /// [b]Note:[/b] Calling this with a tween that's already registered does nothing, is harmless, and
    /// will not cause any warnings to be emitted.
    ///
    /// [b]Inside sequences:[/b] Registering a tween that's inside a [SpireSequence] results in
    /// undefined behavior, since that would make both the Sequence and the TweenManager to tick it.
    /// Undefined behavior here will not cause crashes or memory corruption, but the tween will most likely
    /// not behave as expected.
    #[func]
    pub fn register(tween: Option<Gd<RefCounted>>) {
        if let Some(tween) = tween.log_bad_spire_arg(|| "tween") {
            TM.tween_register(tween);
        }
    }

    /// [b]Behavior:[/b] Unregisters [param tween] from the TweenManager, which means the tween
    /// will no longer tick each frame. Since tweens are ref-counted, this does not necessarily
    /// mean that the tween is immediately freed from memory, as there might be other references to it
    /// (such as the very reference you're using to call this method).
    ///
    /// [b]Note:[/b] Calling this with a tween that's already unregistered does nothing, is harmless, and
    /// will not cause any warnings to be emitted.
    ///
    /// [b]Inside sequences:[/b] A [SpireSequence] automatically unregisters any tweens added to it, which
    /// means that, if the tween is inside a sequence, it is already unregistered. If you want to remove
    /// a tween from a sequence, use [method SpireSequence.remove] instead.
    #[func]
    pub fn unregister(tween: Option<Gd<RefCounted>>) {
        let Some(handle) = tween
        else { return }; // Passing null is fine, don't print errors for it.

        if let Some(tween) = handle.log_bad_spire_arg(|| "tween") {
            TM.tween_unregister(&tween);
        }
    }

    /// [b]Returns:[/b] `true` if [param tween] is currently registered with the TweenManager; `false` otherwise.
    ///
    /// [b]Usage:[/b] Being registered means that the tween will automatically "tick" at the end of
    /// each frame.
    ///
    /// [b]Inside sequences:[/b] This will return `false` if [param tween] is inside a [SpireSequence],
    /// since the Sequence is the one that "owns" the tween and is responsible for ticking it, not the
    /// TweenManager.
    #[func]
    pub fn is_registered(tween: Option<Gd<RefCounted>>) -> bool {
        let Some(handle) = tween
        else { return false }; // Passing null is fine, don't print errors for it.

        handle
            .log_bad_spire_arg(|| "tween")
            .is_some_and(|t| TM.tween_is_registered(&t))
    }
}

pub(crate) fn bridge_registration_constants() -> &'static std::sync::Mutex<Vec<fn()>> {
    &__registration_constants_Spire
}
