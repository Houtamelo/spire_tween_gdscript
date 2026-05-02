use super::*;

#[derive(Debug, Clone)]
pub struct PropertyVec2Node2DFollowData {
    pub owner: Gd<Node2D>,
    pub owner_obj_or_node: ObjectOrNode,
}

impl IProperty<Vector2> for PropertyVec2Node2DFollowData {
    #[inline]
    fn get_property_value(&self) -> Vector2 { self.owner.get_global_position() }

    #[inline]
    fn set_property_value(&mut self, value: Vector2) { self.owner.set_global_position(value); }
}

impl IPropertyData for PropertyVec2Node2DFollowData {
    type Target = Node2D;

    #[inline]
    fn get_property_path(&self) -> NodePath { "global_position".into() }

    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> { Some(&self.owner_obj_or_node) }
}

/// Speed-based tween that chases another `Node2D`'s `global_position`.
///
/// The receiver moves toward `follow_this`'s current position at `speed` units per
/// second — re-evaluated every tick, so the receiver tracks the target wherever it
/// goes. The tween never "completes" in the usual sense — it just keeps catching
/// up — so it loops forever unless explicitly stopped or until `follow_this` is
/// freed (then the tween auto-stops).
///
/// Implemented for any `Gd<T: Inherits<Node2D>>`. Returns an unregistered
/// [`SpireTween<LerpPropertyData<Vector2>>`] — call [`register`](SpireTween::register).
///
/// **Note:** Built on the [`Evaluator::Dynamic`] target mechanism — `follow_this`
/// is captured by closure. If `follow_this` becomes invalid mid-tween, the tween
/// uses the last-known position rather than crashing.
pub trait DoFollow2D<Marker = ()> {
    fn do_follow(&self, follow_this: Gd<Node2D>, speed: f64) -> SpireTween<LerpPropertyData<Vector2>>;
}

impl<T: Inherits<Node2D> + Inherits<Object>> DoFollow2D<()> for Gd<T> {
    fn do_follow(&self, follow_this: Gd<Node2D>, speed: f64) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Node2D> = self.clone().upcast();

        let data = PropertyVec2Node2DFollowData {
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

impl TryFromPathAndObject for PropertyVec2Node2DFollowData {}
