use super::*;

pub trait ITweenable {
    type GdHandle: Signaler;

    /// Creates and attaches the GD handle to the tween inside the RcPtr.
    /// This enables `finished` and `loop_finished` signals for Rust consumers.
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
    LerpMethodData<Vector2> => Gd<SpireMethodVector2>, SpireMethodVector2;
    LerpMethodData<Vector2i> => Gd<SpireMethodVector2i>, SpireMethodVector2i;
    LerpMethodData<Vector3> => Gd<SpireMethodVector3>, SpireMethodVector3;
    LerpMethodData<Vector3i> => Gd<SpireMethodVector3i>, SpireMethodVector3i;
    LerpPropertyData<Variant> => Gd<SpireProperty>, SpireProperty;
    LerpPropertyData<i64> => Gd<SpirePropertyInt>, SpirePropertyInt;
    LerpPropertyData<f64> => Gd<SpirePropertyFloat>, SpirePropertyFloat;
    LerpPropertyData<GString> => Gd<SpirePropertyString>, SpirePropertyString;
    LerpPropertyData<Color> => Gd<SpirePropertyColor>, SpirePropertyColor;
    LerpPropertyData<Vector2> => Gd<SpirePropertyVector2>, SpirePropertyVector2;
    LerpPropertyData<Vector2i> => Gd<SpirePropertyVector2i>, SpirePropertyVector2i;
    LerpPropertyData<Vector3> => Gd<SpirePropertyVector3>, SpirePropertyVector3;
    LerpPropertyData<Vector3i> => Gd<SpirePropertyVector3i>, SpirePropertyVector3i;
}
