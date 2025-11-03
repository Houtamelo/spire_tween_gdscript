use super::*;

pub trait DoMethod<T, Marker = ()>
where
    T: ILerpable,
    LerpMethodData<T>: ITweenable,
{
    fn do_method(
        &self,
        method: impl AsArg<StringName>,
        start_val: T,
        end_val: T,
        duration: f64,
    ) -> SpireTween<LerpMethodData<T>>;
}

impl<C, T> DoMethod<T, ()> for Gd<C>
where
    C: Inherits<Object>,
    T: ILerpable<BasicLerper: Default>,
    AnyTween: From<RcPtr<SpireTween<LerpMethodData<T>>>>,
    LerpMethodData<T>: ITweenable,
{
    fn do_method(
        &self,
        method: impl AsArg<StringName>,
        start_val: T,
        end_val: T,
        duration: f64,
    ) -> SpireTween<LerpMethodData<T>> {
        SpireTween::<LerpMethodData<T>>::new(
            Callable::from_object_method(self, method),
            start_val,
            end_val,
            duration,
        )
    }
}

impl<C, T> DoMethod<T, BaseMarker> for C
where
    C: WithBaseField + Inherits<Object>,
    T: ILerpable<BasicLerper: Default> + 'static,
    AnyTween: From<RcPtr<SpireTween<LerpMethodData<T>>>>,
    LerpMethodData<T>: ITweenable,
{
    fn do_method(
        &self,
        method: impl AsArg<StringName>,
        start_val: T,
        end_val: T,
        duration: f64,
    ) -> SpireTween<LerpMethodData<T>> {
        SpireTween::<LerpMethodData<T>>::new(
            Callable::from_object_method(&self.to_gd(), method),
            start_val,
            end_val,
            duration,
        )
    }
}

pub trait DoVarMethod {
    fn do_var_method(
        &self,
        method: impl AsArg<StringName>,
        start_val: Variant,
        end_val: Variant,
        duration: f64,
    ) -> SpireTween<LerpMethodData<Variant>>;
}

impl<C> DoVarMethod for Gd<C>
where C: Inherits<Object>
{
    fn do_var_method(
        &self,
        method: impl AsArg<StringName>,
        start_val: Variant,
        end_val: Variant,
        duration: f64,
    ) -> SpireTween<LerpMethodData<Variant>> {
        SpireTween::<LerpMethodData<Variant>>::new(
            Callable::from_object_method(self, method),
            start_val,
            end_val,
            duration,
        )
    }
}
