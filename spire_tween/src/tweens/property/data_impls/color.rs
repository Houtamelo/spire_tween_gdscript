use super::*;

#[delegate_impl]
impl IProperty<Color> for PropertyDataColor {
    fn get_property_value(&self) -> Color {
        delegate_property_data_color! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Color);
}

impl PropertyType for Color {
    type Data = PropertyDataColor;
}

impl TyToPropertyTween for Color {
    type GdTween = SpirePropertyColor;
}

#[delegate_impl]
impl IPropertyData for PropertyDataColor {
    type Target = Object;

    fn get_property_path(&self) -> NodePath;
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        delegate_property_data_color! {
            self => |this| this.get_owner()
        }
    }
}
