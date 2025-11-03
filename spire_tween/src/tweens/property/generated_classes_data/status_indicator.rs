use super::*;
#[derive(Debug, Clone)]
pub struct StatusIndicatorStringData {
    pub property: StatusIndicatorStringKind,
    pub owner: Gd<StatusIndicator>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusIndicatorStringKind {
    Tooltip,
}
impl IProperty<GString> for StatusIndicatorStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <StatusIndicatorStringKind>::Tooltip => {
                let obj = &self.owner;
                obj.get_tooltip()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <StatusIndicatorStringKind>::Tooltip => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_tooltip(&val);
            }
        }
    }
}
impl IPropertyData for StatusIndicatorStringData {
    type Target = StatusIndicator;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <StatusIndicatorStringKind>::Tooltip => NodePath::from("tooltip"),
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
impl TryFromPathAndObject for StatusIndicatorStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<StatusIndicator>()
            .ok()
            .and_then(|owner| {
                match path {
                    "tooltip" => {
                        Some(Self {
                            property: <StatusIndicatorStringKind>::Tooltip,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoStatusIndicator<Marker = ()> {
    fn do_tooltip(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<StatusIndicator> + Inherits<Object>> SpireDoStatusIndicator<()>
for Gd<Class> {
    fn do_tooltip(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = StatusIndicatorStringData {
            property: <StatusIndicatorStringKind>::Tooltip,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StatusIndicator>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<StatusIndicator> + Inherits<Object>,
> SpireDoStatusIndicator<BaseMarker> for T {
    fn do_tooltip(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<StatusIndicator> = self.to_gd().upcast();
        let data = StatusIndicatorStringData {
            property: <StatusIndicatorStringKind>::Tooltip,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
