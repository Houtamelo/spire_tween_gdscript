use super::*;
#[derive(Debug, Clone)]
pub struct Camera2DF32Data {
    pub property: Camera2DF32Kind,
    pub owner: Gd<Camera2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Camera2DF32Kind {
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
impl IProperty<f32> for Camera2DF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <Camera2DF32Kind>::DragHorizontalOffset => {
                let obj = &self.owner;
                obj.get_drag_horizontal_offset()
            }
            <Camera2DF32Kind>::DragVerticalOffset => {
                let obj = &self.owner;
                obj.get_drag_vertical_offset()
            }
            <Camera2DF32Kind>::DragLeftMargin => {
                let obj = &self.owner;
                obj.get_drag_margin(Side::LEFT)
            }
            <Camera2DF32Kind>::DragRightMargin => {
                let obj = &self.owner;
                obj.get_drag_margin(Side::RIGHT)
            }
            <Camera2DF32Kind>::DragTopMargin => {
                let obj = &self.owner;
                obj.get_drag_margin(Side::TOP)
            }
            <Camera2DF32Kind>::DragBottomMargin => {
                let obj = &self.owner;
                obj.get_drag_margin(Side::BOTTOM)
            }
            <Camera2DF32Kind>::ZoomX => {
                let obj = &self.owner;
                obj.get_zoom().x
            }
            <Camera2DF32Kind>::ZoomY => {
                let obj = &self.owner;
                obj.get_zoom().y
            }
            <Camera2DF32Kind>::OffsetX => {
                let obj = &self.owner;
                obj.get_offset().x
            }
            <Camera2DF32Kind>::OffsetY => {
                let obj = &self.owner;
                obj.get_offset().y
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <Camera2DF32Kind>::DragHorizontalOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_horizontal_offset(val);
            }
            <Camera2DF32Kind>::DragVerticalOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_vertical_offset(val);
            }
            <Camera2DF32Kind>::DragLeftMargin => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_margin(Side::LEFT, val);
            }
            <Camera2DF32Kind>::DragRightMargin => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_margin(Side::RIGHT, val);
            }
            <Camera2DF32Kind>::DragTopMargin => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_margin(Side::TOP, val);
            }
            <Camera2DF32Kind>::DragBottomMargin => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_drag_margin(Side::BOTTOM, val);
            }
            <Camera2DF32Kind>::ZoomX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_zoom();
                prop_val.x = val;
                obj.set_zoom(prop_val);
            }
            <Camera2DF32Kind>::ZoomY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_zoom();
                prop_val.y = val;
                obj.set_zoom(prop_val);
            }
            <Camera2DF32Kind>::OffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.x = val;
                obj.set_offset(prop_val);
            }
            <Camera2DF32Kind>::OffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.y = val;
                obj.set_offset(prop_val);
            }
        }
    }
}
impl IPropertyData for Camera2DF32Data {
    type Target = Camera2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Camera2DF32Kind>::DragHorizontalOffset => {
                NodePath::from("drag_horizontal_offset")
            }
            <Camera2DF32Kind>::DragVerticalOffset => {
                NodePath::from("drag_vertical_offset")
            }
            <Camera2DF32Kind>::DragLeftMargin => NodePath::from("drag_left_margin"),
            <Camera2DF32Kind>::DragRightMargin => NodePath::from("drag_right_margin"),
            <Camera2DF32Kind>::DragTopMargin => NodePath::from("drag_top_margin"),
            <Camera2DF32Kind>::DragBottomMargin => NodePath::from("drag_bottom_margin"),
            <Camera2DF32Kind>::ZoomX => NodePath::from("zoom:x"),
            <Camera2DF32Kind>::ZoomY => NodePath::from("zoom:y"),
            <Camera2DF32Kind>::OffsetX => NodePath::from("offset:x"),
            <Camera2DF32Kind>::OffsetY => NodePath::from("offset:y"),
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
impl TryFromPathAndObject for Camera2DF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Camera2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "drag_horizontal_offset" => {
                        Some(Self {
                            property: <Camera2DF32Kind>::DragHorizontalOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "drag_vertical_offset" => {
                        Some(Self {
                            property: <Camera2DF32Kind>::DragVerticalOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "drag_left_margin" => {
                        Some(Self {
                            property: <Camera2DF32Kind>::DragLeftMargin,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "drag_right_margin" => {
                        Some(Self {
                            property: <Camera2DF32Kind>::DragRightMargin,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "drag_top_margin" => {
                        Some(Self {
                            property: <Camera2DF32Kind>::DragTopMargin,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "drag_bottom_margin" => {
                        Some(Self {
                            property: <Camera2DF32Kind>::DragBottomMargin,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "zoom:x" => {
                        Some(Self {
                            property: <Camera2DF32Kind>::ZoomX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "zoom:y" => {
                        Some(Self {
                            property: <Camera2DF32Kind>::ZoomY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:x" => {
                        Some(Self {
                            property: <Camera2DF32Kind>::OffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:y" => {
                        Some(Self {
                            property: <Camera2DF32Kind>::OffsetY,
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
pub trait DoCamera2D<Marker = ()> {
    fn do_drag_horizontal_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_drag_vertical_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_drag_left_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_drag_right_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_drag_top_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_drag_bottom_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_zoom_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_zoom_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
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
impl<Class: Inherits<Camera2D> + Inherits<Object>> DoCamera2D<()> for Gd<Class> {
    fn do_drag_horizontal_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragHorizontalOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_drag_vertical_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragVerticalOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_drag_left_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragLeftMargin,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_drag_right_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragRightMargin,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_drag_top_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragTopMargin,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_drag_bottom_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragBottomMargin,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_zoom_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::ZoomX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_zoom_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::ZoomY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::OffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::OffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<Camera2D> + Inherits<Object>> DoCamera2D<BaseMarker>
for T {
    fn do_drag_horizontal_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragHorizontalOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_drag_vertical_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragVerticalOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_drag_left_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragLeftMargin,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_drag_right_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragRightMargin,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_drag_top_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragTopMargin,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_drag_bottom_margin(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::DragBottomMargin,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_zoom_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::ZoomX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_zoom_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::ZoomY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::OffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<Camera2D> = self.to_gd().upcast();
        let data = Camera2DF32Data {
            property: <Camera2DF32Kind>::OffsetY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct Camera2DTweener {}
#[godot_api]
impl Camera2DTweener {
    #[func]
    fn do_drag_horizontal_offset(
        node: Gd<Camera2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_drag_horizontal_offset(to, duration), gd
        }
    }
    #[func]
    fn do_drag_vertical_offset(
        node: Gd<Camera2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_drag_vertical_offset(to, duration), gd
        }
    }
    #[func]
    fn do_drag_left_margin(
        node: Gd<Camera2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_drag_left_margin(to, duration), gd
        }
    }
    #[func]
    fn do_drag_right_margin(
        node: Gd<Camera2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_drag_right_margin(to, duration), gd
        }
    }
    #[func]
    fn do_drag_top_margin(
        node: Gd<Camera2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_drag_top_margin(to, duration), gd
        }
    }
    #[func]
    fn do_drag_bottom_margin(
        node: Gd<Camera2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_drag_bottom_margin(to, duration), gd
        }
    }
    #[func]
    fn do_zoom_x(
        node: Gd<Camera2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_zoom_x(to, duration), gd
        }
    }
    #[func]
    fn do_zoom_y(
        node: Gd<Camera2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_zoom_y(to, duration), gd
        }
    }
    #[func]
    fn do_offset_x(
        node: Gd<Camera2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_offset_x(to, duration), gd
        }
    }
    #[func]
    fn do_offset_y(
        node: Gd<Camera2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_offset_y(to, duration), gd
        }
    }
    #[func]
    fn do_zoom(
        node: Gd<Camera2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_zoom(to, duration), gd
        }
    }
    #[func]
    fn do_offset(
        node: Gd<Camera2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_offset(to, duration), gd
        }
    }
}
