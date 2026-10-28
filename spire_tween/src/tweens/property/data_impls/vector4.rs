use super::*;

#[delegate_impl]
impl IProperty<Vector4> for PropertyDataVec4 {
    fn get_property_value(&self) -> Vector4 {
        delegate_property_data_vec_4! { self.get_property_value() }
    }
    fn set_property_value(&mut self, value: Vector4);
}

impl PropertyType for Vector4 {
    type Data = PropertyDataVec4;
}

impl TyToPropertyTween for Vector4 {
    type GdTween = SpirePropertyVec4;
}

#[delegate_impl]
impl IPropertyData for PropertyDataVec4 {
    type Target = Object;
    fn accessors_are_valid(&self) -> bool;
    fn get_property_path(&self) -> NodePath;
    fn get_owner(&self) -> Option<&ObjectOrNode>;
}
