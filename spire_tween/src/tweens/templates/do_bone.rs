use super::*;

pub trait DoBone<Marker = ()> {
    fn do_bone_position(&self, bone_idx: i32, to: Vector3, duration: f64) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_bone_scale(&self, bone_idx: i32, to: Vector3, duration: f64) -> SpireTween<LerpPropertyData<Vector3>>;
}

impl<T: Inherits<Skeleton3D> + Inherits<Object>> DoBone<()> for Gd<T> {
    fn do_bone_position(&self, bone_idx: i32, to: Vector3, duration: f64) -> SpireTween<LerpPropertyData<Vector3>> {
        let node = self.clone().upcast::<Skeleton3D>();

        let data_via_callable: PropertyDataVector3 = PropertyDataViaCallable::new(
            node.callable("get_bone_pose_position").bind(&[bone_idx.to_variant()]),
            node.callable("set_bone_pose_position").bind(&[bone_idx.to_variant()]),
        )
        .into();

        SpireTween::<LerpPropertyData<Vector3>>::new(data_via_callable, Evaluator::Static(to), duration)
    }

    fn do_bone_scale(&self, bone_idx: i32, to: Vector3, duration: f64) -> SpireTween<LerpPropertyData<Vector3>> {
        let node = self.clone().upcast::<Skeleton3D>();

        let data_via_callable: PropertyDataVector3 = PropertyDataViaCallable::new(
            node.callable("get_bone_pose_scale").bind(&[bone_idx.to_variant()]),
            node.callable("set_bone_pose_scale").bind(&[bone_idx.to_variant()]),
        )
        .into();

        SpireTween::<LerpPropertyData<Vector3>>::new(data_via_callable, Evaluator::Static(to), duration)
    }
}
