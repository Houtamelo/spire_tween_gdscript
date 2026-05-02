use super::*;

#[derive(Debug, Clone)]
pub struct PropertyVec3Node3DFollowData {
    pub owner: Gd<Node3D>,
    pub owner_obj_or_node: ObjectOrNode,
}

impl IProperty<Vector3> for PropertyVec3Node3DFollowData {
    #[inline]
    fn get_property_value(&self) -> Vector3 { self.owner.get_global_position() }

    #[inline]
    fn set_property_value(&mut self, value: Vector3) { self.owner.set_global_position(value); }
}

impl IPropertyData for PropertyVec3Node3DFollowData {
    type Target = Node3D;

    #[inline]
    fn get_property_path(&self) -> NodePath { "global_position".into() }

    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> { Some(&self.owner_obj_or_node) }
}

/// 3D analogue of [`DoFollow2D`]: speed-based tween that chases another `Node3D`'s
/// `global_position`.
///
/// See [`DoFollow2D`] for behavior — same semantics in 3D space (Euclidean speed
/// across `(x, y, z)`).
pub trait DoFollow3D<Marker = ()> {
    fn do_follow(&self, follow_this: Gd<Node3D>, speed: f64) -> SpireTween<LerpPropertyData<Vector3>>;
}

impl<T: Inherits<Node3D> + Inherits<Object>> DoFollow3D<()> for Gd<T> {
    fn do_follow(&self, follow_this: Gd<Node3D>, speed: f64) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Node3D> = self.clone().upcast();

        let data = PropertyVec3Node3DFollowData {
            owner_obj_or_node: ObjectOrNode::Node(owner.clone().upcast()),
            owner,
        };

        let mut last_known_pos = follow_this.get_global_position();

        SpireTween::<LerpPropertyData<Vector3>>::new(
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

impl TryFromPathAndObject for PropertyVec3Node3DFollowData {}
