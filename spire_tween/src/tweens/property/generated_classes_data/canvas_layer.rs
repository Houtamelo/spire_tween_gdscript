use super::*;
#[derive(Debug, Clone)]
pub struct CanvasLayerFloatData {
    pub property: CanvasLayerFloatKind,
    pub owner: Gd<CanvasLayer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasLayerFloatKind {
    Rotation,
    FollowViewportScale,
    OffsetX,
    OffsetY,
    ScaleX,
    ScaleY,
}
impl IProperty<f64> for CanvasLayerFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <CanvasLayerFloatKind>::Rotation => {
                let obj = &self.owner;
                (obj.get_rotation()) as f64
            }
            <CanvasLayerFloatKind>::FollowViewportScale => {
                let obj = &self.owner;
                (obj.get_follow_viewport_scale()) as f64
            }
            <CanvasLayerFloatKind>::OffsetX => {
                let obj = &self.owner;
                (obj.get_offset().x) as f64
            }
            <CanvasLayerFloatKind>::OffsetY => {
                let obj = &self.owner;
                (obj.get_offset().y) as f64
            }
            <CanvasLayerFloatKind>::ScaleX => {
                let obj = &self.owner;
                (obj.get_scale().x) as f64
            }
            <CanvasLayerFloatKind>::ScaleY => {
                let obj = &self.owner;
                (obj.get_scale().y) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <CanvasLayerFloatKind>::Rotation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_rotation(val as f32);
            }
            <CanvasLayerFloatKind>::FollowViewportScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_follow_viewport_scale(val as f32);
            }
            <CanvasLayerFloatKind>::OffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.x = val as f32;
                obj.set_offset(prop_val);
            }
            <CanvasLayerFloatKind>::OffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.y = val as f32;
                obj.set_offset(prop_val);
            }
            <CanvasLayerFloatKind>::ScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.x = val as f32;
                obj.set_scale(prop_val);
            }
            <CanvasLayerFloatKind>::ScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scale();
                prop_val.y = val as f32;
                obj.set_scale(prop_val);
            }
        }
    }
}
impl IPropertyData for CanvasLayerFloatData {
    type Target = CanvasLayer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasLayerFloatKind>::Rotation => NodePath::from("rotation"),
            <CanvasLayerFloatKind>::FollowViewportScale => {
                NodePath::from("follow_viewport_scale")
            }
            <CanvasLayerFloatKind>::OffsetX => NodePath::from("offset:x"),
            <CanvasLayerFloatKind>::OffsetY => NodePath::from("offset:y"),
            <CanvasLayerFloatKind>::ScaleX => NodePath::from("scale:x"),
            <CanvasLayerFloatKind>::ScaleY => NodePath::from("scale:y"),
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
impl TryFromPathAndObject for CanvasLayerFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasLayer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "rotation" => {
                        Some(Self {
                            property: <CanvasLayerFloatKind>::Rotation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "follow_viewport_scale" => {
                        Some(Self {
                            property: <CanvasLayerFloatKind>::FollowViewportScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:x" => {
                        Some(Self {
                            property: <CanvasLayerFloatKind>::OffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:y" => {
                        Some(Self {
                            property: <CanvasLayerFloatKind>::OffsetY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:x" => {
                        Some(Self {
                            property: <CanvasLayerFloatKind>::ScaleX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scale:y" => {
                        Some(Self {
                            property: <CanvasLayerFloatKind>::ScaleY,
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
pub trait SpireDoCanvasLayer<Marker = ()> {
    fn do_rotation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_follow_viewport_scale(
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
    fn do_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<CanvasLayer> + Inherits<Object>> SpireDoCanvasLayer<()>
for Gd<Class> {
    fn do_rotation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::Rotation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_follow_viewport_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::FollowViewportScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
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
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::OffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
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
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::OffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
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
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::ScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
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
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::ScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasLayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scale(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<CanvasLayer> + Inherits<Object>,
> SpireDoCanvasLayer<BaseMarker> for T {
    fn do_rotation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::Rotation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_follow_viewport_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::FollowViewportScale,
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
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::OffsetX,
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
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::OffsetY,
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
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::ScaleX,
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
        let owner: Gd<CanvasLayer> = self.to_gd().upcast();
        let data = CanvasLayerFloatData {
            property: <CanvasLayerFloatKind>::ScaleY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scale(
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
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
