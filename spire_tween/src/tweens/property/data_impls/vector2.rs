use super::*;

#[delegate_impl]
impl IProperty<Vector2> for PropertyDataVec2 {
    fn get_property_value(&self) -> Vector2 {
        delegate_property_data_vec_2! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Vector2);
}

impl PropertyType for Vector2 {
    type Data = PropertyDataVec2;
}

impl TyToPropertyTween for Vector2 {
    type GdTween = SpirePropertyVec2;
}

#[delegate_impl]
impl IPropertyData for PropertyDataVec2 {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn get_owner(&self) -> Option<&ObjectOrNode>;
}
