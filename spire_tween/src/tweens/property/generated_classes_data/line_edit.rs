use super::*;
#[derive(Debug, Clone)]
pub struct LineEditIntData {
    pub property: LineEditIntKind,
    pub owner: Gd<LineEdit>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEditIntKind {
    MaxLength,
}
impl IProperty<i64> for LineEditIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <LineEditIntKind>::MaxLength => {
                let obj = &self.owner;
                (obj.get_max_length()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <LineEditIntKind>::MaxLength => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_max_length(val as i32);
            }
        }
    }
}
impl IPropertyData for LineEditIntData {
    type Target = LineEdit;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LineEditIntKind>::MaxLength => NodePath::from("max_length"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<LineEdit>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<LineEdit>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for LineEditIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<LineEdit>()
            .ok()
            .and_then(|owner| {
                match path {
                    "max_length" => {
                        Some(Self {
                            property: <LineEditIntKind>::MaxLength,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
#[derive(Debug, Clone)]
pub struct LineEditStringData {
    pub property: LineEditStringKind,
    pub owner: Gd<LineEdit>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEditStringKind {
    Text,
    PlaceholderText,
}
impl IProperty<GString> for LineEditStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <LineEditStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
            <LineEditStringKind>::PlaceholderText => {
                let obj = &self.owner;
                obj.get_placeholder()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <LineEditStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
            <LineEditStringKind>::PlaceholderText => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_placeholder(&val);
            }
        }
    }
}
impl IPropertyData for LineEditStringData {
    type Target = LineEdit;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LineEditStringKind>::Text => NodePath::from("text"),
            <LineEditStringKind>::PlaceholderText => NodePath::from("placeholder_text"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<LineEdit>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<LineEdit>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for LineEditStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<LineEdit>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <LineEditStringKind>::Text,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "placeholder_text" => {
                        Some(Self {
                            property: <LineEditStringKind>::PlaceholderText,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoLineEdit<Marker = ()> {
    fn do_max_length(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
    fn do_placeholder_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<LineEdit> + Inherits<Object>> SpireDoLineEdit<()> for Gd<Class> {
    fn do_max_length(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = LineEditIntData {
            property: <LineEditIntKind>::MaxLength,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<LineEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = LineEditStringData {
            property: <LineEditStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<LineEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_placeholder_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = LineEditStringData {
            property: <LineEditStringKind>::PlaceholderText,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<LineEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<LineEdit> + Inherits<Object>,
> SpireDoLineEdit<BaseMarker> for T {
    fn do_max_length(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<LineEdit> = self.to_gd().upcast();
        let data = LineEditIntData {
            property: <LineEditIntKind>::MaxLength,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<LineEdit> = self.to_gd().upcast();
        let data = LineEditStringData {
            property: <LineEditStringKind>::Text,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_placeholder_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<LineEdit> = self.to_gd().upcast();
        let data = LineEditStringData {
            property: <LineEditStringKind>::PlaceholderText,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
