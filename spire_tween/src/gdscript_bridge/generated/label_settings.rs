use super::*;
/**This class provides shortcut constructors to create tweens that animate a [LabelSettings].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoLabelSettings {}
#[godot_api]
impl DoLabelSettings {
    /**[b]Behavior: [/b]Tweens the property [member LabelSettings.font_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_size)]
    fn r#font_size(
        node: Gd<LabelSettings>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_font_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member LabelSettings.outline_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_size)]
    fn r#outline_size(
        node: Gd<LabelSettings>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_outline_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member LabelSettings.font_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_color_r)]
    fn font_color_r(
        node: Gd<LabelSettings>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_font_color_r(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member LabelSettings.font_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_color_g)]
    fn font_color_g(
        node: Gd<LabelSettings>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_font_color_g(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member LabelSettings.font_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_color_b)]
    fn font_color_b(
        node: Gd<LabelSettings>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_font_color_b(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member LabelSettings.font_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_color_a)]
    fn font_color_a(
        node: Gd<LabelSettings>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_font_color_a(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member LabelSettings.outline_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_r)]
    fn outline_color_r(
        node: Gd<LabelSettings>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_r(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member LabelSettings.outline_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_g)]
    fn outline_color_g(
        node: Gd<LabelSettings>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_g(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member LabelSettings.outline_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_b)]
    fn outline_color_b(
        node: Gd<LabelSettings>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_b(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member LabelSettings.outline_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_a)]
    fn outline_color_a(
        node: Gd<LabelSettings>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_a(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member LabelSettings.font_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_color)]
    fn r#font_color(
        node: Gd<LabelSettings>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let tween = node.do_font_color(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member LabelSettings.outline_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color)]
    fn r#outline_color(
        node: Gd<LabelSettings>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let tween = node.do_outline_color(to, duration).register();
        gd_from_native_tween(tween)
    }
}
