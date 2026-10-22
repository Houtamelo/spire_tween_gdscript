use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Line2D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoLine2D {}
#[godot_api]
impl DoLine2D {
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member Line2D.default_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = default_color_r)]
    fn default_color_r(
        node: Gd<Line2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_default_color_r(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member Line2D.default_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = default_color_g)]
    fn default_color_g(
        node: Gd<Line2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_default_color_g(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member Line2D.default_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = default_color_b)]
    fn default_color_b(
        node: Gd<Line2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_default_color_b(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member Line2D.default_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = default_color_a)]
    fn default_color_a(
        node: Gd<Line2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_default_color_a(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Line2D.width] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = width)]
    fn r#width(node: Gd<Line2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_width(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Line2D.default_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = default_color)]
    fn r#default_color(
        node: Gd<Line2D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let tween = node.do_default_color(to, duration).register();
        gd_from_native_tween(tween)
    }
    /// Animates gradient endpoints, preserving interior stops and interpolation.
    /// Copies the gradient on first processing to preserve other users; starts
    /// from default_color when none exists. The private copy remains assigned.
    #[func]
    fn endpoint_colors(
        node: Gd<Line2D>,
        start: Color,
        end: Color,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_endpoint_colors(start, end, duration).register())
    }
}
