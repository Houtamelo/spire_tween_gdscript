use super::*;
/**This class provides shortcut constructors to create tweens that animate a [SubViewport].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoSubViewport {}
#[godot_api]
impl DoSubViewport {
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member SubViewport.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_x)]
    fn size_x(node: Gd<SubViewport>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_subview_do_size_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member SubViewport.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_y)]
    fn size_y(node: Gd<SubViewport>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_subview_do_size_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member SubViewport.size_2d_override] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_2d_override_x)]
    fn size_2d_override_x(
        node: Gd<SubViewport>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(
            node.do_subview_do_size_2d_override_x(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member SubViewport.size_2d_override] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_2d_override_y)]
    fn size_2d_override_y(
        node: Gd<SubViewport>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(
            node.do_subview_do_size_2d_override_y(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member SubViewport.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size)]
    fn r#size(
        node: Gd<SubViewport>,
        to: Vector2i,
        duration: f64,
    ) -> Gd<SpirePropertyVector2i> {
        let inner = UnsafeCell::new(node.do_subview_do_size(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector2i {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member SubViewport.size_2d_override] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_2d_override)]
    fn r#size_2d_override(
        node: Gd<SubViewport>,
        to: Vector2i,
        duration: f64,
    ) -> Gd<SpirePropertyVector2i> {
        let inner = UnsafeCell::new(
            node.do_subview_do_size_2d_override(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyVector2i {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
