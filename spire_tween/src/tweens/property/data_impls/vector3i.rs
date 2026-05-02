use super::*;

#[delegate_impl]
impl IProperty<Vector3i> for PropertyDataVec3i {
    fn get_property_value(&self) -> Vector3i {
        delegate_property_data_vec_3_i! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Vector3i);
}

impl PropertyType for Vector3i {
    type Data = PropertyDataVec3i;
}

impl TyToPropertyTween for Vector3i {
    type GdTween = SpirePropertyVec3i;
}

#[delegate_impl]
impl IPropertyData for PropertyDataVec3i {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn get_owner(&self) -> Option<&ObjectOrNode>;
}
