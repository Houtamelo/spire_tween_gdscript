use super::*;
/**This class provides shortcut constructors to create tweens that animate a [ReflectionProbe].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoReflectionProbe {}
#[godot_api]
impl DoReflectionProbe {
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member ReflectionProbe.ambient_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = ambient_color_r)]
    fn ambient_color_r(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_ambient_color_r(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member ReflectionProbe.ambient_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = ambient_color_g)]
    fn ambient_color_g(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_ambient_color_g(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member ReflectionProbe.ambient_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = ambient_color_b)]
    fn ambient_color_b(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_ambient_color_b(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member ReflectionProbe.ambient_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = ambient_color_a)]
    fn ambient_color_a(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_ambient_color_a(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member ReflectionProbe.ambient_color_energy] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = ambient_color_energy)]
    fn r#ambient_color_energy(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(
            node.do_ambient_color_energy(to, duration).register(),
        );
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member ReflectionProbe.blend_distance] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = blend_distance)]
    fn r#blend_distance(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_blend_distance(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member ReflectionProbe.intensity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = intensity)]
    fn r#intensity(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_intensity(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member ReflectionProbe.max_distance] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = max_distance)]
    fn r#max_distance(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_max_distance(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member ReflectionProbe.origin_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = origin_offset_x)]
    fn origin_offset_x(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_origin_offset_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member ReflectionProbe.origin_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = origin_offset_y)]
    fn origin_offset_y(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_origin_offset_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member ReflectionProbe.origin_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = origin_offset_z)]
    fn origin_offset_z(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_origin_offset_z(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member ReflectionProbe.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_x)]
    fn size_x(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_size_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member ReflectionProbe.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_y)]
    fn size_y(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_size_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member ReflectionProbe.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_z)]
    fn size_z(
        node: Gd<ReflectionProbe>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_size_z(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member ReflectionProbe.origin_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = origin_offset)]
    fn r#origin_offset(
        node: Gd<ReflectionProbe>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(node.do_origin_offset(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector3 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member ReflectionProbe.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size)]
    fn r#size(
        node: Gd<ReflectionProbe>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(node.do_size(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector3 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member ReflectionProbe.ambient_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = ambient_color)]
    fn r#ambient_color(
        node: Gd<ReflectionProbe>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let inner = UnsafeCell::new(node.do_ambient_color(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyColor { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
