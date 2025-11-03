use super::*;
#[derive(Debug, Clone)]
pub struct StaticBody2DFloatData {
    pub property: StaticBody2DFloatKind,
    pub owner: Gd<StaticBody2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticBody2DFloatKind {
    ConstantAngularVelocity,
    ConstantLinearVelocityX,
    ConstantLinearVelocityY,
}
impl IProperty<f64> for StaticBody2DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <StaticBody2DFloatKind>::ConstantAngularVelocity => {
                let obj = &self.owner;
                (obj.get_constant_angular_velocity()) as f64
            }
            <StaticBody2DFloatKind>::ConstantLinearVelocityX => {
                let obj = &self.owner;
                (obj.get_constant_linear_velocity().x) as f64
            }
            <StaticBody2DFloatKind>::ConstantLinearVelocityY => {
                let obj = &self.owner;
                (obj.get_constant_linear_velocity().y) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <StaticBody2DFloatKind>::ConstantAngularVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_constant_angular_velocity(val as f32);
            }
            <StaticBody2DFloatKind>::ConstantLinearVelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_linear_velocity();
                prop_val.x = val as f32;
                obj.set_constant_linear_velocity(prop_val);
            }
            <StaticBody2DFloatKind>::ConstantLinearVelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_linear_velocity();
                prop_val.y = val as f32;
                obj.set_constant_linear_velocity(prop_val);
            }
        }
    }
}
impl IPropertyData for StaticBody2DFloatData {
    type Target = StaticBody2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <StaticBody2DFloatKind>::ConstantAngularVelocity => {
                NodePath::from("constant_angular_velocity")
            }
            <StaticBody2DFloatKind>::ConstantLinearVelocityX => {
                NodePath::from("constant_linear_velocity:x")
            }
            <StaticBody2DFloatKind>::ConstantLinearVelocityY => {
                NodePath::from("constant_linear_velocity:y")
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
            ObjectOrNode::Object(obj) => obj.try_cast::<StaticBody2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<StaticBody2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for StaticBody2DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<StaticBody2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "constant_angular_velocity" => {
                        Some(Self {
                            property: <StaticBody2DFloatKind>::ConstantAngularVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_linear_velocity:x" => {
                        Some(Self {
                            property: <StaticBody2DFloatKind>::ConstantLinearVelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_linear_velocity:y" => {
                        Some(Self {
                            property: <StaticBody2DFloatKind>::ConstantLinearVelocityY,
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
pub struct StaticBody2DVector2Data {
    pub property: StaticBody2DVector2Kind,
    pub owner: Gd<StaticBody2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticBody2DVector2Kind {
    ConstantLinearVelocity,
}
impl IProperty<Vector2> for StaticBody2DVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <StaticBody2DVector2Kind>::ConstantLinearVelocity => {
                let obj = &self.owner;
                obj.get_constant_linear_velocity()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <StaticBody2DVector2Kind>::ConstantLinearVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_constant_linear_velocity(val);
            }
        }
    }
}
impl IPropertyData for StaticBody2DVector2Data {
    type Target = StaticBody2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <StaticBody2DVector2Kind>::ConstantLinearVelocity => {
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
            ObjectOrNode::Object(obj) => obj.try_cast::<StaticBody2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<StaticBody2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for StaticBody2DVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<StaticBody2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "constant_linear_velocity" => {
                        Some(Self {
                            property: <StaticBody2DVector2Kind>::ConstantLinearVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoStaticBody2D<Marker = ()> {
    fn do_constant_angular_velocity(
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
    fn do_constant_linear_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<StaticBody2D> + Inherits<Object>> SpireDoStaticBody2D<()>
for Gd<Class> {
    fn do_constant_angular_velocity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = StaticBody2DFloatData {
            property: <StaticBody2DFloatKind>::ConstantAngularVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody2D>().upcast::<Node>(),
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
        let data = StaticBody2DFloatData {
            property: <StaticBody2DFloatKind>::ConstantLinearVelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody2D>().upcast::<Node>(),
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
        let data = StaticBody2DFloatData {
            property: <StaticBody2DFloatKind>::ConstantLinearVelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_linear_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = StaticBody2DVector2Data {
            property: <StaticBody2DVector2Kind>::ConstantLinearVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<StaticBody2D> + Inherits<Object>,
> SpireDoStaticBody2D<BaseMarker> for T {
    fn do_constant_angular_velocity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<StaticBody2D> = self.to_gd().upcast();
        let data = StaticBody2DFloatData {
            property: <StaticBody2DFloatKind>::ConstantAngularVelocity,
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
        let owner: Gd<StaticBody2D> = self.to_gd().upcast();
        let data = StaticBody2DFloatData {
            property: <StaticBody2DFloatKind>::ConstantLinearVelocityX,
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
        let owner: Gd<StaticBody2D> = self.to_gd().upcast();
        let data = StaticBody2DFloatData {
            property: <StaticBody2DFloatKind>::ConstantLinearVelocityY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_linear_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<StaticBody2D> = self.to_gd().upcast();
        let data = StaticBody2DVector2Data {
            property: <StaticBody2DVector2Kind>::ConstantLinearVelocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
