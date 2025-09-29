use godot::meta::ClassName;

use super::*;

pub trait IProperty<T> {
    fn get_property_value(&self) -> T;
    fn set_property_value(&mut self, value: T);
}

pub trait IPropertyData {
    type Target: Inherits<Object>;

    fn get_property_path(&self) -> NodePath;
    fn get_owner(&self) -> &ObjectOrNode;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn owner_class(&self) -> ClassName { Self::Target::class_name() }
}

pub trait IGeneralPropertyData: Sized {
    fn from_path_and_owner(path_str: &str, path: NodePath, owner: Gd<Object>) -> Self;
}

pub trait PropertyType: Sized {
    type Data: IProperty<Self> + IPropertyData + IGeneralPropertyData;
}

pub trait TyToGdTween {
    type GdTween;
}

pub(crate) trait TryFromPathAndObject {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self>
    where Self: Sized;
}
