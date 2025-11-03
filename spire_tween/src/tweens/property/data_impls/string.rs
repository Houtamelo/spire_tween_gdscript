use super::*;

#[delegate_impl]
impl IProperty<GString> for PropertyDataString {
    fn get_property_value(&self) -> GString {
        delegate_property_data_string! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: GString);
}

impl PropertyType for GString {
    type Data = PropertyDataString;
}

impl TyToPropertyTween for GString {
    type GdTween = SpirePropertyString;
}

#[delegate_impl]
impl IPropertyData for PropertyDataString {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
