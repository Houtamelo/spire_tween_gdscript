use super::*;

#[derive(Debug, Clone)]
pub struct PropertyDataCustom {
    pub path:  NodePath,
    pub owner: ObjectOrNode,
}

impl PropertyType for Variant {
    type Data = PropertyDataCustom;
}

impl TyToPropertyTween for Variant {
    type GdTween = SpireProperty;
}

impl<T: Default + FromGodot + ToGodot> IProperty<T> for PropertyDataCustom {
    fn get_property_value(&self) -> T {
        match &self.owner {
            ObjectOrNode::Object(obj) => {
                obj.get_indexed(&self.path)
                    .try_to::<T>()
                    .map_err(|err| {
                        godot_warn!(
                            "Failed to convert property '{}' value to target type. Error: {err:?}",
                            self.path
                        );
                    })
                    .log_if_err()
                    .unwrap_or_default()
            }
            ObjectOrNode::Node(obj) => {
                obj.get_indexed(&self.path)
                    .try_to::<T>()
                    .map_err(|err| {
                        godot_warn!(
                            "Failed to convert property '{}' value to target type. Error: {err:?}",
                            self.path
                        );
                    })
                    .log_if_err()
                    .unwrap_or_default()
            }
        }
    }

    fn set_property_value(&mut self, value: T) {
        match &mut self.owner {
            ObjectOrNode::Object(obj) => obj.set_indexed(&self.path, &value.to_variant()),
            ObjectOrNode::Node(obj) => obj.set_indexed(&self.path, &value.to_variant()),
        }
    }
}

impl IPropertyData for PropertyDataCustom {
    type Target = Object;

    fn get_property_path(&self) -> NodePath { self.path.clone() }
    fn get_owner(&self) -> &ObjectOrNode { &self.owner }
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        self.owner = owner;
        true
    }
}
impl IGeneralPropertyData for PropertyDataCustom {
    fn from_path_and_owner(_path_str: &str, path: NodePath, owner: Gd<Object>) -> Self {
        Self {
            path,
            owner: ObjectOrNode::from_unchecked_object(owner),
        }
    }
}
