use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Node3D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoNode3D {}
#[godot_api]
impl DoNode3D {
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node3D.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position_x)]
    fn position_x(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_position_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method position_x].
    #[func(rename = move_local_x)]
    fn move_local_x(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::position_x(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position_y)]
    fn position_y(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_position_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method position_y].
    #[func(rename = move_local_y)]
    fn move_local_y(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::position_y(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position_z)]
    fn position_z(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_position_z(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method position_z].
    #[func(rename = move_local_z)]
    fn move_local_z(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::position_z(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node3D.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position_x)]
    fn global_position_x(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_global_position_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method global_position_x].
    #[func(rename = move_x)]
    fn move_x(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::global_position_x(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position_y)]
    fn global_position_y(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_global_position_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method global_position_y].
    #[func(rename = move_y)]
    fn move_y(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::global_position_y(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position_z)]
    fn global_position_z(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_global_position_z(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method global_position_z].
    #[func(rename = move_z)]
    fn move_z(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::global_position_z(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node3D.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale_x)]
    fn scale_x(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_scale_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale_y)]
    fn scale_y(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_scale_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale_z)]
    fn scale_z(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_scale_z(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node3D.rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_x)]
    fn rotation_x(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_rotation_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_y)]
    fn rotation_y(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_rotation_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_z)]
    fn rotation_z(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_rotation_z(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node3D.rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_degrees_x)]
    fn rotation_degrees_x(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_rotation_degrees_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_degrees_y)]
    fn rotation_degrees_y(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_rotation_degrees_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_degrees_z)]
    fn rotation_degrees_z(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_rotation_degrees_z(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node3D.global_rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_x)]
    fn global_rotation_x(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_global_rotation_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.global_rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_y)]
    fn global_rotation_y(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_global_rotation_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.global_rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_z)]
    fn global_rotation_z(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_global_rotation_z(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node3D.global_rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_degrees_x)]
    fn global_rotation_degrees_x(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_global_rotation_degrees_x(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.global_rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_degrees_y)]
    fn global_rotation_degrees_y(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_global_rotation_degrees_y(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.global_rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_degrees_z)]
    fn global_rotation_degrees_z(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_global_rotation_degrees_z(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position)]
    fn r#position(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(node.do_position(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector3 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method position].
    #[func(rename = move_local)]
    fn r#move_local(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        Self::r#position(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position)]
    fn r#global_position(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(node.do_global_position(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector3 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method global_position].
    #[func(rename = move)]
    fn r#move(node: Gd<Node3D>, to: Vector3, duration: f64) -> Gd<SpirePropertyVector3> {
        Self::r#global_position(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale)]
    fn r#scale(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(node.do_scale(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector3 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation)]
    fn r#rotation(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(node.do_rotation(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector3 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_degrees)]
    fn r#rotation_degrees(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(node.do_rotation_degrees(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector3 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.global_rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation)]
    fn r#global_rotation(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(node.do_global_rotation(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector3 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.global_rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_degrees)]
    fn r#global_rotation_degrees(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(
            node.do_global_rotation_degrees(to, duration).register(),
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
