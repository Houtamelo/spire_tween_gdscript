use super::*;
/**This class provides shortcut constructors to create tweens that animate a [AnimatedSprite2D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoAnimatedSprite2D {}
#[godot_api]
impl DoAnimatedSprite2D {
    /**[b]Behavior: [/b]Tweens the property [member AnimatedSprite2D.frame] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = frame)]
    fn r#frame(
        node: Gd<AnimatedSprite2D>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_frame(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member AnimatedSprite2D.speed_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = speed_scale)]
    fn r#speed_scale(
        node: Gd<AnimatedSprite2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_speed_scale(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
