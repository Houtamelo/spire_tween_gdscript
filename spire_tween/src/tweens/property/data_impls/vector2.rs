use super::*;

#[delegate_impl]
impl IProperty<Vector2> for PropertyDataVector2 {
    fn get_property_value(&self) -> Vector2 {
        delegate_property_data_vector_2! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Vector2);
}

impl PropertyType for Vector2 {
    type Data = PropertyDataVector2;
}

impl TyToPropertyTween for Vector2 {
    type GdTween = SpirePropertyVector2;
}

#[delegate_impl]
impl IPropertyData for PropertyDataVector2 {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
