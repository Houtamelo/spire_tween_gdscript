use super::*;
#[derive(Debug, Clone)]
pub struct SpotLight3DFloatData {
    pub property: SpotLight3DFloatKind,
    pub owner: Gd<SpotLight3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotLight3DFloatKind {
    SpotAngle,
    SpotAngleAttenuation,
    SpotAttenuation,
    SpotRange,
}
impl IProperty<f64> for SpotLight3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <SpotLight3DFloatKind>::SpotAngle => {
                let obj = &self.owner;
                (obj.get_spot_angle()) as f64
            }
            <SpotLight3DFloatKind>::SpotAngleAttenuation => {
                let obj = &self.owner;
                (obj.get_spot_angle_attenuation()) as f64
            }
            <SpotLight3DFloatKind>::SpotAttenuation => {
                let obj = &self.owner;
                (obj.get_spot_attenuation()) as f64
            }
            <SpotLight3DFloatKind>::SpotRange => {
                let obj = &self.owner;
                (obj.get_spot_range()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <SpotLight3DFloatKind>::SpotAngle => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_spot_angle(val as f32);
            }
            <SpotLight3DFloatKind>::SpotAngleAttenuation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_spot_angle_attenuation(val as f32);
            }
            <SpotLight3DFloatKind>::SpotAttenuation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_spot_attenuation(val as f32);
            }
            <SpotLight3DFloatKind>::SpotRange => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_spot_range(val as f32);
            }
        }
    }
}
impl IPropertyData for SpotLight3DFloatData {
    type Target = SpotLight3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <SpotLight3DFloatKind>::SpotAngle => NodePath::from("spot_angle"),
            <SpotLight3DFloatKind>::SpotAngleAttenuation => {
                NodePath::from("spot_angle_attenuation")
            }
            <SpotLight3DFloatKind>::SpotAttenuation => NodePath::from("spot_attenuation"),
            <SpotLight3DFloatKind>::SpotRange => NodePath::from("spot_range"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<SpotLight3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<SpotLight3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for SpotLight3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<SpotLight3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "spot_angle" => {
                        Some(Self {
                            property: <SpotLight3DFloatKind>::SpotAngle,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "spot_angle_attenuation" => {
                        Some(Self {
                            property: <SpotLight3DFloatKind>::SpotAngleAttenuation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "spot_attenuation" => {
                        Some(Self {
                            property: <SpotLight3DFloatKind>::SpotAttenuation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "spot_range" => {
                        Some(Self {
                            property: <SpotLight3DFloatKind>::SpotRange,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoSpotLight3D<Marker = ()> {
    fn do_spot_angle(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_spot_angle_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_spot_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_spot_range(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<SpotLight3D> + Inherits<Object>> SpireDoSpotLight3D<()>
for Gd<Class> {
    fn do_spot_angle(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = SpotLight3DFloatData {
            property: <SpotLight3DFloatKind>::SpotAngle,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpotLight3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_spot_angle_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = SpotLight3DFloatData {
            property: <SpotLight3DFloatKind>::SpotAngleAttenuation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpotLight3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_spot_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = SpotLight3DFloatData {
            property: <SpotLight3DFloatKind>::SpotAttenuation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpotLight3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_spot_range(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = SpotLight3DFloatData {
            property: <SpotLight3DFloatKind>::SpotRange,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpotLight3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<SpotLight3D> + Inherits<Object>,
> SpireDoSpotLight3D<BaseMarker> for T {
    fn do_spot_angle(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<SpotLight3D> = self.to_gd().upcast();
        let data = SpotLight3DFloatData {
            property: <SpotLight3DFloatKind>::SpotAngle,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_spot_angle_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<SpotLight3D> = self.to_gd().upcast();
        let data = SpotLight3DFloatData {
            property: <SpotLight3DFloatKind>::SpotAngleAttenuation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_spot_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<SpotLight3D> = self.to_gd().upcast();
        let data = SpotLight3DFloatData {
            property: <SpotLight3DFloatKind>::SpotAttenuation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_spot_range(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<SpotLight3D> = self.to_gd().upcast();
        let data = SpotLight3DFloatData {
            property: <SpotLight3DFloatKind>::SpotRange,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
