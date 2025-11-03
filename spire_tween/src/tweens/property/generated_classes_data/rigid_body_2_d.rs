use super::*;
#[derive(Debug, Clone)]
pub struct RigidBody2DFloatData {
    pub property: RigidBody2DFloatKind,
    pub owner: Gd<RigidBody2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RigidBody2DFloatKind {
    AngularDamp,
    AngularVelocity,
    CenterOfMassX,
    CenterOfMassY,
    ConstantForceX,
    ConstantForceY,
    ConstantTorque,
    GravityScale,
    Inertia,
    LinearDamp,
    LinearVelocityX,
    LinearVelocityY,
    Mass,
}
impl IProperty<f64> for RigidBody2DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <RigidBody2DFloatKind>::AngularDamp => {
                let obj = &self.owner;
                (obj.get_angular_damp()) as f64
            }
            <RigidBody2DFloatKind>::AngularVelocity => {
                let obj = &self.owner;
                (obj.get_angular_velocity()) as f64
            }
            <RigidBody2DFloatKind>::CenterOfMassX => {
                let obj = &self.owner;
                (obj.get_center_of_mass().x) as f64
            }
            <RigidBody2DFloatKind>::CenterOfMassY => {
                let obj = &self.owner;
                (obj.get_center_of_mass().y) as f64
            }
            <RigidBody2DFloatKind>::ConstantForceX => {
                let obj = &self.owner;
                (obj.get_constant_force().x) as f64
            }
            <RigidBody2DFloatKind>::ConstantForceY => {
                let obj = &self.owner;
                (obj.get_constant_force().y) as f64
            }
            <RigidBody2DFloatKind>::ConstantTorque => {
                let obj = &self.owner;
                (obj.get_constant_torque()) as f64
            }
            <RigidBody2DFloatKind>::GravityScale => {
                let obj = &self.owner;
                (obj.get_gravity_scale()) as f64
            }
            <RigidBody2DFloatKind>::Inertia => {
                let obj = &self.owner;
                (obj.get_inertia()) as f64
            }
            <RigidBody2DFloatKind>::LinearDamp => {
                let obj = &self.owner;
                (obj.get_linear_damp()) as f64
            }
            <RigidBody2DFloatKind>::LinearVelocityX => {
                let obj = &self.owner;
                (obj.get_linear_velocity().x) as f64
            }
            <RigidBody2DFloatKind>::LinearVelocityY => {
                let obj = &self.owner;
                (obj.get_linear_velocity().y) as f64
            }
            <RigidBody2DFloatKind>::Mass => {
                let obj = &self.owner;
                (obj.get_mass()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <RigidBody2DFloatKind>::AngularDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_angular_damp(val as f32);
            }
            <RigidBody2DFloatKind>::AngularVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_angular_velocity(val as f32);
            }
            <RigidBody2DFloatKind>::CenterOfMassX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_center_of_mass();
                prop_val.x = val as f32;
                obj.set_center_of_mass(prop_val);
            }
            <RigidBody2DFloatKind>::CenterOfMassY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_center_of_mass();
                prop_val.y = val as f32;
                obj.set_center_of_mass(prop_val);
            }
            <RigidBody2DFloatKind>::ConstantForceX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_force();
                prop_val.x = val as f32;
                obj.set_constant_force(prop_val);
            }
            <RigidBody2DFloatKind>::ConstantForceY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_force();
                prop_val.y = val as f32;
                obj.set_constant_force(prop_val);
            }
            <RigidBody2DFloatKind>::ConstantTorque => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_constant_torque(val as f32);
            }
            <RigidBody2DFloatKind>::GravityScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity_scale(val as f32);
            }
            <RigidBody2DFloatKind>::Inertia => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_inertia(val as f32);
            }
            <RigidBody2DFloatKind>::LinearDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_linear_damp(val as f32);
            }
            <RigidBody2DFloatKind>::LinearVelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_linear_velocity();
                prop_val.x = val as f32;
                obj.set_linear_velocity(prop_val);
            }
            <RigidBody2DFloatKind>::LinearVelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_linear_velocity();
                prop_val.y = val as f32;
                obj.set_linear_velocity(prop_val);
            }
            <RigidBody2DFloatKind>::Mass => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_mass(val as f32);
            }
        }
    }
}
impl IPropertyData for RigidBody2DFloatData {
    type Target = RigidBody2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RigidBody2DFloatKind>::AngularDamp => NodePath::from("angular_damp"),
            <RigidBody2DFloatKind>::AngularVelocity => NodePath::from("angular_velocity"),
            <RigidBody2DFloatKind>::CenterOfMassX => NodePath::from("center_of_mass:x"),
            <RigidBody2DFloatKind>::CenterOfMassY => NodePath::from("center_of_mass:y"),
            <RigidBody2DFloatKind>::ConstantForceX => NodePath::from("constant_force:x"),
            <RigidBody2DFloatKind>::ConstantForceY => NodePath::from("constant_force:y"),
            <RigidBody2DFloatKind>::ConstantTorque => NodePath::from("constant_torque"),
            <RigidBody2DFloatKind>::GravityScale => NodePath::from("gravity_scale"),
            <RigidBody2DFloatKind>::Inertia => NodePath::from("inertia"),
            <RigidBody2DFloatKind>::LinearDamp => NodePath::from("linear_damp"),
            <RigidBody2DFloatKind>::LinearVelocityX => {
                NodePath::from("linear_velocity:x")
            }
            <RigidBody2DFloatKind>::LinearVelocityY => {
                NodePath::from("linear_velocity:y")
            }
            <RigidBody2DFloatKind>::Mass => NodePath::from("mass"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<RigidBody2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<RigidBody2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for RigidBody2DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RigidBody2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "angular_damp" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::AngularDamp,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "angular_velocity" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::AngularVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "center_of_mass:x" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::CenterOfMassX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "center_of_mass:y" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::CenterOfMassY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_force:x" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::ConstantForceX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_force:y" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::ConstantForceY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_torque" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::ConstantTorque,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_scale" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::GravityScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "inertia" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::Inertia,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_damp" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::LinearDamp,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_velocity:x" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::LinearVelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_velocity:y" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::LinearVelocityY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "mass" => {
                        Some(Self {
                            property: <RigidBody2DFloatKind>::Mass,
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
pub struct RigidBody2DVector2Data {
    pub property: RigidBody2DVector2Kind,
    pub owner: Gd<RigidBody2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RigidBody2DVector2Kind {
    CenterOfMass,
    ConstantForce,
    LinearVelocity,
}
impl IProperty<Vector2> for RigidBody2DVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <RigidBody2DVector2Kind>::CenterOfMass => {
                let obj = &self.owner;
                obj.get_center_of_mass()
            }
            <RigidBody2DVector2Kind>::ConstantForce => {
                let obj = &self.owner;
                obj.get_constant_force()
            }
            <RigidBody2DVector2Kind>::LinearVelocity => {
                let obj = &self.owner;
                obj.get_linear_velocity()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <RigidBody2DVector2Kind>::CenterOfMass => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_center_of_mass(val);
            }
            <RigidBody2DVector2Kind>::ConstantForce => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_constant_force(val);
            }
            <RigidBody2DVector2Kind>::LinearVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_linear_velocity(val);
            }
        }
    }
}
impl IPropertyData for RigidBody2DVector2Data {
    type Target = RigidBody2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RigidBody2DVector2Kind>::CenterOfMass => NodePath::from("center_of_mass"),
            <RigidBody2DVector2Kind>::ConstantForce => NodePath::from("constant_force"),
            <RigidBody2DVector2Kind>::LinearVelocity => NodePath::from("linear_velocity"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<RigidBody2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<RigidBody2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for RigidBody2DVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RigidBody2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "center_of_mass" => {
                        Some(Self {
                            property: <RigidBody2DVector2Kind>::CenterOfMass,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_force" => {
                        Some(Self {
                            property: <RigidBody2DVector2Kind>::ConstantForce,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_velocity" => {
                        Some(Self {
                            property: <RigidBody2DVector2Kind>::LinearVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoRigidBody2D<Marker = ()> {
    fn do_angular_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_angular_velocity(
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
    fn do_constant_torque(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_inertia(
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
    fn do_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_linear_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_mass(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>>;
    fn do_center_of_mass(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_constant_force(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_linear_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<RigidBody2D> + Inherits<Object>> SpireDoRigidBody2D<()>
for Gd<Class> {
    fn do_angular_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::AngularDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_angular_velocity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::AngularVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
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
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::CenterOfMassX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
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
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::CenterOfMassY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
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
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::ConstantForceX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
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
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::ConstantForceY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_torque(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::ConstantTorque,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
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
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::GravityScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_inertia(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::Inertia,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
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
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::LinearDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
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
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::LinearVelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::LinearVelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
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
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::LinearVelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::LinearVelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_mass(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::Mass,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_center_of_mass(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = RigidBody2DVector2Data {
            property: <RigidBody2DVector2Kind>::CenterOfMass,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_force(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = RigidBody2DVector2Data {
            property: <RigidBody2DVector2Kind>::ConstantForce,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = RigidBody2DVector2Data {
            property: <RigidBody2DVector2Kind>::LinearVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = RigidBody2DVector2Data {
            property: <RigidBody2DVector2Kind>::LinearVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<RigidBody2D> + Inherits<Object>,
> SpireDoRigidBody2D<BaseMarker> for T {
    fn do_angular_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::AngularDamp,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_angular_velocity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::AngularVelocity,
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
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::CenterOfMassX,
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
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::CenterOfMassY,
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
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::ConstantForceX,
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
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::ConstantForceY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_torque(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::ConstantTorque,
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
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::GravityScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_inertia(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::Inertia,
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
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::LinearDamp,
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
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::LinearVelocityX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::LinearVelocityX,
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
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::LinearVelocityY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::LinearVelocityY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_mass(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DFloatData {
            property: <RigidBody2DFloatKind>::Mass,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_center_of_mass(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DVector2Data {
            property: <RigidBody2DVector2Kind>::CenterOfMass,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_force(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DVector2Data {
            property: <RigidBody2DVector2Kind>::ConstantForce,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_linear_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DVector2Data {
            property: <RigidBody2DVector2Kind>::LinearVelocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DVector2Data {
            property: <RigidBody2DVector2Kind>::LinearVelocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
