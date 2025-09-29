use super::*;
#[derive(Debug, Clone)]
pub struct CanvasLayerF32Data {
    pub property: CanvasLayerF32Kind,
    pub owner: Gd<CanvasLayer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasLayerF32Kind {
    Rotation,
    FollowViewportScale,
    OffsetX,
    OffsetY,
    ScaleX,
    ScaleY,
}
impl IProperty<f32> for CanvasLayerF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <CanvasLayerF32Kind>::Rotation => {
                let obj = &self.owner;
                obj.get_rotation()
            }
            <CanvasLayerF32Kind>::FollowViewportScale => {
                let obj = &self.owner;
                obj.get_follow_viewport_scale()
            }
            <CanvasLayerF32Kind>::OffsetX => {
                let obj = &self.owner;
                obj.get_offset().x
            }
            <CanvasLayerF32Kind>::OffsetY => {
                let obj = &self.owner;
                obj.get_offset().y
            }
            <CanvasLayerF32Kind>::ScaleX => {
                let obj = &self.owner;
                obj.get_scale().x
            }
            <CanvasLayerF32Kind>::ScaleY => {
                let obj = &self.owner;
                obj.get_scale().y
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <CanvasLayerF32Kind>::Rotation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation(val);
            }
            <CanvasLayerF32Kind>::FollowViewportScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_follow_viewport_scale(val);
            }
            <CanvasLayerF32Kind>::OffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.x = val;
                obj.set_offset(prop_val);
            }
            <CanvasLayerF32Kind>::OffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.y = val;
                obj.set_offset(prop_val);
            }
            <CanvasLayerF32Kind>::ScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.x = val;
                obj.set_scale(prop_val);
            }
            <CanvasLayerF32Kind>::ScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.y = val;
                obj.set_scale(prop_val);
            }
        }
    }
}
impl IPropertyData for CanvasLayerF32Data {
    type Target = CanvasLayer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasLayerF32Kind>::Rotation => NodePath::from("rotation"),
            <CanvasLayerF32Kind>::FollowViewportScale => {
                NodePath::from("follow_viewport_scale")
            }
            <CanvasLayerF32Kind>::OffsetX => NodePath::from("offset:x"),
            <CanvasLayerF32Kind>::OffsetY => NodePath::from("offset:y"),
            <CanvasLayerF32Kind>::ScaleX => NodePath::from("scale:x"),
            <CanvasLayerF32Kind>::ScaleY => NodePath::from("scale:y"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CanvasLayer>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CanvasLayer>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CanvasLayerF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasLayer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "rotation" => {
                        Some(Self {
                            property: <CanvasLayerF32Kind>::Rotation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "follow_viewport_scale" => {
                        Some(Self {
                            property: <CanvasLayerF32Kind>::FollowViewportScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:x" => {
                        Some(Self {
                            property: <CanvasLayerF32Kind>::OffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:y" => {
                        Some(Self {
                            property: <CanvasLayerF32Kind>::OffsetY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:x" => {
                        Some(Self {
                            property: <CanvasLayerF32Kind>::ScaleX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:y" => {
                        Some(Self {
                            property: <CanvasLayerF32Kind>::ScaleY,
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
pub struct CanvasLayerVector2Data {
    pub property: CanvasLayerVector2Kind,
    pub owner: Gd<CanvasLayer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasLayerVector2Kind {
    Offset,
    Scale,
}
impl IProperty<Vector2> for CanvasLayerVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <CanvasLayerVector2Kind>::Offset => {
                let obj = &self.owner;
                obj.get_offset()
            }
            <CanvasLayerVector2Kind>::Scale => {
                let obj = &self.owner;
                obj.get_scale()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <CanvasLayerVector2Kind>::Offset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_offset(val);
            }
            <CanvasLayerVector2Kind>::Scale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_scale(val);
            }
        }
    }
}
impl IPropertyData for CanvasLayerVector2Data {
    type Target = CanvasLayer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasLayerVector2Kind>::Offset => NodePath::from("offset"),
            <CanvasLayerVector2Kind>::Scale => NodePath::from("scale"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CanvasLayer>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CanvasLayer>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CanvasLayerVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasLayer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "offset" => {
                        Some(Self {
                            property: <CanvasLayerVector2Kind>::Offset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale" => {
                        Some(Self {
                            property: <CanvasLayerVector2Kind>::Scale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoCanvasLayer<Marker = ()> {
    fn canvas_layer_do_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn canvas_layer_do_follow_viewport_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn canvas_layer_do_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn canvas_layer_do_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn canvas_layer_do_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn canvas_layer_do_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn canvas_layer_do_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn canvas_layer_do_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<CanvasLayer> + Inherits<Object>> DoCanvasLayer<()> for Gd<Class> {
    fn canvas_layer_do_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::Rotation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_follow_viewport_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::FollowViewportScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::OffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::OffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::ScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::ScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = CanvasLayerVector2Data {
            property: <CanvasLayerVector2Kind>::Offset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = CanvasLayerVector2Data {
            property: <CanvasLayerVector2Kind>::Scale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<CanvasLayer> + Inherits<Object>,
> DoCanvasLayer<BaseMarker> for T {
    fn canvas_layer_do_rotation(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::Rotation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_follow_viewport_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::FollowViewportScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::OffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::OffsetY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_scale_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::ScaleX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_scale_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerF32Data {
            property: <CanvasLayerF32Kind>::ScaleY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerVector2Data {
            property: <CanvasLayerVector2Kind>::Offset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_layer_do_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerVector2Data {
            property: <CanvasLayerVector2Kind>::Scale,
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
pub struct CanvasLayerTweener {}
#[godot_api]
impl CanvasLayerTweener {
    #[func]
    fn canvas_layer_do_rotation(
        node: Gd<CanvasLayer>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_layer_do_rotation(to, duration), gd
        }
    }
    #[func]
    fn canvas_layer_do_follow_viewport_scale(
        node: Gd<CanvasLayer>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_layer_do_follow_viewport_scale(to, duration), gd
        }
    }
    #[func]
    fn canvas_layer_do_offset_x(
        node: Gd<CanvasLayer>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_layer_do_offset_x(to, duration), gd
        }
    }
    #[func]
    fn canvas_layer_do_offset_y(
        node: Gd<CanvasLayer>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_layer_do_offset_y(to, duration), gd
        }
    }
    #[func]
    fn canvas_layer_do_scale_x(
        node: Gd<CanvasLayer>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_layer_do_scale_x(to, duration), gd
        }
    }
    #[func]
    fn canvas_layer_do_scale_y(
        node: Gd<CanvasLayer>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_layer_do_scale_y(to, duration), gd
        }
    }
    #[func]
    fn canvas_layer_do_offset(
        node: Gd<CanvasLayer>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_layer_do_offset(to, duration), gd
        }
    }
    #[func]
    fn canvas_layer_do_scale(
        node: Gd<CanvasLayer>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_layer_do_scale(to, duration), gd
        }
    }
}
