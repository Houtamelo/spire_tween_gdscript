use super::*;
#[derive(Debug, Clone)]
pub struct Camera2DFloatData {
    pub property: Camera2DFloatKind,
    pub owner: Gd<Camera2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Camera2DFloatKind {
    DragHorizontalOffset,
    DragVerticalOffset,
    DragLeftMargin,
    DragRightMargin,
    DragTopMargin,
    DragBottomMargin,
    ZoomX,
    ZoomY,
    OffsetX,
    OffsetY,
}
impl IProperty<f64> for Camera2DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <Camera2DFloatKind>::DragHorizontalOffset => {
                let obj = &self.owner;
                (obj.get_drag_horizontal_offset()) as f64
            }
            <Camera2DFloatKind>::DragVerticalOffset => {
                let obj = &self.owner;
                (obj.get_drag_vertical_offset()) as f64
            }
            <Camera2DFloatKind>::DragLeftMargin => {
                let obj = &self.owner;
                (obj.get_drag_margin(Side::LEFT)) as f64
            }
            <Camera2DFloatKind>::DragRightMargin => {
                let obj = &self.owner;
                (obj.get_drag_margin(Side::RIGHT)) as f64
            }
            <Camera2DFloatKind>::DragTopMargin => {
                let obj = &self.owner;
                (obj.get_drag_margin(Side::TOP)) as f64
            }
            <Camera2DFloatKind>::DragBottomMargin => {
                let obj = &self.owner;
                (obj.get_drag_margin(Side::BOTTOM)) as f64
            }
            <Camera2DFloatKind>::ZoomX => {
                let obj = &self.owner;
                (obj.get_zoom().x) as f64
            }
            <Camera2DFloatKind>::ZoomY => {
                let obj = &self.owner;
                (obj.get_zoom().y) as f64
            }
            <Camera2DFloatKind>::OffsetX => {
                let obj = &self.owner;
                (obj.get_offset().x) as f64
            }
            <Camera2DFloatKind>::OffsetY => {
                let obj = &self.owner;
                (obj.get_offset().y) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <Camera2DFloatKind>::DragHorizontalOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_horizontal_offset(val as f32);
            }
            <Camera2DFloatKind>::DragVerticalOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_vertical_offset(val as f32);
            }
            <Camera2DFloatKind>::DragLeftMargin => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_margin(Side::LEFT, val as f32);
            }
            <Camera2DFloatKind>::DragRightMargin => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_margin(Side::RIGHT, val as f32);
            }
            <Camera2DFloatKind>::DragTopMargin => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_margin(Side::TOP, val as f32);
            }
            <Camera2DFloatKind>::DragBottomMargin => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_margin(Side::BOTTOM, val as f32);
            }
            <Camera2DFloatKind>::ZoomX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_zoom();
                prop_val.x = val as f32;
                obj.set_zoom(prop_val);
            }
            <Camera2DFloatKind>::ZoomY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_zoom();
                prop_val.y = val as f32;
                obj.set_zoom(prop_val);
            }
            <Camera2DFloatKind>::OffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.x = val as f32;
                obj.set_offset(prop_val);
            }
            <Camera2DFloatKind>::OffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.y = val as f32;
                obj.set_offset(prop_val);
            }
        }
    }
}
impl IPropertyData for Camera2DFloatData {
    type Target = Camera2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Camera2DFloatKind>::DragHorizontalOffset => {
                NodePath::from("drag_horizontal_offset")
            }
            <Camera2DFloatKind>::DragVerticalOffset => {
                NodePath::from("drag_vertical_offset")
            }
            <Camera2DFloatKind>::DragLeftMargin => NodePath::from("drag_left_margin"),
            <Camera2DFloatKind>::DragRightMargin => NodePath::from("drag_right_margin"),
            <Camera2DFloatKind>::DragTopMargin => NodePath::from("drag_top_margin"),
            <Camera2DFloatKind>::DragBottomMargin => NodePath::from("drag_bottom_margin"),
            <Camera2DFloatKind>::ZoomX => NodePath::from("zoom:x"),
            <Camera2DFloatKind>::ZoomY => NodePath::from("zoom:y"),
            <Camera2DFloatKind>::OffsetX => NodePath::from("offset:x"),
            <Camera2DFloatKind>::OffsetY => NodePath::from("offset:y"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Camera2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Camera2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Camera2DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Camera2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "drag_horizontal_offset" => {
                        Some(Self {
                            property: <Camera2DFloatKind>::DragHorizontalOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "drag_vertical_offset" => {
                        Some(Self {
                            property: <Camera2DFloatKind>::DragVerticalOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "drag_left_margin" => {
                        Some(Self {
                            property: <Camera2DFloatKind>::DragLeftMargin,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "drag_right_margin" => {
                        Some(Self {
                            property: <Camera2DFloatKind>::DragRightMargin,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "drag_top_margin" => {
                        Some(Self {
                            property: <Camera2DFloatKind>::DragTopMargin,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "drag_bottom_margin" => {
                        Some(Self {
                            property: <Camera2DFloatKind>::DragBottomMargin,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "zoom:x" => {
                        Some(Self {
                            property: <Camera2DFloatKind>::ZoomX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "zoom:y" => {
                        Some(Self {
                            property: <Camera2DFloatKind>::ZoomY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:x" => {
                        Some(Self {
                            property: <Camera2DFloatKind>::OffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:y" => {
                        Some(Self {
                            property: <Camera2DFloatKind>::OffsetY,
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
pub struct Camera2DVector2Data {
    pub property: Camera2DVector2Kind,
    pub owner: Gd<Camera2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Camera2DVector2Kind {
    Zoom,
    Offset,
}
impl IProperty<Vector2> for Camera2DVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <Camera2DVector2Kind>::Zoom => {
                let obj = &self.owner;
                obj.get_zoom()
            }
            <Camera2DVector2Kind>::Offset => {
                let obj = &self.owner;
                obj.get_offset()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <Camera2DVector2Kind>::Zoom => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_zoom(val);
            }
            <Camera2DVector2Kind>::Offset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_offset(val);
            }
        }
    }
}
impl IPropertyData for Camera2DVector2Data {
    type Target = Camera2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Camera2DVector2Kind>::Zoom => NodePath::from("zoom"),
            <Camera2DVector2Kind>::Offset => NodePath::from("offset"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Camera2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Camera2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Camera2DVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Camera2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "zoom" => {
                        Some(Self {
                            property: <Camera2DVector2Kind>::Zoom,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset" => {
                        Some(Self {
                            property: <Camera2DVector2Kind>::Offset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoCamera2D<Marker = ()> {
    fn do_drag_horizontal_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_drag_vertical_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_drag_left_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_drag_right_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_drag_top_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_drag_bottom_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_zoom_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_zoom_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_zoom(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<Camera2D> + Inherits<Object>> SpireDoCamera2D<()> for Gd<Class> {
    fn do_drag_horizontal_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragHorizontalOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_drag_vertical_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragVerticalOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_drag_left_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragLeftMargin,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_drag_right_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragRightMargin,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_drag_top_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragTopMargin,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_drag_bottom_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragBottomMargin,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_zoom_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::ZoomX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_zoom_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::ZoomY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::OffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::OffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_zoom(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Camera2DVector2Data {
            property: <Camera2DVector2Kind>::Zoom,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Camera2DVector2Data {
            property: <Camera2DVector2Kind>::Offset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<Camera2D> + Inherits<Object>,
> SpireDoCamera2D<BaseMarker> for T {
    fn do_drag_horizontal_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragHorizontalOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_drag_vertical_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragVerticalOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_drag_left_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragLeftMargin,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_drag_right_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragRightMargin,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_drag_top_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragTopMargin,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_drag_bottom_margin(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::DragBottomMargin,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_zoom_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::ZoomX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_zoom_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::ZoomY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::OffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DFloatData {
            property: <Camera2DFloatKind>::OffsetY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_zoom(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DVector2Data {
            property: <Camera2DVector2Kind>::Zoom,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DVector2Data {
            property: <Camera2DVector2Kind>::Offset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
