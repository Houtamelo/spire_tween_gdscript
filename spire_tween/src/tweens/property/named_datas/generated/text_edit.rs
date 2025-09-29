use super::*;
#[derive(Debug, Clone)]
pub struct TextEditF64Data {
    pub property: TextEditF64Kind,
    pub owner: Gd<TextEdit>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEditF64Kind {
    ScrollVertical,
}
impl IProperty<f64> for TextEditF64Data {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <TextEditF64Kind>::ScrollVertical => {
                let obj = &self.owner;
                obj.get_v_scroll()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <TextEditF64Kind>::ScrollVertical => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_v_scroll(val);
            }
        }
    }
}
impl IPropertyData for TextEditF64Data {
    type Target = TextEdit;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <TextEditF64Kind>::ScrollVertical => NodePath::from("scroll_vertical"),
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
impl TryFromPathAndObject for TextEditF64Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<TextEdit>()
            .ok()
            .and_then(|owner| {
                match path {
                    "scroll_vertical" => {
                        Some(Self {
                            property: <TextEditF64Kind>::ScrollVertical,
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
pub struct TextEditI32Data {
    pub property: TextEditI32Kind,
    pub owner: Gd<TextEdit>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEditI32Kind {
    MinimapWidth,
    ScrollHorizontal,
}
impl IProperty<i32> for TextEditI32Data {
    #[inline]
    fn get_property_value(&self) -> i32 {
        match self.property {
            <TextEditI32Kind>::MinimapWidth => {
                let obj = &self.owner;
                obj.get_minimap_width()
            }
            <TextEditI32Kind>::ScrollHorizontal => {
                let obj = &self.owner;
                obj.get_h_scroll()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i32) {
        match self.property {
            <TextEditI32Kind>::MinimapWidth => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_minimap_width(val);
            }
            <TextEditI32Kind>::ScrollHorizontal => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_h_scroll(val);
            }
        }
    }
}
impl IPropertyData for TextEditI32Data {
    type Target = TextEdit;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <TextEditI32Kind>::MinimapWidth => NodePath::from("minimap_width"),
            <TextEditI32Kind>::ScrollHorizontal => NodePath::from("scroll_horizontal"),
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
impl TryFromPathAndObject for TextEditI32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<TextEdit>()
            .ok()
            .and_then(|owner| {
                match path {
                    "minimap_width" => {
                        Some(Self {
                            property: <TextEditI32Kind>::MinimapWidth,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scroll_horizontal" => {
                        Some(Self {
                            property: <TextEditI32Kind>::ScrollHorizontal,
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
pub struct TextEditGStringData {
    pub property: TextEditGStringKind,
    pub owner: Gd<TextEdit>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEditGStringKind {
    Text,
    PlaceholderText,
}
impl IProperty<GString> for TextEditGStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <TextEditGStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
            <TextEditGStringKind>::PlaceholderText => {
                let obj = &self.owner;
                obj.get_placeholder()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <TextEditGStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
            <TextEditGStringKind>::PlaceholderText => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_placeholder(&val);
            }
        }
    }
}
impl IPropertyData for TextEditGStringData {
    type Target = TextEdit;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <TextEditGStringKind>::Text => NodePath::from("text"),
            <TextEditGStringKind>::PlaceholderText => NodePath::from("placeholder_text"),
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
impl TryFromPathAndObject for TextEditGStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<TextEdit>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <TextEditGStringKind>::Text,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "placeholder_text" => {
                        Some(Self {
                            property: <TextEditGStringKind>::PlaceholderText,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoTextEdit<Marker = ()> {
    fn do_scroll_vertical(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_minimap_width(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>>;
    fn do_scroll_horizontal(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>>;
    fn text_edit_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
    fn text_edit_do_placeholder(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<TextEdit> + Inherits<Object>> DoTextEdit<()> for Gd<Class> {
    fn do_scroll_vertical(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextEditF64Data {
            property: <TextEditF64Kind>::ScrollVertical,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_minimap_width(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let data = TextEditI32Data {
            property: <TextEditI32Kind>::MinimapWidth,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_scroll_horizontal(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let data = TextEditI32Data {
            property: <TextEditI32Kind>::ScrollHorizontal,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn text_edit_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = TextEditGStringData {
            property: <TextEditGStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn text_edit_do_placeholder(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = TextEditGStringData {
            property: <TextEditGStringKind>::PlaceholderText,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<TextEdit> + Inherits<Object>> DoTextEdit<BaseMarker>
for T {
    fn do_scroll_vertical(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextEdit> = self.to_gd().upcast();
        let data = TextEditF64Data {
            property: <TextEditF64Kind>::ScrollVertical,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_minimap_width(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<TextEdit> = self.to_gd().upcast();
        let data = TextEditI32Data {
            property: <TextEditI32Kind>::MinimapWidth,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_scroll_horizontal(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<TextEdit> = self.to_gd().upcast();
        let data = TextEditI32Data {
            property: <TextEditI32Kind>::ScrollHorizontal,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn text_edit_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<TextEdit> = self.to_gd().upcast();
        let data = TextEditGStringData {
            property: <TextEditGStringKind>::Text,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn text_edit_do_placeholder(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<TextEdit> = self.to_gd().upcast();
        let data = TextEditGStringData {
            property: <TextEditGStringKind>::PlaceholderText,
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
pub struct TextEditTweener {}
#[godot_api]
impl TextEditTweener {
    #[func]
    fn do_scroll_vertical(
        node: Gd<TextEdit>,
        to: f64,
        duration: f64,
    ) -> Gd<<f64 as TyToGdTween>::GdTween> {
        let gd = <f64 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_scroll_vertical(to, duration), gd
        }
    }
    #[func]
    fn do_minimap_width(
        node: Gd<TextEdit>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        let gd = <i32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_minimap_width(to, duration), gd
        }
    }
    #[func]
    fn do_scroll_horizontal(
        node: Gd<TextEdit>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        let gd = <i32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_scroll_horizontal(to, duration), gd
        }
    }
    #[func]
    fn text_edit_do_text(
        node: Gd<TextEdit>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.text_edit_do_text(to, duration), gd
        }
    }
    #[func]
    fn text_edit_do_placeholder(
        node: Gd<TextEdit>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.text_edit_do_placeholder(to, duration), gd
        }
    }
}
