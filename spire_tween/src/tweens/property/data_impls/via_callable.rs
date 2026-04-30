use std::marker::PhantomData;

use super::*;

#[derive(Debug, Clone)]
pub struct PropertyDataViaCallable<T> {
    pub owner: Option<ObjectOrNode>,
    pub getter: Callable,
    pub setter: Callable,
    pub _pd: PhantomData<T>,
}

impl<T> PropertyDataViaCallable<T> {
    pub fn new(getter: Callable, setter: Callable) -> Self {
        Self {
            owner: getter.object().map(ObjectOrNode::from_unchecked_object),
            getter,
            setter,
            _pd: PhantomData,
        }
    }
}

impl<T: Default + FromGodot + ToGodot> IProperty<T> for PropertyDataViaCallable<T> {
    fn get_property_value(&self) -> T {
        self.getter
            .call(&[])
            .try_to_relaxed::<T>()
            .log_if_err()
            .unwrap_or_default()
    }

    fn set_property_value(&mut self, value: T) { self.setter.call(&[value.to_variant()]); }
}

impl<T> IPropertyData for PropertyDataViaCallable<T> {
    type Target = Object;

    fn get_property_path(&self) -> NodePath { self.getter.method_name().as_ref().map(NodePath::from).unwrap_or_default() }

    fn get_owner(&self) -> Option<&ObjectOrNode> { self.owner.as_ref() }
}

impl<T> TryFromPathAndObject for PropertyDataViaCallable<T> {}
