use super::*;
/**This class provides shortcut constructors to create tweens that animate a [BaseMaterial3D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoBaseMaterial3D {}
#[godot_api]
impl DoBaseMaterial3D {
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member BaseMaterial3D.albedo_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = albedo_color_r)]
    fn albedo_color_r(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_albedo_color_r(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member BaseMaterial3D.albedo_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = albedo_color_g)]
    fn albedo_color_g(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_albedo_color_g(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member BaseMaterial3D.albedo_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = albedo_color_b)]
    fn albedo_color_b(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_albedo_color_b(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member BaseMaterial3D.albedo_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = albedo_color_a)]
    fn albedo_color_a(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_albedo_color_a(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method albedo_color_a].
    #[func(rename = fade)]
    fn r#fade(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::albedo_color_a(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member BaseMaterial3D.uv1_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv1_offset_x)]
    fn uv1_offset_x(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv1_offset_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member BaseMaterial3D.uv1_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv1_offset_y)]
    fn uv1_offset_y(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv1_offset_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member BaseMaterial3D.uv1_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv1_offset_z)]
    fn uv1_offset_z(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv1_offset_z(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member BaseMaterial3D.uv1_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv1_scale_x)]
    fn uv1_scale_x(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv1_scale_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member BaseMaterial3D.uv1_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv1_scale_y)]
    fn uv1_scale_y(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv1_scale_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member BaseMaterial3D.uv1_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv1_scale_z)]
    fn uv1_scale_z(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv1_scale_z(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member BaseMaterial3D.uv2_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv2_offset_x)]
    fn uv2_offset_x(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv2_offset_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member BaseMaterial3D.uv2_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv2_offset_y)]
    fn uv2_offset_y(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv2_offset_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member BaseMaterial3D.uv2_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv2_offset_z)]
    fn uv2_offset_z(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv2_offset_z(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member BaseMaterial3D.uv2_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv2_scale_x)]
    fn uv2_scale_x(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv2_scale_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member BaseMaterial3D.uv2_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv2_scale_y)]
    fn uv2_scale_y(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv2_scale_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member BaseMaterial3D.uv2_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv2_scale_z)]
    fn uv2_scale_z(
        node: Gd<BaseMaterial3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_uv2_scale_z(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member BaseMaterial3D.uv1_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv1_offset)]
    fn r#uv1_offset(
        node: Gd<BaseMaterial3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_uv1_offset(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member BaseMaterial3D.uv1_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv1_scale)]
    fn r#uv1_scale(
        node: Gd<BaseMaterial3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_uv1_scale(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member BaseMaterial3D.uv2_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv2_offset)]
    fn r#uv2_offset(
        node: Gd<BaseMaterial3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_uv2_offset(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member BaseMaterial3D.uv2_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = uv2_scale)]
    fn r#uv2_scale(
        node: Gd<BaseMaterial3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_uv2_scale(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member BaseMaterial3D.albedo_color] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = albedo_color)]
    fn r#albedo_color(
        node: Gd<BaseMaterial3D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let tween = node.do_albedo_color(to, duration).register();
        gd_from_native_tween(tween)
    }
    /// Captures the current color at construction, then adds [param to] minus
    /// that baseline over time. Equivalent to a relative color property tween.
    #[func]
    fn blendable_albedo_color(
        node: Gd<BaseMaterial3D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        gd_from_native_tween(node.do_blendable_albedo_color(to, duration).register())
    }
    /// Samples the supplied Gradient with eased progress. Includes alpha by
    /// default; disabling it preserves the target's current alpha on each write.
    /// Uses the Gradient's interpolation mode/color space and samples it live.
    #[func]
    fn gradient_albedo_color(
        node: Gd<BaseMaterial3D>,
        gradient: Option<Gd<Gradient>>,
        duration: f64,
        #[opt(default = true)]
        include_alpha: bool,
    ) -> Option<Gd<SpireMethodFloat>> {
        let gradient = gradient.log_null_arg(|| "gradient")?;
        Some(
            gd_from_native_tween(
                node
                    .do_gradient_albedo_color(gradient, duration, include_alpha)
                    .register(),
            ),
        )
    }
}
