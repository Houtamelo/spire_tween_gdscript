use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Light3D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoLight3D {}
#[godot_api]
impl DoLight3D {
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member Light3D.light_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = light_color_r)]
    fn light_color_r(
        node: Gd<Light3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_light_color_r(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method light_color_r].
    #[func(rename = color_r)]
    fn color_r(node: Gd<Light3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::light_color_r(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member Light3D.light_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = light_color_g)]
    fn light_color_g(
        node: Gd<Light3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_light_color_g(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method light_color_g].
    #[func(rename = color_g)]
    fn color_g(node: Gd<Light3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::light_color_g(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member Light3D.light_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = light_color_b)]
    fn light_color_b(
        node: Gd<Light3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_light_color_b(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method light_color_b].
    #[func(rename = color_b)]
    fn color_b(node: Gd<Light3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::light_color_b(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member Light3D.light_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = light_color_a)]
    fn light_color_a(
        node: Gd<Light3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_light_color_a(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method light_color_a].
    #[func(rename = color_a)]
    fn color_a(node: Gd<Light3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::light_color_a(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Light3D.light_temperature] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = light_temperature)]
    fn r#light_temperature(
        node: Gd<Light3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_light_temperature(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Light3D.light_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = light_color)]
    fn r#light_color(
        node: Gd<Light3D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let inner = UnsafeCell::new(node.do_light_color(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyColor { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method light_color].
    #[func(rename = color)]
    fn r#color(node: Gd<Light3D>, to: Color, duration: f64) -> Gd<SpirePropertyColor> {
        Self::r#light_color(node, to, duration)
    }
}
