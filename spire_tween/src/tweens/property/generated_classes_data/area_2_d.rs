use super::*;
#[derive(Debug, Clone)]
pub struct Area2DFloatData {
    pub property: Area2DFloatKind,
    pub owner: Gd<Area2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area2DFloatKind {
    Gravity,
    GravityPointUnitDistance,
    GravityDirectionX,
    GravityDirectionY,
    GravityPointCenterX,
    GravityPointCenterY,
    LinearDamp,
}
impl IProperty<f64> for Area2DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <Area2DFloatKind>::Gravity => {
                let obj = &self.owner;
                (obj.get_gravity()) as f64
            }
            <Area2DFloatKind>::GravityPointUnitDistance => {
                let obj = &self.owner;
                (obj.get_gravity_point_unit_distance()) as f64
            }
            <Area2DFloatKind>::GravityDirectionX => {
                let obj = &self.owner;
                (obj.get_gravity_direction().x) as f64
            }
            <Area2DFloatKind>::GravityDirectionY => {
                let obj = &self.owner;
                (obj.get_gravity_direction().y) as f64
            }
            <Area2DFloatKind>::GravityPointCenterX => {
                let obj = &self.owner;
                (obj.get_gravity_point_center().x) as f64
            }
            <Area2DFloatKind>::GravityPointCenterY => {
                let obj = &self.owner;
                (obj.get_gravity_point_center().y) as f64
            }
            <Area2DFloatKind>::LinearDamp => {
                let obj = &self.owner;
                (obj.get_linear_damp()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <Area2DFloatKind>::Gravity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity(val as f32);
            }
            <Area2DFloatKind>::GravityPointUnitDistance => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_gravity_point_unit_distance(val as f32);
            }
            <Area2DFloatKind>::GravityDirectionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_direction();
                prop_val.x = val as f32;
                obj.set_gravity_direction(prop_val);
            }
            <Area2DFloatKind>::GravityDirectionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_direction();
                prop_val.y = val as f32;
                obj.set_gravity_direction(prop_val);
            }
            <Area2DFloatKind>::GravityPointCenterX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_point_center();
                prop_val.x = val as f32;
                obj.set_gravity_point_center(prop_val);
            }
            <Area2DFloatKind>::GravityPointCenterY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_gravity_point_center();
                prop_val.y = val as f32;
                obj.set_gravity_point_center(prop_val);
            }
            <Area2DFloatKind>::LinearDamp => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_linear_damp(val as f32);
            }
        }
    }
}
impl IPropertyData for Area2DFloatData {
    type Target = Area2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Area2DFloatKind>::Gravity => NodePath::from("gravity"),
            <Area2DFloatKind>::GravityPointUnitDistance => {
                NodePath::from("gravity_point_unit_distance")
            }
            <Area2DFloatKind>::GravityDirectionX => NodePath::from("gravity_direction:x"),
            <Area2DFloatKind>::GravityDirectionY => NodePath::from("gravity_direction:y"),
            <Area2DFloatKind>::GravityPointCenterX => {
                NodePath::from("gravity_point_center:x")
            }
            <Area2DFloatKind>::GravityPointCenterY => {
                NodePath::from("gravity_point_center:y")
            }
            <Area2DFloatKind>::LinearDamp => NodePath::from("linear_damp"),
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
impl TryFromPathAndObject for Area2DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Area2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "gravity" => {
                        Some(Self {
                            property: <Area2DFloatKind>::Gravity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_unit_distance" => {
                        Some(Self {
                            property: <Area2DFloatKind>::GravityPointUnitDistance,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_direction:x" => {
                        Some(Self {
                            property: <Area2DFloatKind>::GravityDirectionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_direction:y" => {
                        Some(Self {
                            property: <Area2DFloatKind>::GravityDirectionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_center:x" => {
                        Some(Self {
                            property: <Area2DFloatKind>::GravityPointCenterX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "gravity_point_center:y" => {
                        Some(Self {
                            property: <Area2DFloatKind>::GravityPointCenterY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "linear_damp" => {
                        Some(Self {
                            property: <Area2DFloatKind>::LinearDamp,
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
pub trait SpireDoArea2D<Marker = ()> {
    fn do_area_gravity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_point_unit_distance(
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
    fn do_area_linear_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_gravity_direction(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_gravity_point_center(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<Area2D> + Inherits<Object>> SpireDoArea2D<()> for Gd<Class> {
    fn do_area_gravity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::Gravity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
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
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::GravityPointUnitDistance,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
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
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::GravityDirectionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
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
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::GravityDirectionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
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
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::GravityPointCenterX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
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
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::GravityPointCenterY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_area_linear_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::LinearDamp,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Area2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_direction(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_center(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<T: WithBaseField + Inherits<Area2D> + Inherits<Object>> SpireDoArea2D<BaseMarker>
for T {
    fn do_area_gravity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::Gravity,
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
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::GravityPointUnitDistance,
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
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::GravityDirectionX,
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
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::GravityDirectionY,
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
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::GravityPointCenterX,
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
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::GravityPointCenterY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_area_linear_damp(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Area2D> = self.to_gd().upcast();
        let data = Area2DFloatData {
            property: <Area2DFloatKind>::LinearDamp,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_direction(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_gravity_point_center(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
