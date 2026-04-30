use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Range].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoRange {}
#[godot_api]
impl DoRange {
    /**[b]Behavior: [/b]Tweens the property [member Range.value] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = value)]
    fn r#value(node: Gd<Range>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_value(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Range.ratio] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = ratio)]
    fn r#ratio(node: Gd<Range>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_ratio(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Range.min_value] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = min_value)]
    fn r#min_value(node: Gd<Range>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_min_value(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Range.max_value] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = max_value)]
    fn r#max_value(node: Gd<Range>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_max_value(to, duration).register();
        gd_from_native_tween(tween)
    }
}
