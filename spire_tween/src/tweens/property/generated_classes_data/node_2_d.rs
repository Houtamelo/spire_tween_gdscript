use super::*;
#[derive(Debug, Clone)]
pub struct Node2DFloatData {
    pub property: Node2DFloatKind,
    pub owner: Gd<Node2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node2DFloatKind {
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
impl IProperty<f64> for Node2DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <Node2DFloatKind>::PositionX => {
                let obj = &self.owner;
                (obj.get_position().x) as f64
            }
            <Node2DFloatKind>::PositionY => {
                let obj = &self.owner;
                (obj.get_position().y) as f64
            }
            <Node2DFloatKind>::GlobalPositionX => {
                let obj = &self.owner;
                (obj.get_global_position().x) as f64
            }
            <Node2DFloatKind>::GlobalPositionY => {
                let obj = &self.owner;
                (obj.get_global_position().y) as f64
            }
            <Node2DFloatKind>::ScaleX => {
                let obj = &self.owner;
                (obj.get_scale().x) as f64
            }
            <Node2DFloatKind>::ScaleY => {
                let obj = &self.owner;
                (obj.get_scale().y) as f64
            }
            <Node2DFloatKind>::GlobalScaleX => {
                let obj = &self.owner;
                (obj.get_global_scale().x) as f64
            }
            <Node2DFloatKind>::GlobalScaleY => {
                let obj = &self.owner;
                (obj.get_global_scale().y) as f64
            }
            <Node2DFloatKind>::Rotation => {
                let obj = &self.owner;
                (obj.get_rotation()) as f64
            }
            <Node2DFloatKind>::RotationDegrees => {
                let obj = &self.owner;
                (obj.get_rotation_degrees()) as f64
            }
            <Node2DFloatKind>::GlobalRotation => {
                let obj = &self.owner;
                (obj.get_global_rotation()) as f64
            }
            <Node2DFloatKind>::GlobalRotationDegrees => {
                let obj = &self.owner;
                (obj.get_global_rotation_degrees()) as f64
            }
            <Node2DFloatKind>::Skew => {
                let obj = &self.owner;
                (obj.get_skew()) as f64
            }
            <Node2DFloatKind>::GlobalSkew => {
                let obj = &self.owner;
                (obj.get_global_skew()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <Node2DFloatKind>::PositionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.x = val as f32;
                obj.set_position(prop_val);
            }
            <Node2DFloatKind>::PositionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.y = val as f32;
                obj.set_position(prop_val);
            }
            <Node2DFloatKind>::GlobalPositionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_position();
                prop_val.x = val as f32;
                obj.set_global_position(prop_val);
            }
            <Node2DFloatKind>::GlobalPositionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_position();
                prop_val.y = val as f32;
                obj.set_global_position(prop_val);
            }
            <Node2DFloatKind>::ScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.x = val as f32;
                obj.set_scale(prop_val);
            }
            <Node2DFloatKind>::ScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.y = val as f32;
                obj.set_scale(prop_val);
            }
            <Node2DFloatKind>::GlobalScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_scale();
                prop_val.x = val as f32;
                obj.set_global_scale(prop_val);
            }
            <Node2DFloatKind>::GlobalScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_scale();
                prop_val.y = val as f32;
                obj.set_global_scale(prop_val);
            }
            <Node2DFloatKind>::Rotation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation(val as f32);
            }
            <Node2DFloatKind>::RotationDegrees => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation_degrees(val as f32);
            }
            <Node2DFloatKind>::GlobalRotation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_rotation(val as f32);
            }
            <Node2DFloatKind>::GlobalRotationDegrees => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_rotation_degrees(val as f32);
            }
            <Node2DFloatKind>::Skew => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_skew(val as f32);
            }
            <Node2DFloatKind>::GlobalSkew => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_skew(val as f32);
            }
        }
    }
}
impl IPropertyData for Node2DFloatData {
    type Target = Node2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Node2DFloatKind>::PositionX => NodePath::from("position:x"),
            <Node2DFloatKind>::PositionY => NodePath::from("position:y"),
            <Node2DFloatKind>::GlobalPositionX => NodePath::from("global_position:x"),
            <Node2DFloatKind>::GlobalPositionY => NodePath::from("global_position:y"),
            <Node2DFloatKind>::ScaleX => NodePath::from("scale:x"),
            <Node2DFloatKind>::ScaleY => NodePath::from("scale:y"),
            <Node2DFloatKind>::GlobalScaleX => NodePath::from("global_scale:x"),
            <Node2DFloatKind>::GlobalScaleY => NodePath::from("global_scale:y"),
            <Node2DFloatKind>::Rotation => NodePath::from("rotation"),
            <Node2DFloatKind>::RotationDegrees => NodePath::from("rotation_degrees"),
            <Node2DFloatKind>::GlobalRotation => NodePath::from("global_rotation"),
            <Node2DFloatKind>::GlobalRotationDegrees => {
                NodePath::from("global_rotation_degrees")
            }
            <Node2DFloatKind>::Skew => NodePath::from("skew"),
            <Node2DFloatKind>::GlobalSkew => NodePath::from("global_skew"),
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
impl TryFromPathAndObject for Node2DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Node2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "position:x" => {
                        Some(Self {
                            property: <Node2DFloatKind>::PositionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "position:y" => {
                        Some(Self {
                            property: <Node2DFloatKind>::PositionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position:x" => {
                        Some(Self {
                            property: <Node2DFloatKind>::GlobalPositionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position:y" => {
                        Some(Self {
                            property: <Node2DFloatKind>::GlobalPositionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:x" => {
                        Some(Self {
                            property: <Node2DFloatKind>::ScaleX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:y" => {
                        Some(Self {
                            property: <Node2DFloatKind>::ScaleY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_scale:x" => {
                        Some(Self {
                            property: <Node2DFloatKind>::GlobalScaleX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_scale:y" => {
                        Some(Self {
                            property: <Node2DFloatKind>::GlobalScaleY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation" => {
                        Some(Self {
                            property: <Node2DFloatKind>::Rotation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation_degrees" => {
                        Some(Self {
                            property: <Node2DFloatKind>::RotationDegrees,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation" => {
                        Some(Self {
                            property: <Node2DFloatKind>::GlobalRotation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation_degrees" => {
                        Some(Self {
                            property: <Node2DFloatKind>::GlobalRotationDegrees,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "skew" => {
                        Some(Self {
                            property: <Node2DFloatKind>::Skew,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_skew" => {
                        Some(Self {
                            property: <Node2DFloatKind>::GlobalSkew,
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
pub trait SpireDoNode2D<Marker = ()> {
    fn do_position_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_move_local_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_position_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_move_local_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_position_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_move_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_position_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_move_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rotation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rotation_degrees(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_rotation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_rotation_degrees(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_skew(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_skew(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_move_local(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_global_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_move(
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
impl<Class: Inherits<Node2D> + Inherits<Object>> SpireDoNode2D<()> for Gd<Class> {
    fn do_position_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::PositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_local_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::PositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::PositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_local_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::PositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_position_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalPositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalPositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_position_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalPositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalPositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::ScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::ScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::Rotation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_degrees(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::RotationDegrees,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalRotation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_degrees(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalRotationDegrees,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_skew(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::Skew,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_skew(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalSkew,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_local(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<T: WithBaseField + Inherits<Node2D> + Inherits<Object>> SpireDoNode2D<BaseMarker>
for T {
    fn do_position_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::PositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_local_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::PositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::PositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_local_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::PositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_position_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalPositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalPositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_position_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalPositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalPositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::ScaleX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::ScaleY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalScaleX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalScaleY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::Rotation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_degrees(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::RotationDegrees,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalRotation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_degrees(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalRotationDegrees,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_skew(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::Skew,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_skew(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node2D> = self.to_gd().upcast();
        let data = Node2DFloatData {
            property: <Node2DFloatKind>::GlobalSkew,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_local(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
