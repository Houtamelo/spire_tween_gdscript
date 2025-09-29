use super::*;

#[delegate_impl]
impl IProperty<f64> for PropertyF64Data {
    fn get_property_value(&self) -> f64 {
        delegate_property_f_64_data! {
            self.get_property_value()
        }
    }
    fn set_property_value(&mut self, value: f64);
}

impl PropertyType for f64 {
    type Data = PropertyF64Data;
}

impl TyToGdTween for f64 {
    type GdTween = InternalTweenPropertyF64;
}

#[delegate_impl]
impl IPropertyData for PropertyF64Data {
    type Target = Object;
    fn get_property_path(&self) -> NodePath;
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool;
    fn get_owner(&self) -> &ObjectOrNode;
}
