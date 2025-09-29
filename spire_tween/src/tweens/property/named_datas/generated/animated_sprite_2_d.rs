use super::*;
#[derive(Debug, Clone)]
pub struct AnimatedSprite2DF32Data {
    pub property: AnimatedSprite2DF32Kind,
    pub owner: Gd<AnimatedSprite2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatedSprite2DF32Kind {
    SpeedScale,
}
impl IProperty<f32> for AnimatedSprite2DF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <AnimatedSprite2DF32Kind>::SpeedScale => {
                let obj = &self.owner;
                obj.get_speed_scale()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <AnimatedSprite2DF32Kind>::SpeedScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_speed_scale(val);
            }
        }
    }
}
impl IPropertyData for AnimatedSprite2DF32Data {
    type Target = AnimatedSprite2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AnimatedSprite2DF32Kind>::SpeedScale => NodePath::from("speed_scale"),
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
impl TryFromPathAndObject for AnimatedSprite2DF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AnimatedSprite2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "speed_scale" => {
                        Some(Self {
                            property: <AnimatedSprite2DF32Kind>::SpeedScale,
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
pub struct AnimatedSprite2DI32Data {
    pub property: AnimatedSprite2DI32Kind,
    pub owner: Gd<AnimatedSprite2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatedSprite2DI32Kind {
    Frame,
}
impl IProperty<i32> for AnimatedSprite2DI32Data {
    #[inline]
    fn get_property_value(&self) -> i32 {
        match self.property {
            <AnimatedSprite2DI32Kind>::Frame => {
                let obj = &self.owner;
                obj.get_frame()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i32) {
        match self.property {
            <AnimatedSprite2DI32Kind>::Frame => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_frame(val);
            }
        }
    }
}
impl IPropertyData for AnimatedSprite2DI32Data {
    type Target = AnimatedSprite2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AnimatedSprite2DI32Kind>::Frame => NodePath::from("frame"),
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
impl TryFromPathAndObject for AnimatedSprite2DI32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AnimatedSprite2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "frame" => {
                        Some(Self {
                            property: <AnimatedSprite2DI32Kind>::Frame,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoAnimatedSprite2D<Marker = ()> {
    fn do_speed_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_frame(&self, end_val: i32, duration: f64) -> SpireTween<LerpPropertyData<i32>>;
}
impl<Class: Inherits<AnimatedSprite2D> + Inherits<Object>> DoAnimatedSprite2D<()>
for Gd<Class> {
    fn do_speed_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = AnimatedSprite2DF32Data {
            property: <AnimatedSprite2DF32Kind>::SpeedScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AnimatedSprite2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_frame(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let data = AnimatedSprite2DI32Data {
            property: <AnimatedSprite2DI32Kind>::Frame,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AnimatedSprite2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<AnimatedSprite2D> + Inherits<Object>,
> DoAnimatedSprite2D<BaseMarker> for T {
    fn do_speed_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<AnimatedSprite2D> = self.to_gd().upcast();
        let data = AnimatedSprite2DF32Data {
            property: <AnimatedSprite2DF32Kind>::SpeedScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_frame(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<AnimatedSprite2D> = self.to_gd().upcast();
        let data = AnimatedSprite2DI32Data {
            property: <AnimatedSprite2DI32Kind>::Frame,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct AnimatedSprite2DTweener {}
#[godot_api]
impl AnimatedSprite2DTweener {
    #[func]
    fn do_speed_scale(
        node: Gd<AnimatedSprite2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_speed_scale(to, duration), gd
        }
    }
    #[func]
    fn do_frame(
        node: Gd<AnimatedSprite2D>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        let gd = <i32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_frame(to, duration), gd
        }
    }
}
