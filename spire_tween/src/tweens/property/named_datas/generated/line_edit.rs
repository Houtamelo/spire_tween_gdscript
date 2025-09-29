use super::*;
#[derive(Debug, Clone)]
pub struct LineEditI32Data {
    pub property: LineEditI32Kind,
    pub owner: Gd<LineEdit>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEditI32Kind {
    MaxLength,
}
impl IProperty<i32> for LineEditI32Data {
    #[inline]
    fn get_property_value(&self) -> i32 {
        match self.property {
            <LineEditI32Kind>::MaxLength => {
                let obj = &self.owner;
                obj.get_max_length()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i32) {
        match self.property {
            <LineEditI32Kind>::MaxLength => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_max_length(val);
            }
        }
    }
}
impl IPropertyData for LineEditI32Data {
    type Target = LineEdit;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LineEditI32Kind>::MaxLength => NodePath::from("max_length"),
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
impl TryFromPathAndObject for LineEditI32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<LineEdit>()
            .ok()
            .and_then(|owner| {
                match path {
                    "max_length" => {
                        Some(Self {
                            property: <LineEditI32Kind>::MaxLength,
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
pub struct LineEditGStringData {
    pub property: LineEditGStringKind,
    pub owner: Gd<LineEdit>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEditGStringKind {
    Text,
    PlaceholderText,
}
impl IProperty<GString> for LineEditGStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <LineEditGStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
            <LineEditGStringKind>::PlaceholderText => {
                let obj = &self.owner;
                obj.get_placeholder()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <LineEditGStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
            <LineEditGStringKind>::PlaceholderText => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_placeholder(&val);
            }
        }
    }
}
impl IPropertyData for LineEditGStringData {
    type Target = LineEdit;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LineEditGStringKind>::Text => NodePath::from("text"),
            <LineEditGStringKind>::PlaceholderText => NodePath::from("placeholder_text"),
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
impl TryFromPathAndObject for LineEditGStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<LineEdit>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <LineEditGStringKind>::Text,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "placeholder_text" => {
                        Some(Self {
                            property: <LineEditGStringKind>::PlaceholderText,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoLineEdit<Marker = ()> {
    fn do_max_length(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>>;
    fn line_edit_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
    fn line_edit_do_placeholder(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<LineEdit> + Inherits<Object>> DoLineEdit<()> for Gd<Class> {
    fn do_max_length(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let data = LineEditI32Data {
            property: <LineEditI32Kind>::MaxLength,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<LineEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn line_edit_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = LineEditGStringData {
            property: <LineEditGStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<LineEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn line_edit_do_placeholder(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = LineEditGStringData {
            property: <LineEditGStringKind>::PlaceholderText,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<LineEdit>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<LineEdit> + Inherits<Object>> DoLineEdit<BaseMarker>
for T {
    fn do_max_length(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<LineEdit> = self.to_gd().upcast();
        let data = LineEditI32Data {
            property: <LineEditI32Kind>::MaxLength,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn line_edit_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<LineEdit> = self.to_gd().upcast();
        let data = LineEditGStringData {
            property: <LineEditGStringKind>::Text,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn line_edit_do_placeholder(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<LineEdit> = self.to_gd().upcast();
        let data = LineEditGStringData {
            property: <LineEditGStringKind>::PlaceholderText,
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
pub struct LineEditTweener {}
#[godot_api]
impl LineEditTweener {
    #[func]
    fn do_max_length(
        node: Gd<LineEdit>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        let gd = <i32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_max_length(to, duration), gd
        }
    }
    #[func]
    fn line_edit_do_text(
        node: Gd<LineEdit>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.line_edit_do_text(to, duration), gd
        }
    }
    #[func]
    fn line_edit_do_placeholder(
        node: Gd<LineEdit>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.line_edit_do_placeholder(to, duration), gd
        }
    }
}
