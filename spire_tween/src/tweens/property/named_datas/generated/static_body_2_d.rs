use super::*;
#[derive(Debug, Clone)]
pub struct StaticBody2DF32Data {
    pub property: StaticBody2DF32Kind,
    pub owner: Gd<StaticBody2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticBody2DF32Kind {
    ConstantAngularVelocity,
    ConstantLinearVelocityX,
    ConstantLinearVelocityY,
}
impl IProperty<f32> for StaticBody2DF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <StaticBody2DF32Kind>::ConstantAngularVelocity => {
                let obj = &self.owner;
                obj.get_constant_angular_velocity()
            }
            <StaticBody2DF32Kind>::ConstantLinearVelocityX => {
                let obj = &self.owner;
                obj.get_constant_linear_velocity().x
            }
            <StaticBody2DF32Kind>::ConstantLinearVelocityY => {
                let obj = &self.owner;
                obj.get_constant_linear_velocity().y
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <StaticBody2DF32Kind>::ConstantAngularVelocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_constant_angular_velocity(val);
            }
            <StaticBody2DF32Kind>::ConstantLinearVelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_linear_velocity();
                prop_val.x = val;
                obj.set_constant_linear_velocity(prop_val);
            }
            <StaticBody2DF32Kind>::ConstantLinearVelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_constant_linear_velocity();
                prop_val.y = val;
                obj.set_constant_linear_velocity(prop_val);
            }
        }
    }
}
impl IPropertyData for StaticBody2DF32Data {
    type Target = StaticBody2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <StaticBody2DF32Kind>::ConstantAngularVelocity => {
                NodePath::from("constant_angular_velocity")
            }
            <StaticBody2DF32Kind>::ConstantLinearVelocityX => {
                NodePath::from("constant_linear_velocity:x")
            }
            <StaticBody2DF32Kind>::ConstantLinearVelocityY => {
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
impl TryFromPathAndObject for StaticBody2DF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<StaticBody2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "constant_angular_velocity" => {
                        Some(Self {
                            property: <StaticBody2DF32Kind>::ConstantAngularVelocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_linear_velocity:x" => {
                        Some(Self {
                            property: <StaticBody2DF32Kind>::ConstantLinearVelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "constant_linear_velocity:y" => {
                        Some(Self {
                            property: <StaticBody2DF32Kind>::ConstantLinearVelocityY,
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
pub trait DoStaticBody2D<Marker = ()> {
    fn static_do_constant_angular_velocity(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn static_do_constant_linear_velocity_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn static_do_constant_linear_velocity_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn static_do_constant_linear_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<StaticBody2D> + Inherits<Object>> DoStaticBody2D<()> for Gd<Class> {
    fn static_do_constant_angular_velocity(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = StaticBody2DF32Data {
            property: <StaticBody2DF32Kind>::ConstantAngularVelocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn static_do_constant_linear_velocity_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = StaticBody2DF32Data {
            property: <StaticBody2DF32Kind>::ConstantLinearVelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn static_do_constant_linear_velocity_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = StaticBody2DF32Data {
            property: <StaticBody2DF32Kind>::ConstantLinearVelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<StaticBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn static_do_constant_linear_velocity(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<StaticBody2D> + Inherits<Object>,
> DoStaticBody2D<BaseMarker> for T {
    fn static_do_constant_angular_velocity(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<StaticBody2D> = self.to_gd().upcast();
        let data = StaticBody2DF32Data {
            property: <StaticBody2DF32Kind>::ConstantAngularVelocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn static_do_constant_linear_velocity_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<StaticBody2D> = self.to_gd().upcast();
        let data = StaticBody2DF32Data {
            property: <StaticBody2DF32Kind>::ConstantLinearVelocityX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn static_do_constant_linear_velocity_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<StaticBody2D> = self.to_gd().upcast();
        let data = StaticBody2DF32Data {
            property: <StaticBody2DF32Kind>::ConstantLinearVelocityY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn static_do_constant_linear_velocity(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct StaticBody2DTweener {}
#[godot_api]
impl StaticBody2DTweener {
    #[func]
    fn static_do_constant_angular_velocity(
        node: Gd<StaticBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.static_do_constant_angular_velocity(to, duration), gd
        }
    }
    #[func]
    fn static_do_constant_linear_velocity_x(
        node: Gd<StaticBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.static_do_constant_linear_velocity_x(to, duration), gd
        }
    }
    #[func]
    fn static_do_constant_linear_velocity_y(
        node: Gd<StaticBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.static_do_constant_linear_velocity_y(to, duration), gd
        }
    }
    #[func]
    fn static_do_constant_linear_velocity(
        node: Gd<StaticBody2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.static_do_constant_linear_velocity(to, duration), gd
        }
    }
}
