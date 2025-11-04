use super::*;
#[derive(Debug, Clone)]
pub struct FogVolumeFloatData {
    pub property: FogVolumeFloatKind,
    pub owner: Gd<FogVolume>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FogVolumeFloatKind {
    SizeX,
    SizeY,
    SizeZ,
}
impl IProperty<f64> for FogVolumeFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <FogVolumeFloatKind>::SizeX => {
                let obj = &self.owner;
                (obj.get_size().x) as f64
            }
            <FogVolumeFloatKind>::SizeY => {
                let obj = &self.owner;
                (obj.get_size().y) as f64
            }
            <FogVolumeFloatKind>::SizeZ => {
                let obj = &self.owner;
                (obj.get_size().z) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <FogVolumeFloatKind>::SizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.x = val as f32;
                obj.set_size(prop_val);
            }
            <FogVolumeFloatKind>::SizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.y = val as f32;
                obj.set_size(prop_val);
            }
            <FogVolumeFloatKind>::SizeZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.z = val as f32;
                obj.set_size(prop_val);
            }
        }
    }
}
impl IPropertyData for FogVolumeFloatData {
    type Target = FogVolume;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <FogVolumeFloatKind>::SizeX => NodePath::from("size:x"),
            <FogVolumeFloatKind>::SizeY => NodePath::from("size:y"),
            <FogVolumeFloatKind>::SizeZ => NodePath::from("size:z"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<FogVolume>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<FogVolume>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for FogVolumeFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<FogVolume>()
            .ok()
            .and_then(|owner| {
                match path {
                    "size:x" => {
                        Some(Self {
                            property: <FogVolumeFloatKind>::SizeX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:y" => {
                        Some(Self {
                            property: <FogVolumeFloatKind>::SizeY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:z" => {
                        Some(Self {
                            property: <FogVolumeFloatKind>::SizeZ,
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
pub struct FogVolumeVector3Data {
    pub property: FogVolumeVector3Kind,
    pub owner: Gd<FogVolume>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FogVolumeVector3Kind {
    Size,
}
impl IProperty<Vector3> for FogVolumeVector3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <FogVolumeVector3Kind>::Size => {
                let obj = &self.owner;
                obj.get_size()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <FogVolumeVector3Kind>::Size => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size(val);
            }
        }
    }
}
impl IPropertyData for FogVolumeVector3Data {
    type Target = FogVolume;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <FogVolumeVector3Kind>::Size => NodePath::from("size"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<FogVolume>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<FogVolume>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for FogVolumeVector3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<FogVolume>()
            .ok()
            .and_then(|owner| {
                match path {
                    "size" => {
                        Some(Self {
                            property: <FogVolumeVector3Kind>::Size,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoFogVolume<Marker = ()> {
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
impl<Class: Inherits<FogVolume> + Inherits<Object>> SpireDoFogVolume<()> for Gd<Class> {
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = FogVolumeFloatData {
            property: <FogVolumeFloatKind>::SizeX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<FogVolume>().upcast::<Node>(),
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
        let data = FogVolumeFloatData {
            property: <FogVolumeFloatKind>::SizeY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<FogVolume>().upcast::<Node>(),
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
        let data = FogVolumeFloatData {
            property: <FogVolumeFloatKind>::SizeZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<FogVolume>().upcast::<Node>(),
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
        let data = FogVolumeVector3Data {
            property: <FogVolumeVector3Kind>::Size,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<FogVolume>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<FogVolume> + Inherits<Object>,
> SpireDoFogVolume<BaseMarker> for T {
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<FogVolume> = self.to_gd().upcast();
        let data = FogVolumeFloatData {
            property: <FogVolumeFloatKind>::SizeX,
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
        let owner: Gd<FogVolume> = self.to_gd().upcast();
        let data = FogVolumeFloatData {
            property: <FogVolumeFloatKind>::SizeY,
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
        let owner: Gd<FogVolume> = self.to_gd().upcast();
        let data = FogVolumeFloatData {
            property: <FogVolumeFloatKind>::SizeZ,
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
        let owner: Gd<FogVolume> = self.to_gd().upcast();
        let data = FogVolumeVector3Data {
            property: <FogVolumeVector3Kind>::Size,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
