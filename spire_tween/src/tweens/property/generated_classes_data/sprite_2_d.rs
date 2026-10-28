use super::*;
#[derive(Debug, Clone)]
pub struct Sprite2DIntData {
    pub property: Sprite2DIntKind,
    pub owner: Gd<Sprite2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sprite2DIntKind {
    Frame,
}
impl IProperty<i64> for Sprite2DIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <Sprite2DIntKind>::Frame => {
                let obj = &self.owner;
                (obj.get_frame()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <Sprite2DIntKind>::Frame => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_frame(val as i32);
            }
        }
    }
}
impl IPropertyData for Sprite2DIntData {
    type Target = Sprite2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Sprite2DIntKind>::Frame => NodePath::from("frame"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for Sprite2DIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Sprite2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "frame" => {
                        Some(Self {
                            property: <Sprite2DIntKind>::Frame,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
#[derive(Debug, Clone)]
pub struct Sprite2DVec2Data {
    pub property: Sprite2DVec2Kind,
    pub owner: Gd<Sprite2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sprite2DVec2Kind {
    RegionRectPOSITION,
    RegionRectSIZE,
}
impl IProperty<Vector2> for Sprite2DVec2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <Sprite2DVec2Kind>::RegionRectPOSITION => {
                let obj = &self.owner;
                obj.get_region_rect().position
            }
            <Sprite2DVec2Kind>::RegionRectSIZE => {
                let obj = &self.owner;
                obj.get_region_rect().size
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <Sprite2DVec2Kind>::RegionRectPOSITION => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_region_rect();
                prop_val.position = val;
                obj.set_region_rect(prop_val);
            }
            <Sprite2DVec2Kind>::RegionRectSIZE => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_region_rect();
                prop_val.size = val;
                obj.set_region_rect(prop_val);
            }
        }
    }
}
impl IPropertyData for Sprite2DVec2Data {
    type Target = Sprite2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Sprite2DVec2Kind>::RegionRectPOSITION => {
                NodePath::from("region_rect:position")
            }
            <Sprite2DVec2Kind>::RegionRectSIZE => NodePath::from("region_rect:size"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for Sprite2DVec2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Sprite2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "region_rect:position" => {
                        Some(Self {
                            property: <Sprite2DVec2Kind>::RegionRectPOSITION,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "region_rect:size" => {
                        Some(Self {
                            property: <Sprite2DVec2Kind>::RegionRectSIZE,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
#[derive(Debug, Clone)]
pub struct Sprite2DRect2Data {
    pub property: Sprite2DRect2Kind,
    pub owner: Gd<Sprite2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sprite2DRect2Kind {
    RegionRect,
}
impl IProperty<Rect2> for Sprite2DRect2Data {
    #[inline]
    fn get_property_value(&self) -> Rect2 {
        match self.property {
            <Sprite2DRect2Kind>::RegionRect => {
                let obj = &self.owner;
                obj.get_region_rect()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Rect2) {
        match self.property {
            <Sprite2DRect2Kind>::RegionRect => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_region_rect(val);
            }
        }
    }
}
impl IPropertyData for Sprite2DRect2Data {
    type Target = Sprite2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Sprite2DRect2Kind>::RegionRect => NodePath::from("region_rect"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for Sprite2DRect2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Sprite2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "region_rect" => {
                        Some(Self {
                            property: <Sprite2DRect2Kind>::RegionRect,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoSprite2D<Marker = ()> {
    fn do_frame(&self, end_val: i64, duration: f64) -> SpireTween<LerpPropertyData<i64>>;
    fn do_region_rect_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_region_rect_size(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_region_rect(
        &self,
        end_val: Rect2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Rect2>>;
}
impl<Class: Inherits<Sprite2D> + Inherits<Object>> SpireDoSprite2D<()> for Gd<Class> {
    fn do_frame(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = Sprite2DIntData {
            property: <Sprite2DIntKind>::Frame,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_region_rect_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Sprite2DVec2Data {
            property: <Sprite2DVec2Kind>::RegionRectPOSITION,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_region_rect_size(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Sprite2DVec2Data {
            property: <Sprite2DVec2Kind>::RegionRectSIZE,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_region_rect(
        &self,
        end_val: Rect2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Rect2>> {
        let data = Sprite2DRect2Data {
            property: <Sprite2DRect2Kind>::RegionRect,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Rect2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<Sprite2D> + Inherits<Object>,
> SpireDoSprite2D<BaseMarker> for T {
    fn do_frame(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<Sprite2D> = self.to_gd().upcast();
        let data = Sprite2DIntData {
            property: <Sprite2DIntKind>::Frame,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_region_rect_position(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Sprite2D> = self.to_gd().upcast();
        let data = Sprite2DVec2Data {
            property: <Sprite2DVec2Kind>::RegionRectPOSITION,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_region_rect_size(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Sprite2D> = self.to_gd().upcast();
        let data = Sprite2DVec2Data {
            property: <Sprite2DVec2Kind>::RegionRectSIZE,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_region_rect(
        &self,
        end_val: Rect2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Rect2>> {
        let owner: Gd<Sprite2D> = self.to_gd().upcast();
        let data = Sprite2DRect2Data {
            property: <Sprite2DRect2Kind>::RegionRect,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Rect2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
