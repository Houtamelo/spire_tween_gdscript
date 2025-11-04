use super::*;
#[derive(Debug, Clone)]
pub struct Node3DFloatData {
    pub property: Node3DFloatKind,
    pub owner: Gd<Node3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node3DFloatKind {
    PositionX,
    PositionY,
    PositionZ,
    GlobalPositionX,
    GlobalPositionY,
    GlobalPositionZ,
    ScaleX,
    ScaleY,
    ScaleZ,
    RotationX,
    RotationY,
    RotationZ,
    RotationDegreesX,
    RotationDegreesY,
    RotationDegreesZ,
    GlobalRotationX,
    GlobalRotationY,
    GlobalRotationZ,
    GlobalRotationDegreesX,
    GlobalRotationDegreesY,
    GlobalRotationDegreesZ,
}
impl IProperty<f64> for Node3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <Node3DFloatKind>::PositionX => {
                let obj = &self.owner;
                (obj.get_position().x) as f64
            }
            <Node3DFloatKind>::PositionY => {
                let obj = &self.owner;
                (obj.get_position().y) as f64
            }
            <Node3DFloatKind>::PositionZ => {
                let obj = &self.owner;
                (obj.get_position().z) as f64
            }
            <Node3DFloatKind>::GlobalPositionX => {
                let obj = &self.owner;
                (obj.get_global_position().x) as f64
            }
            <Node3DFloatKind>::GlobalPositionY => {
                let obj = &self.owner;
                (obj.get_global_position().y) as f64
            }
            <Node3DFloatKind>::GlobalPositionZ => {
                let obj = &self.owner;
                (obj.get_global_position().z) as f64
            }
            <Node3DFloatKind>::ScaleX => {
                let obj = &self.owner;
                (obj.get_scale().x) as f64
            }
            <Node3DFloatKind>::ScaleY => {
                let obj = &self.owner;
                (obj.get_scale().y) as f64
            }
            <Node3DFloatKind>::ScaleZ => {
                let obj = &self.owner;
                (obj.get_scale().z) as f64
            }
            <Node3DFloatKind>::RotationX => {
                let obj = &self.owner;
                (obj.get_rotation().x) as f64
            }
            <Node3DFloatKind>::RotationY => {
                let obj = &self.owner;
                (obj.get_rotation().y) as f64
            }
            <Node3DFloatKind>::RotationZ => {
                let obj = &self.owner;
                (obj.get_rotation().z) as f64
            }
            <Node3DFloatKind>::RotationDegreesX => {
                let obj = &self.owner;
                (obj.get_rotation_degrees().x) as f64
            }
            <Node3DFloatKind>::RotationDegreesY => {
                let obj = &self.owner;
                (obj.get_rotation_degrees().y) as f64
            }
            <Node3DFloatKind>::RotationDegreesZ => {
                let obj = &self.owner;
                (obj.get_rotation_degrees().z) as f64
            }
            <Node3DFloatKind>::GlobalRotationX => {
                let obj = &self.owner;
                (obj.get_global_rotation().x) as f64
            }
            <Node3DFloatKind>::GlobalRotationY => {
                let obj = &self.owner;
                (obj.get_global_rotation().y) as f64
            }
            <Node3DFloatKind>::GlobalRotationZ => {
                let obj = &self.owner;
                (obj.get_global_rotation().z) as f64
            }
            <Node3DFloatKind>::GlobalRotationDegreesX => {
                let obj = &self.owner;
                (obj.get_global_rotation_degrees().x) as f64
            }
            <Node3DFloatKind>::GlobalRotationDegreesY => {
                let obj = &self.owner;
                (obj.get_global_rotation_degrees().y) as f64
            }
            <Node3DFloatKind>::GlobalRotationDegreesZ => {
                let obj = &self.owner;
                (obj.get_global_rotation_degrees().z) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <Node3DFloatKind>::PositionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.x = val as f32;
                obj.set_position(prop_val);
            }
            <Node3DFloatKind>::PositionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.y = val as f32;
                obj.set_position(prop_val);
            }
            <Node3DFloatKind>::PositionZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.z = val as f32;
                obj.set_position(prop_val);
            }
            <Node3DFloatKind>::GlobalPositionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_position();
                prop_val.x = val as f32;
                obj.set_global_position(prop_val);
            }
            <Node3DFloatKind>::GlobalPositionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_position();
                prop_val.y = val as f32;
                obj.set_global_position(prop_val);
            }
            <Node3DFloatKind>::GlobalPositionZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_position();
                prop_val.z = val as f32;
                obj.set_global_position(prop_val);
            }
            <Node3DFloatKind>::ScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.x = val as f32;
                obj.set_scale(prop_val);
            }
            <Node3DFloatKind>::ScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.y = val as f32;
                obj.set_scale(prop_val);
            }
            <Node3DFloatKind>::ScaleZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.z = val as f32;
                obj.set_scale(prop_val);
            }
            <Node3DFloatKind>::RotationX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_rotation();
                prop_val.x = val as f32;
                obj.set_rotation(prop_val);
            }
            <Node3DFloatKind>::RotationY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_rotation();
                prop_val.y = val as f32;
                obj.set_rotation(prop_val);
            }
            <Node3DFloatKind>::RotationZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_rotation();
                prop_val.z = val as f32;
                obj.set_rotation(prop_val);
            }
            <Node3DFloatKind>::RotationDegreesX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_rotation_degrees();
                prop_val.x = val as f32;
                obj.set_rotation_degrees(prop_val);
            }
            <Node3DFloatKind>::RotationDegreesY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_rotation_degrees();
                prop_val.y = val as f32;
                obj.set_rotation_degrees(prop_val);
            }
            <Node3DFloatKind>::RotationDegreesZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_rotation_degrees();
                prop_val.z = val as f32;
                obj.set_rotation_degrees(prop_val);
            }
            <Node3DFloatKind>::GlobalRotationX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_rotation();
                prop_val.x = val as f32;
                obj.set_global_rotation(prop_val);
            }
            <Node3DFloatKind>::GlobalRotationY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_rotation();
                prop_val.y = val as f32;
                obj.set_global_rotation(prop_val);
            }
            <Node3DFloatKind>::GlobalRotationZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_rotation();
                prop_val.z = val as f32;
                obj.set_global_rotation(prop_val);
            }
            <Node3DFloatKind>::GlobalRotationDegreesX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_rotation_degrees();
                prop_val.x = val as f32;
                obj.set_global_rotation_degrees(prop_val);
            }
            <Node3DFloatKind>::GlobalRotationDegreesY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_rotation_degrees();
                prop_val.y = val as f32;
                obj.set_global_rotation_degrees(prop_val);
            }
            <Node3DFloatKind>::GlobalRotationDegreesZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_rotation_degrees();
                prop_val.z = val as f32;
                obj.set_global_rotation_degrees(prop_val);
            }
        }
    }
}
impl IPropertyData for Node3DFloatData {
    type Target = Node3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Node3DFloatKind>::PositionX => NodePath::from("position:x"),
            <Node3DFloatKind>::PositionY => NodePath::from("position:y"),
            <Node3DFloatKind>::PositionZ => NodePath::from("position:z"),
            <Node3DFloatKind>::GlobalPositionX => NodePath::from("global_position:x"),
            <Node3DFloatKind>::GlobalPositionY => NodePath::from("global_position:y"),
            <Node3DFloatKind>::GlobalPositionZ => NodePath::from("global_position:z"),
            <Node3DFloatKind>::ScaleX => NodePath::from("scale:x"),
            <Node3DFloatKind>::ScaleY => NodePath::from("scale:y"),
            <Node3DFloatKind>::ScaleZ => NodePath::from("scale:z"),
            <Node3DFloatKind>::RotationX => NodePath::from("rotation:x"),
            <Node3DFloatKind>::RotationY => NodePath::from("rotation:y"),
            <Node3DFloatKind>::RotationZ => NodePath::from("rotation:z"),
            <Node3DFloatKind>::RotationDegreesX => NodePath::from("rotation_degrees:x"),
            <Node3DFloatKind>::RotationDegreesY => NodePath::from("rotation_degrees:y"),
            <Node3DFloatKind>::RotationDegreesZ => NodePath::from("rotation_degrees:z"),
            <Node3DFloatKind>::GlobalRotationX => NodePath::from("global_rotation:x"),
            <Node3DFloatKind>::GlobalRotationY => NodePath::from("global_rotation:y"),
            <Node3DFloatKind>::GlobalRotationZ => NodePath::from("global_rotation:z"),
            <Node3DFloatKind>::GlobalRotationDegreesX => {
                NodePath::from("global_rotation_degrees:x")
            }
            <Node3DFloatKind>::GlobalRotationDegreesY => {
                NodePath::from("global_rotation_degrees:y")
            }
            <Node3DFloatKind>::GlobalRotationDegreesZ => {
                NodePath::from("global_rotation_degrees:z")
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
            ObjectOrNode::Object(obj) => obj.try_cast::<Node3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Node3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Node3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Node3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "position:x" => {
                        Some(Self {
                            property: <Node3DFloatKind>::PositionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "position:y" => {
                        Some(Self {
                            property: <Node3DFloatKind>::PositionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "position:z" => {
                        Some(Self {
                            property: <Node3DFloatKind>::PositionZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position:x" => {
                        Some(Self {
                            property: <Node3DFloatKind>::GlobalPositionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position:y" => {
                        Some(Self {
                            property: <Node3DFloatKind>::GlobalPositionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position:z" => {
                        Some(Self {
                            property: <Node3DFloatKind>::GlobalPositionZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:x" => {
                        Some(Self {
                            property: <Node3DFloatKind>::ScaleX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:y" => {
                        Some(Self {
                            property: <Node3DFloatKind>::ScaleY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:z" => {
                        Some(Self {
                            property: <Node3DFloatKind>::ScaleZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation:x" => {
                        Some(Self {
                            property: <Node3DFloatKind>::RotationX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation:y" => {
                        Some(Self {
                            property: <Node3DFloatKind>::RotationY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation:z" => {
                        Some(Self {
                            property: <Node3DFloatKind>::RotationZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation_degrees:x" => {
                        Some(Self {
                            property: <Node3DFloatKind>::RotationDegreesX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation_degrees:y" => {
                        Some(Self {
                            property: <Node3DFloatKind>::RotationDegreesY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation_degrees:z" => {
                        Some(Self {
                            property: <Node3DFloatKind>::RotationDegreesZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation:x" => {
                        Some(Self {
                            property: <Node3DFloatKind>::GlobalRotationX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation:y" => {
                        Some(Self {
                            property: <Node3DFloatKind>::GlobalRotationY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation:z" => {
                        Some(Self {
                            property: <Node3DFloatKind>::GlobalRotationZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation_degrees:x" => {
                        Some(Self {
                            property: <Node3DFloatKind>::GlobalRotationDegreesX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation_degrees:y" => {
                        Some(Self {
                            property: <Node3DFloatKind>::GlobalRotationDegreesY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation_degrees:z" => {
                        Some(Self {
                            property: <Node3DFloatKind>::GlobalRotationDegreesZ,
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
pub struct Node3DVector3Data {
    pub property: Node3DVector3Kind,
    pub owner: Gd<Node3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node3DVector3Kind {
    Position,
    GlobalPosition,
    Scale,
    Rotation,
    RotationDegrees,
    GlobalRotation,
    GlobalRotationDegrees,
}
impl IProperty<Vector3> for Node3DVector3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <Node3DVector3Kind>::Position => {
                let obj = &self.owner;
                obj.get_position()
            }
            <Node3DVector3Kind>::GlobalPosition => {
                let obj = &self.owner;
                obj.get_global_position()
            }
            <Node3DVector3Kind>::Scale => {
                let obj = &self.owner;
                obj.get_scale()
            }
            <Node3DVector3Kind>::Rotation => {
                let obj = &self.owner;
                obj.get_rotation()
            }
            <Node3DVector3Kind>::RotationDegrees => {
                let obj = &self.owner;
                obj.get_rotation_degrees()
            }
            <Node3DVector3Kind>::GlobalRotation => {
                let obj = &self.owner;
                obj.get_global_rotation()
            }
            <Node3DVector3Kind>::GlobalRotationDegrees => {
                let obj = &self.owner;
                obj.get_global_rotation_degrees()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <Node3DVector3Kind>::Position => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_position(val);
            }
            <Node3DVector3Kind>::GlobalPosition => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_position(val);
            }
            <Node3DVector3Kind>::Scale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_scale(val);
            }
            <Node3DVector3Kind>::Rotation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation(val);
            }
            <Node3DVector3Kind>::RotationDegrees => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation_degrees(val);
            }
            <Node3DVector3Kind>::GlobalRotation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_rotation(val);
            }
            <Node3DVector3Kind>::GlobalRotationDegrees => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_rotation_degrees(val);
            }
        }
    }
}
impl IPropertyData for Node3DVector3Data {
    type Target = Node3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Node3DVector3Kind>::Position => NodePath::from("position"),
            <Node3DVector3Kind>::GlobalPosition => NodePath::from("global_position"),
            <Node3DVector3Kind>::Scale => NodePath::from("scale"),
            <Node3DVector3Kind>::Rotation => NodePath::from("rotation"),
            <Node3DVector3Kind>::RotationDegrees => NodePath::from("rotation_degrees"),
            <Node3DVector3Kind>::GlobalRotation => NodePath::from("global_rotation"),
            <Node3DVector3Kind>::GlobalRotationDegrees => {
                NodePath::from("global_rotation_degrees")
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
            ObjectOrNode::Object(obj) => obj.try_cast::<Node3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Node3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Node3DVector3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Node3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "position" => {
                        Some(Self {
                            property: <Node3DVector3Kind>::Position,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position" => {
                        Some(Self {
                            property: <Node3DVector3Kind>::GlobalPosition,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale" => {
                        Some(Self {
                            property: <Node3DVector3Kind>::Scale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation" => {
                        Some(Self {
                            property: <Node3DVector3Kind>::Rotation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation_degrees" => {
                        Some(Self {
                            property: <Node3DVector3Kind>::RotationDegrees,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation" => {
                        Some(Self {
                            property: <Node3DVector3Kind>::GlobalRotation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_rotation_degrees" => {
                        Some(Self {
                            property: <Node3DVector3Kind>::GlobalRotationDegrees,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoNode3D<Marker = ()> {
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
    fn do_position_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_move_local_z(
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
    fn do_global_position_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_move_z(
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
    fn do_scale_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rotation_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rotation_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rotation_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rotation_degrees_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rotation_degrees_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rotation_degrees_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_rotation_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_rotation_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_rotation_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_rotation_degrees_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_rotation_degrees_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_global_rotation_degrees_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_position(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_move_local(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_global_position(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_move(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_scale(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_rotation(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_rotation_degrees(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_global_rotation(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_global_rotation_degrees(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
}
impl<Class: Inherits<Node3D> + Inherits<Object>> SpireDoNode3D<()> for Gd<Class> {
    fn do_position_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
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
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
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
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
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
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_local_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
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
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
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
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
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
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
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
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_position_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
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
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::ScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
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
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::ScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scale_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::ScaleZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_degrees_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationDegreesX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_degrees_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationDegreesY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_degrees_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationDegreesZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_degrees_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationDegreesX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_degrees_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationDegreesY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_degrees_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationDegreesZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::Position,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_local(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::Position,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_position(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::GlobalPosition,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::GlobalPosition,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scale(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::Scale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::Rotation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_degrees(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::RotationDegrees,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::GlobalRotation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_degrees(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::GlobalRotationDegrees,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Node3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<T: WithBaseField + Inherits<Node3D> + Inherits<Object>> SpireDoNode3D<BaseMarker>
for T {
    fn do_position_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionX,
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
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionX,
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
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionY,
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
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_local_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::PositionZ,
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
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionX,
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
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionX,
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
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionY,
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
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_position_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalPositionZ,
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
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::ScaleX,
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
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::ScaleY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scale_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::ScaleZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_degrees_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationDegreesX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_degrees_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationDegreesY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_degrees_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::RotationDegreesZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_degrees_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationDegreesX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_degrees_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationDegreesY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_degrees_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DFloatData {
            property: <Node3DFloatKind>::GlobalRotationDegreesZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::Position,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move_local(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::Position,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_position(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::GlobalPosition,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_move(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::GlobalPosition,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scale(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::Scale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::Rotation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rotation_degrees(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::RotationDegrees,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::GlobalRotation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_global_rotation_degrees(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Node3D> = self.to_gd().upcast();
        let data = Node3DVector3Data {
            property: <Node3DVector3Kind>::GlobalRotationDegrees,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
