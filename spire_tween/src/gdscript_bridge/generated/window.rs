use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Window].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoWindow {}
#[godot_api]
impl DoWindow {
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Window.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position_x)]
    fn position_x(node: Gd<Window>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_position_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Window.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position_y)]
    fn position_y(node: Gd<Window>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_position_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Window.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_x)]
    fn size_x(node: Gd<Window>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_size_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Window.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_y)]
    fn size_y(node: Gd<Window>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_size_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Window.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position)]
    fn r#position(
        node: Gd<Window>,
        to: Vector2i,
        duration: f64,
    ) -> Gd<SpirePropertyVector2i> {
        let inner = UnsafeCell::new(node.do_position(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector2i {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Window.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size)]
    fn r#size(
        node: Gd<Window>,
        to: Vector2i,
        duration: f64,
    ) -> Gd<SpirePropertyVector2i> {
        let inner = UnsafeCell::new(node.do_size(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector2i {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
