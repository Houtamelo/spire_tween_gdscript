use super::*;

pub trait DoMethod<TVal, Marker = ()> {
    type Return;

    fn do_method(
        &self,
        method: impl Into<StringName>,
        start_val: TVal,
        end_val: TVal,
        duration: f64,
    ) -> SpireTween<MethodData<Self::Return>>;
}

impl<T, Val> DoMethod<Val, ()> for Gd<T>
where
    T: Inherits<Object>,
    Val: SpireLerp,
    AnyTween: From<SpireTween<MethodData<Val>>>,
{
    type Return = Val;

    fn do_method(
        &self,
        method: impl Into<StringName>,
        start_val: Val,
        end_val: Val,
        duration: f64,
    ) -> SpireTween<MethodData<Val>> {
        SpireTween::<MethodData<Val>>::new(
            method.into(),
            ObjectOrNode::from_unchecked_object(self.to_godot().upcast()),
            start_val,
            end_val,
            duration,
            AutoPlay(true),
        )
    }
}

impl<T, Val> DoMethod<Val, BaseMarker> for T
where
    T: WithBaseField + Inherits<Object>,
    Val: SpireLerp + 'static,
    AnyTween: From<SpireTween<MethodData<Val>>>,
{
    type Return = Val;

    fn do_method(
        &self,
        method: impl Into<StringName>,
        start_val: Val,
        end_val: Val,
        duration: f64,
    ) -> SpireTween<MethodData<Val>> {
        SpireTween::<MethodData<Val>>::new(
            method.into(),
            ObjectOrNode::from_unchecked_object(self.to_gd().upcast()),
            start_val,
            end_val,
            duration,
            AutoPlay(true),
        )
    }
}

pub trait DoVarMethod {
    fn do_var_method(
        &self,
        method: impl Into<StringName>,
        start_val: Variant,
        end_val: Variant,
        duration: f64,
        lerp_fn: impl Fn(&Variant, &Variant, f64) -> Variant + 'static,
    ) -> SpireTween<MethodData<Variant>>;
}

impl<T> DoVarMethod for Gd<T>
where T: Inherits<Object>
{
    fn do_var_method(
        &self,
        method: impl Into<StringName>,
        start_val: Variant,
        end_val: Variant,
        duration: f64,
        lerp_fn: impl Fn(&Variant, &Variant, f64) -> Variant + 'static,
    ) -> SpireTween<MethodData<Variant>> {
        SpireTween::<MethodData<Variant>>::new(
            method.into(),
            ObjectOrNode::from_unchecked_object(self.to_godot().upcast()),
            start_val,
            end_val,
            duration,
            AutoPlay(true),
            lerp_fn,
        )
    }
}
