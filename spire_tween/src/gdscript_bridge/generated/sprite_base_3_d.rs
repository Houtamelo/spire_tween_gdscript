use super::*;
/**This class provides shortcut constructors to create tweens that animate a [SpriteBase3D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoSpriteBase3D {}
#[godot_api]
impl DoSpriteBase3D {
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member SpriteBase3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_r)]
    fn modulate_r(
        node: Gd<SpriteBase3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_r(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_r].
    #[func(rename = color_r)]
    fn color_r(
        node: Gd<SpriteBase3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::modulate_r(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member SpriteBase3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_g)]
    fn modulate_g(
        node: Gd<SpriteBase3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_g(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_g].
    #[func(rename = color_g)]
    fn color_g(
        node: Gd<SpriteBase3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::modulate_g(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member SpriteBase3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_b)]
    fn modulate_b(
        node: Gd<SpriteBase3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_b(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_b].
    #[func(rename = color_b)]
    fn color_b(
        node: Gd<SpriteBase3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::modulate_b(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member SpriteBase3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_a)]
    fn modulate_a(
        node: Gd<SpriteBase3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_a(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_a].
    #[func(rename = color_a)]
    fn color_a(
        node: Gd<SpriteBase3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::modulate_a(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member SpriteBase3D.offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = offset_x)]
    fn offset_x(
        node: Gd<SpriteBase3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_offset_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member SpriteBase3D.offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = offset_y)]
    fn offset_y(
        node: Gd<SpriteBase3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_offset_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member SpriteBase3D.pixel_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = pixel_size)]
    fn r#pixel_size(
        node: Gd<SpriteBase3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_pixel_size(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member SpriteBase3D.offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = offset)]
    fn r#offset(
        node: Gd<SpriteBase3D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let inner = UnsafeCell::new(node.do_offset(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector2 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member SpriteBase3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate)]
    fn r#modulate(
        node: Gd<SpriteBase3D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let inner = UnsafeCell::new(node.do_modulate(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyColor { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate].
    #[func(rename = color)]
    fn r#color(
        node: Gd<SpriteBase3D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        Self::r#modulate(node, to, duration)
    }
}
