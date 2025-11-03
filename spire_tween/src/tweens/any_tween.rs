use super::*;

#[delegated_enum(impl_conversions)]
#[derive(Clone)]
pub enum AnyTween {
    Property(PropertyTween),
    Method(MethodTween),
    DelayedCall(RcPtr<SpireTween<Callable>>),
    Sequence(RcPtr<SpireTween<Sequence>>),
}

#[delegate_impl]
impl SpireTweener for AnyTween {
    #[inline]
    fn play(&mut self);
    #[inline]
    fn pause(&mut self);
    #[inline]
    fn stop(&mut self);
    #[inline]
    fn process(&mut self, delta_time: f64, is_tree_paused: bool) -> AdvanceTimeResult;
    #[inline]
    fn force_complete(&mut self);
}

#[delegate_impl]
impl InnerTypeName for AnyTween {
    #[inline]
    fn inner_type_name(&self) -> &'static str;
}

define_base_methods! { AnyTween }

#[delegated_enum(impl_conversions)]
#[derive(Clone)]
pub enum WeakAnyTween {
    Property(WeakPropertyTween),
    Method(WeakMethodTween),
    DelayedCall(WeakPtr<SpireTween<Callable>>),
    Sequence(WeakPtr<SpireTween<Sequence>>),
}

impl WeakAnyTween {
    #[inline]
    pub fn upgrade(&self) -> Option<AnyTween> {
        delegate_weak_any_tween! { self => |arg| arg.upgrade().map(Into::into) }
    }
}

define_tween_ptr_methods! { Strong = AnyTween; Weak = WeakAnyTween; }

pub fn tween_from_gd_handle(handle: Gd<RefCounted>) -> Result<AnyTween, Gd<RefCounted>> {
    macro_rules! match_tys {
        ($Handle: expr, $($Ty: ty => $EnumVar: ident,)*) => {
            match_class! { $Handle,
                $(
                    tween @ $Ty => {
                        let inner = tween.bind().to_ref().clone();
                        Ok(AnyTween::$EnumVar(inner.into()))
                    },
                )*
                other => Err(other),
            }
        };
    }

    match_tys! { handle,
        SpireSequence => Sequence,
        SpireDelayedCall => DelayedCall,
        //SpirePropertyI32 => Property,
        SpirePropertyInt => Property,
        //SpirePropertyF32 => Property,
        SpirePropertyFloat => Property,
        SpirePropertyString => Property,
        SpirePropertyColor => Property,
        SpirePropertyVector2 => Property,
        SpirePropertyVector2i => Property,
        SpirePropertyVector3 => Property,
        SpirePropertyVector3i => Property,
        SpireProperty => Property,
        //SpireMethodI32 => Method,
        SpireMethodInt => Method,
        //SpireMethodF32 => Method,
        SpireMethodFloat => Method,
        SpireMethodString => Method,
        SpireMethodColor => Method,
        SpireMethodVector2 => Method,
        SpireMethodVector2i => Method,
        SpireMethodVector3 => Method,
        SpireMethodVector3i => Method,
        SpireMethod => Method,
    }
}
