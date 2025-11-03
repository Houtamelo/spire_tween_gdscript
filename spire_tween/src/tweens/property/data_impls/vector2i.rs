use super::*;

#[delegate_impl]
impl IProperty<Vector2i> for PropertyDataVector2i {
    fn get_property_value(&self) -> Vector2i {
        delegate_property_data_vector_2_i! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Vector2i);
}

impl PropertyType for Vector2i {
    type Data = PropertyDataVector2i;
}

impl TyToPropertyTween for Vector2i {
    type GdTween = SpirePropertyVector2i;
}

#[delegate_impl]
impl IPropertyData for PropertyDataVector2i {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
