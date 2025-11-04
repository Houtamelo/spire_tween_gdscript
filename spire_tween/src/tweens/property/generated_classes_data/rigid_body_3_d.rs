use super::*;
#[derive(Debug, Clone)]
pub struct RigidBody3DFloatData {
    pub property: RigidBody3DFloatKind,
    pub owner: Gd<RigidBody3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RigidBody3DFloatKind {
    AngularDamp,
    AngularVelocityX,
    AngularVelocityY,
    AngularVelocityZ,
    CenterOfMassX,
    CenterOfMassY,
    CenterOfMassZ,
    ConstantForceX,
    ConstantForceY,
    ConstantForceZ,
    ConstantTorqueX,
    ConstantTorqueY,
    ConstantTorqueZ,
    GravityScale,
    InertiaX,
    InertiaY,
    InertiaZ,
    LinearDamp,
    LinearVelocityX,
    LinearVelocityY,
    LinearVelocityZ,
    Mass,
}
impl IProperty<f64> for RigidBody3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <RigidBody3DFloatKind>::AngularDamp => {
                let obj = &self.owner;
                (obj.get_angular_damp()) as f64
            }
            <RigidBody3DFloatKind>::AngularVelocityX => {
                let obj = &self.owner;
                (obj.get_angular_velocity().x) as f64
            }
            <RigidBody3DFloatKind>::AngularVelocityY => {
                let obj = &self.owner;
                (obj.get_angular_velocity().y) as f64
            }
            <RigidBody3DFloatKind>::AngularVelocityZ => {
                let obj = &self.owner;
                (obj.get_angular_velocity().z) as f64
            }
            <RigidBody3DFloatKind>::CenterOfMassX => {
                let obj = &self.owner;
                (obj.get_center_of_mass().x) as f64
            }
            <RigidBody3DFloatKind>::CenterOfMassY => {
                let obj = &self.owner;
                (obj.get_center_of_mass().y) as f64
            }
            <RigidBody3DFloatKind>::CenterOfMassZ => {
                let obj = &self.owner;
                (obj.get_center_of_mass().z) as f64
            }
            <RigidBody3DFloatKind>::ConstantForceX => {
                let obj = &self.owner;
                (obj.get_constant_force().x) as f64
            }
            <RigidBody3DFloatKind>::ConstantForceY => {
                let obj = &self.owner;
                (obj.get_constant_force().y) as f64
            }
            <RigidBody3DFloatKind>::ConstantForceZ => {
                let obj = &self.owner;
                (obj.get_constant_force().z) as f64
            }
            <RigidBody3DFloatKind>::ConstantTorqueX => {
                let obj = &self.owner;
                (obj.get_constant_torque().x) as f64
            }
            <RigidBody3DFloatKind>::ConstantTorqueY => {
                let obj = &self.owner;
                (obj.get_constant_torque().y) as f64
            }
            <RigidBody3DFloatKind>::ConstantTorqueZ => {
                let obj = &self.owner;
                (obj.get_constant_torque().z) as f64
            }
            <RigidBody3DFloatKind>::GravityScale => {
                let obj = &self.owner;
                (obj.get_gravity_scale()) as f64
            }
            <RigidBody3DFloatKind>::InertiaX => {
                let obj = &self.owner;
                (obj.get_inertia().x) as f64
            }
            <RigidBody3DFloatKind>::InertiaY => {
                let obj = &self.owner;
                (obj.get_inertia().y) as f64
            }
            <RigidBody3DFloatKind>::InertiaZ => {
                let obj = &self.owner;
                (obj.get_inertia().z) as f64
            }
            <RigidBody3DFloatKind>::LinearDamp => {
                let obj = &self.owner;
                (obj.get_linear_damp()) as f64
            }
            <RigidBody3DFloatKind>::LinearVelocityX => {
                let obj = &self.owner;
                (obj.get_linear_velocity().x) as f64
            }
            <RigidBody3DFloatKind>::LinearVelocityY => {
                let obj = &self.owner;
                (obj.get_linear_velocity().y) as f64
            }
            <RigidBody3DFloatKind>::LinearVelocityZ => {
                let obj = &self.owner;
                (obj.get_linear_velocity().z) as f64
            }
            <RigidBody3DFloatKind>::Mass => {
                let obj = &self.owner;
                (obj.get_mass()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <RigidBody3DFloatKind>::AngularDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_angular_damp(val as f32);
            }
            <RigidBody3DFloatKind>::AngularVelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_angular_velocity();
                prop_val.x = val as f32;
                obj.set_angular_velocity(prop_val);
            }
            <RigidBody3DFloatKind>::AngularVelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_angular_velocity();
                prop_val.y = val as f32;
                obj.set_angular_velocity(prop_val);
            }
            <RigidBody3DFloatKind>::AngularVelocityZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_angular_velocity();
                prop_val.z = val as f32;
                obj.set_angular_velocity(prop_val);
            }
            <RigidBody3DFloatKind>::CenterOfMassX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_center_of_mass();
                prop_val.x = val as f32;
                obj.set_center_of_mass(prop_val);
            }
            <RigidBody3DFloatKind>::CenterOfMassY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_center_of_mass();
                prop_val.y = val as f32;
                obj.set_center_of_mass(prop_val);
            }
            <RigidBody3DFloatKind>::CenterOfMassZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_center_of_mass();
                prop_val.z = val as f32;
                obj.set_center_of_mass(prop_val);
            }
            <RigidBody3DFloatKind>::ConstantForceX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_force();
                prop_val.x = val as f32;
                obj.set_constant_force(prop_val);
            }
            <RigidBody3DFloatKind>::ConstantForceY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_force();
                prop_val.y = val as f32;
                obj.set_constant_force(prop_val);
            }
            <RigidBody3DFloatKind>::ConstantForceZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_force();
                prop_val.z = val as f32;
                obj.set_constant_force(prop_val);
            }
            <RigidBody3DFloatKind>::ConstantTorqueX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_torque();
                prop_val.x = val as f32;
                obj.set_constant_torque(prop_val);
            }
            <RigidBody3DFloatKind>::ConstantTorqueY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_torque();
                prop_val.y = val as f32;
                obj.set_constant_torque(prop_val);
            }
            <RigidBody3DFloatKind>::ConstantTorqueZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_torque();
                prop_val.z = val as f32;
                obj.set_constant_torque(prop_val);
            }
            <RigidBody3DFloatKind>::GravityScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity_scale(val as f32);
            }
            <RigidBody3DFloatKind>::InertiaX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_inertia();
                prop_val.x = val as f32;
                obj.set_inertia(prop_val);
            }
            <RigidBody3DFloatKind>::InertiaY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_inertia();
                prop_val.y = val as f32;
                obj.set_inertia(prop_val);
            }
            <RigidBody3DFloatKind>::InertiaZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_inertia();
                prop_val.z = val as f32;
                obj.set_inertia(prop_val);
            }
            <RigidBody3DFloatKind>::LinearDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_linear_damp(val as f32);
            }
            <RigidBody3DFloatKind>::LinearVelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_linear_velocity();
                prop_val.x = val as f32;
                obj.set_linear_velocity(prop_val);
            }
            <RigidBody3DFloatKind>::LinearVelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_linear_velocity();
                prop_val.y = val as f32;
                obj.set_linear_velocity(prop_val);
            }
            <RigidBody3DFloatKind>::LinearVelocityZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_linear_velocity();
                prop_val.z = val as f32;
                obj.set_linear_velocity(prop_val);
            }
            <RigidBody3DFloatKind>::Mass => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_mass(val as f32);
            }
        }
    }
}
impl IPropertyData for RigidBody3DFloatData {
    type Target = RigidBody3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RigidBody3DFloatKind>::AngularDamp => NodePath::from("angular_damp"),
            <RigidBody3DFloatKind>::AngularVelocityX => {
                NodePath::from("angular_velocity:x")
            }
            <RigidBody3DFloatKind>::AngularVelocityY => {
                NodePath::from("angular_velocity:y")
            }
            <RigidBody3DFloatKind>::AngularVelocityZ => {
                NodePath::from("angular_velocity:z")
            }
            <RigidBody3DFloatKind>::CenterOfMassX => NodePath::from("center_of_mass:x"),
            <RigidBody3DFloatKind>::CenterOfMassY => NodePath::from("center_of_mass:y"),
            <RigidBody3DFloatKind>::CenterOfMassZ => NodePath::from("center_of_mass:z"),
            <RigidBody3DFloatKind>::ConstantForceX => NodePath::from("constant_force:x"),
            <RigidBody3DFloatKind>::ConstantForceY => NodePath::from("constant_force:y"),
            <RigidBody3DFloatKind>::ConstantForceZ => NodePath::from("constant_force:z"),
            <RigidBody3DFloatKind>::ConstantTorqueX => {
                NodePath::from("constant_torque:x")
            }
            <RigidBody3DFloatKind>::ConstantTorqueY => {
                NodePath::from("constant_torque:y")
            }
            <RigidBody3DFloatKind>::ConstantTorqueZ => {
                NodePath::from("constant_torque:z")
            }
            <RigidBody3DFloatKind>::GravityScale => NodePath::from("gravity_scale"),
            <RigidBody3DFloatKind>::InertiaX => NodePath::from("inertia:x"),
            <RigidBody3DFloatKind>::InertiaY => NodePath::from("inertia:y"),
            <RigidBody3DFloatKind>::InertiaZ => NodePath::from("inertia:z"),
            <RigidBody3DFloatKind>::LinearDamp => NodePath::from("linear_damp"),
            <RigidBody3DFloatKind>::LinearVelocityX => {
                NodePath::from("linear_velocity:x")
            }
            <RigidBody3DFloatKind>::LinearVelocityY => {
                NodePath::from("linear_velocity:y")
            }
            <RigidBody3DFloatKind>::LinearVelocityZ => {
                NodePath::from("linear_velocity:z")
            }
            <RigidBody3DFloatKind>::Mass => NodePath::from("mass"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<RigidBody3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<RigidBody3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for RigidBody3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RigidBody3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "angular_damp" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::AngularDamp,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "angular_velocity:x" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::AngularVelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "angular_velocity:y" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::AngularVelocityY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "angular_velocity:z" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::AngularVelocityZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "center_of_mass:x" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::CenterOfMassX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "center_of_mass:y" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::CenterOfMassY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "center_of_mass:z" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::CenterOfMassZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_force:x" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::ConstantForceX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_force:y" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::ConstantForceY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_force:z" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::ConstantForceZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_torque:x" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::ConstantTorqueX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_torque:y" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::ConstantTorqueY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_torque:z" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::ConstantTorqueZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_scale" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::GravityScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "inertia:x" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::InertiaX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "inertia:y" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::InertiaY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "inertia:z" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::InertiaZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_damp" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::LinearDamp,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_velocity:x" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::LinearVelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_velocity:y" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::LinearVelocityY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_velocity:z" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::LinearVelocityZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "mass" => {
                        Some(Self {
                            property: <RigidBody3DFloatKind>::Mass,
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
pub struct RigidBody3DVector3Data {
    pub property: RigidBody3DVector3Kind,
    pub owner: Gd<RigidBody3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RigidBody3DVector3Kind {
    AngularVelocity,
    CenterOfMass,
    ConstantForce,
    ConstantTorque,
    Inertia,
    LinearVelocity,
}
impl IProperty<Vector3> for RigidBody3DVector3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <RigidBody3DVector3Kind>::AngularVelocity => {
                let obj = &self.owner;
                obj.get_angular_velocity()
            }
            <RigidBody3DVector3Kind>::CenterOfMass => {
                let obj = &self.owner;
                obj.get_center_of_mass()
            }
            <RigidBody3DVector3Kind>::ConstantForce => {
                let obj = &self.owner;
                obj.get_constant_force()
            }
            <RigidBody3DVector3Kind>::ConstantTorque => {
                let obj = &self.owner;
                obj.get_constant_torque()
            }
            <RigidBody3DVector3Kind>::Inertia => {
                let obj = &self.owner;
                obj.get_inertia()
            }
            <RigidBody3DVector3Kind>::LinearVelocity => {
                let obj = &self.owner;
                obj.get_linear_velocity()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <RigidBody3DVector3Kind>::AngularVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_angular_velocity(val);
            }
            <RigidBody3DVector3Kind>::CenterOfMass => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_center_of_mass(val);
            }
            <RigidBody3DVector3Kind>::ConstantForce => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_constant_force(val);
            }
            <RigidBody3DVector3Kind>::ConstantTorque => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_constant_torque(val);
            }
            <RigidBody3DVector3Kind>::Inertia => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_inertia(val);
            }
            <RigidBody3DVector3Kind>::LinearVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_linear_velocity(val);
            }
        }
    }
}
impl IPropertyData for RigidBody3DVector3Data {
    type Target = RigidBody3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RigidBody3DVector3Kind>::AngularVelocity => {
                NodePath::from("angular_velocity")
            }
            <RigidBody3DVector3Kind>::CenterOfMass => NodePath::from("center_of_mass"),
            <RigidBody3DVector3Kind>::ConstantForce => NodePath::from("constant_force"),
            <RigidBody3DVector3Kind>::ConstantTorque => NodePath::from("constant_torque"),
            <RigidBody3DVector3Kind>::Inertia => NodePath::from("inertia"),
            <RigidBody3DVector3Kind>::LinearVelocity => NodePath::from("linear_velocity"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<RigidBody3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<RigidBody3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for RigidBody3DVector3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RigidBody3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "angular_velocity" => {
                        Some(Self {
                            property: <RigidBody3DVector3Kind>::AngularVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "center_of_mass" => {
                        Some(Self {
                            property: <RigidBody3DVector3Kind>::CenterOfMass,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_force" => {
                        Some(Self {
                            property: <RigidBody3DVector3Kind>::ConstantForce,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_torque" => {
                        Some(Self {
                            property: <RigidBody3DVector3Kind>::ConstantTorque,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "inertia" => {
                        Some(Self {
                            property: <RigidBody3DVector3Kind>::Inertia,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_velocity" => {
                        Some(Self {
                            property: <RigidBody3DVector3Kind>::LinearVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoRigidBody3D<Marker = ()> {
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
    fn do_center_of_mass_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_center_of_mass_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_center_of_mass_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_force_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_force_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_force_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_torque_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_torque_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_torque_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_inertia_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_inertia_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_inertia_z(
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
    fn do_center_of_mass(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_constant_force(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_constant_torque(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_inertia(
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
impl<Class: Inherits<RigidBody3D> + Inherits<Object>> SpireDoRigidBody3D<()>
for Gd<Class> {
    fn do_angular_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::AngularDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
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
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::AngularVelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
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
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::AngularVelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
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
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::AngularVelocityZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_center_of_mass_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::CenterOfMassX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_center_of_mass_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::CenterOfMassY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_center_of_mass_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::CenterOfMassZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_force_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantForceX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_force_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantForceY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_force_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantForceZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_torque_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantTorqueX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_torque_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantTorqueY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_torque_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantTorqueZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
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
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::GravityScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_inertia_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::InertiaX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_inertia_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::InertiaY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_inertia_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::InertiaZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
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
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::LinearDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
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
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::LinearVelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
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
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::LinearVelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
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
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::LinearVelocityZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_mass(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::Mass,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
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
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::AngularVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_center_of_mass(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::CenterOfMass,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_force(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::ConstantForce,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_torque(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::ConstantTorque,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_inertia(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::Inertia,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
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
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::LinearVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<RigidBody3D> + Inherits<Object>,
> SpireDoRigidBody3D<BaseMarker> for T {
    fn do_angular_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::AngularDamp,
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
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::AngularVelocityX,
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
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::AngularVelocityY,
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
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::AngularVelocityZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_center_of_mass_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::CenterOfMassX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_center_of_mass_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::CenterOfMassY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_center_of_mass_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::CenterOfMassZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_force_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantForceX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_force_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantForceY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_force_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantForceZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_torque_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantTorqueX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_torque_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantTorqueY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_torque_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::ConstantTorqueZ,
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
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::GravityScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_inertia_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::InertiaX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_inertia_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::InertiaY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_inertia_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::InertiaZ,
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
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::LinearDamp,
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
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::LinearVelocityX,
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
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::LinearVelocityY,
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
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::LinearVelocityZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_mass(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DFloatData {
            property: <RigidBody3DFloatKind>::Mass,
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
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::AngularVelocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_center_of_mass(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::CenterOfMass,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_force(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::ConstantForce,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_torque(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::ConstantTorque,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_inertia(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::Inertia,
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
        let owner: Gd<RigidBody3D> = self.to_gd().upcast();
        let data = RigidBody3DVector3Data {
            property: <RigidBody3DVector3Kind>::LinearVelocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
