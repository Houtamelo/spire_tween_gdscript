use super::*;
#[derive(Debug, Clone)]
pub struct Area3DFloatData {
    pub property: Area3DFloatKind,
    pub owner: Gd<Area3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area3DFloatKind {
    AngularDamp,
    Gravity,
    GravityDirectionX,
    GravityDirectionY,
    GravityDirectionZ,
    GravityPointCenterX,
    GravityPointCenterY,
    GravityPointCenterZ,
    GravityPointUnitDistance,
    LinearDamp,
    ReverbBusAmount,
    ReverbBusUniformity,
    WindAttenuationFactor,
    WindForceMagnitude,
}
impl IProperty<f64> for Area3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <Area3DFloatKind>::AngularDamp => {
                let obj = &self.owner;
                (obj.get_angular_damp()) as f64
            }
            <Area3DFloatKind>::Gravity => {
                let obj = &self.owner;
                (obj.get_gravity()) as f64
            }
            <Area3DFloatKind>::GravityDirectionX => {
                let obj = &self.owner;
                (obj.get_gravity_direction().x) as f64
            }
            <Area3DFloatKind>::GravityDirectionY => {
                let obj = &self.owner;
                (obj.get_gravity_direction().y) as f64
            }
            <Area3DFloatKind>::GravityDirectionZ => {
                let obj = &self.owner;
                (obj.get_gravity_direction().z) as f64
            }
            <Area3DFloatKind>::GravityPointCenterX => {
                let obj = &self.owner;
                (obj.get_gravity_point_center().x) as f64
            }
            <Area3DFloatKind>::GravityPointCenterY => {
                let obj = &self.owner;
                (obj.get_gravity_point_center().y) as f64
            }
            <Area3DFloatKind>::GravityPointCenterZ => {
                let obj = &self.owner;
                (obj.get_gravity_point_center().z) as f64
            }
            <Area3DFloatKind>::GravityPointUnitDistance => {
                let obj = &self.owner;
                (obj.get_gravity_point_unit_distance()) as f64
            }
            <Area3DFloatKind>::LinearDamp => {
                let obj = &self.owner;
                (obj.get_linear_damp()) as f64
            }
            <Area3DFloatKind>::ReverbBusAmount => {
                let obj = &self.owner;
                (obj.get_reverb_amount()) as f64
            }
            <Area3DFloatKind>::ReverbBusUniformity => {
                let obj = &self.owner;
                (obj.get_reverb_uniformity()) as f64
            }
            <Area3DFloatKind>::WindAttenuationFactor => {
                let obj = &self.owner;
                (obj.get_wind_attenuation_factor()) as f64
            }
            <Area3DFloatKind>::WindForceMagnitude => {
                let obj = &self.owner;
                (obj.get_wind_force_magnitude()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <Area3DFloatKind>::AngularDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_angular_damp(val as f32);
            }
            <Area3DFloatKind>::Gravity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity(val as f32);
            }
            <Area3DFloatKind>::GravityDirectionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_direction();
                prop_val.x = val as f32;
                obj.set_gravity_direction(prop_val);
            }
            <Area3DFloatKind>::GravityDirectionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_direction();
                prop_val.y = val as f32;
                obj.set_gravity_direction(prop_val);
            }
            <Area3DFloatKind>::GravityDirectionZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_direction();
                prop_val.z = val as f32;
                obj.set_gravity_direction(prop_val);
            }
            <Area3DFloatKind>::GravityPointCenterX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_point_center();
                prop_val.x = val as f32;
                obj.set_gravity_point_center(prop_val);
            }
            <Area3DFloatKind>::GravityPointCenterY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_point_center();
                prop_val.y = val as f32;
                obj.set_gravity_point_center(prop_val);
            }
            <Area3DFloatKind>::GravityPointCenterZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_point_center();
                prop_val.z = val as f32;
                obj.set_gravity_point_center(prop_val);
            }
            <Area3DFloatKind>::GravityPointUnitDistance => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity_point_unit_distance(val as f32);
            }
            <Area3DFloatKind>::LinearDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_linear_damp(val as f32);
            }
            <Area3DFloatKind>::ReverbBusAmount => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_reverb_amount(val as f32);
            }
            <Area3DFloatKind>::ReverbBusUniformity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_reverb_uniformity(val as f32);
            }
            <Area3DFloatKind>::WindAttenuationFactor => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_wind_attenuation_factor(val as f32);
            }
            <Area3DFloatKind>::WindForceMagnitude => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_wind_force_magnitude(val as f32);
            }
        }
    }
}
impl IPropertyData for Area3DFloatData {
    type Target = Area3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Area3DFloatKind>::AngularDamp => NodePath::from("angular_damp"),
            <Area3DFloatKind>::Gravity => NodePath::from("gravity"),
            <Area3DFloatKind>::GravityDirectionX => NodePath::from("gravity_direction:x"),
            <Area3DFloatKind>::GravityDirectionY => NodePath::from("gravity_direction:y"),
            <Area3DFloatKind>::GravityDirectionZ => NodePath::from("gravity_direction:z"),
            <Area3DFloatKind>::GravityPointCenterX => {
                NodePath::from("gravity_point_center:x")
            }
            <Area3DFloatKind>::GravityPointCenterY => {
                NodePath::from("gravity_point_center:y")
            }
            <Area3DFloatKind>::GravityPointCenterZ => {
                NodePath::from("gravity_point_center:z")
            }
            <Area3DFloatKind>::GravityPointUnitDistance => {
                NodePath::from("gravity_point_unit_distance")
            }
            <Area3DFloatKind>::LinearDamp => NodePath::from("linear_damp"),
            <Area3DFloatKind>::ReverbBusAmount => NodePath::from("reverb_bus_amount"),
            <Area3DFloatKind>::ReverbBusUniformity => {
                NodePath::from("reverb_bus_uniformity")
            }
            <Area3DFloatKind>::WindAttenuationFactor => {
                NodePath::from("wind_attenuation_factor")
            }
            <Area3DFloatKind>::WindForceMagnitude => {
                NodePath::from("wind_force_magnitude")
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
            ObjectOrNode::Object(obj) => obj.try_cast::<Area3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Area3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Area3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Area3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "angular_damp" => {
                        Some(Self {
                            property: <Area3DFloatKind>::AngularDamp,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity" => {
                        Some(Self {
                            property: <Area3DFloatKind>::Gravity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_direction:x" => {
                        Some(Self {
                            property: <Area3DFloatKind>::GravityDirectionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_direction:y" => {
                        Some(Self {
                            property: <Area3DFloatKind>::GravityDirectionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_direction:z" => {
                        Some(Self {
                            property: <Area3DFloatKind>::GravityDirectionZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_center:x" => {
                        Some(Self {
                            property: <Area3DFloatKind>::GravityPointCenterX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_center:y" => {
                        Some(Self {
                            property: <Area3DFloatKind>::GravityPointCenterY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_center:z" => {
                        Some(Self {
                            property: <Area3DFloatKind>::GravityPointCenterZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_unit_distance" => {
                        Some(Self {
                            property: <Area3DFloatKind>::GravityPointUnitDistance,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_damp" => {
                        Some(Self {
                            property: <Area3DFloatKind>::LinearDamp,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "reverb_bus_amount" => {
                        Some(Self {
                            property: <Area3DFloatKind>::ReverbBusAmount,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "reverb_bus_uniformity" => {
                        Some(Self {
                            property: <Area3DFloatKind>::ReverbBusUniformity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "wind_attenuation_factor" => {
                        Some(Self {
                            property: <Area3DFloatKind>::WindAttenuationFactor,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "wind_force_magnitude" => {
                        Some(Self {
                            property: <Area3DFloatKind>::WindForceMagnitude,
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
pub struct Area3DVector3Data {
    pub property: Area3DVector3Kind,
    pub owner: Gd<Area3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area3DVector3Kind {
    GravityDirection,
    GravityPointCenter,
}
impl IProperty<Vector3> for Area3DVector3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <Area3DVector3Kind>::GravityDirection => {
                let obj = &self.owner;
                obj.get_gravity_direction()
            }
            <Area3DVector3Kind>::GravityPointCenter => {
                let obj = &self.owner;
                obj.get_gravity_point_center()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <Area3DVector3Kind>::GravityDirection => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity_direction(val);
            }
            <Area3DVector3Kind>::GravityPointCenter => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity_point_center(val);
            }
        }
    }
}
impl IPropertyData for Area3DVector3Data {
    type Target = Area3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Area3DVector3Kind>::GravityDirection => NodePath::from("gravity_direction"),
            <Area3DVector3Kind>::GravityPointCenter => {
                NodePath::from("gravity_point_center")
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
            ObjectOrNode::Object(obj) => obj.try_cast::<Area3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Area3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Area3DVector3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Area3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "gravity_direction" => {
                        Some(Self {
                            property: <Area3DVector3Kind>::GravityDirection,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_center" => {
                        Some(Self {
                            property: <Area3DVector3Kind>::GravityPointCenter,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoArea3D<Marker = ()> {
    fn do_angular_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_direction_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_direction_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_direction_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_point_center_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_point_center_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_point_center_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_point_unit_distance(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_linear_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_reverb_bus_amount(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_reverb_bus_uniformity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_wind_attenuation_factor(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_wind_force_magnitude(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_direction(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_gravity_point_center(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
}
impl<Class: Inherits<Area3D> + Inherits<Object>> SpireDoArea3D<()> for Gd<Class> {
    fn do_angular_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::AngularDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::Gravity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_direction_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityDirectionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_direction_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityDirectionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_direction_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityDirectionZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_center_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityPointCenterX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_center_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityPointCenterY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_center_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityPointCenterZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_unit_distance(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityPointUnitDistance,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::LinearDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_reverb_bus_amount(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::ReverbBusAmount,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_reverb_bus_uniformity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::ReverbBusUniformity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_wind_attenuation_factor(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::WindAttenuationFactor,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_wind_force_magnitude(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::WindForceMagnitude,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_direction(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = Area3DVector3Data {
            property: <Area3DVector3Kind>::GravityDirection,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_center(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = Area3DVector3Data {
            property: <Area3DVector3Kind>::GravityPointCenter,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<T: WithBaseField + Inherits<Area3D> + Inherits<Object>> SpireDoArea3D<BaseMarker>
for T {
    fn do_angular_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::AngularDamp,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::Gravity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_direction_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityDirectionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_direction_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityDirectionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_direction_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityDirectionZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_center_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityPointCenterX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_center_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityPointCenterY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_center_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityPointCenterZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_unit_distance(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::GravityPointUnitDistance,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::LinearDamp,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_reverb_bus_amount(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::ReverbBusAmount,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_reverb_bus_uniformity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::ReverbBusUniformity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_wind_attenuation_factor(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::WindAttenuationFactor,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_wind_force_magnitude(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DFloatData {
            property: <Area3DFloatKind>::WindForceMagnitude,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_direction(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DVector3Data {
            property: <Area3DVector3Kind>::GravityDirection,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_center(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Area3D> = self.to_gd().upcast();
        let data = Area3DVector3Data {
            property: <Area3DVector3Kind>::GravityPointCenter,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
