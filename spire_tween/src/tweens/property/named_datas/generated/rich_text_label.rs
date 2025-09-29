use super::*;
#[derive(Debug, Clone)]
pub struct RichTextLabelF32Data {
    pub property: RichTextLabelF32Kind,
    pub owner: Gd<RichTextLabel>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextLabelF32Kind {
    VisibleRatio,
}
impl IProperty<f32> for RichTextLabelF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <RichTextLabelF32Kind>::VisibleRatio => {
                let obj = &self.owner;
                obj.get_visible_ratio()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <RichTextLabelF32Kind>::VisibleRatio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_ratio(val);
            }
        }
    }
}
impl IPropertyData for RichTextLabelF32Data {
    type Target = RichTextLabel;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RichTextLabelF32Kind>::VisibleRatio => NodePath::from("visible_ratio"),
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
impl TryFromPathAndObject for RichTextLabelF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RichTextLabel>()
            .ok()
            .and_then(|owner| {
                match path {
                    "visible_ratio" => {
                        Some(Self {
                            property: <RichTextLabelF32Kind>::VisibleRatio,
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
pub struct RichTextLabelI32Data {
    pub property: RichTextLabelI32Kind,
    pub owner: Gd<RichTextLabel>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextLabelI32Kind {
    VisibleCharacters,
}
impl IProperty<i32> for RichTextLabelI32Data {
    #[inline]
    fn get_property_value(&self) -> i32 {
        match self.property {
            <RichTextLabelI32Kind>::VisibleCharacters => {
                let obj = &self.owner;
                obj.get_visible_characters()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i32) {
        match self.property {
            <RichTextLabelI32Kind>::VisibleCharacters => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_characters(val);
            }
        }
    }
}
impl IPropertyData for RichTextLabelI32Data {
    type Target = RichTextLabel;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RichTextLabelI32Kind>::VisibleCharacters => {
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
impl TryFromPathAndObject for RichTextLabelI32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RichTextLabel>()
            .ok()
            .and_then(|owner| {
                match path {
                    "visible_characters" => {
                        Some(Self {
                            property: <RichTextLabelI32Kind>::VisibleCharacters,
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
pub struct RichTextLabelGStringData {
    pub property: RichTextLabelGStringKind,
    pub owner: Gd<RichTextLabel>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextLabelGStringKind {
    Text,
}
impl IProperty<GString> for RichTextLabelGStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <RichTextLabelGStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <RichTextLabelGStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
        }
    }
}
impl IPropertyData for RichTextLabelGStringData {
    type Target = RichTextLabel;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RichTextLabelGStringKind>::Text => NodePath::from("text"),
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
impl TryFromPathAndObject for RichTextLabelGStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RichTextLabel>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <RichTextLabelGStringKind>::Text,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoRichTextLabel<Marker = ()> {
    fn rich_do_visible_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn rich_do_visible_characters(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>>;
    fn rich_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<RichTextLabel> + Inherits<Object>> DoRichTextLabel<()>
for Gd<Class> {
    fn rich_do_visible_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = RichTextLabelF32Data {
            property: <RichTextLabelF32Kind>::VisibleRatio,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RichTextLabel>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rich_do_visible_characters(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let data = RichTextLabelI32Data {
            property: <RichTextLabelI32Kind>::VisibleCharacters,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RichTextLabel>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rich_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = RichTextLabelGStringData {
            property: <RichTextLabelGStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RichTextLabel>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<RichTextLabel> + Inherits<Object>,
> DoRichTextLabel<BaseMarker> for T {
    fn rich_do_visible_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelF32Data {
            property: <RichTextLabelF32Kind>::VisibleRatio,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rich_do_visible_characters(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelI32Data {
            property: <RichTextLabelI32Kind>::VisibleCharacters,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rich_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelGStringData {
            property: <RichTextLabelGStringKind>::Text,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct RichTextLabelTweener {}
#[godot_api]
impl RichTextLabelTweener {
    #[func]
    fn rich_do_visible_ratio(
        node: Gd<RichTextLabel>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.rich_do_visible_ratio(to, duration), gd
        }
    }
    #[func]
    fn rich_do_visible_characters(
        node: Gd<RichTextLabel>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        let gd = <i32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.rich_do_visible_characters(to, duration), gd
        }
    }
    #[func]
    fn rich_do_text(
        node: Gd<RichTextLabel>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.rich_do_text(to, duration), gd
        }
    }
}
