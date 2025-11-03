use super::*;

#[delegate_impl]
impl IProperty<Vector3> for PropertyDataVector3 {
    fn get_property_value(&self) -> Vector3 {
        delegate_property_data_vector_3! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Vector3);
}

impl PropertyType for Vector3 {
    type Data = PropertyDataVector3;
}

impl TyToPropertyTween for Vector3 {
    type GdTween = SpirePropertyVector3;
}

#[delegate_impl]
impl IPropertyData for PropertyDataVector3 {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
