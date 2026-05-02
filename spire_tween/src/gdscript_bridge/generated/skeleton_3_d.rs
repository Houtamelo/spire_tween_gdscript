use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Skeleton3D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoSkeleton3D {}
#[godot_api]
impl DoSkeleton3D {
    /**[b]Behavior: [/b]Tweens the property [member Skeleton3D.motion_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = motion_scale)]
    fn r#motion_scale(
        node: Gd<Skeleton3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_motion_scale(to, duration).register();
        gd_from_native_tween(tween)
    }
    #[func]
    fn bone_position(
        node: Gd<Skeleton3D>,
        bone_idx: i32,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_bone_position(bone_idx, to, duration).register();
        gd_from_native_tween(tween)
    }
    #[func]
    fn bone_scale(
        node: Gd<Skeleton3D>,
        bone_idx: i32,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_bone_scale(bone_idx, to, duration).register();
        gd_from_native_tween(tween)
    }
}
