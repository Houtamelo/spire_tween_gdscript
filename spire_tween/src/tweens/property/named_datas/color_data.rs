use super::*;

#[delegate_impl]
impl IProperty<Color> for PropertyColorData {
    fn get_property_value(&self) -> Color {
        delegate_property_color_data! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: Color);
}

impl PropertyType for Color {
    type Data = PropertyColorData;
}

impl TyToGdTween for Color {
    type GdTween = InternalTweenPropertyColor;
}

#[delegate_impl]
impl IPropertyData for PropertyColorData {
    type Target = Object;

    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;

    fn get_owner(&self) -> &ObjectOrNode {
        delegate_property_color_data! {
            self => |this| this.get_owner()
        }
    }
}
