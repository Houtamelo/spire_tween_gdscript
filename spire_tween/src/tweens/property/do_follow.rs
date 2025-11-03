use super::*;
use crate::gdscript_bridge::*;

#[godot_api(secondary)]
impl DoNode2D {
    #[func]
    pub fn follow(
        node: Gd<Node2D>,
        follow_this: Gd<Node2D>,
        speed: f64,
    ) -> Gd<SpirePropertyVector2> {
        let inner = UnsafeCell::new(node.do_follow(follow_this, speed).register());
        Gd::from_init_fn(|base| SpirePropertyVector2 { base, inner })
    }
}

#[derive(Debug, Clone)]
pub struct PropertyVector2Node2DFollowData {
    pub owner: Gd<Node2D>,
    pub owner_obj_or_node: ObjectOrNode,
}

impl IProperty<Vector2> for PropertyVector2Node2DFollowData {
    fn get_property_value(&self) -> Vector2 { self.owner.get_global_position() }

    fn set_property_value(&mut self, value: Vector2) { self.owner.set_global_position(value); }
}

impl IPropertyData for PropertyVector2Node2DFollowData {
    type Target = Node2D;

    fn get_property_path(&self) -> NodePath { "global_position".into() }

    fn get_owner(&self) -> &ObjectOrNode { &self.owner_obj_or_node }

    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(node2d) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Node2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Node2D>().ok(),
        } {
            self.owner = node2d;
            true
        } else {
            false
        }
    }
}

pub trait DoFollow2D<Marker = ()> {
    fn do_follow(
        &self,
        follow_this: Gd<Node2D>,
        speed: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}

impl<T: Inherits<Node2D> + Inherits<Object>> DoFollow2D<()> for Gd<T> {
    fn do_follow(
        &self,
        follow_this: Gd<Node2D>,
        speed: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Node2D> = self.clone().upcast();

        let data = PropertyVector2Node2DFollowData {
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };

        let mut last_known_pos = follow_this.get_global_position();

        SpireTween::<LerpPropertyData<Vector2>>::new(
            data.into(),
            Evaluator::Dynamic(Box::new({
                move || {
                    if follow_this.is_instance_valid() {
                        last_known_pos = follow_this.get_global_position();
                    }

                    last_known_pos
                }
            })),
            speed,
        )
        .as_speed_based()
    }
}

impl TryFromPathAndObject for PropertyVector2Node2DFollowData {
    fn try_from_path_and_object(_path: &str, _object: Gd<Object>) -> Option<Self>
    where Self: Sized {
        None
    }
}
