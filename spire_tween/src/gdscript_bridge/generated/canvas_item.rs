use super::*;
/**This class provides shortcut constructors to create tweens that animate a [CanvasItem].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoCanvasItem {}
#[godot_api]
impl DoCanvasItem {
    /**[b]Behavior: [/b]Tweens the property [member CanvasItem.z_index] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = z_index)]
    fn r#z_index(node: Gd<CanvasItem>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_z_index(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member CanvasItem.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_r)]
    fn modulate_r(
        node: Gd<CanvasItem>,
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
    fn color_r(node: Gd<CanvasItem>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_r(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member CanvasItem.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_g)]
    fn modulate_g(
        node: Gd<CanvasItem>,
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
    fn color_g(node: Gd<CanvasItem>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_g(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member CanvasItem.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_b)]
    fn modulate_b(
        node: Gd<CanvasItem>,
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
    fn color_b(node: Gd<CanvasItem>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_b(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member CanvasItem.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_a)]
    fn modulate_a(
        node: Gd<CanvasItem>,
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
    fn color_a(node: Gd<CanvasItem>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_a(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member CanvasItem.self_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = self_modulate_r)]
    fn self_modulate_r(
        node: Gd<CanvasItem>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_self_modulate_r(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method self_modulate_r].
    #[func(rename = self_color_r)]
    fn self_color_r(
        node: Gd<CanvasItem>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::self_modulate_r(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member CanvasItem.self_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = self_modulate_g)]
    fn self_modulate_g(
        node: Gd<CanvasItem>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_self_modulate_g(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method self_modulate_g].
    #[func(rename = self_color_g)]
    fn self_color_g(
        node: Gd<CanvasItem>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::self_modulate_g(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member CanvasItem.self_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = self_modulate_b)]
    fn self_modulate_b(
        node: Gd<CanvasItem>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_self_modulate_b(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method self_modulate_b].
    #[func(rename = self_color_b)]
    fn self_color_b(
        node: Gd<CanvasItem>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::self_modulate_b(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member CanvasItem.self_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = self_modulate_a)]
    fn self_modulate_a(
        node: Gd<CanvasItem>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_self_modulate_a(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method self_modulate_a].
    #[func(rename = self_color_a)]
    fn self_color_a(
        node: Gd<CanvasItem>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::self_modulate_a(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member CanvasItem.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate)]
    fn r#modulate(
        node: Gd<CanvasItem>,
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
        node: Gd<CanvasItem>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        Self::r#modulate(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member CanvasItem.self_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = self_modulate)]
    fn r#self_modulate(
        node: Gd<CanvasItem>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let inner = UnsafeCell::new(node.do_self_modulate(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyColor { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method self_modulate].
    #[func(rename = self_color)]
    fn r#self_color(
        node: Gd<CanvasItem>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        Self::r#self_modulate(node, to, duration)
    }
}
