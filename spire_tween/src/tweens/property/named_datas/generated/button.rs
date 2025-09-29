use super::*;
#[derive(Debug, Clone)]
pub struct ButtonGStringData {
    pub property: ButtonGStringKind,
    pub owner: Gd<Button>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonGStringKind {
    Text,
}
impl IProperty<GString> for ButtonGStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <ButtonGStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <ButtonGStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
        }
    }
}
impl IPropertyData for ButtonGStringData {
    type Target = Button;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ButtonGStringKind>::Text => NodePath::from("text"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Button>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Button>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ButtonGStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Button>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <ButtonGStringKind>::Text,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoButton<Marker = ()> {
    fn button_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<Button> + Inherits<Object>> DoButton<()> for Gd<Class> {
    fn button_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = ButtonGStringData {
            property: <ButtonGStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Button>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<Button> + Inherits<Object>> DoButton<BaseMarker> for T {
    fn button_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<Button> = self.to_gd().upcast();
        let data = ButtonGStringData {
            property: <ButtonGStringKind>::Text,
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
pub struct ButtonTweener {}
#[godot_api]
impl ButtonTweener {
    #[func]
    fn button_do_text(
        node: Gd<Button>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.button_do_text(to, duration), gd
        }
    }
}
