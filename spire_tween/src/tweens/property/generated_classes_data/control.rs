use super::*;
#[derive(Debug, Clone)]
pub struct ControlFloatData {
    pub property: ControlFloatKind,
    pub owner: Gd<Control>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFloatKind {
    AnchorBottom,
    AnchorLeft,
    AnchorRight,
    AnchorTop,
    Rotation,
    RotationDegrees,
    CustomMinimumSizeX,
    CustomMinimumSizeY,
    PositionX,
    PositionY,
    GlobalPositionX,
    GlobalPositionY,
    PivotOffsetX,
    PivotOffsetY,
    ScaleX,
    ScaleY,
    SizeX,
    SizeY,
}
impl IProperty<f64> for ControlFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <ControlFloatKind>::AnchorBottom => {
                let obj = &self.owner;
                (obj.get_anchor(Side::BOTTOM)) as f64
            }
            <ControlFloatKind>::AnchorLeft => {
                let obj = &self.owner;
                (obj.get_anchor(Side::LEFT)) as f64
            }
            <ControlFloatKind>::AnchorRight => {
                let obj = &self.owner;
                (obj.get_anchor(Side::RIGHT)) as f64
            }
            <ControlFloatKind>::AnchorTop => {
                let obj = &self.owner;
                (obj.get_anchor(Side::TOP)) as f64
            }
            <ControlFloatKind>::Rotation => {
                let obj = &self.owner;
                (obj.get_rotation()) as f64
            }
            <ControlFloatKind>::RotationDegrees => {
                let obj = &self.owner;
                (obj.get_rotation_degrees()) as f64
            }
            <ControlFloatKind>::CustomMinimumSizeX => {
                let obj = &self.owner;
                (obj.get_custom_minimum_size().x) as f64
            }
            <ControlFloatKind>::CustomMinimumSizeY => {
                let obj = &self.owner;
                (obj.get_custom_minimum_size().y) as f64
            }
            <ControlFloatKind>::PositionX => {
                let obj = &self.owner;
                (obj.get_position().x) as f64
            }
            <ControlFloatKind>::PositionY => {
                let obj = &self.owner;
                (obj.get_position().y) as f64
            }
            <ControlFloatKind>::GlobalPositionX => {
                let obj = &self.owner;
                (obj.get_global_position().x) as f64
            }
            <ControlFloatKind>::GlobalPositionY => {
                let obj = &self.owner;
                (obj.get_global_position().y) as f64
            }
            <ControlFloatKind>::PivotOffsetX => {
                let obj = &self.owner;
                (obj.get_pivot_offset().x) as f64
            }
            <ControlFloatKind>::PivotOffsetY => {
                let obj = &self.owner;
                (obj.get_pivot_offset().y) as f64
            }
            <ControlFloatKind>::ScaleX => {
                let obj = &self.owner;
                (obj.get_scale().x) as f64
            }
            <ControlFloatKind>::ScaleY => {
                let obj = &self.owner;
                (obj.get_scale().y) as f64
            }
            <ControlFloatKind>::SizeX => {
                let obj = &self.owner;
                (obj.get_size().x) as f64
            }
            <ControlFloatKind>::SizeY => {
                let obj = &self.owner;
                (obj.get_size().y) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <ControlFloatKind>::AnchorBottom => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_anchor(Side::BOTTOM, val as f32);
            }
            <ControlFloatKind>::AnchorLeft => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_anchor(Side::LEFT, val as f32);
            }
            <ControlFloatKind>::AnchorRight => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_anchor(Side::RIGHT, val as f32);
            }
            <ControlFloatKind>::AnchorTop => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_anchor(Side::TOP, val as f32);
            }
            <ControlFloatKind>::Rotation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation(val as f32);
            }
            <ControlFloatKind>::RotationDegrees => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation_degrees(val as f32);
            }
            <ControlFloatKind>::CustomMinimumSizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_custom_minimum_size();
                prop_val.x = val as f32;
                obj.set_custom_minimum_size(prop_val);
            }
            <ControlFloatKind>::CustomMinimumSizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_custom_minimum_size();
                prop_val.y = val as f32;
                obj.set_custom_minimum_size(prop_val);
            }
            <ControlFloatKind>::PositionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.x = val as f32;
                obj.set_position(prop_val);
            }
            <ControlFloatKind>::PositionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.y = val as f32;
                obj.set_position(prop_val);
            }
            <ControlFloatKind>::GlobalPositionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_position();
                prop_val.x = val as f32;
                obj.set_global_position(prop_val);
            }
            <ControlFloatKind>::GlobalPositionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_position();
                prop_val.y = val as f32;
                obj.set_global_position(prop_val);
            }
            <ControlFloatKind>::PivotOffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_pivot_offset();
                prop_val.x = val as f32;
                obj.set_pivot_offset(prop_val);
            }
            <ControlFloatKind>::PivotOffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_pivot_offset();
                prop_val.y = val as f32;
                obj.set_pivot_offset(prop_val);
            }
            <ControlFloatKind>::ScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.x = val as f32;
                obj.set_scale(prop_val);
            }
            <ControlFloatKind>::ScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.y = val as f32;
                obj.set_scale(prop_val);
            }
            <ControlFloatKind>::SizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.x = val as f32;
                obj.set_size(prop_val);
            }
            <ControlFloatKind>::SizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.y = val as f32;
                obj.set_size(prop_val);
            }
        }
    }
}
impl IPropertyData for ControlFloatData {
    type Target = Control;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ControlFloatKind>::AnchorBottom => NodePath::from("anchor_bottom"),
            <ControlFloatKind>::AnchorLeft => NodePath::from("anchor_left"),
            <ControlFloatKind>::AnchorRight => NodePath::from("anchor_right"),
            <ControlFloatKind>::AnchorTop => NodePath::from("anchor_top"),
            <ControlFloatKind>::Rotation => NodePath::from("rotation"),
            <ControlFloatKind>::RotationDegrees => NodePath::from("rotation_degrees"),
            <ControlFloatKind>::CustomMinimumSizeX => {
                NodePath::from("custom_minimum_size:x")
            }
            <ControlFloatKind>::CustomMinimumSizeY => {
                NodePath::from("custom_minimum_size:y")
            }
            <ControlFloatKind>::PositionX => NodePath::from("position:x"),
            <ControlFloatKind>::PositionY => NodePath::from("position:y"),
            <ControlFloatKind>::GlobalPositionX => NodePath::from("global_position:x"),
            <ControlFloatKind>::GlobalPositionY => NodePath::from("global_position:y"),
            <ControlFloatKind>::PivotOffsetX => NodePath::from("pivot_offset:x"),
            <ControlFloatKind>::PivotOffsetY => NodePath::from("pivot_offset:y"),
            <ControlFloatKind>::ScaleX => NodePath::from("scale:x"),
            <ControlFloatKind>::ScaleY => NodePath::from("scale:y"),
            <ControlFloatKind>::SizeX => NodePath::from("size:x"),
            <ControlFloatKind>::SizeY => NodePath::from("size:y"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Control>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Control>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ControlFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Control>()
            .ok()
            .and_then(|owner| {
                match path {
                    "anchor_bottom" => {
                        Some(Self {
                            property: <ControlFloatKind>::AnchorBottom,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "anchor_left" => {
                        Some(Self {
                            property: <ControlFloatKind>::AnchorLeft,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "anchor_right" => {
                        Some(Self {
                            property: <ControlFloatKind>::AnchorRight,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "anchor_top" => {
                        Some(Self {
                            property: <ControlFloatKind>::AnchorTop,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation" => {
                        Some(Self {
                            property: <ControlFloatKind>::Rotation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation_degrees" => {
                        Some(Self {
                            property: <ControlFloatKind>::RotationDegrees,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "custom_minimum_size:x" => {
                        Some(Self {
                            property: <ControlFloatKind>::CustomMinimumSizeX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "custom_minimum_size:y" => {
                        Some(Self {
                            property: <ControlFloatKind>::CustomMinimumSizeY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "position:x" => {
                        Some(Self {
                            property: <ControlFloatKind>::PositionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "position:y" => {
                        Some(Self {
                            property: <ControlFloatKind>::PositionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position:x" => {
                        Some(Self {
                            property: <ControlFloatKind>::GlobalPositionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position:y" => {
                        Some(Self {
                            property: <ControlFloatKind>::GlobalPositionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "pivot_offset:x" => {
                        Some(Self {
                            property: <ControlFloatKind>::PivotOffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "pivot_offset:y" => {
                        Some(Self {
                            property: <ControlFloatKind>::PivotOffsetY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:x" => {
                        Some(Self {
                            property: <ControlFloatKind>::ScaleX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:y" => {
                        Some(Self {
                            property: <ControlFloatKind>::ScaleY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:x" => {
                        Some(Self {
                            property: <ControlFloatKind>::SizeX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:y" => {
                        Some(Self {
                            property: <ControlFloatKind>::SizeY,
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
pub struct ControlVector2Data {
    pub property: ControlVector2Kind,
    pub owner: Gd<Control>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlVector2Kind {
    CustomMinimumSize,
    Position,
    GlobalPosition,
    PivotOffset,
    Scale,
    Size,
}
impl IProperty<Vector2> for ControlVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <ControlVector2Kind>::CustomMinimumSize => {
                let obj = &self.owner;
                obj.get_custom_minimum_size()
            }
            <ControlVector2Kind>::Position => {
                let obj = &self.owner;
                obj.get_position()
            }
            <ControlVector2Kind>::GlobalPosition => {
                let obj = &self.owner;
                obj.get_global_position()
            }
            <ControlVector2Kind>::PivotOffset => {
                let obj = &self.owner;
                obj.get_pivot_offset()
            }
            <ControlVector2Kind>::Scale => {
                let obj = &self.owner;
                obj.get_scale()
            }
            <ControlVector2Kind>::Size => {
                let obj = &self.owner;
                obj.get_size()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <ControlVector2Kind>::CustomMinimumSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_custom_minimum_size(val);
            }
            <ControlVector2Kind>::Position => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_position(val);
            }
            <ControlVector2Kind>::GlobalPosition => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_global_position(val);
            }
            <ControlVector2Kind>::PivotOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_pivot_offset(val);
            }
            <ControlVector2Kind>::Scale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_scale(val);
            }
            <ControlVector2Kind>::Size => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size(val);
            }
        }
    }
}
impl IPropertyData for ControlVector2Data {
    type Target = Control;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ControlVector2Kind>::CustomMinimumSize => {
                NodePath::from("custom_minimum_size")
            }
            <ControlVector2Kind>::Position => NodePath::from("position"),
            <ControlVector2Kind>::GlobalPosition => NodePath::from("global_position"),
            <ControlVector2Kind>::PivotOffset => NodePath::from("pivot_offset"),
            <ControlVector2Kind>::Scale => NodePath::from("scale"),
            <ControlVector2Kind>::Size => NodePath::from("size"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Control>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Control>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ControlVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Control>()
            .ok()
            .and_then(|owner| {
                match path {
                    "custom_minimum_size" => {
                        Some(Self {
                            property: <ControlVector2Kind>::CustomMinimumSize,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "position" => {
                        Some(Self {
                            property: <ControlVector2Kind>::Position,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position" => {
                        Some(Self {
                            property: <ControlVector2Kind>::GlobalPosition,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "pivot_offset" => {
                        Some(Self {
                            property: <ControlVector2Kind>::PivotOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale" => {
                        Some(Self {
                            property: <ControlVector2Kind>::Scale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size" => {
                        Some(Self {
                            property: <ControlVector2Kind>::Size,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoControl<Marker = ()> {
    fn do_anchor_bottom(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_anchor_left(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_anchor_right(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_anchor_top(
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
    fn do_custom_minimum_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_custom_minimum_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
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
    fn do_pivot_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_pivot_offset_y(
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
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_custom_minimum_size(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
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
    fn do_pivot_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_size(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<Control> + Inherits<Object>> SpireDoControl<()> for Gd<Class> {
    fn do_anchor_bottom(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ControlFloatData {
            property: <ControlFloatKind>::AnchorBottom,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_anchor_left(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ControlFloatData {
            property: <ControlFloatKind>::AnchorLeft,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_anchor_right(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ControlFloatData {
            property: <ControlFloatKind>::AnchorRight,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_anchor_top(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ControlFloatData {
            property: <ControlFloatKind>::AnchorTop,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlFloatData {
            property: <ControlFloatKind>::Rotation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlFloatData {
            property: <ControlFloatKind>::RotationDegrees,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_custom_minimum_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ControlFloatData {
            property: <ControlFloatKind>::CustomMinimumSizeX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_custom_minimum_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ControlFloatData {
            property: <ControlFloatKind>::CustomMinimumSizeY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ControlFloatData {
            property: <ControlFloatKind>::PositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlFloatData {
            property: <ControlFloatKind>::PositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlFloatData {
            property: <ControlFloatKind>::PositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlFloatData {
            property: <ControlFloatKind>::PositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlFloatData {
            property: <ControlFloatKind>::GlobalPositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlFloatData {
            property: <ControlFloatKind>::GlobalPositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlFloatData {
            property: <ControlFloatKind>::GlobalPositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlFloatData {
            property: <ControlFloatKind>::GlobalPositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_pivot_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ControlFloatData {
            property: <ControlFloatKind>::PivotOffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_pivot_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ControlFloatData {
            property: <ControlFloatKind>::PivotOffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlFloatData {
            property: <ControlFloatKind>::ScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlFloatData {
            property: <ControlFloatKind>::ScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ControlFloatData {
            property: <ControlFloatKind>::SizeX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ControlFloatData {
            property: <ControlFloatKind>::SizeY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_custom_minimum_size(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::CustomMinimumSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::Position,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::Position,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::GlobalPosition,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::GlobalPosition,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_pivot_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::PivotOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
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
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::Scale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::Size,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<T: WithBaseField + Inherits<Control> + Inherits<Object>> SpireDoControl<BaseMarker>
for T {
    fn do_anchor_bottom(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::AnchorBottom,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_anchor_left(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::AnchorLeft,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_anchor_right(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::AnchorRight,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_anchor_top(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::AnchorTop,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::Rotation,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::RotationDegrees,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_custom_minimum_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::CustomMinimumSizeX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_custom_minimum_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::CustomMinimumSizeY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::PositionX,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::PositionX,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::PositionY,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::PositionY,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::GlobalPositionX,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::GlobalPositionX,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::GlobalPositionY,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::GlobalPositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_pivot_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::PivotOffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_pivot_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::PivotOffsetY,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::ScaleX,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::ScaleY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::SizeX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlFloatData {
            property: <ControlFloatKind>::SizeY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_custom_minimum_size(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::CustomMinimumSize,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::Position,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::Position,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::GlobalPosition,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::GlobalPosition,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_pivot_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::PivotOffset,
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
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::Scale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlVector2Data {
            property: <ControlVector2Kind>::Size,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
