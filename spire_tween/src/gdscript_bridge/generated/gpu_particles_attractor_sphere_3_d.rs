use super::*;
/**This class provides shortcut constructors to create tweens that animate a [GpuParticlesAttractorSphere3D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoGpuParticlesAttractorSphere3D {}
#[godot_api]
impl DoGpuParticlesAttractorSphere3D {
    /**[b]Behavior: [/b]Tweens the property [member GpuParticlesAttractorSphere3D.radius] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = radius)]
    fn r#radius(
        node: Gd<GpuParticlesAttractorSphere3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_radius(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
