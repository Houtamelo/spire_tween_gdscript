use super::*;
#[derive(Debug, Clone)]
pub struct AnimationPlayerFloatData {
    pub property: AnimationPlayerFloatKind,
    pub owner: Gd<AnimationPlayer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPlayerFloatKind {
    SpeedScale,
}
impl IProperty<f64> for AnimationPlayerFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <AnimationPlayerFloatKind>::SpeedScale => {
                let obj = &self.owner;
                (obj.get_speed_scale()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <AnimationPlayerFloatKind>::SpeedScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_speed_scale(val as f32);
            }
        }
    }
}
impl IPropertyData for AnimationPlayerFloatData {
    type Target = AnimationPlayer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AnimationPlayerFloatKind>::SpeedScale => NodePath::from("speed_scale"),
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
impl TryFromPathAndObject for AnimationPlayerFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AnimationPlayer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "speed_scale" => {
                        Some(Self {
                            property: <AnimationPlayerFloatKind>::SpeedScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoAnimationPlayer<Marker = ()> {
    fn do_speed_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<AnimationPlayer> + Inherits<Object>> SpireDoAnimationPlayer<()>
for Gd<Class> {
    fn do_speed_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = AnimationPlayerFloatData {
            property: <AnimationPlayerFloatKind>::SpeedScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AnimationPlayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<AnimationPlayer> + Inherits<Object>,
> SpireDoAnimationPlayer<BaseMarker> for T {
    fn do_speed_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<AnimationPlayer> = self.to_gd().upcast();
        let data = AnimationPlayerFloatData {
            property: <AnimationPlayerFloatKind>::SpeedScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
