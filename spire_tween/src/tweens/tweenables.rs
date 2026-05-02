use super::*;

/// Marker trait carried by every type that can serve as the inner data `T` of a
/// [`SpireTween<T>`]. Pairs each inner data type with the GDScript-facing handle
/// that wraps it (used when [`SpireTween::register_with_gd_handle`] is called and
/// Godot signals need to be wired up).
///
/// Implemented for: [`Sequence`], [`Callable`] (delayed call), [`LerpMethodData<T>`]
/// for the supported `T`s, and [`LerpPropertyData<T>`] for the supported `T`s. You
/// only implement this if you're adding a brand-new tween category.
pub trait ITweenable {
    /// Type of the GDScript-facing wrapper (`Gd<SpireSequence>`, `Gd<SpirePropertyVec2>`, …).
    /// Required to dispatch Godot `finished` / `loop_finished` signals.
    type GdHandle: Signaler;

    /// Creates the `Gd` wrapper and attaches it to the tween inside the [`RcPtr`].
    /// This enables `finished` and `loop_finished` Godot signals on the wrapper.
    /// Pure-Rust consumers don't need this — connect callbacks via
    /// [`SpireTween::finished_connect`] / [`SpireTween::loop_finished_connect`]
    /// instead, after calling [`SpireTween::register`].
    fn attach_gd_handle(tween: &RcPtr<SpireTween<Self>>)
    where Self: Sized;
}

macro_rules! impl_tweenables {
    ($($Ty:ty => $Handle:ty, $Wrapper:ty);* $(;)?) => {
        $(
            impl ITweenable for $Ty {
                type GdHandle = $Handle;

                fn attach_gd_handle(tween: &RcPtr<SpireTween<Self>>) {
                    let gd: Gd<$Wrapper> = gd_from_native_tween::<$Wrapper>(tween.clone());
                    // gd_from_native_tween already sets gd_handle inside the tween.
                    // We just need to keep the Gd alive — but it's RefCounted and
                    // gd_handle holds a clone, so it stays alive as long as the tween does.
                    drop(gd);
                }
            }
        )*
    };
}

impl_tweenables! {
    Callable => Gd<SpireDelayedCall>, SpireDelayedCall;
    Sequence => Gd<SpireSequence>, SpireSequence;
    LerpMethodData<Variant> => Gd<SpireMethod>, SpireMethod;
    LerpMethodData<i64> => Gd<SpireMethodInt>, SpireMethodInt;
    LerpMethodData<f64> => Gd<SpireMethodFloat>, SpireMethodFloat;
    LerpMethodData<GString> => Gd<SpireMethodString>, SpireMethodString;
    LerpMethodData<Color> => Gd<SpireMethodColor>, SpireMethodColor;
    LerpMethodData<Vector2> => Gd<SpireMethodVec2>, SpireMethodVec2;
    LerpMethodData<Vector2i> => Gd<SpireMethodVec2i>, SpireMethodVec2i;
    LerpMethodData<Vector3> => Gd<SpireMethodVec3>, SpireMethodVec3;
    LerpMethodData<Vector3i> => Gd<SpireMethodVec3i>, SpireMethodVec3i;
    LerpPropertyData<Variant> => Gd<SpireProperty>, SpireProperty;
    LerpPropertyData<i64> => Gd<SpirePropertyInt>, SpirePropertyInt;
    LerpPropertyData<f64> => Gd<SpirePropertyFloat>, SpirePropertyFloat;
    LerpPropertyData<GString> => Gd<SpirePropertyString>, SpirePropertyString;
    LerpPropertyData<Color> => Gd<SpirePropertyColor>, SpirePropertyColor;
    LerpPropertyData<Vector2> => Gd<SpirePropertyVec2>, SpirePropertyVec2;
    LerpPropertyData<Vector2i> => Gd<SpirePropertyVec2i>, SpirePropertyVec2i;
    LerpPropertyData<Vector3> => Gd<SpirePropertyVec3>, SpirePropertyVec3;
    LerpPropertyData<Vector3i> => Gd<SpirePropertyVec3i>, SpirePropertyVec3i;
}
