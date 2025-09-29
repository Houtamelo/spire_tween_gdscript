use super::*;
#[derive(Debug, Clone)]
pub struct StatusIndicatorGStringData {
    pub property: StatusIndicatorGStringKind,
    pub owner: Gd<StatusIndicator>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusIndicatorGStringKind {
    Tooltip,
}
impl IProperty<GString> for StatusIndicatorGStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <StatusIndicatorGStringKind>::Tooltip => {
                let obj = &self.owner;
                obj.get_tooltip()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <StatusIndicatorGStringKind>::Tooltip => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_tooltip(&val);
            }
        }
    }
}
impl IPropertyData for StatusIndicatorGStringData {
    type Target = StatusIndicator;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <StatusIndicatorGStringKind>::Tooltip => NodePath::from("tooltip"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<StatusIndicator>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<StatusIndicator>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for StatusIndicatorGStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<StatusIndicator>()
            .ok()
            .and_then(|owner| {
                match path {
                    "tooltip" => {
                        Some(Self {
                            property: <StatusIndicatorGStringKind>::Tooltip,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoStatusIndicator<Marker = ()> {
    fn do_tooltip(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<StatusIndicator> + Inherits<Object>> DoStatusIndicator<()>
for Gd<Class> {
    fn do_tooltip(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = StatusIndicatorGStringData {
            property: <StatusIndicatorGStringKind>::Tooltip,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StatusIndicator>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<StatusIndicator> + Inherits<Object>,
> DoStatusIndicator<BaseMarker> for T {
    fn do_tooltip(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<StatusIndicator> = self.to_gd().upcast();
        let data = StatusIndicatorGStringData {
            property: <StatusIndicatorGStringKind>::Tooltip,
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
pub struct StatusIndicatorTweener {}
#[godot_api]
impl StatusIndicatorTweener {
    #[func]
    fn do_tooltip(
        node: Gd<StatusIndicator>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tooltip(to, duration), gd
        }
    }
}
