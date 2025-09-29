use super::*;
#[derive(Debug, Clone)]
pub struct LinkButtonGStringData {
    pub property: LinkButtonGStringKind,
    pub owner: Gd<LinkButton>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkButtonGStringKind {
    Text,
}
impl IProperty<GString> for LinkButtonGStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <LinkButtonGStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <LinkButtonGStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
        }
    }
}
impl IPropertyData for LinkButtonGStringData {
    type Target = LinkButton;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LinkButtonGStringKind>::Text => NodePath::from("text"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<LinkButton>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<LinkButton>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for LinkButtonGStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<LinkButton>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <LinkButtonGStringKind>::Text,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoLinkButton<Marker = ()> {
    fn link_button_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<LinkButton> + Inherits<Object>> DoLinkButton<()> for Gd<Class> {
    fn link_button_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = LinkButtonGStringData {
            property: <LinkButtonGStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<LinkButton>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<LinkButton> + Inherits<Object>> DoLinkButton<BaseMarker>
for T {
    fn link_button_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<LinkButton> = self.to_gd().upcast();
        let data = LinkButtonGStringData {
            property: <LinkButtonGStringKind>::Text,
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
pub struct LinkButtonTweener {}
#[godot_api]
impl LinkButtonTweener {
    #[func]
    fn link_button_do_text(
        node: Gd<LinkButton>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.link_button_do_text(to, duration), gd
        }
    }
}
