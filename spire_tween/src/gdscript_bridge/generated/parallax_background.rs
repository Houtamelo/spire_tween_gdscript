use super::*;
/**This class provides shortcut constructors to create tweens that animate a [ParallaxBackground].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoParallaxBackground {}
#[godot_api]
impl DoParallaxBackground {
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member ParallaxBackground.scroll_base_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_base_offset_x)]
    fn scroll_base_offset_x(
        node: Gd<ParallaxBackground>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_scroll_base_offset_x(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member ParallaxBackground.scroll_base_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_base_offset_y)]
    fn scroll_base_offset_y(
        node: Gd<ParallaxBackground>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_scroll_base_offset_y(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member ParallaxBackground.scroll_base_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_base_scale_x)]
    fn scroll_base_scale_x(
        node: Gd<ParallaxBackground>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_scroll_base_scale_x(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member ParallaxBackground.scroll_base_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_base_scale_y)]
    fn scroll_base_scale_y(
        node: Gd<ParallaxBackground>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_scroll_base_scale_y(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member ParallaxBackground.scroll_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_offset_x)]
    fn scroll_offset_x(
        node: Gd<ParallaxBackground>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_scroll_offset_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member ParallaxBackground.scroll_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_offset_y)]
    fn scroll_offset_y(
        node: Gd<ParallaxBackground>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_scroll_offset_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member ParallaxBackground.scroll_base_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_base_offset)]
    fn r#scroll_base_offset(
        node: Gd<ParallaxBackground>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let inner = UnsafeCell::new(node.do_scroll_base_offset(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector2 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member ParallaxBackground.scroll_base_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_base_scale)]
    fn r#scroll_base_scale(
        node: Gd<ParallaxBackground>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let inner = UnsafeCell::new(node.do_scroll_base_scale(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector2 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member ParallaxBackground.scroll_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_offset)]
    fn r#scroll_offset(
        node: Gd<ParallaxBackground>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let inner = UnsafeCell::new(node.do_scroll_offset(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector2 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
