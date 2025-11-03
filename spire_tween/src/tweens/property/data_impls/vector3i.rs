use super::*;

#[delegate_impl]
impl IProperty<Vector3i> for PropertyDataVector3i {
    fn get_property_value(&self) -> Vector3i {
        delegate_property_data_vector_3_i! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Vector3i);
}

impl PropertyType for Vector3i {
    type Data = PropertyDataVector3i;
}

impl TyToPropertyTween for Vector3i {
    type GdTween = SpirePropertyVector3i;
}

#[delegate_impl]
impl IPropertyData for PropertyDataVector3i {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
