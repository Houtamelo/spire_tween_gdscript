use super::*;

pub trait ITweenable {
    type GdHandle: Signaler;
}

macro_rules! impl_tweenables {
    ($($Ty:ty => $Handle:ty),* $(,)?) => {
        $(
            impl ITweenable for $Ty {
                type GdHandle = $Handle;
            }
        )*
    };
}

impl_tweenables! {
    Callable => Gd<SpireDelayedCall>,
    Sequence => Gd<SpireSequence>,
    LerpMethodData<Variant> => Gd<SpireMethod>,
    //LerpMethodData<i32> => Gd<SpireMethodI32>,
    LerpMethodData<i64> => Gd<SpireMethodInt>,
    //LerpMethodData<f32> => Gd<SpireMethodF32>,
    LerpMethodData<f64> => Gd<SpireMethodFloat>,
    LerpMethodData<GString> => Gd<SpireMethodString>,
    LerpMethodData<Color> => Gd<SpireMethodColor>,
    LerpMethodData<Vector2> => Gd<SpireMethodVector2>,
    LerpMethodData<Vector2i> => Gd<SpireMethodVector2i>,
    LerpMethodData<Vector3> => Gd<SpireMethodVector3>,
    LerpMethodData<Vector3i> => Gd<SpireMethodVector3i>,
    LerpPropertyData<Variant> => Gd<SpireProperty>,
    //LerpPropertyData<i32> => Gd<SpirePropertyI32>,
    LerpPropertyData<i64> => Gd<SpirePropertyInt>,
    //LerpPropertyData<f32> => Gd<SpirePropertyF32>,
    LerpPropertyData<f64> => Gd<SpirePropertyFloat>,
    LerpPropertyData<GString> => Gd<SpirePropertyString>,
    LerpPropertyData<Color> => Gd<SpirePropertyColor>,
    LerpPropertyData<Vector2> => Gd<SpirePropertyVector2>,
    LerpPropertyData<Vector2i> => Gd<SpirePropertyVector2i>,
    LerpPropertyData<Vector3> => Gd<SpirePropertyVector3>,
    LerpPropertyData<Vector3i> => Gd<SpirePropertyVector3i>,
}
