use super::*;
#[derive(Debug, Clone)]
pub struct AnimationPlayerF32Data {
    pub property: AnimationPlayerF32Kind,
    pub owner: Gd<AnimationPlayer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPlayerF32Kind {
    SpeedScale,
}
impl IProperty<f32> for AnimationPlayerF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <AnimationPlayerF32Kind>::SpeedScale => {
                let obj = &self.owner;
                obj.get_speed_scale()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <AnimationPlayerF32Kind>::SpeedScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_speed_scale(val);
            }
        }
    }
}
impl IPropertyData for AnimationPlayerF32Data {
    type Target = AnimationPlayer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AnimationPlayerF32Kind>::SpeedScale => NodePath::from("speed_scale"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<AnimationPlayer>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<AnimationPlayer>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for AnimationPlayerF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AnimationPlayer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "speed_scale" => {
                        Some(Self {
                            property: <AnimationPlayerF32Kind>::SpeedScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoAnimationPlayer<Marker = ()> {
    fn do_speed_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
}
impl<Class: Inherits<AnimationPlayer> + Inherits<Object>> DoAnimationPlayer<()>
for Gd<Class> {
    fn do_speed_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = AnimationPlayerF32Data {
            property: <AnimationPlayerF32Kind>::SpeedScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AnimationPlayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<AnimationPlayer> + Inherits<Object>,
> DoAnimationPlayer<BaseMarker> for T {
    fn do_speed_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<AnimationPlayer> = self.to_gd().upcast();
        let data = AnimationPlayerF32Data {
            property: <AnimationPlayerF32Kind>::SpeedScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct AnimationPlayerTweener {}
#[godot_api]
impl AnimationPlayerTweener {
    #[func]
    fn do_speed_scale(
        node: Gd<AnimationPlayer>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_speed_scale(to, duration), gd
        }
    }
}
