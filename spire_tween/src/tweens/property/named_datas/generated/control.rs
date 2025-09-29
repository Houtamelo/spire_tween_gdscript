use super::*;
#[derive(Debug, Clone)]
pub struct ControlF32Data {
    pub property: ControlF32Kind,
    pub owner: Gd<Control>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlF32Kind {
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
impl IProperty<f32> for ControlF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <ControlF32Kind>::AnchorBottom => {
                let obj = &self.owner;
                obj.get_anchor(Side::BOTTOM)
            }
            <ControlF32Kind>::AnchorLeft => {
                let obj = &self.owner;
                obj.get_anchor(Side::LEFT)
            }
            <ControlF32Kind>::AnchorRight => {
                let obj = &self.owner;
                obj.get_anchor(Side::RIGHT)
            }
            <ControlF32Kind>::AnchorTop => {
                let obj = &self.owner;
                obj.get_anchor(Side::TOP)
            }
            <ControlF32Kind>::Rotation => {
                let obj = &self.owner;
                obj.get_rotation()
            }
            <ControlF32Kind>::RotationDegrees => {
                let obj = &self.owner;
                obj.get_rotation_degrees()
            }
            <ControlF32Kind>::CustomMinimumSizeX => {
                let obj = &self.owner;
                obj.get_custom_minimum_size().x
            }
            <ControlF32Kind>::CustomMinimumSizeY => {
                let obj = &self.owner;
                obj.get_custom_minimum_size().y
            }
            <ControlF32Kind>::PositionX => {
                let obj = &self.owner;
                obj.get_position().x
            }
            <ControlF32Kind>::PositionY => {
                let obj = &self.owner;
                obj.get_position().y
            }
            <ControlF32Kind>::GlobalPositionX => {
                let obj = &self.owner;
                obj.get_global_position().x
            }
            <ControlF32Kind>::GlobalPositionY => {
                let obj = &self.owner;
                obj.get_global_position().y
            }
            <ControlF32Kind>::PivotOffsetX => {
                let obj = &self.owner;
                obj.get_pivot_offset().x
            }
            <ControlF32Kind>::PivotOffsetY => {
                let obj = &self.owner;
                obj.get_pivot_offset().y
            }
            <ControlF32Kind>::ScaleX => {
                let obj = &self.owner;
                obj.get_scale().x
            }
            <ControlF32Kind>::ScaleY => {
                let obj = &self.owner;
                obj.get_scale().y
            }
            <ControlF32Kind>::SizeX => {
                let obj = &self.owner;
                obj.get_size().x
            }
            <ControlF32Kind>::SizeY => {
                let obj = &self.owner;
                obj.get_size().y
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <ControlF32Kind>::AnchorBottom => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_anchor(Side::BOTTOM, val);
            }
            <ControlF32Kind>::AnchorLeft => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_anchor(Side::LEFT, val);
            }
            <ControlF32Kind>::AnchorRight => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_anchor(Side::RIGHT, val);
            }
            <ControlF32Kind>::AnchorTop => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_anchor(Side::TOP, val);
            }
            <ControlF32Kind>::Rotation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation(val);
            }
            <ControlF32Kind>::RotationDegrees => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation_degrees(val);
            }
            <ControlF32Kind>::CustomMinimumSizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_custom_minimum_size();
                prop_val.x = val;
                obj.set_custom_minimum_size(prop_val);
            }
            <ControlF32Kind>::CustomMinimumSizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_custom_minimum_size();
                prop_val.y = val;
                obj.set_custom_minimum_size(prop_val);
            }
            <ControlF32Kind>::PositionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.x = val;
                obj.set_position(prop_val);
            }
            <ControlF32Kind>::PositionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.y = val;
                obj.set_position(prop_val);
            }
            <ControlF32Kind>::GlobalPositionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_position();
                prop_val.x = val;
                obj.set_global_position(prop_val);
            }
            <ControlF32Kind>::GlobalPositionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_global_position();
                prop_val.y = val;
                obj.set_global_position(prop_val);
            }
            <ControlF32Kind>::PivotOffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_pivot_offset();
                prop_val.x = val;
                obj.set_pivot_offset(prop_val);
            }
            <ControlF32Kind>::PivotOffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_pivot_offset();
                prop_val.y = val;
                obj.set_pivot_offset(prop_val);
            }
            <ControlF32Kind>::ScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.x = val;
                obj.set_scale(prop_val);
            }
            <ControlF32Kind>::ScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.y = val;
                obj.set_scale(prop_val);
            }
            <ControlF32Kind>::SizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.x = val;
                obj.set_size(prop_val);
            }
            <ControlF32Kind>::SizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.y = val;
                obj.set_size(prop_val);
            }
        }
    }
}
impl IPropertyData for ControlF32Data {
    type Target = Control;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ControlF32Kind>::AnchorBottom => NodePath::from("anchor_bottom"),
            <ControlF32Kind>::AnchorLeft => NodePath::from("anchor_left"),
            <ControlF32Kind>::AnchorRight => NodePath::from("anchor_right"),
            <ControlF32Kind>::AnchorTop => NodePath::from("anchor_top"),
            <ControlF32Kind>::Rotation => NodePath::from("rotation"),
            <ControlF32Kind>::RotationDegrees => NodePath::from("rotation_degrees"),
            <ControlF32Kind>::CustomMinimumSizeX => {
                NodePath::from("custom_minimum_size:x")
            }
            <ControlF32Kind>::CustomMinimumSizeY => {
                NodePath::from("custom_minimum_size:y")
            }
            <ControlF32Kind>::PositionX => NodePath::from("position:x"),
            <ControlF32Kind>::PositionY => NodePath::from("position:y"),
            <ControlF32Kind>::GlobalPositionX => NodePath::from("global_position:x"),
            <ControlF32Kind>::GlobalPositionY => NodePath::from("global_position:y"),
            <ControlF32Kind>::PivotOffsetX => NodePath::from("pivot_offset:x"),
            <ControlF32Kind>::PivotOffsetY => NodePath::from("pivot_offset:y"),
            <ControlF32Kind>::ScaleX => NodePath::from("scale:x"),
            <ControlF32Kind>::ScaleY => NodePath::from("scale:y"),
            <ControlF32Kind>::SizeX => NodePath::from("size:x"),
            <ControlF32Kind>::SizeY => NodePath::from("size:y"),
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
impl TryFromPathAndObject for ControlF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Control>()
            .ok()
            .and_then(|owner| {
                match path {
                    "anchor_bottom" => {
                        Some(Self {
                            property: <ControlF32Kind>::AnchorBottom,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "anchor_left" => {
                        Some(Self {
                            property: <ControlF32Kind>::AnchorLeft,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "anchor_right" => {
                        Some(Self {
                            property: <ControlF32Kind>::AnchorRight,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "anchor_top" => {
                        Some(Self {
                            property: <ControlF32Kind>::AnchorTop,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation" => {
                        Some(Self {
                            property: <ControlF32Kind>::Rotation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "rotation_degrees" => {
                        Some(Self {
                            property: <ControlF32Kind>::RotationDegrees,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "custom_minimum_size:x" => {
                        Some(Self {
                            property: <ControlF32Kind>::CustomMinimumSizeX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "custom_minimum_size:y" => {
                        Some(Self {
                            property: <ControlF32Kind>::CustomMinimumSizeY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "position:x" => {
                        Some(Self {
                            property: <ControlF32Kind>::PositionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "position:y" => {
                        Some(Self {
                            property: <ControlF32Kind>::PositionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position:x" => {
                        Some(Self {
                            property: <ControlF32Kind>::GlobalPositionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "global_position:y" => {
                        Some(Self {
                            property: <ControlF32Kind>::GlobalPositionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "pivot_offset:x" => {
                        Some(Self {
                            property: <ControlF32Kind>::PivotOffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "pivot_offset:y" => {
                        Some(Self {
                            property: <ControlF32Kind>::PivotOffsetY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:x" => {
                        Some(Self {
                            property: <ControlF32Kind>::ScaleX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:y" => {
                        Some(Self {
                            property: <ControlF32Kind>::ScaleY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:x" => {
                        Some(Self {
                            property: <ControlF32Kind>::SizeX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:y" => {
                        Some(Self {
                            property: <ControlF32Kind>::SizeY,
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
pub trait DoControl<Marker = ()> {
    fn do_anchor_bottom(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_anchor_left(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_anchor_right(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_anchor_top(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn control_do_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn control_do_rotation_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_custom_minimum_size_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_custom_minimum_size_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn control_do_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn control_do_move_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn control_do_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn control_do_move_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn control_do_global_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn control_do_global_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_pivot_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_pivot_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn control_do_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn control_do_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_size_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_size_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_custom_minimum_size(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn control_do_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn control_do_move(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn control_do_global_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_pivot_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn control_do_scale(
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
impl<Class: Inherits<Control> + Inherits<Object>> DoControl<()> for Gd<Class> {
    fn do_anchor_bottom(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::AnchorBottom,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_anchor_left(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::AnchorLeft,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_anchor_right(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::AnchorRight,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_anchor_top(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::AnchorTop,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::Rotation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_rotation_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::RotationDegrees,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_custom_minimum_size_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::CustomMinimumSizeX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_custom_minimum_size_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::CustomMinimumSizeY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::PositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_move_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::PositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::PositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_move_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::PositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_global_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::GlobalPositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_global_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::GlobalPositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_pivot_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::PivotOffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_pivot_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::PivotOffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::ScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::ScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_size_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::SizeX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_size_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ControlF32Data {
            property: <ControlF32Kind>::SizeY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Control>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_position(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_move(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_global_position(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_scale(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<Control> + Inherits<Object>> DoControl<BaseMarker>
for T {
    fn do_anchor_bottom(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::AnchorBottom,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_anchor_left(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::AnchorLeft,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_anchor_right(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::AnchorRight,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_anchor_top(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::AnchorTop,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::Rotation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_rotation_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::RotationDegrees,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_custom_minimum_size_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::CustomMinimumSizeX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_custom_minimum_size_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::CustomMinimumSizeY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::PositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_move_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::PositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::PositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_move_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::PositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_global_position_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::GlobalPositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_global_position_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::GlobalPositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_pivot_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::PivotOffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_pivot_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::PivotOffsetY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::ScaleX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::ScaleY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_size_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::SizeX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_size_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Control> = self.to_gd().upcast();
        let data = ControlF32Data {
            property: <ControlF32Kind>::SizeY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_position(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_move(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_global_position(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn control_do_scale(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct ControlTweener {}
#[godot_api]
impl ControlTweener {
    #[func]
    fn do_anchor_bottom(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_anchor_bottom(to, duration), gd
        }
    }
    #[func]
    fn do_anchor_left(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_anchor_left(to, duration), gd
        }
    }
    #[func]
    fn do_anchor_right(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_anchor_right(to, duration), gd
        }
    }
    #[func]
    fn do_anchor_top(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_anchor_top(to, duration), gd
        }
    }
    #[func]
    fn control_do_rotation(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_rotation(to, duration), gd
        }
    }
    #[func]
    fn control_do_rotation_degrees(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_rotation_degrees(to, duration), gd
        }
    }
    #[func]
    fn do_custom_minimum_size_x(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_custom_minimum_size_x(to, duration), gd
        }
    }
    #[func]
    fn do_custom_minimum_size_y(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_custom_minimum_size_y(to, duration), gd
        }
    }
    #[func]
    fn control_do_position_x(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_position_x(to, duration), gd
        }
    }
    #[func]
    fn control_do_move_x(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_position_x(to, duration), gd
        }
    }
    #[func]
    fn control_do_position_y(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_position_y(to, duration), gd
        }
    }
    #[func]
    fn control_do_move_y(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_position_y(to, duration), gd
        }
    }
    #[func]
    fn control_do_global_position_x(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_global_position_x(to, duration), gd
        }
    }
    #[func]
    fn control_do_global_position_y(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_global_position_y(to, duration), gd
        }
    }
    #[func]
    fn do_pivot_offset_x(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_pivot_offset_x(to, duration), gd
        }
    }
    #[func]
    fn do_pivot_offset_y(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_pivot_offset_y(to, duration), gd
        }
    }
    #[func]
    fn control_do_scale_x(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_scale_x(to, duration), gd
        }
    }
    #[func]
    fn control_do_scale_y(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_scale_y(to, duration), gd
        }
    }
    #[func]
    fn do_size_x(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_size_x(to, duration), gd
        }
    }
    #[func]
    fn do_size_y(
        node: Gd<Control>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_size_y(to, duration), gd
        }
    }
    #[func]
    fn do_custom_minimum_size(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_custom_minimum_size(to, duration), gd
        }
    }
    #[func]
    fn control_do_position(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_position(to, duration), gd
        }
    }
    #[func]
    fn control_do_move(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_position(to, duration), gd
        }
    }
    #[func]
    fn control_do_global_position(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_global_position(to, duration), gd
        }
    }
    #[func]
    fn do_pivot_offset(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_pivot_offset(to, duration), gd
        }
    }
    #[func]
    fn control_do_scale(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.control_do_scale(to, duration), gd
        }
    }
    #[func]
    fn do_size(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_size(to, duration), gd
        }
    }
}
