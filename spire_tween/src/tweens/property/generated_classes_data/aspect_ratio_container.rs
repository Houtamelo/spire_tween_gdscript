use super::*;
#[derive(Debug, Clone)]
pub struct AspectRatioContainerFloatData {
    pub property: AspectRatioContainerFloatKind,
    pub owner: Gd<AspectRatioContainer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectRatioContainerFloatKind {
    Ratio,
}
impl IProperty<f64> for AspectRatioContainerFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <AspectRatioContainerFloatKind>::Ratio => {
                let obj = &self.owner;
                (obj.get_ratio()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <AspectRatioContainerFloatKind>::Ratio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_ratio(val as f32);
            }
        }
    }
}
impl IPropertyData for AspectRatioContainerFloatData {
    type Target = AspectRatioContainer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AspectRatioContainerFloatKind>::Ratio => NodePath::from("ratio"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<AspectRatioContainer>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<AspectRatioContainer>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for AspectRatioContainerFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AspectRatioContainer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "ratio" => {
                        Some(Self {
                            property: <AspectRatioContainerFloatKind>::Ratio,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoAspectRatioContainer<Marker = ()> {
    fn do_container_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<
    Class: Inherits<AspectRatioContainer> + Inherits<Object>,
> SpireDoAspectRatioContainer<()> for Gd<Class> {
    fn do_container_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = AspectRatioContainerFloatData {
            property: <AspectRatioContainerFloatKind>::Ratio,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AspectRatioContainer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<AspectRatioContainer> + Inherits<Object>,
> SpireDoAspectRatioContainer<BaseMarker> for T {
    fn do_container_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<AspectRatioContainer> = self.to_gd().upcast();
        let data = AspectRatioContainerFloatData {
            property: <AspectRatioContainerFloatKind>::Ratio,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
