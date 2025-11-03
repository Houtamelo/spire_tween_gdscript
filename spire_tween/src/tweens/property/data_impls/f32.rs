/*
use super::*;

#[delegate_impl]
impl IProperty<f32> for PropertyF32Data {
    fn get_property_value(&self) -> f32 {
        delegate_property_f_32_data! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: f32);
}

impl PropertyType for f32 {
    type Data = PropertyF32Data;
}

impl TyToPropertyTween for f32 {
    type GdTween = SpirePropertyF32;
}

#[delegate_impl]
impl IPropertyData for PropertyF32Data {
    type Target = Object;

    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
*/
