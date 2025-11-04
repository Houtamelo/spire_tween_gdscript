use super::*;
#[derive(Debug, Clone)]
pub struct GpuParticlesAttractorSphere3DFloatData {
    pub property: GpuParticlesAttractorSphere3DFloatKind,
    pub owner: Gd<GpuParticlesAttractorSphere3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuParticlesAttractorSphere3DFloatKind {
    Radius,
}
impl IProperty<f64> for GpuParticlesAttractorSphere3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <GpuParticlesAttractorSphere3DFloatKind>::Radius => {
                let obj = &self.owner;
                (obj.get_radius()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <GpuParticlesAttractorSphere3DFloatKind>::Radius => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_radius(val as f32);
            }
        }
    }
}
impl IPropertyData for GpuParticlesAttractorSphere3DFloatData {
    type Target = GpuParticlesAttractorSphere3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <GpuParticlesAttractorSphere3DFloatKind>::Radius => NodePath::from("radius"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => {
                obj.try_cast::<GpuParticlesAttractorSphere3D>().ok()
            }
            ObjectOrNode::Node(obj) => {
                obj.try_cast::<GpuParticlesAttractorSphere3D>().ok()
            }
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for GpuParticlesAttractorSphere3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<GpuParticlesAttractorSphere3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "radius" => {
                        Some(Self {
                            property: <GpuParticlesAttractorSphere3DFloatKind>::Radius,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoGpuParticlesAttractorSphere3D<Marker = ()> {
    fn do_radius(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<
    Class: Inherits<GpuParticlesAttractorSphere3D> + Inherits<Object>,
> SpireDoGpuParticlesAttractorSphere3D<()> for Gd<Class> {
    fn do_radius(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = GpuParticlesAttractorSphere3DFloatData {
            property: <GpuParticlesAttractorSphere3DFloatKind>::Radius,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<GpuParticlesAttractorSphere3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<GpuParticlesAttractorSphere3D> + Inherits<Object>,
> SpireDoGpuParticlesAttractorSphere3D<BaseMarker> for T {
    fn do_radius(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<GpuParticlesAttractorSphere3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractorSphere3DFloatData {
            property: <GpuParticlesAttractorSphere3DFloatKind>::Radius,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
