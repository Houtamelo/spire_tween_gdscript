use super::*;
#[derive(Debug, Clone)]
pub struct ParallaxBackgroundFloatData {
    pub property: ParallaxBackgroundFloatKind,
    pub owner: Gd<ParallaxBackground>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParallaxBackgroundFloatKind {
    ScrollBaseOffsetX,
    ScrollBaseOffsetY,
    ScrollBaseScaleX,
    ScrollBaseScaleY,
    ScrollOffsetX,
    ScrollOffsetY,
}
impl IProperty<f64> for ParallaxBackgroundFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <ParallaxBackgroundFloatKind>::ScrollBaseOffsetX => {
                let obj = &self.owner;
                (obj.get_scroll_base_offset().x) as f64
            }
            <ParallaxBackgroundFloatKind>::ScrollBaseOffsetY => {
                let obj = &self.owner;
                (obj.get_scroll_base_offset().y) as f64
            }
            <ParallaxBackgroundFloatKind>::ScrollBaseScaleX => {
                let obj = &self.owner;
                (obj.get_scroll_base_scale().x) as f64
            }
            <ParallaxBackgroundFloatKind>::ScrollBaseScaleY => {
                let obj = &self.owner;
                (obj.get_scroll_base_scale().y) as f64
            }
            <ParallaxBackgroundFloatKind>::ScrollOffsetX => {
                let obj = &self.owner;
                (obj.get_scroll_offset().x) as f64
            }
            <ParallaxBackgroundFloatKind>::ScrollOffsetY => {
                let obj = &self.owner;
                (obj.get_scroll_offset().y) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <ParallaxBackgroundFloatKind>::ScrollBaseOffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scroll_base_offset();
                prop_val.x = val as f32;
                obj.set_scroll_base_offset(prop_val);
            }
            <ParallaxBackgroundFloatKind>::ScrollBaseOffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scroll_base_offset();
                prop_val.y = val as f32;
                obj.set_scroll_base_offset(prop_val);
            }
            <ParallaxBackgroundFloatKind>::ScrollBaseScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scroll_base_scale();
                prop_val.x = val as f32;
                obj.set_scroll_base_scale(prop_val);
            }
            <ParallaxBackgroundFloatKind>::ScrollBaseScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scroll_base_scale();
                prop_val.y = val as f32;
                obj.set_scroll_base_scale(prop_val);
            }
            <ParallaxBackgroundFloatKind>::ScrollOffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scroll_offset();
                prop_val.x = val as f32;
                obj.set_scroll_offset(prop_val);
            }
            <ParallaxBackgroundFloatKind>::ScrollOffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_scroll_offset();
                prop_val.y = val as f32;
                obj.set_scroll_offset(prop_val);
            }
        }
    }
}
impl IPropertyData for ParallaxBackgroundFloatData {
    type Target = ParallaxBackground;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ParallaxBackgroundFloatKind>::ScrollBaseOffsetX => {
                NodePath::from("scroll_base_offset:x")
            }
            <ParallaxBackgroundFloatKind>::ScrollBaseOffsetY => {
                NodePath::from("scroll_base_offset:y")
            }
            <ParallaxBackgroundFloatKind>::ScrollBaseScaleX => {
                NodePath::from("scroll_base_scale:x")
            }
            <ParallaxBackgroundFloatKind>::ScrollBaseScaleY => {
                NodePath::from("scroll_base_scale:y")
            }
            <ParallaxBackgroundFloatKind>::ScrollOffsetX => {
                NodePath::from("scroll_offset:x")
            }
            <ParallaxBackgroundFloatKind>::ScrollOffsetY => {
                NodePath::from("scroll_offset:y")
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
            ObjectOrNode::Object(obj) => obj.try_cast::<ParallaxBackground>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<ParallaxBackground>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ParallaxBackgroundFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ParallaxBackground>()
            .ok()
            .and_then(|owner| {
                match path {
                    "scroll_base_offset:x" => {
                        Some(Self {
                            property: <ParallaxBackgroundFloatKind>::ScrollBaseOffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scroll_base_offset:y" => {
                        Some(Self {
                            property: <ParallaxBackgroundFloatKind>::ScrollBaseOffsetY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scroll_base_scale:x" => {
                        Some(Self {
                            property: <ParallaxBackgroundFloatKind>::ScrollBaseScaleX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scroll_base_scale:y" => {
                        Some(Self {
                            property: <ParallaxBackgroundFloatKind>::ScrollBaseScaleY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scroll_offset:x" => {
                        Some(Self {
                            property: <ParallaxBackgroundFloatKind>::ScrollOffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scroll_offset:y" => {
                        Some(Self {
                            property: <ParallaxBackgroundFloatKind>::ScrollOffsetY,
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
pub struct ParallaxBackgroundVector2Data {
    pub property: ParallaxBackgroundVector2Kind,
    pub owner: Gd<ParallaxBackground>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParallaxBackgroundVector2Kind {
    ScrollBaseOffset,
    ScrollBaseScale,
    ScrollOffset,
}
impl IProperty<Vector2> for ParallaxBackgroundVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <ParallaxBackgroundVector2Kind>::ScrollBaseOffset => {
                let obj = &self.owner;
                obj.get_scroll_base_offset()
            }
            <ParallaxBackgroundVector2Kind>::ScrollBaseScale => {
                let obj = &self.owner;
                obj.get_scroll_base_scale()
            }
            <ParallaxBackgroundVector2Kind>::ScrollOffset => {
                let obj = &self.owner;
                obj.get_scroll_offset()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <ParallaxBackgroundVector2Kind>::ScrollBaseOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_scroll_base_offset(val);
            }
            <ParallaxBackgroundVector2Kind>::ScrollBaseScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_scroll_base_scale(val);
            }
            <ParallaxBackgroundVector2Kind>::ScrollOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_scroll_offset(val);
            }
        }
    }
}
impl IPropertyData for ParallaxBackgroundVector2Data {
    type Target = ParallaxBackground;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ParallaxBackgroundVector2Kind>::ScrollBaseOffset => {
                NodePath::from("scroll_base_offset")
            }
            <ParallaxBackgroundVector2Kind>::ScrollBaseScale => {
                NodePath::from("scroll_base_scale")
            }
            <ParallaxBackgroundVector2Kind>::ScrollOffset => {
                NodePath::from("scroll_offset")
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
            ObjectOrNode::Object(obj) => obj.try_cast::<ParallaxBackground>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<ParallaxBackground>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ParallaxBackgroundVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ParallaxBackground>()
            .ok()
            .and_then(|owner| {
                match path {
                    "scroll_base_offset" => {
                        Some(Self {
                            property: <ParallaxBackgroundVector2Kind>::ScrollBaseOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scroll_base_scale" => {
                        Some(Self {
                            property: <ParallaxBackgroundVector2Kind>::ScrollBaseScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "scroll_offset" => {
                        Some(Self {
                            property: <ParallaxBackgroundVector2Kind>::ScrollOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoParallaxBackground<Marker = ()> {
    fn do_scroll_base_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_scroll_base_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_scroll_base_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_scroll_base_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_scroll_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_scroll_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_scroll_base_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_scroll_base_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_scroll_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<
    Class: Inherits<ParallaxBackground> + Inherits<Object>,
> SpireDoParallaxBackground<()> for Gd<Class> {
    fn do_scroll_base_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollBaseOffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ParallaxBackground>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_base_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollBaseOffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ParallaxBackground>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_base_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollBaseScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ParallaxBackground>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_base_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollBaseScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ParallaxBackground>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollOffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ParallaxBackground>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollOffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ParallaxBackground>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_base_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = ParallaxBackgroundVector2Data {
            property: <ParallaxBackgroundVector2Kind>::ScrollBaseOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ParallaxBackground>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_base_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = ParallaxBackgroundVector2Data {
            property: <ParallaxBackgroundVector2Kind>::ScrollBaseScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ParallaxBackground>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = ParallaxBackgroundVector2Data {
            property: <ParallaxBackgroundVector2Kind>::ScrollOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ParallaxBackground>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<ParallaxBackground> + Inherits<Object>,
> SpireDoParallaxBackground<BaseMarker> for T {
    fn do_scroll_base_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ParallaxBackground> = self.to_gd().upcast();
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollBaseOffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_base_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ParallaxBackground> = self.to_gd().upcast();
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollBaseOffsetY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_base_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ParallaxBackground> = self.to_gd().upcast();
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollBaseScaleX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_base_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ParallaxBackground> = self.to_gd().upcast();
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollBaseScaleY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ParallaxBackground> = self.to_gd().upcast();
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollOffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ParallaxBackground> = self.to_gd().upcast();
        let data = ParallaxBackgroundFloatData {
            property: <ParallaxBackgroundFloatKind>::ScrollOffsetY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_base_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<ParallaxBackground> = self.to_gd().upcast();
        let data = ParallaxBackgroundVector2Data {
            property: <ParallaxBackgroundVector2Kind>::ScrollBaseOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_base_scale(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<ParallaxBackground> = self.to_gd().upcast();
        let data = ParallaxBackgroundVector2Data {
            property: <ParallaxBackgroundVector2Kind>::ScrollBaseScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_scroll_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<ParallaxBackground> = self.to_gd().upcast();
        let data = ParallaxBackgroundVector2Data {
            property: <ParallaxBackgroundVector2Kind>::ScrollOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
