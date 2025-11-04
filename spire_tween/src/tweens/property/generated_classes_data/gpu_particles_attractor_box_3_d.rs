use super::*;
#[derive(Debug, Clone)]
pub struct GpuParticlesAttractorBox3DFloatData {
    pub property: GpuParticlesAttractorBox3DFloatKind,
    pub owner: Gd<GpuParticlesAttractorBox3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuParticlesAttractorBox3DFloatKind {
    SizeX,
    SizeY,
    SizeZ,
}
impl IProperty<f64> for GpuParticlesAttractorBox3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <GpuParticlesAttractorBox3DFloatKind>::SizeX => {
                let obj = &self.owner;
                (obj.get_size().x) as f64
            }
            <GpuParticlesAttractorBox3DFloatKind>::SizeY => {
                let obj = &self.owner;
                (obj.get_size().y) as f64
            }
            <GpuParticlesAttractorBox3DFloatKind>::SizeZ => {
                let obj = &self.owner;
                (obj.get_size().z) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <GpuParticlesAttractorBox3DFloatKind>::SizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.x = val as f32;
                obj.set_size(prop_val);
            }
            <GpuParticlesAttractorBox3DFloatKind>::SizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.y = val as f32;
                obj.set_size(prop_val);
            }
            <GpuParticlesAttractorBox3DFloatKind>::SizeZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.z = val as f32;
                obj.set_size(prop_val);
            }
        }
    }
}
impl IPropertyData for GpuParticlesAttractorBox3DFloatData {
    type Target = GpuParticlesAttractorBox3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <GpuParticlesAttractorBox3DFloatKind>::SizeX => NodePath::from("size:x"),
            <GpuParticlesAttractorBox3DFloatKind>::SizeY => NodePath::from("size:y"),
            <GpuParticlesAttractorBox3DFloatKind>::SizeZ => NodePath::from("size:z"),
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
                obj.try_cast::<GpuParticlesAttractorBox3D>().ok()
            }
            ObjectOrNode::Node(obj) => obj.try_cast::<GpuParticlesAttractorBox3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for GpuParticlesAttractorBox3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<GpuParticlesAttractorBox3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "size:x" => {
                        Some(Self {
                            property: <GpuParticlesAttractorBox3DFloatKind>::SizeX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:y" => {
                        Some(Self {
                            property: <GpuParticlesAttractorBox3DFloatKind>::SizeY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:z" => {
                        Some(Self {
                            property: <GpuParticlesAttractorBox3DFloatKind>::SizeZ,
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
pub struct GpuParticlesAttractorBox3DVector3Data {
    pub property: GpuParticlesAttractorBox3DVector3Kind,
    pub owner: Gd<GpuParticlesAttractorBox3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuParticlesAttractorBox3DVector3Kind {
    Size,
}
impl IProperty<Vector3> for GpuParticlesAttractorBox3DVector3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <GpuParticlesAttractorBox3DVector3Kind>::Size => {
                let obj = &self.owner;
                obj.get_size()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <GpuParticlesAttractorBox3DVector3Kind>::Size => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size(val);
            }
        }
    }
}
impl IPropertyData for GpuParticlesAttractorBox3DVector3Data {
    type Target = GpuParticlesAttractorBox3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <GpuParticlesAttractorBox3DVector3Kind>::Size => NodePath::from("size"),
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
                obj.try_cast::<GpuParticlesAttractorBox3D>().ok()
            }
            ObjectOrNode::Node(obj) => obj.try_cast::<GpuParticlesAttractorBox3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for GpuParticlesAttractorBox3DVector3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<GpuParticlesAttractorBox3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "size" => {
                        Some(Self {
                            property: <GpuParticlesAttractorBox3DVector3Kind>::Size,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoGpuParticlesAttractorBox3D<Marker = ()> {
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
    Class: Inherits<GpuParticlesAttractorBox3D> + Inherits<Object>,
> SpireDoGpuParticlesAttractorBox3D<()> for Gd<Class> {
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = GpuParticlesAttractorBox3DFloatData {
            property: <GpuParticlesAttractorBox3DFloatKind>::SizeX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<GpuParticlesAttractorBox3D>().upcast::<Node>(),
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
        let data = GpuParticlesAttractorBox3DFloatData {
            property: <GpuParticlesAttractorBox3DFloatKind>::SizeY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<GpuParticlesAttractorBox3D>().upcast::<Node>(),
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
        let data = GpuParticlesAttractorBox3DFloatData {
            property: <GpuParticlesAttractorBox3DFloatKind>::SizeZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<GpuParticlesAttractorBox3D>().upcast::<Node>(),
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
        let data = GpuParticlesAttractorBox3DVector3Data {
            property: <GpuParticlesAttractorBox3DVector3Kind>::Size,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<GpuParticlesAttractorBox3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<GpuParticlesAttractorBox3D> + Inherits<Object>,
> SpireDoGpuParticlesAttractorBox3D<BaseMarker> for T {
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<GpuParticlesAttractorBox3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractorBox3DFloatData {
            property: <GpuParticlesAttractorBox3DFloatKind>::SizeX,
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
        let owner: Gd<GpuParticlesAttractorBox3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractorBox3DFloatData {
            property: <GpuParticlesAttractorBox3DFloatKind>::SizeY,
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
        let owner: Gd<GpuParticlesAttractorBox3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractorBox3DFloatData {
            property: <GpuParticlesAttractorBox3DFloatKind>::SizeZ,
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
        let owner: Gd<GpuParticlesAttractorBox3D> = self.to_gd().upcast();
        let data = GpuParticlesAttractorBox3DVector3Data {
            property: <GpuParticlesAttractorBox3DVector3Kind>::Size,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
