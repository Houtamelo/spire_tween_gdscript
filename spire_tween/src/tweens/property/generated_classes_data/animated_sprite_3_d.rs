use super::*;
#[derive(Debug, Clone)]
pub struct AnimatedSprite3DIntData {
    pub property: AnimatedSprite3DIntKind,
    pub owner: Gd<AnimatedSprite3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatedSprite3DIntKind {
    Frame,
}
impl IProperty<i64> for AnimatedSprite3DIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <AnimatedSprite3DIntKind>::Frame => {
                let obj = &self.owner;
                (obj.get_frame()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <AnimatedSprite3DIntKind>::Frame => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_frame(val as i32);
            }
        }
    }
}
impl IPropertyData for AnimatedSprite3DIntData {
    type Target = AnimatedSprite3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AnimatedSprite3DIntKind>::Frame => NodePath::from("frame"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<AnimatedSprite3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<AnimatedSprite3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for AnimatedSprite3DIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AnimatedSprite3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "frame" => {
                        Some(Self {
                            property: <AnimatedSprite3DIntKind>::Frame,
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
pub struct AnimatedSprite3DFloatData {
    pub property: AnimatedSprite3DFloatKind,
    pub owner: Gd<AnimatedSprite3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatedSprite3DFloatKind {
    FrameProgress,
    SpeedScale,
}
impl IProperty<f64> for AnimatedSprite3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <AnimatedSprite3DFloatKind>::FrameProgress => {
                let obj = &self.owner;
                (obj.get_frame_progress()) as f64
            }
            <AnimatedSprite3DFloatKind>::SpeedScale => {
                let obj = &self.owner;
                (obj.get_speed_scale()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <AnimatedSprite3DFloatKind>::FrameProgress => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_frame_progress(val as f32);
            }
            <AnimatedSprite3DFloatKind>::SpeedScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_speed_scale(val as f32);
            }
        }
    }
}
impl IPropertyData for AnimatedSprite3DFloatData {
    type Target = AnimatedSprite3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AnimatedSprite3DFloatKind>::FrameProgress => {
                NodePath::from("frame_progress")
            }
            <AnimatedSprite3DFloatKind>::SpeedScale => NodePath::from("speed_scale"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<AnimatedSprite3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<AnimatedSprite3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for AnimatedSprite3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AnimatedSprite3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "frame_progress" => {
                        Some(Self {
                            property: <AnimatedSprite3DFloatKind>::FrameProgress,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "speed_scale" => {
                        Some(Self {
                            property: <AnimatedSprite3DFloatKind>::SpeedScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoAnimatedSprite3D<Marker = ()> {
    fn do_frame(&self, end_val: i64, duration: f64) -> SpireTween<LerpPropertyData<i64>>;
    fn do_frame_progress(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_speed_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<AnimatedSprite3D> + Inherits<Object>> SpireDoAnimatedSprite3D<()>
for Gd<Class> {
    fn do_frame(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = AnimatedSprite3DIntData {
            property: <AnimatedSprite3DIntKind>::Frame,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AnimatedSprite3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_frame_progress(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = AnimatedSprite3DFloatData {
            property: <AnimatedSprite3DFloatKind>::FrameProgress,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AnimatedSprite3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_speed_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = AnimatedSprite3DFloatData {
            property: <AnimatedSprite3DFloatKind>::SpeedScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AnimatedSprite3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<AnimatedSprite3D> + Inherits<Object>,
> SpireDoAnimatedSprite3D<BaseMarker> for T {
    fn do_frame(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<AnimatedSprite3D> = self.to_gd().upcast();
        let data = AnimatedSprite3DIntData {
            property: <AnimatedSprite3DIntKind>::Frame,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_frame_progress(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<AnimatedSprite3D> = self.to_gd().upcast();
        let data = AnimatedSprite3DFloatData {
            property: <AnimatedSprite3DFloatKind>::FrameProgress,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_speed_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<AnimatedSprite3D> = self.to_gd().upcast();
        let data = AnimatedSprite3DFloatData {
            property: <AnimatedSprite3DFloatKind>::SpeedScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
