use super::*;
#[derive(Debug, Clone)]
pub struct Node2DF32Data {
    pub property: Node2DF32Kind,
    pub owner: Gd<Node2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node2DF32Kind {
    PositionX,
    PositionY,
    GlobalPositionX,
    GlobalPositionY,
    ScaleX,
    ScaleY,
    GlobalScaleX,
    GlobalScaleY,
    Rotation,
    RotationDegrees,
    GlobalRotation,
    GlobalRotationDegrees,
    Skew,
    GlobalSkew,
}
impl IProperty<f32> for Node2DF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <Node2DF32Kind>::PositionX => {
                let obj = &self.owner;
                obj.get_position().x
            }
            <Node2DF32Kind>::PositionY => {
                let obj = &self.owner;
                obj.get_position().y
            }
            <Node2DF32Kind>::GlobalPositionX => {
                let obj = &self.owner;
                obj.get_global_position().x
            }
            <Node2DF32Kind>::GlobalPositionY => {
                let obj = &self.owner;
                obj.get_global_position().y
            }
            <Node2DF32Kind>::ScaleX => {
                let obj = &self.owner;
                obj.get_scale().x
            }
            <Node2DF32Kind>::ScaleY => {
                let obj = &self.owner;
                obj.get_scale().y
            }
            <Node2DF32Kind>::GlobalScaleX => {
                let obj = &self.owner;
                obj.get_global_scale().x
            }
            <Node2DF32Kind>::GlobalScaleY => {
                let obj = &self.owner;
                obj.get_global_scale().y
            }
            <Node2DF32Kind>::Rotation => {
                let obj = &self.owner;
                obj.get_rotation()
            }
            <Node2DF32Kind>::RotationDegrees => {
                let obj = &self.owner;
                obj.get_rotation_degrees()
            }
            <Node2DF32Kind>::GlobalRotation => {
                let obj = &self.owner;
                obj.get_global_rotation()
            }
            <Node2DF32Kind>::GlobalRotationDegrees => {
                let obj = &self.owner;
                obj.get_global_rotation_degrees()
            }
            <Node2DF32Kind>::Skew => {
                let obj = &self.owner;
                obj.get_skew()
            }
            <Node2DF32Kind>::GlobalSkew => {
                let obj = &self.owner;
                obj.get_global_skew()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <Node2DF32Kind>::PositionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.x = val;
                obj.set_position(prop_val);
            }
            <Node2DF32Kind>::PositionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.y = val;
                obj.set_position(prop_val);
            }
            <Node2DF32Kind>::GlobalPositionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_position();
                prop_val.x = val;
                obj.set_global_position(prop_val);
            }
            <Node2DF32Kind>::GlobalPositionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_position();
                prop_val.y = val;
                obj.set_global_position(prop_val);
            }
            <Node2DF32Kind>::ScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.x = val;
                obj.set_scale(prop_val);
            }
            <Node2DF32Kind>::ScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.y = val;
                obj.set_scale(prop_val);
            }
            <Node2DF32Kind>::GlobalScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_scale();
                prop_val.x = val;
                obj.set_global_scale(prop_val);
            }
            <Node2DF32Kind>::GlobalScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_scale();
                prop_val.y = val;
                obj.set_global_scale(prop_val);
            }
            <Node2DF32Kind>::Rotation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation(val);
            }
            <Node2DF32Kind>::RotationDegrees => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation_degrees(val);
            }
            <Node2DF32Kind>::GlobalRotation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_rotation(val);
            }
            <Node2DF32Kind>::GlobalRotationDegrees => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_rotation_degrees(val);
            }
            <Node2DF32Kind>::Skew => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_skew(val);
            }
            <Node2DF32Kind>::GlobalSkew => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_skew(val);
            }
        }
    }
}
impl IPropertyData for Node2DF32Data {
    type Target = Node2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Node2DF32Kind>::PositionX => NodePath::from("position:x"),
            <Node2DF32Kind>::PositionY => NodePath::from("position:y"),
            <Node2DF32Kind>::GlobalPositionX => NodePath::from("global_position:x"),
            <Node2DF32Kind>::GlobalPositionY => NodePath::from("global_position:y"),
            <Node2DF32Kind>::ScaleX => NodePath::from("scale:x"),
            <Node2DF32Kind>::ScaleY => NodePath::from("scale:y"),
            <Node2DF32Kind>::GlobalScaleX => NodePath::from("global_scale:x"),
            <Node2DF32Kind>::GlobalScaleY => NodePath::from("global_scale:y"),
            <Node2DF32Kind>::Rotation => NodePath::from("rotation"),
            <Node2DF32Kind>::RotationDegrees => NodePath::from("rotation_degrees"),
            <Node2DF32Kind>::GlobalRotation => NodePath::from("global_rotation"),
            <Node2DF32Kind>::GlobalRotationDegrees => {
                NodePath::from("global_rotation_degrees")
            }
            <Node2DF32Kind>::Skew => NodePath::from("skew"),
            <Node2DF32Kind>::GlobalSkew => NodePath::from("global_skew"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Node2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Node2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Node2DF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Node2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "position:x" => {
                        Some(Self {
                            property: <Node2DF32Kind>::PositionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "position:y" => {
                        Some(Self {
                            property: <Node2DF32Kind>::PositionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position:x" => {
                        Some(Self {
                            property: <Node2DF32Kind>::GlobalPositionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position:y" => {
                        Some(Self {
                            property: <Node2DF32Kind>::GlobalPositionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:x" => {
                        Some(Self {
                            property: <Node2DF32Kind>::ScaleX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:y" => {
                        Some(Self {
                            property: <Node2DF32Kind>::ScaleY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_scale:x" => {
                        Some(Self {
                            property: <Node2DF32Kind>::GlobalScaleX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_scale:y" => {
                        Some(Self {
                            property: <Node2DF32Kind>::GlobalScaleY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation" => {
                        Some(Self {
                            property: <Node2DF32Kind>::Rotation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation_degrees" => {
                        Some(Self {
                            property: <Node2DF32Kind>::RotationDegrees,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation" => {
                        Some(Self {
                            property: <Node2DF32Kind>::GlobalRotation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation_degrees" => {
                        Some(Self {
                            property: <Node2DF32Kind>::GlobalRotationDegrees,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "skew" => {
                        Some(Self {
                            property: <Node2DF32Kind>::Skew,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_skew" => {
                        Some(Self {
                            property: <Node2DF32Kind>::GlobalSkew,
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
pub struct Node2DVector2Data {
    pub property: Node2DVector2Kind,
    pub owner: Gd<Node2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node2DVector2Kind {
    Position,
    GlobalPosition,
    Scale,
    GlobalScale,
}
impl IProperty<Vector2> for Node2DVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <Node2DVector2Kind>::Position => {
                let obj = &self.owner;
                obj.get_position()
            }
            <Node2DVector2Kind>::GlobalPosition => {
                let obj = &self.owner;
                obj.get_global_position()
            }
            <Node2DVector2Kind>::Scale => {
                let obj = &self.owner;
                obj.get_scale()
            }
            <Node2DVector2Kind>::GlobalScale => {
                let obj = &self.owner;
                obj.get_global_scale()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <Node2DVector2Kind>::Position => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_position(val);
            }
            <Node2DVector2Kind>::GlobalPosition => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_position(val);
            }
            <Node2DVector2Kind>::Scale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_scale(val);
            }
            <Node2DVector2Kind>::GlobalScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_scale(val);
            }
        }
    }
}
impl IPropertyData for Node2DVector2Data {
    type Target = Node2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Node2DVector2Kind>::Position => NodePath::from("position"),
            <Node2DVector2Kind>::GlobalPosition => NodePath::from("global_position"),
            <Node2DVector2Kind>::Scale => NodePath::from("scale"),
            <Node2DVector2Kind>::GlobalScale => NodePath::from("global_scale"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Node2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Node2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Node2DVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Node2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "position" => {
                        Some(Self {
                            property: <Node2DVector2Kind>::Position,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position" => {
                        Some(Self {
                            property: <Node2DVector2Kind>::GlobalPosition,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale" => {
                        Some(Self {
                            property: <Node2DVector2Kind>::Scale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_scale" => {
                        Some(Self {
                            property: <Node2DVector2Kind>::GlobalScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoNode2D<Marker = ()> {
    fn do_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_move_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_move_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_global_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_global_move_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_global_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_global_move_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_global_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_global_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_rotation_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_global_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_global_rotation_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_skew(&self, end_val: f32, duration: f64) -> SpireTween<LerpPropertyData<f32>>;
    fn do_global_skew(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_move(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_global_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_global_move(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_global_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<Node2D> + Inherits<Object>> DoNode2D<()> for Gd<Class> {
    fn do_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::PositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_move_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::PositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::PositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_move_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::PositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalPositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_move_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalPositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalPositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_move_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalPositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::ScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::ScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::Rotation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_rotation_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::RotationDegrees,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalRotation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_rotation_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalRotationDegrees,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_skew(&self, end_val: f32, duration: f64) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::Skew,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_skew(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalSkew,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::Position,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_move(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::Position,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::GlobalPosition,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_move(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::GlobalPosition,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::Scale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::GlobalScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<Node2D> + Inherits<Object>> DoNode2D<BaseMarker> for T {
    fn do_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::PositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_move_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::PositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::PositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_move_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::PositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalPositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_move_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalPositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalPositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_move_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalPositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::ScaleX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::ScaleY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalScaleX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalScaleY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::Rotation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_rotation_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::RotationDegrees,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalRotation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_rotation_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalRotationDegrees,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_skew(&self, end_val: f32, duration: f64) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::Skew,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_skew(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DF32Data {
            property: <Node2DF32Kind>::GlobalSkew,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::Position,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_move(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::Position,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::GlobalPosition,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_move(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::GlobalPosition,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::Scale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_global_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DVector2Data {
            property: <Node2DVector2Kind>::GlobalScale,
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
pub struct Node2DTweener {}
#[godot_api]
impl Node2DTweener {
    #[func]
    fn do_position_x(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_position_x(to, duration), gd
        }
    }
    #[func]
    fn do_move_x(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_position_x(to, duration), gd
        }
    }
    #[func]
    fn do_position_y(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_position_y(to, duration), gd
        }
    }
    #[func]
    fn do_move_y(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_position_y(to, duration), gd
        }
    }
    #[func]
    fn do_global_position_x(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_position_x(to, duration), gd
        }
    }
    #[func]
    fn do_global_move_x(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_position_x(to, duration), gd
        }
    }
    #[func]
    fn do_global_position_y(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_position_y(to, duration), gd
        }
    }
    #[func]
    fn do_global_move_y(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_position_y(to, duration), gd
        }
    }
    #[func]
    fn do_scale_x(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_scale_x(to, duration), gd
        }
    }
    #[func]
    fn do_scale_y(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_scale_y(to, duration), gd
        }
    }
    #[func]
    fn do_global_scale_x(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_scale_x(to, duration), gd
        }
    }
    #[func]
    fn do_global_scale_y(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_scale_y(to, duration), gd
        }
    }
    #[func]
    fn do_rotation(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_rotation(to, duration), gd
        }
    }
    #[func]
    fn do_rotation_degrees(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_rotation_degrees(to, duration), gd
        }
    }
    #[func]
    fn do_global_rotation(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_rotation(to, duration), gd
        }
    }
    #[func]
    fn do_global_rotation_degrees(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_rotation_degrees(to, duration), gd
        }
    }
    #[func]
    fn do_skew(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_skew(to, duration), gd
        }
    }
    #[func]
    fn do_global_skew(
        node: Gd<Node2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_skew(to, duration), gd
        }
    }
    #[func]
    fn do_position(
        node: Gd<Node2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_position(to, duration), gd
        }
    }
    #[func]
    fn do_move(
        node: Gd<Node2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_position(to, duration), gd
        }
    }
    #[func]
    fn do_global_position(
        node: Gd<Node2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_position(to, duration), gd
        }
    }
    #[func]
    fn do_global_move(
        node: Gd<Node2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_position(to, duration), gd
        }
    }
    #[func]
    fn do_scale(
        node: Gd<Node2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_scale(to, duration), gd
        }
    }
    #[func]
    fn do_global_scale(
        node: Gd<Node2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_global_scale(to, duration), gd
        }
    }
}
