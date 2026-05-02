use super::*;

/// Generic property-tween constructor. Implemented for both `Gd<T>` (when you have
/// a handle) and any `T: WithBaseField` (when you're in a `#[godot_api]` impl and
/// need to tween yourself).
///
/// The `Marker` type parameter is plumbing to disambiguate those two impls — it's
/// inferred at the call site, you never pass it explicitly.
///
/// # Returned tween
///
/// Builds a tween in [`LerpMode::Absolute`] mode targeting `property_path` on the
/// receiver. The target is supplied as an [`Evaluator<T>`] — usually
/// [`Evaluator::Static`] for a fixed value, but
/// [`Evaluator::Dynamic`] / [`Evaluator::Callable`] enable moving targets.
///
/// You **must** call [`register`](SpireTween::register) /
/// [`register_with_gd_handle`](SpireTween::register_with_gd_handle) on the returned
/// tween, otherwise it's just data and won't tick.
///
/// For common properties prefer the per-property shortcuts (`DoNode2D::do_position`,
/// `DoCanvasItem::do_modulate`, …) — they pre-fill the path and pick the right
/// specialized data adapter at compile time.
///
/// # Example
///
/// ```ignore
/// use spire_tween::prelude::*;
///
/// my_node
///     .do_property("position:x", Evaluator::Static(640.0_f64), 2.0)
///     .as_relative(0.0)
///     .register();
/// ```
pub trait DoProperty<T: PropertyType, Marker = ()>
where LerpPropertyData<T>: ITweenable
{
    fn do_property(
        &self,
        property_path: impl AsArg<NodePath>,
        to: Evaluator<T>,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<T>>;
}

impl<T, C> DoProperty<T, ()> for Gd<C>
where
    C: Inherits<Object>,
    T: PropertyType,
    <T as ILerpable>::Lerper: Default,
    AnyTween: From<RcPtr<SpireTween<LerpPropertyData<T>>>>,
    LerpPropertyData<T>: ITweenable,
{
    fn do_property(
        &self,
        property_path: impl AsArg<NodePath>,
        to: Evaluator<T>,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<T>> {
        let property = property_path.into_arg().cow_into_owned();
        let property_str = &property.to_string();
        let owner = self.clone().upcast();
        let data = T::Data::from_path_and_owner(property_str, property, owner);
        SpireTween::<LerpPropertyData<T>>::new(data, to, duration)
    }
}

impl<C, T> DoProperty<T, BaseMarker> for C
where
    C: WithBaseField + Inherits<Object>,
    T: PropertyType,
    <T as ILerpable>::Lerper: Default,
    AnyTween: From<RcPtr<SpireTween<LerpPropertyData<T>>>>,
    LerpPropertyData<T>: ITweenable,
{
    fn do_property(
        &self,
        property_path: impl AsArg<NodePath>,
        to: Evaluator<T>,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<T>> {
        let property = property_path.into_arg().cow_into_owned();
        let property_str = &property.to_string();
        let owner = self.to_gd().upcast();
        let data = T::Data::from_path_and_owner(property_str, property, owner.clone());
        SpireTween::<LerpPropertyData<T>>::new(data, to, duration)
    }
}
