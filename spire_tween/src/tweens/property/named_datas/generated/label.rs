use super::*;
#[derive(Debug, Clone)]
pub struct LabelF32Data {
    pub property: LabelF32Kind,
    pub owner: Gd<Label>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelF32Kind {
    VisibleRatio,
}
impl IProperty<f32> for LabelF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <LabelF32Kind>::VisibleRatio => {
                let obj = &self.owner;
                obj.get_visible_ratio()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <LabelF32Kind>::VisibleRatio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_ratio(val);
            }
        }
    }
}
impl IPropertyData for LabelF32Data {
    type Target = Label;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelF32Kind>::VisibleRatio => NodePath::from("visible_ratio"),
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
impl TryFromPathAndObject for LabelF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label>()
            .ok()
            .and_then(|owner| {
                match path {
                    "visible_ratio" => {
                        Some(Self {
                            property: <LabelF32Kind>::VisibleRatio,
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
pub struct LabelI32Data {
    pub property: LabelI32Kind,
    pub owner: Gd<Label>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelI32Kind {
    MaxLinesVisible,
    VisibleCharacters,
}
impl IProperty<i32> for LabelI32Data {
    #[inline]
    fn get_property_value(&self) -> i32 {
        match self.property {
            <LabelI32Kind>::MaxLinesVisible => {
                let obj = &self.owner;
                obj.get_max_lines_visible()
            }
            <LabelI32Kind>::VisibleCharacters => {
                let obj = &self.owner;
                obj.get_visible_characters()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i32) {
        match self.property {
            <LabelI32Kind>::MaxLinesVisible => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_max_lines_visible(val);
            }
            <LabelI32Kind>::VisibleCharacters => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_characters(val);
            }
        }
    }
}
impl IPropertyData for LabelI32Data {
    type Target = Label;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelI32Kind>::MaxLinesVisible => NodePath::from("max_lines_visible"),
            <LabelI32Kind>::VisibleCharacters => NodePath::from("visible_characters"),
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
impl TryFromPathAndObject for LabelI32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label>()
            .ok()
            .and_then(|owner| {
                match path {
                    "max_lines_visible" => {
                        Some(Self {
                            property: <LabelI32Kind>::MaxLinesVisible,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "visible_characters" => {
                        Some(Self {
                            property: <LabelI32Kind>::VisibleCharacters,
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
pub struct LabelGStringData {
    pub property: LabelGStringKind,
    pub owner: Gd<Label>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelGStringKind {
    Text,
}
impl IProperty<GString> for LabelGStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <LabelGStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <LabelGStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
        }
    }
}
impl IPropertyData for LabelGStringData {
    type Target = Label;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelGStringKind>::Text => NodePath::from("text"),
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
impl TryFromPathAndObject for LabelGStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <LabelGStringKind>::Text,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoLabel<Marker = ()> {
    fn label_do_visible_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn label_do_max_lines_visible(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>>;
    fn label_do_visible_characters(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>>;
    fn label_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<Label> + Inherits<Object>> DoLabel<()> for Gd<Class> {
    fn label_do_visible_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = LabelF32Data {
            property: <LabelF32Kind>::VisibleRatio,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn label_do_max_lines_visible(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let data = LabelI32Data {
            property: <LabelI32Kind>::MaxLinesVisible,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn label_do_visible_characters(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let data = LabelI32Data {
            property: <LabelI32Kind>::VisibleCharacters,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn label_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = LabelGStringData {
            property: <LabelGStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<Label> + Inherits<Object>> DoLabel<BaseMarker> for T {
    fn label_do_visible_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelF32Data {
            property: <LabelF32Kind>::VisibleRatio,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn label_do_max_lines_visible(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelI32Data {
            property: <LabelI32Kind>::MaxLinesVisible,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn label_do_visible_characters(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelI32Data {
            property: <LabelI32Kind>::VisibleCharacters,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn label_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelGStringData {
            property: <LabelGStringKind>::Text,
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
pub struct LabelTweener {}
#[godot_api]
impl LabelTweener {
    #[func]
    fn label_do_visible_ratio(
        node: Gd<Label>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.label_do_visible_ratio(to, duration), gd
        }
    }
    #[func]
    fn label_do_max_lines_visible(
        node: Gd<Label>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        let gd = <i32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.label_do_max_lines_visible(to, duration), gd
        }
    }
    #[func]
    fn label_do_visible_characters(
        node: Gd<Label>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        let gd = <i32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.label_do_visible_characters(to, duration), gd
        }
    }
    #[func]
    fn label_do_text(
        node: Gd<Label>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.label_do_text(to, duration), gd
        }
    }
}
