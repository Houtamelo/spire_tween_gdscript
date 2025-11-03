use super::*;

#[delegate_impl]
impl IProperty<i64> for PropertyDataInt {
    fn get_property_value(&self) -> i64 {
        delegate_property_data_int! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: i64);
}
impl PropertyType for i64 {
    type Data = PropertyDataInt;
}

impl TyToPropertyTween for i64 {
    type GdTween = SpirePropertyInt;
}

#[delegate_impl]
impl IPropertyData for PropertyDataInt {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
