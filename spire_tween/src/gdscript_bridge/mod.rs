use super::*;

mod do_call;
mod do_delayed_call;
mod do_property;
mod generated;
mod sequence;

#[allow(unused_imports)]
pub use self::{do_call::*, do_delayed_call::*, do_property::*, generated::*, sequence::*};

#[derive(GodotClass)]
#[class(base = Object, no_init)]
/// This is "namespace" class that serves as the main entrypoint for using SpireTween within GDScript.
/// It contains all the enums used by Spire, as well as static constructors that allow you to
/// create all the different types of tweens provided by Spire.
/// See each function's documentation for more details.
pub struct Spire {}

#[godot_api]
impl Spire {
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
        let Some(handle) = tween else { return }; // Passing null is fine, don't print errors for it.

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
        let Some(handle) = tween else { return false }; // Passing null is fine, don't print errors for it.

        handle
            .log_bad_spire_arg(|| "tween")
            .is_some_and(|t| TM.tween_is_registered(&t))
    }
}

pub(crate) fn bridge_registration_constants() -> &'static std::sync::Mutex<(Vec<fn()>, Vec<fn()>)> {
    Spire::__registration_storage()
}
