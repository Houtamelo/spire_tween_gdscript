/*
use super::*;

#[delegate_impl]
impl IProperty<i32> for PropertyI32Data {
    fn get_property_value(&self) -> i32 {
        delegate_property_i_32_data! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: i32);
}

impl PropertyType for i32 {
    type Data = PropertyI32Data;
}

impl TyToPropertyTween for i32 {
    type GdTween = SpirePropertyI32;
}

#[delegate_impl]
impl IPropertyData for PropertyI32Data {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
*/
