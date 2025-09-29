use super::*;

#[delegate_impl]
impl IProperty<Vector3> for PropertyVector3Data {
    fn get_property_value(&self) -> Vector3 {
        delegate_property_vector_3_data! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Vector3);
}

impl PropertyType for Vector3 {
    type Data = PropertyVector3Data;
}

impl TyToGdTween for Vector3 {
    type GdTween = InternalTweenPropertyVector3;
}

#[delegate_impl]
impl IPropertyData for PropertyVector3Data {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
