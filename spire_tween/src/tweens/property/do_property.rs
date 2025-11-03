use super::*;

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
