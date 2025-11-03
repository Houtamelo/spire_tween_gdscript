use super::*;

pub trait Signaler {
    fn emit_finished(&self);
    fn emit_loop_finished(&self);
}

macro_rules! impl_invoke_signals {
    ($($Ty:ty),* $(,)?) => {
        $(
            impl Signaler for $Ty {
                fn emit_finished(&self) {
                    self.signals().finished().emit();
                }

                fn emit_loop_finished(&self) {
                    self.signals().loop_finished().emit();
                }
            }
        )*
    };
}

impl_invoke_signals! {
    Gd<SpireDelayedCall>,
    Gd<SpireSequence>,
    Gd<SpireMethod>,
    //Gd<SpireMethodI32>,
    Gd<SpireMethodInt>,
    //Gd<SpireMethodF32>,
    Gd<SpireMethodFloat>,
    Gd<SpireMethodString>,
    Gd<SpireMethodColor>,
    Gd<SpireMethodVector2>,
    Gd<SpireMethodVector2i>,
    Gd<SpireMethodVector3>,
    Gd<SpireMethodVector3i>,
    Gd<SpireProperty>,
    //Gd<SpirePropertyI32>,
    Gd<SpirePropertyInt>,
    //Gd<SpirePropertyF32>,
    Gd<SpirePropertyFloat>,
    Gd<SpirePropertyString>,
    Gd<SpirePropertyColor>,
    Gd<SpirePropertyVector2>,
    Gd<SpirePropertyVector2i>,
    Gd<SpirePropertyVector3>,
    Gd<SpirePropertyVector3i>,
}
