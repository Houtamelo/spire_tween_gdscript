use super::*;

#[delegate_impl]
impl IProperty<f64> for PropertyDataFloat {
    fn get_property_value(&self) -> f64 {
        delegate_property_data_float! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: f64);
}

impl PropertyType for f64 {
    type Data = PropertyDataFloat;
}

impl TyToPropertyTween for f64 {
    type GdTween = SpirePropertyFloat;
}

#[delegate_impl]
impl IPropertyData for PropertyDataFloat {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
