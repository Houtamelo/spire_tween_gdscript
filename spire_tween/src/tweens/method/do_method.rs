use super::*;

/// Generic method-tween constructor. Implemented for `Gd<T>` and for any
/// `T: WithBaseField` (Self).
///
/// On every tick, looks up `method` on the receiver and invokes it with the
/// interpolated `T` value (between `start_val` and `end_val`). Auto-binds the tween
/// to the receiver so it auto-stops when the receiver is freed.
///
/// The `Marker` parameter disambiguates `Gd<T>` vs `WithBaseField` impls and is
/// inferred — you never pass it.
///
/// You **must** call [`register`](SpireTween::register) on the returned tween.
///
/// # Example
///
/// ```ignore
/// // Tween a method with signature `fn _set_fill(&mut self, value: f64)`:
/// my_node.do_method("_set_fill", 0.0_f64, 1.0_f64, 8.0).register();
/// ```
///
/// For dynamically-typed values use [`DoVarMethod`]. For an arbitrary
/// [`Callable`] (rather than looking up a method by name) construct
/// [`SpireTween::<LerpMethodData<T>>::new`] directly.
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

/// Variant-typed counterpart of [`DoMethod`] — for properties whose type isn't
/// natively tracked by Spire's type-driven dispatch (or when you only have
/// `Variant`s on hand).
///
/// The constructed tween uses the default `CustomBasicLerper` (which falls back to
/// `godot::global::lerp` with type inference). For full control over the lerping
/// behavior use [`SpireTween::<LerpMethodData<Variant>>::new_custom`] with a hand-built
/// [`CustomBasicLerper`].
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
