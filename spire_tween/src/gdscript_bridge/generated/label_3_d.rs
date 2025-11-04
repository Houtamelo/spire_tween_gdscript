use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Label3D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoLabel3D {}
#[godot_api]
impl DoLabel3D {
    /**[b]Behavior: [/b]Tweens the property [member Label3D.font_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_size)]
    fn r#font_size(node: Gd<Label3D>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_font_size(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.outline_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_size)]
    fn r#outline_size(
        node: Gd<Label3D>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_outline_size(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.line_spacing] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = line_spacing)]
    fn r#line_spacing(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_line_spacing(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member Label3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_r)]
    fn modulate_r(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_r(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_r].
    #[func(rename = color_r)]
    fn color_r(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_r(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member Label3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_g)]
    fn modulate_g(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_g(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_g].
    #[func(rename = color_g)]
    fn color_g(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_g(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member Label3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_b)]
    fn modulate_b(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_b(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_b].
    #[func(rename = color_b)]
    fn color_b(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_b(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member Label3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_a)]
    fn modulate_a(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_a(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_a].
    #[func(rename = color_a)]
    fn color_a(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_a(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Label3D.offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = offset_x)]
    fn offset_x(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_offset_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Label3D.offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = offset_y)]
    fn offset_y(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_offset_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member Label3D.outline_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_modulate_r)]
    fn outline_modulate_r(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_outline_modulate_r(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method outline_modulate_r].
    #[func(rename = outline_color_r)]
    fn outline_color_r(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::outline_modulate_r(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member Label3D.outline_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_modulate_g)]
    fn outline_modulate_g(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_outline_modulate_g(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method outline_modulate_g].
    #[func(rename = outline_color_g)]
    fn outline_color_g(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::outline_modulate_g(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member Label3D.outline_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_modulate_b)]
    fn outline_modulate_b(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_outline_modulate_b(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method outline_modulate_b].
    #[func(rename = outline_color_b)]
    fn outline_color_b(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::outline_modulate_b(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member Label3D.outline_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_modulate_a)]
    fn outline_modulate_a(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_outline_modulate_a(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method outline_modulate_a].
    #[func(rename = outline_color_a)]
    fn outline_color_a(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::outline_modulate_a(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.pixel_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = pixel_size)]
    fn r#pixel_size(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_pixel_size(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = offset)]
    fn r#offset(
        node: Gd<Label3D>,
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
    /**[b]Behavior: [/b]Tweens the property [member Label3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate)]
    fn r#modulate(
        node: Gd<Label3D>,
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
    fn r#color(node: Gd<Label3D>, to: Color, duration: f64) -> Gd<SpirePropertyColor> {
        Self::r#modulate(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.outline_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_modulate)]
    fn r#outline_modulate(
        node: Gd<Label3D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let inner = UnsafeCell::new(node.do_outline_modulate(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyColor { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method outline_modulate].
    #[func(rename = outline_color)]
    fn r#outline_color(
        node: Gd<Label3D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        Self::r#outline_modulate(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.text] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = text)]
    fn r#text(node: Gd<Label3D>, to: GString, duration: f64) -> Gd<SpirePropertyString> {
        let inner = UnsafeCell::new(node.do_label3d_do_text(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyString { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
