use super::*;

#[delegate_impl]
impl IProperty<Quaternion> for PropertyDataQuaternion {
    fn get_property_value(&self) -> Quaternion {
        delegate_property_data_quaternion! { self.get_property_value() }
    }
    fn set_property_value(&mut self, value: Quaternion);
}

impl PropertyType for Quaternion {
    type Data = PropertyDataQuaternion;
}

impl TyToPropertyTween for Quaternion {
    type GdTween = SpirePropertyQuaternion;
}

#[delegate_impl]
impl IPropertyData for PropertyDataQuaternion {
    type Target = Object;
    fn accessors_are_valid(&self) -> bool;
    fn get_property_path(&self) -> NodePath;
    fn get_owner(&self) -> Option<&ObjectOrNode>;
}
