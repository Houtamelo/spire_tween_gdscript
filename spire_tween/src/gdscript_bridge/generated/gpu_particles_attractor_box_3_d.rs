use super::*;
/**This class provides shortcut constructors to create tweens that animate a [GpuParticlesAttractorBox3D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoGpuParticlesAttractorBox3D {}
#[godot_api]
impl DoGpuParticlesAttractorBox3D {
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member GpuParticlesAttractorBox3D.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_x)]
    fn size_x(
        node: Gd<GpuParticlesAttractorBox3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_size_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member GpuParticlesAttractorBox3D.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_y)]
    fn size_y(
        node: Gd<GpuParticlesAttractorBox3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_size_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member GpuParticlesAttractorBox3D.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_z)]
    fn size_z(
        node: Gd<GpuParticlesAttractorBox3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_size_z(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member GpuParticlesAttractorBox3D.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size)]
    fn r#size(
        node: Gd<GpuParticlesAttractorBox3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_size(to, duration).register();
        gd_from_native_tween(tween)
    }
}
