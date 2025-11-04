use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Decal].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoDecal {}
#[godot_api]
impl DoDecal {
    /**[b]Behavior: [/b]Tweens the property [member Decal.albedo_mix] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = albedo_mix)]
    fn r#albedo_mix(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_albedo_mix(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Decal.emission_energy] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = emission_energy)]
    fn r#emission_energy(
        node: Gd<Decal>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_emission_energy(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member Decal.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_r)]
    fn modulate_r(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_r(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_r].
    #[func(rename = color_r)]
    fn color_r(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_r(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member Decal.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_g)]
    fn modulate_g(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_g(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_g].
    #[func(rename = color_g)]
    fn color_g(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_g(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member Decal.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_b)]
    fn modulate_b(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_b(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_b].
    #[func(rename = color_b)]
    fn color_b(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_b(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member Decal.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_a)]
    fn modulate_a(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_modulate_a(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate_a].
    #[func(rename = color_a)]
    fn color_a(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_a(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Decal.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_x)]
    fn size_x(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_size_x(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Decal.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_y)]
    fn size_y(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_size_y(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Decal.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_z)]
    fn size_z(node: Gd<Decal>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_size_z(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Decal.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size)]
    fn r#size(node: Gd<Decal>, to: Vector3, duration: f64) -> Gd<SpirePropertyVector3> {
        let inner = UnsafeCell::new(node.do_size(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyVector3 {
            base,
            inner,
        });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member Decal.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate)]
    fn r#modulate(node: Gd<Decal>, to: Color, duration: f64) -> Gd<SpirePropertyColor> {
        let inner = UnsafeCell::new(node.do_modulate(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyColor { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    ///Alias for [method modulate].
    #[func(rename = color)]
    fn r#color(node: Gd<Decal>, to: Color, duration: f64) -> Gd<SpirePropertyColor> {
        Self::r#modulate(node, to, duration)
    }
}
