use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Sprite2D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoSprite2D {}
#[godot_api]
impl DoSprite2D {
    /**[b]Behavior: [/b]Tweens the property [member Sprite2D.frame] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = frame)]
    fn r#frame(node: Gd<Sprite2D>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let tween = node.do_frame(to, duration).register();
        gd_from_native_tween(tween)
    }
}
