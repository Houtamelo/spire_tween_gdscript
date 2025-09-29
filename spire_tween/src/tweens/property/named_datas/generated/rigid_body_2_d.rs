use super::*;
#[derive(Debug, Clone)]
pub struct RigidBody2DF32Data {
    pub property: RigidBody2DF32Kind,
    pub owner: Gd<RigidBody2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RigidBody2DF32Kind {
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
    Mass,
}
impl IProperty<f32> for RigidBody2DF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <RigidBody2DF32Kind>::AngularDamp => {
                let obj = &self.owner;
                obj.get_angular_damp()
            }
            <RigidBody2DF32Kind>::AngularVelocity => {
                let obj = &self.owner;
                obj.get_angular_velocity()
            }
            <RigidBody2DF32Kind>::CenterOfMassX => {
                let obj = &self.owner;
                obj.get_center_of_mass().x
            }
            <RigidBody2DF32Kind>::CenterOfMassY => {
                let obj = &self.owner;
                obj.get_center_of_mass().y
            }
            <RigidBody2DF32Kind>::ConstantForceX => {
                let obj = &self.owner;
                obj.get_constant_force().x
            }
            <RigidBody2DF32Kind>::ConstantForceY => {
                let obj = &self.owner;
                obj.get_constant_force().y
            }
            <RigidBody2DF32Kind>::ConstantTorque => {
                let obj = &self.owner;
                obj.get_constant_torque()
            }
            <RigidBody2DF32Kind>::GravityScale => {
                let obj = &self.owner;
                obj.get_gravity_scale()
            }
            <RigidBody2DF32Kind>::Inertia => {
                let obj = &self.owner;
                obj.get_inertia()
            }
            <RigidBody2DF32Kind>::LinearDamp => {
                let obj = &self.owner;
                obj.get_linear_damp()
            }
            <RigidBody2DF32Kind>::Mass => {
                let obj = &self.owner;
                obj.get_mass()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <RigidBody2DF32Kind>::AngularDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_angular_damp(val);
            }
            <RigidBody2DF32Kind>::AngularVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_angular_velocity(val);
            }
            <RigidBody2DF32Kind>::CenterOfMassX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_center_of_mass();
                prop_val.x = val;
                obj.set_center_of_mass(prop_val);
            }
            <RigidBody2DF32Kind>::CenterOfMassY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_center_of_mass();
                prop_val.y = val;
                obj.set_center_of_mass(prop_val);
            }
            <RigidBody2DF32Kind>::ConstantForceX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_force();
                prop_val.x = val;
                obj.set_constant_force(prop_val);
            }
            <RigidBody2DF32Kind>::ConstantForceY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_force();
                prop_val.y = val;
                obj.set_constant_force(prop_val);
            }
            <RigidBody2DF32Kind>::ConstantTorque => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_constant_torque(val);
            }
            <RigidBody2DF32Kind>::GravityScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity_scale(val);
            }
            <RigidBody2DF32Kind>::Inertia => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_inertia(val);
            }
            <RigidBody2DF32Kind>::LinearDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_linear_damp(val);
            }
            <RigidBody2DF32Kind>::Mass => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_mass(val);
            }
        }
    }
}
impl IPropertyData for RigidBody2DF32Data {
    type Target = RigidBody2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RigidBody2DF32Kind>::AngularDamp => NodePath::from("angular_damp"),
            <RigidBody2DF32Kind>::AngularVelocity => NodePath::from("angular_velocity"),
            <RigidBody2DF32Kind>::CenterOfMassX => NodePath::from("center_of_mass:x"),
            <RigidBody2DF32Kind>::CenterOfMassY => NodePath::from("center_of_mass:y"),
            <RigidBody2DF32Kind>::ConstantForceX => NodePath::from("constant_force:x"),
            <RigidBody2DF32Kind>::ConstantForceY => NodePath::from("constant_force:y"),
            <RigidBody2DF32Kind>::ConstantTorque => NodePath::from("constant_torque"),
            <RigidBody2DF32Kind>::GravityScale => NodePath::from("gravity_scale"),
            <RigidBody2DF32Kind>::Inertia => NodePath::from("inertia"),
            <RigidBody2DF32Kind>::LinearDamp => NodePath::from("linear_damp"),
            <RigidBody2DF32Kind>::Mass => NodePath::from("mass"),
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
impl TryFromPathAndObject for RigidBody2DF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RigidBody2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "angular_damp" => {
                        Some(Self {
                            property: <RigidBody2DF32Kind>::AngularDamp,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "angular_velocity" => {
                        Some(Self {
                            property: <RigidBody2DF32Kind>::AngularVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "center_of_mass:x" => {
                        Some(Self {
                            property: <RigidBody2DF32Kind>::CenterOfMassX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "center_of_mass:y" => {
                        Some(Self {
                            property: <RigidBody2DF32Kind>::CenterOfMassY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_force:x" => {
                        Some(Self {
                            property: <RigidBody2DF32Kind>::ConstantForceX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_force:y" => {
                        Some(Self {
                            property: <RigidBody2DF32Kind>::ConstantForceY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_torque" => {
                        Some(Self {
                            property: <RigidBody2DF32Kind>::ConstantTorque,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_scale" => {
                        Some(Self {
                            property: <RigidBody2DF32Kind>::GravityScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "inertia" => {
                        Some(Self {
                            property: <RigidBody2DF32Kind>::Inertia,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_damp" => {
                        Some(Self {
                            property: <RigidBody2DF32Kind>::LinearDamp,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "mass" => {
                        Some(Self {
                            property: <RigidBody2DF32Kind>::Mass,
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
                    _ => None,
                }
            })
    }
}
pub trait DoRigidBody2D<Marker = ()> {
    fn do_angular_damp(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_angular_velocity(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_center_of_mass_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_center_of_mass_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_constant_force_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_constant_force_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_constant_torque(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_gravity_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_inertia(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_linear_damp(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_mass(&self, end_val: f32, duration: f64) -> SpireTween<LerpPropertyData<f32>>;
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
}
impl<Class: Inherits<RigidBody2D> + Inherits<Object>> DoRigidBody2D<()> for Gd<Class> {
    fn do_angular_damp(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::AngularDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_angular_velocity(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::AngularVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_center_of_mass_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::CenterOfMassX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_center_of_mass_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::CenterOfMassY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_constant_force_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::ConstantForceX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_constant_force_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::ConstantForceY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_constant_torque(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::ConstantTorque,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_gravity_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::GravityScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_inertia(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::Inertia,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_linear_damp(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::LinearDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_mass(&self, end_val: f32, duration: f64) -> SpireTween<LerpPropertyData<f32>> {
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::Mass,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<RigidBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<RigidBody2D> + Inherits<Object>,
> DoRigidBody2D<BaseMarker> for T {
    fn do_angular_damp(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::AngularDamp,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_angular_velocity(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::AngularVelocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_center_of_mass_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::CenterOfMassX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_center_of_mass_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::CenterOfMassY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_constant_force_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::ConstantForceX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_constant_force_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::ConstantForceY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_constant_torque(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::ConstantTorque,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_gravity_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::GravityScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_inertia(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::Inertia,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_linear_damp(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::LinearDamp,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_mass(&self, end_val: f32, duration: f64) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<RigidBody2D> = self.to_gd().upcast();
        let data = RigidBody2DF32Data {
            property: <RigidBody2DF32Kind>::Mass,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct RigidBody2DTweener {}
#[godot_api]
impl RigidBody2DTweener {
    #[func]
    fn do_angular_damp(
        node: Gd<RigidBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_angular_damp(to, duration), gd
        }
    }
    #[func]
    fn do_angular_velocity(
        node: Gd<RigidBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_angular_velocity(to, duration), gd
        }
    }
    #[func]
    fn do_center_of_mass_x(
        node: Gd<RigidBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_center_of_mass_x(to, duration), gd
        }
    }
    #[func]
    fn do_center_of_mass_y(
        node: Gd<RigidBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_center_of_mass_y(to, duration), gd
        }
    }
    #[func]
    fn do_constant_force_x(
        node: Gd<RigidBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_constant_force_x(to, duration), gd
        }
    }
    #[func]
    fn do_constant_force_y(
        node: Gd<RigidBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_constant_force_y(to, duration), gd
        }
    }
    #[func]
    fn do_constant_torque(
        node: Gd<RigidBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_constant_torque(to, duration), gd
        }
    }
    #[func]
    fn do_gravity_scale(
        node: Gd<RigidBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_gravity_scale(to, duration), gd
        }
    }
    #[func]
    fn do_inertia(
        node: Gd<RigidBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_inertia(to, duration), gd
        }
    }
    #[func]
    fn do_linear_damp(
        node: Gd<RigidBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_linear_damp(to, duration), gd
        }
    }
    #[func]
    fn do_mass(
        node: Gd<RigidBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_mass(to, duration), gd
        }
    }
    #[func]
    fn do_center_of_mass(
        node: Gd<RigidBody2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_center_of_mass(to, duration), gd
        }
    }
    #[func]
    fn do_constant_force(
        node: Gd<RigidBody2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_constant_force(to, duration), gd
        }
    }
}
