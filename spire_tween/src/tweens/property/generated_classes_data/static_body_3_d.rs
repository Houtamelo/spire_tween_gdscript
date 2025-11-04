use super::*;
#[derive(Debug, Clone)]
pub struct StaticBody3DFloatData {
    pub property: StaticBody3DFloatKind,
    pub owner: Gd<StaticBody3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticBody3DFloatKind {
    ConstantAngularVelocityX,
    ConstantAngularVelocityY,
    ConstantAngularVelocityZ,
    ConstantLinearVelocityX,
    ConstantLinearVelocityY,
    ConstantLinearVelocityZ,
}
impl IProperty<f64> for StaticBody3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <StaticBody3DFloatKind>::ConstantAngularVelocityX => {
                let obj = &self.owner;
                (obj.get_constant_angular_velocity().x) as f64
            }
            <StaticBody3DFloatKind>::ConstantAngularVelocityY => {
                let obj = &self.owner;
                (obj.get_constant_angular_velocity().y) as f64
            }
            <StaticBody3DFloatKind>::ConstantAngularVelocityZ => {
                let obj = &self.owner;
                (obj.get_constant_angular_velocity().z) as f64
            }
            <StaticBody3DFloatKind>::ConstantLinearVelocityX => {
                let obj = &self.owner;
                (obj.get_constant_linear_velocity().x) as f64
            }
            <StaticBody3DFloatKind>::ConstantLinearVelocityY => {
                let obj = &self.owner;
                (obj.get_constant_linear_velocity().y) as f64
            }
            <StaticBody3DFloatKind>::ConstantLinearVelocityZ => {
                let obj = &self.owner;
                (obj.get_constant_linear_velocity().z) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <StaticBody3DFloatKind>::ConstantAngularVelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_angular_velocity();
                prop_val.x = val as f32;
                obj.set_constant_angular_velocity(prop_val);
            }
            <StaticBody3DFloatKind>::ConstantAngularVelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_angular_velocity();
                prop_val.y = val as f32;
                obj.set_constant_angular_velocity(prop_val);
            }
            <StaticBody3DFloatKind>::ConstantAngularVelocityZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_angular_velocity();
                prop_val.z = val as f32;
                obj.set_constant_angular_velocity(prop_val);
            }
            <StaticBody3DFloatKind>::ConstantLinearVelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_linear_velocity();
                prop_val.x = val as f32;
                obj.set_constant_linear_velocity(prop_val);
            }
            <StaticBody3DFloatKind>::ConstantLinearVelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_linear_velocity();
                prop_val.y = val as f32;
                obj.set_constant_linear_velocity(prop_val);
            }
            <StaticBody3DFloatKind>::ConstantLinearVelocityZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_linear_velocity();
                prop_val.z = val as f32;
                obj.set_constant_linear_velocity(prop_val);
            }
        }
    }
}
impl IPropertyData for StaticBody3DFloatData {
    type Target = StaticBody3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <StaticBody3DFloatKind>::ConstantAngularVelocityX => {
                NodePath::from("constant_angular_velocity:x")
            }
            <StaticBody3DFloatKind>::ConstantAngularVelocityY => {
                NodePath::from("constant_angular_velocity:y")
            }
            <StaticBody3DFloatKind>::ConstantAngularVelocityZ => {
                NodePath::from("constant_angular_velocity:z")
            }
            <StaticBody3DFloatKind>::ConstantLinearVelocityX => {
                NodePath::from("constant_linear_velocity:x")
            }
            <StaticBody3DFloatKind>::ConstantLinearVelocityY => {
                NodePath::from("constant_linear_velocity:y")
            }
            <StaticBody3DFloatKind>::ConstantLinearVelocityZ => {
                NodePath::from("constant_linear_velocity:z")
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
            ObjectOrNode::Object(obj) => obj.try_cast::<StaticBody3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<StaticBody3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for StaticBody3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<StaticBody3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "constant_angular_velocity:x" => {
                        Some(Self {
                            property: <StaticBody3DFloatKind>::ConstantAngularVelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_angular_velocity:y" => {
                        Some(Self {
                            property: <StaticBody3DFloatKind>::ConstantAngularVelocityY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_angular_velocity:z" => {
                        Some(Self {
                            property: <StaticBody3DFloatKind>::ConstantAngularVelocityZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_linear_velocity:x" => {
                        Some(Self {
                            property: <StaticBody3DFloatKind>::ConstantLinearVelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_linear_velocity:y" => {
                        Some(Self {
                            property: <StaticBody3DFloatKind>::ConstantLinearVelocityY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_linear_velocity:z" => {
                        Some(Self {
                            property: <StaticBody3DFloatKind>::ConstantLinearVelocityZ,
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
pub struct StaticBody3DVector3Data {
    pub property: StaticBody3DVector3Kind,
    pub owner: Gd<StaticBody3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticBody3DVector3Kind {
    ConstantAngularVelocity,
    ConstantLinearVelocity,
}
impl IProperty<Vector3> for StaticBody3DVector3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <StaticBody3DVector3Kind>::ConstantAngularVelocity => {
                let obj = &self.owner;
                obj.get_constant_angular_velocity()
            }
            <StaticBody3DVector3Kind>::ConstantLinearVelocity => {
                let obj = &self.owner;
                obj.get_constant_linear_velocity()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <StaticBody3DVector3Kind>::ConstantAngularVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_constant_angular_velocity(val);
            }
            <StaticBody3DVector3Kind>::ConstantLinearVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_constant_linear_velocity(val);
            }
        }
    }
}
impl IPropertyData for StaticBody3DVector3Data {
    type Target = StaticBody3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <StaticBody3DVector3Kind>::ConstantAngularVelocity => {
                NodePath::from("constant_angular_velocity")
            }
            <StaticBody3DVector3Kind>::ConstantLinearVelocity => {
                NodePath::from("constant_linear_velocity")
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
            ObjectOrNode::Object(obj) => obj.try_cast::<StaticBody3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<StaticBody3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for StaticBody3DVector3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<StaticBody3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "constant_angular_velocity" => {
                        Some(Self {
                            property: <StaticBody3DVector3Kind>::ConstantAngularVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_linear_velocity" => {
                        Some(Self {
                            property: <StaticBody3DVector3Kind>::ConstantLinearVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoStaticBody3D<Marker = ()> {
    fn do_constant_angular_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_angular_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_angular_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_linear_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_linear_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_linear_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_angular_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_constant_linear_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
}
impl<Class: Inherits<StaticBody3D> + Inherits<Object>> SpireDoStaticBody3D<()>
for Gd<Class> {
    fn do_constant_angular_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantAngularVelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_angular_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantAngularVelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_angular_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantAngularVelocityZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_linear_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantLinearVelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_linear_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantLinearVelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_linear_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantLinearVelocityZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_angular_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = StaticBody3DVector3Data {
            property: <StaticBody3DVector3Kind>::ConstantAngularVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_linear_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = StaticBody3DVector3Data {
            property: <StaticBody3DVector3Kind>::ConstantLinearVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<StaticBody3D> + Inherits<Object>,
> SpireDoStaticBody3D<BaseMarker> for T {
    fn do_constant_angular_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<StaticBody3D> = self.to_gd().upcast();
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantAngularVelocityX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_angular_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<StaticBody3D> = self.to_gd().upcast();
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantAngularVelocityY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_angular_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<StaticBody3D> = self.to_gd().upcast();
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantAngularVelocityZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_linear_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<StaticBody3D> = self.to_gd().upcast();
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantLinearVelocityX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_linear_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<StaticBody3D> = self.to_gd().upcast();
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantLinearVelocityY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_linear_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<StaticBody3D> = self.to_gd().upcast();
        let data = StaticBody3DFloatData {
            property: <StaticBody3DFloatKind>::ConstantLinearVelocityZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_angular_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<StaticBody3D> = self.to_gd().upcast();
        let data = StaticBody3DVector3Data {
            property: <StaticBody3DVector3Kind>::ConstantAngularVelocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_linear_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<StaticBody3D> = self.to_gd().upcast();
        let data = StaticBody3DVector3Data {
            property: <StaticBody3DVector3Kind>::ConstantLinearVelocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
