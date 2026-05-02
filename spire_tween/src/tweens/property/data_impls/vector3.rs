use super::*;

#[delegate_impl]
impl IProperty<Vector3> for PropertyDataVec3 {
    fn get_property_value(&self) -> Vector3 {
        delegate_property_data_vec_3! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Vector3);
}

impl PropertyType for Vector3 {
    type Data = PropertyDataVec3;
}

impl TyToPropertyTween for Vector3 {
    type GdTween = SpirePropertyVec3;
}

#[delegate_impl]
impl IPropertyData for PropertyDataVec3 {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn get_owner(&self) -> Option<&ObjectOrNode>;
}
