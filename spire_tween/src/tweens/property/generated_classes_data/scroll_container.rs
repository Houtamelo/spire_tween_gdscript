use super::*;
#[derive(Debug, Clone)]
pub struct ScrollContainerIntData {
    pub property: ScrollContainerIntKind,
    pub owner: Gd<ScrollContainer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollContainerIntKind {
    ScrollHorizontal,
    ScrollVertical,
}
impl IProperty<i64> for ScrollContainerIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <ScrollContainerIntKind>::ScrollHorizontal => {
                let obj = &self.owner;
                (obj.get_h_scroll()) as i64
            }
            <ScrollContainerIntKind>::ScrollVertical => {
                let obj = &self.owner;
                (obj.get_v_scroll()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <ScrollContainerIntKind>::ScrollHorizontal => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_h_scroll(val as i32);
            }
            <ScrollContainerIntKind>::ScrollVertical => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_v_scroll(val as i32);
            }
        }
    }
}
impl IPropertyData for ScrollContainerIntData {
    type Target = ScrollContainer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ScrollContainerIntKind>::ScrollHorizontal => {
                NodePath::from("scroll_horizontal")
            }
            <ScrollContainerIntKind>::ScrollVertical => NodePath::from("scroll_vertical"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for ScrollContainerIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ScrollContainer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "scroll_horizontal" => {
                        Some(Self {
                            property: <ScrollContainerIntKind>::ScrollHorizontal,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "scroll_vertical" => {
                        Some(Self {
                            property: <ScrollContainerIntKind>::ScrollVertical,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoScrollContainer<Marker = ()> {
    fn do_scroll_horizontal(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_scroll_vertical(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
}
impl<Class: Inherits<ScrollContainer> + Inherits<Object>> SpireDoScrollContainer<()>
for Gd<Class> {
    fn do_scroll_horizontal(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = ScrollContainerIntData {
            property: <ScrollContainerIntKind>::ScrollHorizontal,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_vertical(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = ScrollContainerIntData {
            property: <ScrollContainerIntKind>::ScrollVertical,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<ScrollContainer> + Inherits<Object>,
> SpireDoScrollContainer<BaseMarker> for T {
    fn do_scroll_horizontal(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<ScrollContainer> = self.to_gd().upcast();
        let data = ScrollContainerIntData {
            property: <ScrollContainerIntKind>::ScrollHorizontal,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_vertical(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<ScrollContainer> = self.to_gd().upcast();
        let data = ScrollContainerIntData {
            property: <ScrollContainerIntKind>::ScrollVertical,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
