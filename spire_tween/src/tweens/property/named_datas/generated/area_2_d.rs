use super::*;
#[derive(Debug, Clone)]
pub struct Area2DF32Data {
    pub property: Area2DF32Kind,
    pub owner: Gd<Area2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area2DF32Kind {
    Gravity,
    GravityPointUnitDistance,
    GravityDirectionX,
    GravityDirectionY,
    GravityPointCenterX,
    GravityPointCenterY,
    LinearDamp,
}
impl IProperty<f32> for Area2DF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <Area2DF32Kind>::Gravity => {
                let obj = &self.owner;
                obj.get_gravity()
            }
            <Area2DF32Kind>::GravityPointUnitDistance => {
                let obj = &self.owner;
                obj.get_gravity_point_unit_distance()
            }
            <Area2DF32Kind>::GravityDirectionX => {
                let obj = &self.owner;
                obj.get_gravity_direction().x
            }
            <Area2DF32Kind>::GravityDirectionY => {
                let obj = &self.owner;
                obj.get_gravity_direction().y
            }
            <Area2DF32Kind>::GravityPointCenterX => {
                let obj = &self.owner;
                obj.get_gravity_point_center().x
            }
            <Area2DF32Kind>::GravityPointCenterY => {
                let obj = &self.owner;
                obj.get_gravity_point_center().y
            }
            <Area2DF32Kind>::LinearDamp => {
                let obj = &self.owner;
                obj.get_linear_damp()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <Area2DF32Kind>::Gravity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity(val);
            }
            <Area2DF32Kind>::GravityPointUnitDistance => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity_point_unit_distance(val);
            }
            <Area2DF32Kind>::GravityDirectionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_direction();
                prop_val.x = val;
                obj.set_gravity_direction(prop_val);
            }
            <Area2DF32Kind>::GravityDirectionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_direction();
                prop_val.y = val;
                obj.set_gravity_direction(prop_val);
            }
            <Area2DF32Kind>::GravityPointCenterX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_point_center();
                prop_val.x = val;
                obj.set_gravity_point_center(prop_val);
            }
            <Area2DF32Kind>::GravityPointCenterY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_point_center();
                prop_val.y = val;
                obj.set_gravity_point_center(prop_val);
            }
            <Area2DF32Kind>::LinearDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_linear_damp(val);
            }
        }
    }
}
impl IPropertyData for Area2DF32Data {
    type Target = Area2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Area2DF32Kind>::Gravity => NodePath::from("gravity"),
            <Area2DF32Kind>::GravityPointUnitDistance => {
                NodePath::from("gravity_point_unit_distance")
            }
            <Area2DF32Kind>::GravityDirectionX => NodePath::from("gravity_direction:x"),
            <Area2DF32Kind>::GravityDirectionY => NodePath::from("gravity_direction:y"),
            <Area2DF32Kind>::GravityPointCenterX => {
                NodePath::from("gravity_point_center:x")
            }
            <Area2DF32Kind>::GravityPointCenterY => {
                NodePath::from("gravity_point_center:y")
            }
            <Area2DF32Kind>::LinearDamp => NodePath::from("linear_damp"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Area2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Area2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Area2DF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Area2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "gravity" => {
                        Some(Self {
                            property: <Area2DF32Kind>::Gravity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_unit_distance" => {
                        Some(Self {
                            property: <Area2DF32Kind>::GravityPointUnitDistance,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_direction:x" => {
                        Some(Self {
                            property: <Area2DF32Kind>::GravityDirectionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_direction:y" => {
                        Some(Self {
                            property: <Area2DF32Kind>::GravityDirectionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_center:x" => {
                        Some(Self {
                            property: <Area2DF32Kind>::GravityPointCenterX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_center:y" => {
                        Some(Self {
                            property: <Area2DF32Kind>::GravityPointCenterY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_damp" => {
                        Some(Self {
                            property: <Area2DF32Kind>::LinearDamp,
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
pub struct Area2DVector2Data {
    pub property: Area2DVector2Kind,
    pub owner: Gd<Area2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area2DVector2Kind {
    GravityDirection,
    GravityPointCenter,
}
impl IProperty<Vector2> for Area2DVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <Area2DVector2Kind>::GravityDirection => {
                let obj = &self.owner;
                obj.get_gravity_direction()
            }
            <Area2DVector2Kind>::GravityPointCenter => {
                let obj = &self.owner;
                obj.get_gravity_point_center()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <Area2DVector2Kind>::GravityDirection => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity_direction(val);
            }
            <Area2DVector2Kind>::GravityPointCenter => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity_point_center(val);
            }
        }
    }
}
impl IPropertyData for Area2DVector2Data {
    type Target = Area2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Area2DVector2Kind>::GravityDirection => NodePath::from("gravity_direction"),
            <Area2DVector2Kind>::GravityPointCenter => {
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
            ObjectOrNode::Object(obj) => obj.try_cast::<Area2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Area2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Area2DVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Area2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "gravity_direction" => {
                        Some(Self {
                            property: <Area2DVector2Kind>::GravityDirection,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_center" => {
                        Some(Self {
                            property: <Area2DVector2Kind>::GravityPointCenter,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoArea2D<Marker = ()> {
    fn area_do_gravity(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn area_do_gravity_point_unit_distance(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn area_do_gravity_direction_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn area_do_gravity_direction_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn area_do_gravity_point_center_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn area_do_gravity_point_center_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn area_do_linear_damp(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn area_do_gravity_direction(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn area_do_gravity_point_center(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<Area2D> + Inherits<Object>> DoArea2D<()> for Gd<Class> {
    fn area_do_gravity(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::Gravity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_point_unit_distance(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::GravityPointUnitDistance,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_direction_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::GravityDirectionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_direction_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::GravityDirectionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_point_center_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::GravityPointCenterX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_point_center_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::GravityPointCenterY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_linear_damp(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::LinearDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_direction(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Area2DVector2Data {
            property: <Area2DVector2Kind>::GravityDirection,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_point_center(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Area2DVector2Data {
            property: <Area2DVector2Kind>::GravityPointCenter,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<Area2D> + Inherits<Object>> DoArea2D<BaseMarker> for T {
    fn area_do_gravity(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::Gravity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_point_unit_distance(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::GravityPointUnitDistance,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_direction_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::GravityDirectionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_direction_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::GravityDirectionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_point_center_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::GravityPointCenterX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_point_center_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::GravityPointCenterY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_linear_damp(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DF32Data {
            property: <Area2DF32Kind>::LinearDamp,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_direction(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DVector2Data {
            property: <Area2DVector2Kind>::GravityDirection,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn area_do_gravity_point_center(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DVector2Data {
            property: <Area2DVector2Kind>::GravityPointCenter,
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
pub struct Area2DTweener {}
#[godot_api]
impl Area2DTweener {
    #[func]
    fn area_do_gravity(
        node: Gd<Area2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.area_do_gravity(to, duration), gd
        }
    }
    #[func]
    fn area_do_gravity_point_unit_distance(
        node: Gd<Area2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.area_do_gravity_point_unit_distance(to, duration), gd
        }
    }
    #[func]
    fn area_do_gravity_direction_x(
        node: Gd<Area2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.area_do_gravity_direction_x(to, duration), gd
        }
    }
    #[func]
    fn area_do_gravity_direction_y(
        node: Gd<Area2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.area_do_gravity_direction_y(to, duration), gd
        }
    }
    #[func]
    fn area_do_gravity_point_center_x(
        node: Gd<Area2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.area_do_gravity_point_center_x(to, duration), gd
        }
    }
    #[func]
    fn area_do_gravity_point_center_y(
        node: Gd<Area2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.area_do_gravity_point_center_y(to, duration), gd
        }
    }
    #[func]
    fn area_do_linear_damp(
        node: Gd<Area2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.area_do_linear_damp(to, duration), gd
        }
    }
    #[func]
    fn area_do_gravity_direction(
        node: Gd<Area2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.area_do_gravity_direction(to, duration), gd
        }
    }
    #[func]
    fn area_do_gravity_point_center(
        node: Gd<Area2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.area_do_gravity_point_center(to, duration), gd
        }
    }
}
