use super::*;

pub trait DoBone<Marker = ()> {
    fn do_bone_position(&self, bone_idx: i32, to: Vector3, duration: f64) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_bone_scale(&self, bone_idx: i32, to: Vector3, duration: f64) -> SpireTween<LerpPropertyData<Vector3>>;
}

impl<T: Inherits<Skeleton3D> + Inherits<Object>> DoBone<()> for Gd<T> {
    fn do_bone_position(&self, bone_idx: i32, to: Vector3, duration: f64) -> SpireTween<LerpPropertyData<Vector3>> {
        let node = self.clone().upcast::<Skeleton3D>();

        // Note: `Callable::bind()` does NOT propagate to built-in C++ methods like
        // `set_bone_pose_position` — calling the bound callable silently no-ops on
        // the underlying object. We use explicit `Callable::from_fn` closures instead.
        let getter_node = node.clone();
        let getter = Callable::from_fn("get_bone_pose_position_bound", move |_| {
            getter_node.get_bone_pose_position(bone_idx).to_variant()
        });
        let mut setter_node = node.clone();
        let setter = Callable::from_fn("set_bone_pose_position_bound", move |args| {
            let value: Vector3 = args
                .first()
                .and_then(|v| v.try_to_relaxed::<Vector3>().ok())
                .unwrap_or_default();
            setter_node.set_bone_pose_position(bone_idx, value);
            Variant::nil()
        });

        let data_via_callable: PropertyDataVector3 = PropertyDataViaCallable::new(getter, setter).into();

        SpireTween::<LerpPropertyData<Vector3>>::new(data_via_callable, Evaluator::Static(to), duration)
    }

    fn do_bone_scale(&self, bone_idx: i32, to: Vector3, duration: f64) -> SpireTween<LerpPropertyData<Vector3>> {
        let node = self.clone().upcast::<Skeleton3D>();

        let getter_node = node.clone();
        let getter = Callable::from_fn("get_bone_pose_scale_bound", move |_| {
            getter_node.get_bone_pose_scale(bone_idx).to_variant()
        });
        let mut setter_node = node.clone();
        let setter = Callable::from_fn("set_bone_pose_scale_bound", move |args| {
            let value: Vector3 = args
                .first()
                .and_then(|v| v.try_to_relaxed::<Vector3>().ok())
                .unwrap_or_default();
            setter_node.set_bone_pose_scale(bone_idx, value);
            Variant::nil()
        });

        let data_via_callable: PropertyDataVector3 = PropertyDataViaCallable::new(getter, setter).into();

        SpireTween::<LerpPropertyData<Vector3>>::new(data_via_callable, Evaluator::Static(to), duration)
    }
}
