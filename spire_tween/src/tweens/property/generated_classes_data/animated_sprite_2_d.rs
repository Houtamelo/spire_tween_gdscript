use super::*;
#[derive(Debug, Clone)]
pub struct AnimatedSprite2DIntData {
    pub property: AnimatedSprite2DIntKind,
    pub owner: Gd<AnimatedSprite2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatedSprite2DIntKind {
    Frame,
}
impl IProperty<i64> for AnimatedSprite2DIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <AnimatedSprite2DIntKind>::Frame => {
                let obj = &self.owner;
                (obj.get_frame()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <AnimatedSprite2DIntKind>::Frame => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_frame(val as i32);
            }
        }
    }
}
impl IPropertyData for AnimatedSprite2DIntData {
    type Target = AnimatedSprite2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AnimatedSprite2DIntKind>::Frame => NodePath::from("frame"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<AnimatedSprite2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<AnimatedSprite2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for AnimatedSprite2DIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AnimatedSprite2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "frame" => {
                        Some(Self {
                            property: <AnimatedSprite2DIntKind>::Frame,
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
pub struct AnimatedSprite2DFloatData {
    pub property: AnimatedSprite2DFloatKind,
    pub owner: Gd<AnimatedSprite2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatedSprite2DFloatKind {
    SpeedScale,
}
impl IProperty<f64> for AnimatedSprite2DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <AnimatedSprite2DFloatKind>::SpeedScale => {
                let obj = &self.owner;
                (obj.get_speed_scale()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <AnimatedSprite2DFloatKind>::SpeedScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_speed_scale(val as f32);
            }
        }
    }
}
impl IPropertyData for AnimatedSprite2DFloatData {
    type Target = AnimatedSprite2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AnimatedSprite2DFloatKind>::SpeedScale => NodePath::from("speed_scale"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<AnimatedSprite2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<AnimatedSprite2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for AnimatedSprite2DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AnimatedSprite2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "speed_scale" => {
                        Some(Self {
                            property: <AnimatedSprite2DFloatKind>::SpeedScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoAnimatedSprite2D<Marker = ()> {
    fn do_frame(&self, end_val: i64, duration: f64) -> SpireTween<LerpPropertyData<i64>>;
    fn do_speed_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<AnimatedSprite2D> + Inherits<Object>> SpireDoAnimatedSprite2D<()>
for Gd<Class> {
    fn do_frame(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = AnimatedSprite2DIntData {
            property: <AnimatedSprite2DIntKind>::Frame,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AnimatedSprite2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_speed_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = AnimatedSprite2DFloatData {
            property: <AnimatedSprite2DFloatKind>::SpeedScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AnimatedSprite2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<AnimatedSprite2D> + Inherits<Object>,
> SpireDoAnimatedSprite2D<BaseMarker> for T {
    fn do_frame(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<AnimatedSprite2D> = self.to_gd().upcast();
        let data = AnimatedSprite2DIntData {
            property: <AnimatedSprite2DIntKind>::Frame,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_speed_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<AnimatedSprite2D> = self.to_gd().upcast();
        let data = AnimatedSprite2DFloatData {
            property: <AnimatedSprite2DFloatKind>::SpeedScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
