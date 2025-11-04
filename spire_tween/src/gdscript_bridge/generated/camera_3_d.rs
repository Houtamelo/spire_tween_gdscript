use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Camera3D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoCamera3D {}
#[godot_api]
impl DoCamera3D {
    /**[b]Behavior: [/b]Tweens the property [member Camera3D.fov] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = fov)]
    fn r#fov(node: Gd<Camera3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_fov(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Camera3D.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size)]
    fn r#size(node: Gd<Camera3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_size(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Camera3D.frustum_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = frustum_offset_x)]
    fn frustum_offset_x(
        node: Gd<Camera3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_frustum_offset_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Camera3D.frustum_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = frustum_offset_y)]
    fn frustum_offset_y(
        node: Gd<Camera3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_frustum_offset_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Camera3D.h_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = h_offset)]
    fn r#h_offset(node: Gd<Camera3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_h_offset(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Camera3D.v_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = v_offset)]
    fn r#v_offset(node: Gd<Camera3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_v_offset(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Camera3D.frustum_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = frustum_offset)]
    fn r#frustum_offset(
        node: Gd<Camera3D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let inner = UnsafeCell::new(node.do_frustum_offset(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector2 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
