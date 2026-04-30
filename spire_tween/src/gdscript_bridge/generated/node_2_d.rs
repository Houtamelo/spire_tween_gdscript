use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Node2D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoNode2D {}
#[godot_api]
impl DoNode2D {
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node2D.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position_x)]
    fn position_x(node: Gd<Node2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_position_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method position_x].
    #[func(rename = move_local_x)]
    fn move_local_x(node: Gd<Node2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::position_x(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node2D.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position_y)]
    fn position_y(node: Gd<Node2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_position_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method position_y].
    #[func(rename = move_local_y)]
    fn move_local_y(node: Gd<Node2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::position_y(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node2D.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position_x)]
    fn global_position_x(
        node: Gd<Node2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_position_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method global_position_x].
    #[func(rename = move_x)]
    fn move_x(node: Gd<Node2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::global_position_x(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node2D.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position_y)]
    fn global_position_y(
        node: Gd<Node2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_position_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method global_position_y].
    #[func(rename = move_y)]
    fn move_y(node: Gd<Node2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::global_position_y(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node2D.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale_x)]
    fn scale_x(node: Gd<Node2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_scale_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node2D.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale_y)]
    fn scale_y(node: Gd<Node2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_scale_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node2D.global_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_scale_x)]
    fn global_scale_x(
        node: Gd<Node2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_scale_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node2D.global_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_scale_y)]
    fn global_scale_y(
        node: Gd<Node2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_scale_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node2D.rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation)]
    fn r#rotation(node: Gd<Node2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node2D.rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_degrees)]
    fn r#rotation_degrees(
        node: Gd<Node2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation_degrees(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node2D.global_rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation)]
    fn r#global_rotation(
        node: Gd<Node2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_rotation(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node2D.global_rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_degrees)]
    fn r#global_rotation_degrees(
        node: Gd<Node2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_rotation_degrees(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node2D.skew] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = skew)]
    fn r#skew(node: Gd<Node2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_skew(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node2D.global_skew] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_skew)]
    fn r#global_skew(
        node: Gd<Node2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_skew(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node2D.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position)]
    fn r#position(
        node: Gd<Node2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_position(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method position].
    #[func(rename = move_local)]
    fn r#move_local(
        node: Gd<Node2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        Self::r#position(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node2D.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position)]
    fn r#global_position(
        node: Gd<Node2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_global_position(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method global_position].
    #[func(rename = move)]
    fn r#move(node: Gd<Node2D>, to: Vector2, duration: f64) -> Gd<SpirePropertyVector2> {
        Self::r#global_position(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node2D.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale)]
    fn r#scale(
        node: Gd<Node2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_scale(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node2D.global_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_scale)]
    fn r#global_scale(
        node: Gd<Node2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_global_scale(to, duration).register();
        gd_from_native_tween(tween)
    }
    #[func]
    fn follow(
        node: Gd<Node2D>,
        follow_this: Gd<Node2D>,
        speed: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_follow(follow_this, speed).register();
        gd_from_native_tween(tween)
    }
    #[func]
    fn shake(
        node: Gd<Node2D>,
        radius_min: real,
        radius_max: real,
        vibratio: real,
        frequency: f64,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        let inner = node
            .do_shake(radius_min, radius_max, vibratio, frequency, duration)
            .register();
        gd_from_native_tween(inner)
    }
    #[func]
    fn ellipsis(
        node: Gd<Node2D>,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector2,
        to_radius: Vector2,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        let tween = node
            .do_ellipsis(center, from_angle, to_angle, from_radius, to_radius, duration)
            .register();
        gd_from_native_tween(tween)
    }
    #[func]
    fn circle(
        node: Gd<Node2D>,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        radius: f32,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        Self::ellipsis(
            node,
            center,
            from_angle,
            to_angle,
            Vector2::splat(radius),
            Vector2::splat(radius),
            duration,
        )
    }
    #[func]
    fn spiral(
        node: Gd<Node2D>,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        scale: Vector2,
        duration: f64,
        rotation: f32,
        shear: f32,
        mode: Spiral,
        log_growth: Vector2,
    ) -> Gd<SpireMethodFloat> {
        let inner = node
            .do_spiral(
                center,
                from_angle,
                to_angle,
                scale,
                duration,
                rotation,
                shear,
                mode,
                log_growth,
            )
            .register();
        gd_from_native_tween(inner)
    }
    #[func]
    fn contour_shape(
        node: Gd<Node2D>,
        vertices: Array<Vector2>,
        duration_or_speed: f64,
        is_speed_based: bool,
    ) -> Gd<SpireSequence> {
        let tween = node
            .do_contour_shape(vertices, duration_or_speed, is_speed_based)
            .register();
        gd_from_native_tween(tween)
    }
}
