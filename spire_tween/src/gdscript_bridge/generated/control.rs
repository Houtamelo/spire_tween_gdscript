use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Control].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoControl {}
#[godot_api]
impl DoControl {
    /**[b]Behavior: [/b]Tweens the property [member Control.anchor_bottom] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = anchor_bottom)]
    fn r#anchor_bottom(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_anchor_bottom(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.anchor_left] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = anchor_left)]
    fn r#anchor_left(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_anchor_left(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.anchor_right] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = anchor_right)]
    fn r#anchor_right(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_anchor_right(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.anchor_top] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = anchor_top)]
    fn r#anchor_top(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_anchor_top(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation)]
    fn r#rotation(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_degrees)]
    fn r#rotation_degrees(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation_degrees(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.custom_minimum_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = custom_minimum_size_x)]
    fn custom_minimum_size_x(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_custom_minimum_size_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.custom_minimum_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = custom_minimum_size_y)]
    fn custom_minimum_size_y(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_custom_minimum_size_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position_x)]
    fn position_x(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_position_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method position_x].
    #[func(rename = move_local_x)]
    fn move_local_x(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::position_x(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position_y)]
    fn position_y(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_position_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method position_y].
    #[func(rename = move_local_y)]
    fn move_local_y(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::position_y(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position_x)]
    fn global_position_x(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_position_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method global_position_x].
    #[func(rename = move_x)]
    fn move_x(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::global_position_x(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position_y)]
    fn global_position_y(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_position_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method global_position_y].
    #[func(rename = move_y)]
    fn move_y(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::global_position_y(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.pivot_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = pivot_offset_x)]
    fn pivot_offset_x(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_pivot_offset_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.pivot_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = pivot_offset_y)]
    fn pivot_offset_y(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_pivot_offset_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale_x)]
    fn scale_x(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_scale_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale_y)]
    fn scale_y(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_scale_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_x)]
    fn size_x(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_size_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_y)]
    fn size_y(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_size_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.custom_minimum_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = custom_minimum_size)]
    fn r#custom_minimum_size(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVec2> {
        let tween = node.do_custom_minimum_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position)]
    fn r#position(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVec2> {
        let tween = node.do_position(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method position].
    #[func(rename = move_local)]
    fn r#move_local(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVec2> {
        Self::r#position(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position)]
    fn r#global_position(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVec2> {
        let tween = node.do_global_position(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method global_position].
    #[func(rename = move)]
    fn r#move(node: Gd<Control>, to: Vector2, duration: f64) -> Gd<SpirePropertyVec2> {
        Self::r#global_position(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.pivot_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = pivot_offset)]
    fn r#pivot_offset(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVec2> {
        let tween = node.do_pivot_offset(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale)]
    fn r#scale(node: Gd<Control>, to: Vector2, duration: f64) -> Gd<SpirePropertyVec2> {
        let tween = node.do_scale(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size)]
    fn r#size(node: Gd<Control>, to: Vector2, duration: f64) -> Gd<SpirePropertyVec2> {
        let tween = node.do_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /// Interpolates local-position shake waypoints using DOTween's sampling math.
    /// [param strength] shapes each axis; its length is the initial waypoint magnitude.
    /// [param vibrato] times [param duration] gives the segment count, clamped to 2-65536.
    /// Nonpositive vibrato and counts above 65536 emit warnings. Short animations
    /// with positive vibrato use the minimum of two without a warning.
    /// [param randomness] is in degrees, from 0 to 180. [param fade_out] decreases
    /// magnitude and increases segment durations; otherwise durations are equal.
    /// [param blendable] replaces this tween's previous offset on the current position;
    /// disabling it writes relative to the start captured on first processing.
    /// Absolute property writers can disrupt blendable offset bookkeeping.
    /// Generates one path per playback: Restart/Incremental repeat it without drift;
    /// Yoyo retraces it. Replay regenerates; resume does not. The method value is
    /// sampled modulo one, so easing overshoot wraps. Keep its range at 0 to 1.
    /// Stop leaves position untouched. Natural/forced completion restores zero offset
    /// before finished handlers, but not necessarily before stopped handlers.
    /// Freed owners cancel. Invalid parameters emit warnings:
    /// nonpositive/nonfinite duration becomes 1 second; negative/nonfinite strength
    /// components become zero; finite randomness is clamped to 0-180 degrees,
    /// and nonfinite randomness becomes 90 degrees. Returns an already registered method tween.
    #[func]
    fn shake(
        node: Gd<Control>,
        duration: f64,
        strength: Vector2,
        #[opt(default = 10)]
        vibrato: i32,
        #[opt(default = 90.0)]
        randomness: real,
        #[opt(default = true)]
        fade_out: bool,
        #[opt(default = ShakeRandomnessMode::Full)]
        randomness_mode: ShakeRandomnessMode,
        #[opt(default = true)]
        blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        let tween = node
            .do_shake(
                duration,
                strength,
                vibrato,
                randomness,
                fade_out,
                randomness_mode,
                blendable,
            );
        gd_from_native_tween(tween.register())
    }
    /// Expands [param to] to a uniform absolute scale. Already registered.
    #[func]
    fn uniform_scale(
        node: Gd<Control>,
        to: real,
        duration: f64,
    ) -> Gd<SpirePropertyVec2> {
        gd_from_native_tween(node.do_uniform_scale(to, duration).register())
    }
    ///Adds a local-position delta, blending with other relative tweens.
    #[func]
    fn blendable_position(
        node: Gd<Control>,
        by: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVec2> {
        gd_from_native_tween(node.do_blendable_position(by, duration).register())
    }
    ///Adds a global-position delta, blending with other relative tweens.
    #[func]
    fn blendable_global_position(
        node: Gd<Control>,
        by: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVec2> {
        gd_from_native_tween(node.do_blendable_global_position(by, duration).register())
    }
    ///Adds a scale delta; this is additive, not multiplicative.
    #[func]
    fn blendable_scale(
        node: Gd<Control>,
        by: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVec2> {
        gd_from_native_tween(node.do_blendable_scale(by, duration).register())
    }
    /// Jumps to a local position ([method global_jump] uses global coordinates).
    /// Captures the start on first processing. Horizontal motion is linear;
    /// vertical displacement uses OutQuad with repeated OutQuad/Yoyo hops.
    /// Positive height is upward: -Y in 2D/UI, +Y in 3D. Jump counts below
    /// one become one. Easing remaps overall progress. Supports Restart/Yoyo,
    /// not Incremental. Directly sets position; does not perform physics movement.
    #[func]
    fn jump(
        node: Gd<Control>,
        to: Vector2,
        height: f64,
        jumps: i32,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_jump(to, height, jumps, duration).register())
    }
    /// Jumps to a local position ([method global_jump] uses global coordinates).
    /// Captures the start on first processing. Horizontal motion is linear;
    /// vertical displacement uses OutQuad with repeated OutQuad/Yoyo hops.
    /// Positive height is upward: -Y in 2D/UI, +Y in 3D. Jump counts below
    /// one become one. Easing remaps overall progress. Supports Restart/Yoyo,
    /// not Incremental. Directly sets position; does not perform physics movement.
    #[func]
    fn global_jump(
        node: Gd<Control>,
        to: Vector2,
        height: f64,
        jumps: i32,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_global_jump(to, height, jumps, duration).register())
    }
    /// Tweens the paired normalized anchors in parallel: min = left/top,
    /// max = right/bottom. Preserves the scalar anchor setters' offset behavior.
    #[func]
    fn anchor_min(node: Gd<Control>, to: Vector2, duration: f64) -> Gd<SpireSequence> {
        gd_from_native_tween(node.do_anchor_min(to, duration).register())
    }
    /// Tweens the paired normalized anchors in parallel: min = left/top,
    /// max = right/bottom. Preserves the scalar anchor setters' offset behavior.
    #[func]
    fn anchor_max(node: Gd<Control>, to: Vector2, duration: f64) -> Gd<SpireSequence> {
        gd_from_native_tween(node.do_anchor_max(to, duration).register())
    }
    /// Elliptical motion with angles in radians; positive rotation is clockwise.
    /// Uses local Control coordinates; the global variant uses global coordinates.
    /// Interpolates start/end radii. Container layout may reposition the Control.
    #[func]
    fn ellipsis(
        node: Gd<Control>,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector2,
        to_radius: Vector2,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node
                .do_ellipsis(
                    center,
                    from_angle,
                    to_angle,
                    from_radius,
                    to_radius,
                    duration,
                )
                .register(),
        )
    }
    /// Elliptical motion with angles in radians; positive rotation is clockwise.
    /// Uses local Control coordinates; the global variant uses global coordinates.
    /// Interpolates start/end radii. Container layout may reposition the Control.
    #[func]
    fn global_ellipsis(
        node: Gd<Control>,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector2,
        to_radius: Vector2,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node
                .do_global_ellipsis(
                    center,
                    from_angle,
                    to_angle,
                    from_radius,
                    to_radius,
                    duration,
                )
                .register(),
        )
    }
    /// Circular counterpart of the ellipse helper. Radius is constant; angles
    /// are radians. Uses local coordinates; global_circle uses global coordinates.
    #[func]
    fn circle(
        node: Gd<Control>,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        radius: f32,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node.do_circle(center, from_angle, to_angle, radius, duration).register(),
        )
    }
    /// Circular counterpart of the ellipse helper. Radius is constant; angles
    /// are radians. Uses local coordinates; global_circle uses global coordinates.
    #[func]
    fn global_circle(
        node: Gd<Control>,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        radius: f32,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node
                .do_global_circle(center, from_angle, to_angle, radius, duration)
                .register(),
        )
    }
}
