use super::*;
#[derive(Debug, Clone)]
pub struct RichTextLabelIntData {
    pub property: RichTextLabelIntKind,
    pub owner: Gd<RichTextLabel>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextLabelIntKind {
    VisibleCharacters,
}
impl IProperty<i64> for RichTextLabelIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <RichTextLabelIntKind>::VisibleCharacters => {
                let obj = &self.owner;
                (obj.get_visible_characters()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <RichTextLabelIntKind>::VisibleCharacters => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_characters(val as i32);
            }
        }
    }
}
impl IPropertyData for RichTextLabelIntData {
    type Target = RichTextLabel;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RichTextLabelIntKind>::VisibleCharacters => {
                NodePath::from("visible_characters")
            }
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<RichTextLabel>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<RichTextLabel>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for RichTextLabelIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RichTextLabel>()
            .ok()
            .and_then(|owner| {
                match path {
                    "visible_characters" => {
                        Some(Self {
                            property: <RichTextLabelIntKind>::VisibleCharacters,
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
pub struct RichTextLabelFloatData {
    pub property: RichTextLabelFloatKind,
    pub owner: Gd<RichTextLabel>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextLabelFloatKind {
    VisibleRatio,
}
impl IProperty<f64> for RichTextLabelFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <RichTextLabelFloatKind>::VisibleRatio => {
                let obj = &self.owner;
                (obj.get_visible_ratio()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <RichTextLabelFloatKind>::VisibleRatio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_ratio(val as f32);
            }
        }
    }
}
impl IPropertyData for RichTextLabelFloatData {
    type Target = RichTextLabel;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RichTextLabelFloatKind>::VisibleRatio => NodePath::from("visible_ratio"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<RichTextLabel>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<RichTextLabel>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for RichTextLabelFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RichTextLabel>()
            .ok()
            .and_then(|owner| {
                match path {
                    "visible_ratio" => {
                        Some(Self {
                            property: <RichTextLabelFloatKind>::VisibleRatio,
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
pub struct RichTextLabelStringData {
    pub property: RichTextLabelStringKind,
    pub owner: Gd<RichTextLabel>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextLabelStringKind {
    Text,
}
impl IProperty<GString> for RichTextLabelStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <RichTextLabelStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <RichTextLabelStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
        }
    }
}
impl IPropertyData for RichTextLabelStringData {
    type Target = RichTextLabel;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RichTextLabelStringKind>::Text => NodePath::from("text"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<RichTextLabel>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<RichTextLabel>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for RichTextLabelStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RichTextLabel>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <RichTextLabelStringKind>::Text,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoRichTextLabel<Marker = ()> {
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
impl<Class: Inherits<RichTextLabel> + Inherits<Object>> SpireDoRichTextLabel<()>
for Gd<Class> {
    fn do_visible_characters(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::VisibleCharacters,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RichTextLabel>().upcast::<Node>(),
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
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::VisibleRatio,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RichTextLabel>().upcast::<Node>(),
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
        let data = RichTextLabelStringData {
            property: <RichTextLabelStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RichTextLabel>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<RichTextLabel> + Inherits<Object>,
> SpireDoRichTextLabel<BaseMarker> for T {
    fn do_visible_characters(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::VisibleCharacters,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::VisibleRatio,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelStringData {
            property: <RichTextLabelStringKind>::Text,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
