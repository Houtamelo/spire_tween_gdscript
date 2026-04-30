use super::*;

#[derive(Debug, Clone)]
pub struct PropertyVector2Node2DFollowData {
    pub owner: Gd<Node2D>,
    pub owner_obj_or_node: ObjectOrNode,
}

impl IProperty<Vector2> for PropertyVector2Node2DFollowData {
    #[inline]
    fn get_property_value(&self) -> Vector2 { self.owner.get_global_position() }

    #[inline]
    fn set_property_value(&mut self, value: Vector2) { self.owner.set_global_position(value); }
}

impl IPropertyData for PropertyVector2Node2DFollowData {
    type Target = Node2D;

    #[inline]
    fn get_property_path(&self) -> NodePath { "global_position".into() }

    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> { Some(&self.owner_obj_or_node) }
}

/// Speed-based tween that chases another `Node2D`'s `global_position`.
pub trait DoFollow2D<Marker = ()> {
    fn do_follow(&self, follow_this: Gd<Node2D>, speed: f64) -> SpireTween<LerpPropertyData<Vector2>>;
}

impl<T: Inherits<Node2D> + Inherits<Object>> DoFollow2D<()> for Gd<T> {
    fn do_follow(&self, follow_this: Gd<Node2D>, speed: f64) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Node2D> = self.clone().upcast();

        let data = PropertyVector2Node2DFollowData {
            owner_obj_or_node: ObjectOrNode::Node(owner.clone().upcast()),
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

impl TryFromPathAndObject for PropertyVector2Node2DFollowData {}
