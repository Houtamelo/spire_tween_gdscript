use super::*;

pub trait IProperty<T> {
    fn get_property_value(&self) -> T;
    fn set_property_value(&mut self, value: T);
}

pub trait IPropertyData {
    type Target: Inherits<Object>;

    fn get_property_path(&self) -> NodePath;
    fn get_owner(&self) -> Option<&ObjectOrNode>;
}

pub trait IGeneralPropertyData: Sized {
    fn from_path_and_owner(path_str: &str, path: NodePath, owner: Gd<Object>) -> Self;
}

pub trait PropertyType: Sized + ILerpable {
    type Data: IProperty<Self> + IPropertyData + IGeneralPropertyData;
}

#[allow(dead_code)]
pub trait TyToPropertyTween {
    type GdTween;
}

pub(crate) trait TryFromPathAndObject {
    fn try_from_path_and_object(_path: &str, _object: Gd<Object>) -> Option<Self>
    where Self: Sized {
        None
    }
}
