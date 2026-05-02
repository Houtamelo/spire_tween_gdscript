use super::*;

#[delegate_impl]
impl IProperty<Vector2i> for PropertyDataVec2i {
    fn get_property_value(&self) -> Vector2i {
        delegate_property_data_vec_2_i! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Vector2i);
}

impl PropertyType for Vector2i {
    type Data = PropertyDataVec2i;
}

impl TyToPropertyTween for Vector2i {
    type GdTween = SpirePropertyVec2i;
}

#[delegate_impl]
impl IPropertyData for PropertyDataVec2i {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn get_owner(&self) -> Option<&ObjectOrNode>;
}
