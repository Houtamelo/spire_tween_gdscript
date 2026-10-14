use super::*;

#[delegated_enum(impl_conversions)]
#[derive(Clone)]
pub enum AnyTween {
    Property(AnyPropertyTween),
    Method(AnyMethodTween),
    DelayedCall(TweenPtr<Callable>),
    Sequence(TweenPtr<Sequence>),
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
    fn process_internal(&mut self, delta_time: f64, is_tree_paused: bool) -> AdvanceTimeResult;
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
    DelayedCall(WeakTweenPtr<Callable>),
    Sequence(WeakTweenPtr<Sequence>),
}

impl WeakAnyTween {
    pub fn upgrade(&self) -> Option<AnyTween> {
        delegate_weak_any_tween! { self.upgrade().map(Into::into) }
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
        SpirePropertyInt => Property,
        SpirePropertyFloat => Property,
        SpirePropertyString => Property,
        SpirePropertyColor => Property,
        SpirePropertyVec2 => Property,
        SpirePropertyVec2i => Property,
        SpirePropertyVec3 => Property,
        SpirePropertyVec3i => Property,
        SpireProperty => Property,
        SpireMethodInt => Method,
        SpireMethodFloat => Method,
        SpireMethodString => Method,
        SpireMethodColor => Method,
        SpireMethodVec2 => Method,
        SpireMethodVec2i => Method,
        SpireMethodVec3 => Method,
        SpireMethodVec3i => Method,
        SpireMethod => Method,
    }
}
