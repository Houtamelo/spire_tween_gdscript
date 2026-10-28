use super::*;

#[delegate_impl]
impl IProperty<Rect2> for PropertyDataRect2 {
    fn get_property_value(&self) -> Rect2 {
        delegate_property_data_rect_2! { self.get_property_value() }
    }
    fn set_property_value(&mut self, value: Rect2);
}

impl PropertyType for Rect2 {
    type Data = PropertyDataRect2;
}

impl TyToPropertyTween for Rect2 {
    type GdTween = SpirePropertyRect2;
}

#[delegate_impl]
impl IPropertyData for PropertyDataRect2 {
    type Target = Object;
    fn accessors_are_valid(&self) -> bool;
    fn get_property_path(&self) -> NodePath;
    fn get_owner(&self) -> Option<&ObjectOrNode>;
}
