use super::*;

#[delegate_impl]
impl IProperty<Vector2> for PropertyVector2Data {
    fn get_property_value(&self) -> Vector2 {
        delegate_property_vector_2_data! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Vector2);
}

impl PropertyType for Vector2 {
    type Data = PropertyVector2Data;
}

impl TyToGdTween for Vector2 {
    type GdTween = InternalTweenPropertyVector2;
}

#[delegate_impl]
impl IPropertyData for PropertyVector2Data {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
