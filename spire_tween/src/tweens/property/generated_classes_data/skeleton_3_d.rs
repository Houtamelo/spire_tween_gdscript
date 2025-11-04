use super::*;
#[derive(Debug, Clone)]
pub struct Skeleton3DFloatData {
    pub property: Skeleton3DFloatKind,
    pub owner: Gd<Skeleton3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skeleton3DFloatKind {
    MotionScale,
}
impl IProperty<f64> for Skeleton3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <Skeleton3DFloatKind>::MotionScale => {
                let obj = &self.owner;
                (obj.get_motion_scale()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <Skeleton3DFloatKind>::MotionScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_motion_scale(val as f32);
            }
        }
    }
}
impl IPropertyData for Skeleton3DFloatData {
    type Target = Skeleton3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Skeleton3DFloatKind>::MotionScale => NodePath::from("motion_scale"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Skeleton3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Skeleton3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Skeleton3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Skeleton3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "motion_scale" => {
                        Some(Self {
                            property: <Skeleton3DFloatKind>::MotionScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoSkeleton3D<Marker = ()> {
    fn do_motion_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<Skeleton3D> + Inherits<Object>> SpireDoSkeleton3D<()>
for Gd<Class> {
    fn do_motion_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Skeleton3DFloatData {
            property: <Skeleton3DFloatKind>::MotionScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Skeleton3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<Skeleton3D> + Inherits<Object>,
> SpireDoSkeleton3D<BaseMarker> for T {
    fn do_motion_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Skeleton3D> = self.to_gd().upcast();
        let data = Skeleton3DFloatData {
            property: <Skeleton3DFloatKind>::MotionScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
