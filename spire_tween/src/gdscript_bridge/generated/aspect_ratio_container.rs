use super::*;
/**This class provides shortcut constructors to create tweens that animate a [AspectRatioContainer].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoAspectRatioContainer {}
#[godot_api]
impl DoAspectRatioContainer {
    /**[b]Behavior: [/b]Tweens the property [member AspectRatioContainer.ratio] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = ratio)]
    fn r#ratio(
        node: Gd<AspectRatioContainer>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_container_ratio(to, duration).register();
        gd_from_native_tween(tween)
    }
}
