use super::*;

#[delegate_impl]
impl IProperty<i64> for PropertyI64Data {
    fn get_property_value(&self) -> i64 {
        delegate_property_i_64_data! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: i64);
}
impl PropertyType for i64 {
    type Data = PropertyI64Data;
}

impl TyToGdTween for i64 {
    type GdTween = InternalTweenPropertyI64;
}

#[delegate_impl]
impl IPropertyData for PropertyI64Data {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
