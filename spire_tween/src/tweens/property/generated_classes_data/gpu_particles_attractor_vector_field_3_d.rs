use super::*;
#[derive(Debug, Clone)]
pub struct GpuParticlesAttractorVectorField3DFloatData {
    pub property: GpuParticlesAttractorVectorField3DFloatKind,
    pub owner: Gd<GpuParticlesAttractorVectorField3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuParticlesAttractorVectorField3DFloatKind {
    SizeX,
    SizeY,
    SizeZ,
}
impl IProperty<f64> for GpuParticlesAttractorVectorField3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <GpuParticlesAttractorVectorField3DFloatKind>::SizeX => {
                let obj = &self.owner;
                (obj.get_size().x) as f64
            }
            <GpuParticlesAttractorVectorField3DFloatKind>::SizeY => {
                let obj = &self.owner;
                (obj.get_size().y) as f64
            }
            <GpuParticlesAttractorVectorField3DFloatKind>::SizeZ => {
                let obj = &self.owner;
                (obj.get_size().z) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <GpuParticlesAttractorVectorField3DFloatKind>::SizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.x = val as f32;
                obj.set_size(prop_val);
            }
            <GpuParticlesAttractorVectorField3DFloatKind>::SizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.y = val as f32;
                obj.set_size(prop_val);
            }
            <GpuParticlesAttractorVectorField3DFloatKind>::SizeZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.z = val as f32;
                obj.set_size(prop_val);
            }
        }
    }
}
impl IPropertyData for GpuParticlesAttractorVectorField3DFloatData {
    type Target = GpuParticlesAttractorVectorField3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <GpuParticlesAttractorVectorField3DFloatKind>::SizeX => {
                NodePath::from("size:x")
            }
            <GpuParticlesAttractorVectorField3DFloatKind>::SizeY => {
                NodePath::from("size:y")
            }
            <GpuParticlesAttractorVectorField3DFloatKind>::SizeZ => {
                NodePath::from("size:z")
            }
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
                obj.try_cast::<GpuParticlesAttractorVectorField3D>().ok()
            }
            ObjectOrNode::Node(obj) => {
                obj.try_cast::<GpuParticlesAttractorVectorField3D>().ok()
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
impl TryFromPathAndObject for GpuParticlesAttractorVectorField3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<GpuParticlesAttractorVectorField3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "size:x" => {
                        Some(Self {
                            property: <GpuParticlesAttractorVectorField3DFloatKind>::SizeX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:y" => {
                        Some(Self {
                            property: <GpuParticlesAttractorVectorField3DFloatKind>::SizeY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:z" => {
                        Some(Self {
                            property: <GpuParticlesAttractorVectorField3DFloatKind>::SizeZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
#[derive(Debug, Clone)]
pub struct GpuParticlesAttractorVectorField3DVector3Data {
    pub property: GpuParticlesAttractorVectorField3DVector3Kind,
    pub owner: Gd<GpuParticlesAttractorVectorField3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuParticlesAttractorVectorField3DVector3Kind {
    Size,
}
impl IProperty<Vector3> for GpuParticlesAttractorVectorField3DVector3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <GpuParticlesAttractorVectorField3DVector3Kind>::Size => {
                let obj = &self.owner;
                obj.get_size()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <GpuParticlesAttractorVectorField3DVector3Kind>::Size => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size(val);
            }
        }
    }
}
impl IPropertyData for GpuParticlesAttractorVectorField3DVector3Data {
    type Target = GpuParticlesAttractorVectorField3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <GpuParticlesAttractorVectorField3DVector3Kind>::Size => {
                NodePath::from("size")
            }
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
                obj.try_cast::<GpuParticlesAttractorVectorField3D>().ok()
            }
            ObjectOrNode::Node(obj) => {
                obj.try_cast::<GpuParticlesAttractorVectorField3D>().ok()
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
impl TryFromPathAndObject for GpuParticlesAttractorVectorField3DVector3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<GpuParticlesAttractorVectorField3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "size" => {
                        Some(Self {
                            property: <GpuParticlesAttractorVectorField3DVector3Kind>::Size,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoGpuParticlesAttractorVectorField3D<Marker = ()> {
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_size_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_size(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
}
impl<
    Class: Inherits<GpuParticlesAttractorVectorField3D> + Inherits<Object>,
> SpireDoGpuParticlesAttractorVectorField3D<()> for Gd<Class> {
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = GpuParticlesAttractorVectorField3DFloatData {
            property: <GpuParticlesAttractorVectorField3DFloatKind>::SizeX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self
                    .clone()
                    .upcast::<GpuParticlesAttractorVectorField3D>()
                    .upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = GpuParticlesAttractorVectorField3DFloatData {
            property: <GpuParticlesAttractorVectorField3DFloatKind>::SizeY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self
                    .clone()
                    .upcast::<GpuParticlesAttractorVectorField3D>()
                    .upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = GpuParticlesAttractorVectorField3DFloatData {
            property: <GpuParticlesAttractorVectorField3DFloatKind>::SizeZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self
                    .clone()
                    .upcast::<GpuParticlesAttractorVectorField3D>()
                    .upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = GpuParticlesAttractorVectorField3DVector3Data {
            property: <GpuParticlesAttractorVectorField3DVector3Kind>::Size,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self
                    .clone()
                    .upcast::<GpuParticlesAttractorVectorField3D>()
                    .upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<GpuParticlesAttractorVectorField3D> + Inherits<Object>,
> SpireDoGpuParticlesAttractorVectorField3D<BaseMarker> for T {
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<GpuParticlesAttractorVectorField3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractorVectorField3DFloatData {
            property: <GpuParticlesAttractorVectorField3DFloatKind>::SizeX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<GpuParticlesAttractorVectorField3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractorVectorField3DFloatData {
            property: <GpuParticlesAttractorVectorField3DFloatKind>::SizeY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<GpuParticlesAttractorVectorField3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractorVectorField3DFloatData {
            property: <GpuParticlesAttractorVectorField3DFloatKind>::SizeZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<GpuParticlesAttractorVectorField3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractorVectorField3DVector3Data {
            property: <GpuParticlesAttractorVectorField3DVector3Kind>::Size,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
