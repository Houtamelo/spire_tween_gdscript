use super::*;
#[derive(Debug, Clone)]
pub struct PhysicalBone3DFloatData {
    pub property: PhysicalBone3DFloatKind,
    pub owner: Gd<PhysicalBone3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalBone3DFloatKind {
    AngularDamp,
    AngularVelocityX,
    AngularVelocityY,
    AngularVelocityZ,
    Bounce,
    Friction,
    GravityScale,
    JointRotationX,
    JointRotationY,
    JointRotationZ,
    LinearDamp,
    LinearVelocityX,
    LinearVelocityY,
    LinearVelocityZ,
    Mass,
}
impl IProperty<f64> for PhysicalBone3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <PhysicalBone3DFloatKind>::AngularDamp => {
                let obj = &self.owner;
                (obj.get_angular_damp()) as f64
            }
            <PhysicalBone3DFloatKind>::AngularVelocityX => {
                let obj = &self.owner;
                (obj.get_angular_velocity().x) as f64
            }
            <PhysicalBone3DFloatKind>::AngularVelocityY => {
                let obj = &self.owner;
                (obj.get_angular_velocity().y) as f64
            }
            <PhysicalBone3DFloatKind>::AngularVelocityZ => {
                let obj = &self.owner;
                (obj.get_angular_velocity().z) as f64
            }
            <PhysicalBone3DFloatKind>::Bounce => {
                let obj = &self.owner;
                (obj.get_bounce()) as f64
            }
            <PhysicalBone3DFloatKind>::Friction => {
                let obj = &self.owner;
                (obj.get_friction()) as f64
            }
            <PhysicalBone3DFloatKind>::GravityScale => {
                let obj = &self.owner;
                (obj.get_gravity_scale()) as f64
            }
            <PhysicalBone3DFloatKind>::JointRotationX => {
                let obj = &self.owner;
                (obj.get_joint_rotation().x) as f64
            }
            <PhysicalBone3DFloatKind>::JointRotationY => {
                let obj = &self.owner;
                (obj.get_joint_rotation().y) as f64
            }
            <PhysicalBone3DFloatKind>::JointRotationZ => {
                let obj = &self.owner;
                (obj.get_joint_rotation().z) as f64
            }
            <PhysicalBone3DFloatKind>::LinearDamp => {
                let obj = &self.owner;
                (obj.get_linear_damp()) as f64
            }
            <PhysicalBone3DFloatKind>::LinearVelocityX => {
                let obj = &self.owner;
                (obj.get_linear_velocity().x) as f64
            }
            <PhysicalBone3DFloatKind>::LinearVelocityY => {
                let obj = &self.owner;
                (obj.get_linear_velocity().y) as f64
            }
            <PhysicalBone3DFloatKind>::LinearVelocityZ => {
                let obj = &self.owner;
                (obj.get_linear_velocity().z) as f64
            }
            <PhysicalBone3DFloatKind>::Mass => {
                let obj = &self.owner;
                (obj.get_mass()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <PhysicalBone3DFloatKind>::AngularDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_angular_damp(val as f32);
            }
            <PhysicalBone3DFloatKind>::AngularVelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_angular_velocity();
                prop_val.x = val as f32;
                obj.set_angular_velocity(prop_val);
            }
            <PhysicalBone3DFloatKind>::AngularVelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_angular_velocity();
                prop_val.y = val as f32;
                obj.set_angular_velocity(prop_val);
            }
            <PhysicalBone3DFloatKind>::AngularVelocityZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_angular_velocity();
                prop_val.z = val as f32;
                obj.set_angular_velocity(prop_val);
            }
            <PhysicalBone3DFloatKind>::Bounce => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_bounce(val as f32);
            }
            <PhysicalBone3DFloatKind>::Friction => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_friction(val as f32);
            }
            <PhysicalBone3DFloatKind>::GravityScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity_scale(val as f32);
            }
            <PhysicalBone3DFloatKind>::JointRotationX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_joint_rotation();
                prop_val.x = val as f32;
                obj.set_joint_rotation(prop_val);
            }
            <PhysicalBone3DFloatKind>::JointRotationY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_joint_rotation();
                prop_val.y = val as f32;
                obj.set_joint_rotation(prop_val);
            }
            <PhysicalBone3DFloatKind>::JointRotationZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_joint_rotation();
                prop_val.z = val as f32;
                obj.set_joint_rotation(prop_val);
            }
            <PhysicalBone3DFloatKind>::LinearDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_linear_damp(val as f32);
            }
            <PhysicalBone3DFloatKind>::LinearVelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_linear_velocity();
                prop_val.x = val as f32;
                obj.set_linear_velocity(prop_val);
            }
            <PhysicalBone3DFloatKind>::LinearVelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_linear_velocity();
                prop_val.y = val as f32;
                obj.set_linear_velocity(prop_val);
            }
            <PhysicalBone3DFloatKind>::LinearVelocityZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_linear_velocity();
                prop_val.z = val as f32;
                obj.set_linear_velocity(prop_val);
            }
            <PhysicalBone3DFloatKind>::Mass => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_mass(val as f32);
            }
        }
    }
}
impl IPropertyData for PhysicalBone3DFloatData {
    type Target = PhysicalBone3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <PhysicalBone3DFloatKind>::AngularDamp => NodePath::from("angular_damp"),
            <PhysicalBone3DFloatKind>::AngularVelocityX => {
                NodePath::from("angular_velocity:x")
            }
            <PhysicalBone3DFloatKind>::AngularVelocityY => {
                NodePath::from("angular_velocity:y")
            }
            <PhysicalBone3DFloatKind>::AngularVelocityZ => {
                NodePath::from("angular_velocity:z")
            }
            <PhysicalBone3DFloatKind>::Bounce => NodePath::from("bounce"),
            <PhysicalBone3DFloatKind>::Friction => NodePath::from("friction"),
            <PhysicalBone3DFloatKind>::GravityScale => NodePath::from("gravity_scale"),
            <PhysicalBone3DFloatKind>::JointRotationX => {
                NodePath::from("joint_rotation:x")
            }
            <PhysicalBone3DFloatKind>::JointRotationY => {
                NodePath::from("joint_rotation:y")
            }
            <PhysicalBone3DFloatKind>::JointRotationZ => {
                NodePath::from("joint_rotation:z")
            }
            <PhysicalBone3DFloatKind>::LinearDamp => NodePath::from("linear_damp"),
            <PhysicalBone3DFloatKind>::LinearVelocityX => {
                NodePath::from("linear_velocity:x")
            }
            <PhysicalBone3DFloatKind>::LinearVelocityY => {
                NodePath::from("linear_velocity:y")
            }
            <PhysicalBone3DFloatKind>::LinearVelocityZ => {
                NodePath::from("linear_velocity:z")
            }
            <PhysicalBone3DFloatKind>::Mass => NodePath::from("mass"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<PhysicalBone3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<PhysicalBone3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for PhysicalBone3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<PhysicalBone3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "angular_damp" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::AngularDamp,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "angular_velocity:x" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::AngularVelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "angular_velocity:y" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::AngularVelocityY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "angular_velocity:z" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::AngularVelocityZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "bounce" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::Bounce,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "friction" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::Friction,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_scale" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::GravityScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "joint_rotation:x" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::JointRotationX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "joint_rotation:y" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::JointRotationY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "joint_rotation:z" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::JointRotationZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_damp" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::LinearDamp,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_velocity:x" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::LinearVelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_velocity:y" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::LinearVelocityY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_velocity:z" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::LinearVelocityZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "mass" => {
                        Some(Self {
                            property: <PhysicalBone3DFloatKind>::Mass,
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
pub struct PhysicalBone3DVector3Data {
    pub property: PhysicalBone3DVector3Kind,
    pub owner: Gd<PhysicalBone3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalBone3DVector3Kind {
    AngularVelocity,
    JointRotation,
    LinearVelocity,
}
impl IProperty<Vector3> for PhysicalBone3DVector3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <PhysicalBone3DVector3Kind>::AngularVelocity => {
                let obj = &self.owner;
                obj.get_angular_velocity()
            }
            <PhysicalBone3DVector3Kind>::JointRotation => {
                let obj = &self.owner;
                obj.get_joint_rotation()
            }
            <PhysicalBone3DVector3Kind>::LinearVelocity => {
                let obj = &self.owner;
                obj.get_linear_velocity()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <PhysicalBone3DVector3Kind>::AngularVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_angular_velocity(val);
            }
            <PhysicalBone3DVector3Kind>::JointRotation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_joint_rotation(val);
            }
            <PhysicalBone3DVector3Kind>::LinearVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_linear_velocity(val);
            }
        }
    }
}
impl IPropertyData for PhysicalBone3DVector3Data {
    type Target = PhysicalBone3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <PhysicalBone3DVector3Kind>::AngularVelocity => {
                NodePath::from("angular_velocity")
            }
            <PhysicalBone3DVector3Kind>::JointRotation => {
                NodePath::from("joint_rotation")
            }
            <PhysicalBone3DVector3Kind>::LinearVelocity => {
                NodePath::from("linear_velocity")
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
            ObjectOrNode::Object(obj) => obj.try_cast::<PhysicalBone3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<PhysicalBone3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for PhysicalBone3DVector3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<PhysicalBone3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "angular_velocity" => {
                        Some(Self {
                            property: <PhysicalBone3DVector3Kind>::AngularVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "joint_rotation" => {
                        Some(Self {
                            property: <PhysicalBone3DVector3Kind>::JointRotation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_velocity" => {
                        Some(Self {
                            property: <PhysicalBone3DVector3Kind>::LinearVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoPhysicalBone3D<Marker = ()> {
    fn do_angular_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_angular_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_angular_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_angular_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_bounce(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_friction(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_joint_rotation_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_joint_rotation_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_joint_rotation_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_linear_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_linear_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_linear_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_linear_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_mass(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>>;
    fn do_angular_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_joint_rotation(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_linear_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
}
impl<Class: Inherits<PhysicalBone3D> + Inherits<Object>> SpireDoPhysicalBone3D<()>
for Gd<Class> {
    fn do_angular_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::AngularDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_angular_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::AngularVelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_angular_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::AngularVelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_angular_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::AngularVelocityZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_bounce(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::Bounce,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_friction(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::Friction,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::GravityScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_joint_rotation_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::JointRotationX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_joint_rotation_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::JointRotationY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_joint_rotation_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::JointRotationZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
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
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::LinearDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::LinearVelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::LinearVelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::LinearVelocityZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_mass(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::Mass,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_angular_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = PhysicalBone3DVector3Data {
            property: <PhysicalBone3DVector3Kind>::AngularVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_joint_rotation(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = PhysicalBone3DVector3Data {
            property: <PhysicalBone3DVector3Kind>::JointRotation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = PhysicalBone3DVector3Data {
            property: <PhysicalBone3DVector3Kind>::LinearVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PhysicalBone3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<PhysicalBone3D> + Inherits<Object>,
> SpireDoPhysicalBone3D<BaseMarker> for T {
    fn do_angular_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::AngularDamp,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_angular_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::AngularVelocityX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_angular_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::AngularVelocityY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_angular_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::AngularVelocityZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_bounce(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::Bounce,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_friction(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::Friction,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::GravityScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_joint_rotation_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::JointRotationX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_joint_rotation_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::JointRotationY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_joint_rotation_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::JointRotationZ,
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
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::LinearDamp,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::LinearVelocityX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::LinearVelocityY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::LinearVelocityZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_mass(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DFloatData {
            property: <PhysicalBone3DFloatKind>::Mass,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_angular_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DVector3Data {
            property: <PhysicalBone3DVector3Kind>::AngularVelocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_joint_rotation(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DVector3Data {
            property: <PhysicalBone3DVector3Kind>::JointRotation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<PhysicalBone3D> = self.to_gd().upcast();
        let data = PhysicalBone3DVector3Data {
            property: <PhysicalBone3DVector3Kind>::LinearVelocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
