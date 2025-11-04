use super::*;
/**This class provides shortcut constructors to create tweens that animate a [StaticBody3D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoStaticBody3D {}
#[godot_api]
impl DoStaticBody3D {
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member StaticBody3D.constant_angular_velocity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_angular_velocity_x)]
    fn constant_angular_velocity_x(
        node: Gd<StaticBody3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_constant_angular_velocity_x(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member StaticBody3D.constant_angular_velocity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_angular_velocity_y)]
    fn constant_angular_velocity_y(
        node: Gd<StaticBody3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_constant_angular_velocity_y(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member StaticBody3D.constant_angular_velocity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_angular_velocity_z)]
    fn constant_angular_velocity_z(
        node: Gd<StaticBody3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_constant_angular_velocity_z(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member StaticBody3D.constant_linear_velocity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_linear_velocity_x)]
    fn constant_linear_velocity_x(
        node: Gd<StaticBody3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_constant_linear_velocity_x(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member StaticBody3D.constant_linear_velocity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_linear_velocity_y)]
    fn constant_linear_velocity_y(
        node: Gd<StaticBody3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_constant_linear_velocity_y(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member StaticBody3D.constant_linear_velocity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_linear_velocity_z)]
    fn constant_linear_velocity_z(
        node: Gd<StaticBody3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_constant_linear_velocity_z(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member StaticBody3D.constant_angular_velocity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_angular_velocity)]
    fn r#constant_angular_velocity(
        node: Gd<StaticBody3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(
            node.do_constant_angular_velocity(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyVector3 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member StaticBody3D.constant_linear_velocity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_linear_velocity)]
    fn r#constant_linear_velocity(
        node: Gd<StaticBody3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(
            node.do_constant_linear_velocity(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyVector3 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
