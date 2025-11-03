use super::*;
#[derive(Debug, Clone)]
pub struct LabelIntData {
    pub property: LabelIntKind,
    pub owner: Gd<Label>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelIntKind {
    MaxLinesVisible,
    VisibleCharacters,
}
impl IProperty<i64> for LabelIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <LabelIntKind>::MaxLinesVisible => {
                let obj = &self.owner;
                (obj.get_max_lines_visible()) as i64
            }
            <LabelIntKind>::VisibleCharacters => {
                let obj = &self.owner;
                (obj.get_visible_characters()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <LabelIntKind>::MaxLinesVisible => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_max_lines_visible(val as i32);
            }
            <LabelIntKind>::VisibleCharacters => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_characters(val as i32);
            }
        }
    }
}
impl IPropertyData for LabelIntData {
    type Target = Label;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelIntKind>::MaxLinesVisible => NodePath::from("max_lines_visible"),
            <LabelIntKind>::VisibleCharacters => NodePath::from("visible_characters"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Label>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Label>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for LabelIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label>()
            .ok()
            .and_then(|owner| {
                match path {
                    "max_lines_visible" => {
                        Some(Self {
                            property: <LabelIntKind>::MaxLinesVisible,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "visible_characters" => {
                        Some(Self {
                            property: <LabelIntKind>::VisibleCharacters,
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
pub struct LabelFloatData {
    pub property: LabelFloatKind,
    pub owner: Gd<Label>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelFloatKind {
    VisibleRatio,
}
impl IProperty<f64> for LabelFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <LabelFloatKind>::VisibleRatio => {
                let obj = &self.owner;
                (obj.get_visible_ratio()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <LabelFloatKind>::VisibleRatio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_ratio(val as f32);
            }
        }
    }
}
impl IPropertyData for LabelFloatData {
    type Target = Label;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelFloatKind>::VisibleRatio => NodePath::from("visible_ratio"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Label>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Label>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for LabelFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label>()
            .ok()
            .and_then(|owner| {
                match path {
                    "visible_ratio" => {
                        Some(Self {
                            property: <LabelFloatKind>::VisibleRatio,
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
pub struct LabelStringData {
    pub property: LabelStringKind,
    pub owner: Gd<Label>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelStringKind {
    Text,
}
impl IProperty<GString> for LabelStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <LabelStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <LabelStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
        }
    }
}
impl IPropertyData for LabelStringData {
    type Target = Label;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelStringKind>::Text => NodePath::from("text"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Label>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Label>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for LabelStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <LabelStringKind>::Text,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoLabel<Marker = ()> {
    fn do_max_lines_visible(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_visible_characters(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_visible_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<Label> + Inherits<Object>> SpireDoLabel<()> for Gd<Class> {
    fn do_max_lines_visible(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = LabelIntData {
            property: <LabelIntKind>::MaxLinesVisible,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_visible_characters(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = LabelIntData {
            property: <LabelIntKind>::VisibleCharacters,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_visible_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = LabelFloatData {
            property: <LabelFloatKind>::VisibleRatio,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = LabelStringData {
            property: <LabelStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<T: WithBaseField + Inherits<Label> + Inherits<Object>> SpireDoLabel<BaseMarker>
for T {
    fn do_max_lines_visible(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelIntData {
            property: <LabelIntKind>::MaxLinesVisible,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_visible_characters(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelIntData {
            property: <LabelIntKind>::VisibleCharacters,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_visible_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelFloatData {
            property: <LabelFloatKind>::VisibleRatio,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelStringData {
            property: <LabelStringKind>::Text,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
