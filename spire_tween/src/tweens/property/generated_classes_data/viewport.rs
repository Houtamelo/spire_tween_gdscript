use super::*;
#[derive(Debug, Clone)]
pub struct ViewportFloatData {
    pub property: ViewportFloatKind,
    pub owner: Gd<Viewport>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewportFloatKind {
    Scaling3DScale,
}
impl IProperty<f64> for ViewportFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <ViewportFloatKind>::Scaling3DScale => {
                let obj = &self.owner;
                (obj.get_scaling_3d_scale()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <ViewportFloatKind>::Scaling3DScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_scaling_3d_scale(val as f32);
            }
        }
    }
}
impl IPropertyData for ViewportFloatData {
    type Target = Viewport;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ViewportFloatKind>::Scaling3DScale => NodePath::from("scaling_3d_scale"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Viewport>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Viewport>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ViewportFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Viewport>()
            .ok()
            .and_then(|owner| {
                match path {
                    "scaling_3d_scale" => {
                        Some(Self {
                            property: <ViewportFloatKind>::Scaling3DScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoViewport<Marker = ()> {
    fn do_scaling_3d_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<Viewport> + Inherits<Object>> SpireDoViewport<()> for Gd<Class> {
    fn do_scaling_3d_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ViewportFloatData {
            property: <ViewportFloatKind>::Scaling3DScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Viewport>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<Viewport> + Inherits<Object>,
> SpireDoViewport<BaseMarker> for T {
    fn do_scaling_3d_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Viewport> = self.to_gd().upcast();
        let data = ViewportFloatData {
            property: <ViewportFloatKind>::Scaling3DScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
