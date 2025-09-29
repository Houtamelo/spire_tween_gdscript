use super::*;

pub trait DoProperty<TVal: PropertyType, Marker = ()> {
    fn do_property(
        &self,
        property_path: impl AsArg<NodePath>,
        to: Evaluator<TVal>,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<TVal>>;
}

impl<T, Val> DoProperty<Val, ()> for Gd<T>
where
    T: Inherits<Object>,
    Val: PropertyType + SpireLerp,
    AnyTween: From<SpireTween<LerpPropertyData<Val>>>,
{
    fn do_property(
        &self,
        property_path: impl AsArg<NodePath>,
        to: Evaluator<Val>,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Val>> {
        let property = property_path.into_arg().cow_into_owned();
        let property_str = &property.to_string();
        let owner = self.clone().upcast();
        let data = Val::Data::from_path_and_owner(property_str, property, owner);

        SpireTween::<LerpPropertyData<Val>>::new(data, to, duration, AutoPlay(true))
    }
}

impl<T, Val> DoProperty<Val, BaseMarker> for T
where
    T: WithBaseField + Inherits<Object>,
    Val: SpireLerp + PropertyType,
    AnyTween: From<SpireTween<LerpPropertyData<Val>>>,
{
    fn do_property(
        &self,
        property_path: impl AsArg<NodePath>,
        to: Evaluator<Val>,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Val>> {
        let property = property_path.into_arg().cow_into_owned();
        let property_str = &property.to_string();
        let owner = self.to_gd().upcast();
        let data = Val::Data::from_path_and_owner(property_str, property, owner.clone());

        SpireTween::<LerpPropertyData<Val>>::new(data, to, duration, AutoPlay(true))
    }
}
