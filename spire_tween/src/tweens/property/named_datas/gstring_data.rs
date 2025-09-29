use super::*;

#[delegate_impl]
impl IProperty<GString> for PropertyGStringData {
    fn get_property_value(&self) -> GString {
        delegate_property_g_string_data! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: GString);
}

impl PropertyType for GString {
    type Data = PropertyGStringData;
}

impl TyToGdTween for GString {
    type GdTween = InternalTweenPropertyString;
}

#[delegate_impl]
impl IPropertyData for PropertyGStringData {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
