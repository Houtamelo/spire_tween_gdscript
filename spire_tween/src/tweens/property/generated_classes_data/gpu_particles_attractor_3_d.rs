use super::*;
#[derive(Debug, Clone)]
pub struct GpuParticlesAttractor3DFloatData {
    pub property: GpuParticlesAttractor3DFloatKind,
    pub owner: Gd<GpuParticlesAttractor3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuParticlesAttractor3DFloatKind {
    Attenuation,
    Directionality,
    Strength,
}
impl IProperty<f64> for GpuParticlesAttractor3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <GpuParticlesAttractor3DFloatKind>::Attenuation => {
                let obj = &self.owner;
                (obj.get_attenuation()) as f64
            }
            <GpuParticlesAttractor3DFloatKind>::Directionality => {
                let obj = &self.owner;
                (obj.get_directionality()) as f64
            }
            <GpuParticlesAttractor3DFloatKind>::Strength => {
                let obj = &self.owner;
                (obj.get_strength()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <GpuParticlesAttractor3DFloatKind>::Attenuation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_attenuation(val as f32);
            }
            <GpuParticlesAttractor3DFloatKind>::Directionality => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_directionality(val as f32);
            }
            <GpuParticlesAttractor3DFloatKind>::Strength => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_strength(val as f32);
            }
        }
    }
}
impl IPropertyData for GpuParticlesAttractor3DFloatData {
    type Target = GpuParticlesAttractor3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <GpuParticlesAttractor3DFloatKind>::Attenuation => {
                NodePath::from("attenuation")
            }
            <GpuParticlesAttractor3DFloatKind>::Directionality => {
                NodePath::from("directionality")
            }
            <GpuParticlesAttractor3DFloatKind>::Strength => NodePath::from("strength"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<GpuParticlesAttractor3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<GpuParticlesAttractor3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for GpuParticlesAttractor3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<GpuParticlesAttractor3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "attenuation" => {
                        Some(Self {
                            property: <GpuParticlesAttractor3DFloatKind>::Attenuation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "directionality" => {
                        Some(Self {
                            property: <GpuParticlesAttractor3DFloatKind>::Directionality,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "strength" => {
                        Some(Self {
                            property: <GpuParticlesAttractor3DFloatKind>::Strength,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoGpuParticlesAttractor3D<Marker = ()> {
    fn do_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_directionality(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_strength(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<
    Class: Inherits<GpuParticlesAttractor3D> + Inherits<Object>,
> SpireDoGpuParticlesAttractor3D<()> for Gd<Class> {
    fn do_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = GpuParticlesAttractor3DFloatData {
            property: <GpuParticlesAttractor3DFloatKind>::Attenuation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<GpuParticlesAttractor3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_directionality(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = GpuParticlesAttractor3DFloatData {
            property: <GpuParticlesAttractor3DFloatKind>::Directionality,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<GpuParticlesAttractor3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_strength(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = GpuParticlesAttractor3DFloatData {
            property: <GpuParticlesAttractor3DFloatKind>::Strength,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<GpuParticlesAttractor3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<GpuParticlesAttractor3D> + Inherits<Object>,
> SpireDoGpuParticlesAttractor3D<BaseMarker> for T {
    fn do_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<GpuParticlesAttractor3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractor3DFloatData {
            property: <GpuParticlesAttractor3DFloatKind>::Attenuation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_directionality(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<GpuParticlesAttractor3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractor3DFloatData {
            property: <GpuParticlesAttractor3DFloatKind>::Directionality,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_strength(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<GpuParticlesAttractor3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractor3DFloatData {
            property: <GpuParticlesAttractor3DFloatKind>::Strength,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
