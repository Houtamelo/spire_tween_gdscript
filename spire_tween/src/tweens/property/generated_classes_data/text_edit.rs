use super::*;
#[derive(Debug, Clone)]
pub struct TextEditIntData {
    pub property: TextEditIntKind,
    pub owner: Gd<TextEdit>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEditIntKind {
    MinimapWidth,
    ScrollHorizontal,
}
impl IProperty<i64> for TextEditIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <TextEditIntKind>::MinimapWidth => {
                let obj = &self.owner;
                (obj.get_minimap_width()) as i64
            }
            <TextEditIntKind>::ScrollHorizontal => {
                let obj = &self.owner;
                (obj.get_h_scroll()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <TextEditIntKind>::MinimapWidth => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_minimap_width(val as i32);
            }
            <TextEditIntKind>::ScrollHorizontal => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_h_scroll(val as i32);
            }
        }
    }
}
impl IPropertyData for TextEditIntData {
    type Target = TextEdit;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <TextEditIntKind>::MinimapWidth => NodePath::from("minimap_width"),
            <TextEditIntKind>::ScrollHorizontal => NodePath::from("scroll_horizontal"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<TextEdit>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<TextEdit>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for TextEditIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<TextEdit>()
            .ok()
            .and_then(|owner| {
                match path {
                    "minimap_width" => {
                        Some(Self {
                            property: <TextEditIntKind>::MinimapWidth,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scroll_horizontal" => {
                        Some(Self {
                            property: <TextEditIntKind>::ScrollHorizontal,
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
pub struct TextEditFloatData {
    pub property: TextEditFloatKind,
    pub owner: Gd<TextEdit>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEditFloatKind {
    ScrollVertical,
}
impl IProperty<f64> for TextEditFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <TextEditFloatKind>::ScrollVertical => {
                let obj = &self.owner;
                obj.get_v_scroll()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <TextEditFloatKind>::ScrollVertical => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_v_scroll(val);
            }
        }
    }
}
impl IPropertyData for TextEditFloatData {
    type Target = TextEdit;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <TextEditFloatKind>::ScrollVertical => NodePath::from("scroll_vertical"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<TextEdit>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<TextEdit>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for TextEditFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<TextEdit>()
            .ok()
            .and_then(|owner| {
                match path {
                    "scroll_vertical" => {
                        Some(Self {
                            property: <TextEditFloatKind>::ScrollVertical,
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
pub struct TextEditStringData {
    pub property: TextEditStringKind,
    pub owner: Gd<TextEdit>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEditStringKind {
    Text,
    PlaceholderText,
}
impl IProperty<GString> for TextEditStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <TextEditStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
            <TextEditStringKind>::PlaceholderText => {
                let obj = &self.owner;
                obj.get_placeholder()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <TextEditStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
            <TextEditStringKind>::PlaceholderText => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_placeholder(&val);
            }
        }
    }
}
impl IPropertyData for TextEditStringData {
    type Target = TextEdit;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <TextEditStringKind>::Text => NodePath::from("text"),
            <TextEditStringKind>::PlaceholderText => NodePath::from("placeholder_text"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<TextEdit>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<TextEdit>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for TextEditStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<TextEdit>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <TextEditStringKind>::Text,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "placeholder_text" => {
                        Some(Self {
                            property: <TextEditStringKind>::PlaceholderText,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoTextEdit<Marker = ()> {
    fn do_minimap_width(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_scroll_horizontal(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_scroll_vertical(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
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
impl<Class: Inherits<TextEdit> + Inherits<Object>> SpireDoTextEdit<()> for Gd<Class> {
    fn do_minimap_width(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = TextEditIntData {
            property: <TextEditIntKind>::MinimapWidth,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_horizontal(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = TextEditIntData {
            property: <TextEditIntKind>::ScrollHorizontal,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_vertical(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextEditFloatData {
            property: <TextEditFloatKind>::ScrollVertical,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextEdit>().upcast::<Node>(),
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
        let data = TextEditStringData {
            property: <TextEditStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextEdit>().upcast::<Node>(),
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
        let data = TextEditStringData {
            property: <TextEditStringKind>::PlaceholderText,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<TextEdit> + Inherits<Object>,
> SpireDoTextEdit<BaseMarker> for T {
    fn do_minimap_width(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<TextEdit> = self.to_gd().upcast();
        let data = TextEditIntData {
            property: <TextEditIntKind>::MinimapWidth,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_horizontal(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<TextEdit> = self.to_gd().upcast();
        let data = TextEditIntData {
            property: <TextEditIntKind>::ScrollHorizontal,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_vertical(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextEdit> = self.to_gd().upcast();
        let data = TextEditFloatData {
            property: <TextEditFloatKind>::ScrollVertical,
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
        let owner: Gd<TextEdit> = self.to_gd().upcast();
        let data = TextEditStringData {
            property: <TextEditStringKind>::Text,
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
        let owner: Gd<TextEdit> = self.to_gd().upcast();
        let data = TextEditStringData {
            property: <TextEditStringKind>::PlaceholderText,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
