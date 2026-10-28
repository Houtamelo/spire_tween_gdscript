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
        let tween = node.do_position_x(to, duration).register();
        gd_from_native_tween(tween)
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
        let tween = node.do_position_y(to, duration).register();
        gd_from_native_tween(tween)
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
        let tween = node.do_position_z(to, duration).register();
        gd_from_native_tween(tween)
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
        let tween = node.do_global_position_x(to, duration).register();
        gd_from_native_tween(tween)
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
        let tween = node.do_global_position_y(to, duration).register();
        gd_from_native_tween(tween)
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
        let tween = node.do_global_position_z(to, duration).register();
        gd_from_native_tween(tween)
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
        let tween = node.do_scale_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale_y)]
    fn scale_y(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_scale_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale_z)]
    fn scale_z(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_scale_z(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node3D.rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_x)]
    fn rotation_x(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_y)]
    fn rotation_y(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_z)]
    fn rotation_z(node: Gd<Node3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation_z(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node3D.rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_degrees_x)]
    fn rotation_degrees_x(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation_degrees_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_degrees_y)]
    fn rotation_degrees_y(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation_degrees_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_degrees_z)]
    fn rotation_degrees_z(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation_degrees_z(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node3D.global_rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_x)]
    fn global_rotation_x(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_rotation_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.global_rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_y)]
    fn global_rotation_y(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_rotation_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.global_rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_z)]
    fn global_rotation_z(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_rotation_z(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Node3D.global_rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_degrees_x)]
    fn global_rotation_degrees_x(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_rotation_degrees_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Node3D.global_rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_degrees_y)]
    fn global_rotation_degrees_y(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_rotation_degrees_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `z` component of the property [member Node3D.global_rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_degrees_z)]
    fn global_rotation_degrees_z(
        node: Gd<Node3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_rotation_degrees_z(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position)]
    fn r#position(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_position(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method position].
    #[func(rename = move_local)]
    fn r#move_local(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        Self::r#position(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position)]
    fn r#global_position(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_global_position(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method global_position].
    #[func(rename = move)]
    fn r#move(node: Gd<Node3D>, to: Vector3, duration: f64) -> Gd<SpirePropertyVec3> {
        Self::r#global_position(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale)]
    fn r#scale(node: Gd<Node3D>, to: Vector3, duration: f64) -> Gd<SpirePropertyVec3> {
        let tween = node.do_scale(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation)]
    fn r#rotation(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_rotation(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_degrees)]
    fn r#rotation_degrees(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_rotation_degrees(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.global_rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation)]
    fn r#global_rotation(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_global_rotation(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Node3D.global_rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_rotation_degrees)]
    fn r#global_rotation_degrees(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_global_rotation_degrees(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens local orientation along the shortest arc. Relative mode composes local rotation deltas; speed is radians per second.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = quaternion)]
    fn r#quaternion(
        node: Gd<Node3D>,
        to: Quaternion,
        duration: f64,
    ) -> Gd<SpirePropertyQuaternion> {
        let tween = node.do_quaternion(to, duration).register();
        gd_from_native_tween(tween)
    }
    /// Moves [param node] toward [param follow_this]'s global position at [param speed] units per second,
    /// sampling the target every processing step. By default, the tween finishes once it reaches the target.
    /// For continuous following, call [code]set_loops(-1, Spire.LOOP_MODE_RESTART)[/code] on the returned handle.
    /// If [param follow_this] is freed, the tween continues toward its last known global position with the
    /// configured loop setting instead of cancelling. The returned tween is already registered.
    #[func]
    fn follow(
        node: Gd<Node3D>,
        follow_this: Gd<Node3D>,
        speed: f64,
    ) -> Gd<SpirePropertyVec3> {
        let tween = node.do_follow(follow_this, speed).register();
        gd_from_native_tween(tween)
    }
    #[func]
    fn ellipsis(
        node: Gd<Node3D>,
        center: Vector3,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector3,
        to_radius: Vector3,
        axis: Vector3,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        let tween = node
            .do_ellipsis(
                center,
                from_angle,
                to_angle,
                from_radius,
                to_radius,
                axis,
                duration,
            )
            .register();
        gd_from_native_tween(tween)
    }
    /// Moves [param node] around [param center] (global coordinates) at a
    /// constant [param radius], in the plane perpendicular to [param axis].
    /// Angles are in radians; their difference controls direction and turns.
    /// The axis is normalized, with [constant Vector3.UP] used for a zero axis.
    ///
    /// Equivalent to [method ellipsis] with `(radius, radius, 0)` at both
    /// radius endpoints: there is no displacement along the axis.
    /// The returned [SpireMethodFloat] interpolates progress from 0 to 1.
    #[func]
    fn circle(
        node: Gd<Node3D>,
        center: Vector3,
        from_angle: f32,
        to_angle: f32,
        radius: f32,
        axis: Vector3,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        let radii = Vector3::new(radius, radius, 0.0);
        Self::ellipsis(node, center, from_angle, to_angle, radii, radii, axis, duration)
    }
    /// Expands [param to] to a uniform absolute scale. Already registered.
    #[func]
    fn uniform_scale(
        node: Gd<Node3D>,
        to: real,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        gd_from_native_tween(node.do_uniform_scale(to, duration).register())
    }
    ///Adds a local-position delta, blending with other relative tweens.
    #[func]
    fn blendable_position(
        node: Gd<Node3D>,
        by: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        gd_from_native_tween(node.do_blendable_position(by, duration).register())
    }
    ///Adds a global-position delta, blending with other relative tweens.
    #[func]
    fn blendable_global_position(
        node: Gd<Node3D>,
        by: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
        gd_from_native_tween(node.do_blendable_global_position(by, duration).register())
    }
    ///Adds a scale delta; this is additive, not multiplicative.
    #[func]
    fn blendable_scale(
        node: Gd<Node3D>,
        by: Vector3,
        duration: f64,
    ) -> Gd<SpirePropertyVec3> {
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
        node: Gd<Node3D>,
        to: Vector3,
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
        node: Gd<Node3D>,
        to: Vector3,
        height: f64,
        jumps: i32,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_global_jump(to, height, jumps, duration).register())
    }
    /// Alternating decaying waypoint punch. Captures the start on first step.
    /// Rotation uses radians; scale deltas are additive. Stop leaves the
    /// target untouched; successful/forced completion restores the offset.
    /// Loops reuse the path; Yoyo retraces it. Easing remaps overall progress.
    /// Invalid duration becomes 1 second, invalid vector components become
    /// zero, elasticity clamps to 0-1, segment count clamps to 2-65536; warns.
    /// Clearing native finish connections removes restoration. Registered.
    #[func]
    fn punch_position(
        node: Gd<Node3D>,
        punch: Vector3,
        duration: f64,
        #[opt(default = 10)]
        vibrato: i32,
        #[opt(default = 1.0)]
        elasticity: f64,
        #[opt(default = true)]
        blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node
                .do_punch_position(punch, duration, vibrato, elasticity, blendable)
                .register(),
        )
    }
    /// Alternating decaying waypoint punch. Captures the start on first step.
    /// Rotation uses radians; scale deltas are additive. Stop leaves the
    /// target untouched; successful/forced completion restores the offset.
    /// Loops reuse the path; Yoyo retraces it. Easing remaps overall progress.
    /// Invalid duration becomes 1 second, invalid vector components become
    /// zero, elasticity clamps to 0-1, segment count clamps to 2-65536; warns.
    /// Clearing native finish connections removes restoration. Registered.
    #[func]
    fn punch_rotation(
        node: Gd<Node3D>,
        punch: Vector3,
        duration: f64,
        #[opt(default = 10)]
        vibrato: i32,
        #[opt(default = 1.0)]
        elasticity: f64,
        #[opt(default = true)]
        blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node
                .do_punch_rotation(punch, duration, vibrato, elasticity, blendable)
                .register(),
        )
    }
    /// Alternating decaying waypoint punch. Captures the start on first step.
    /// Rotation uses radians; scale deltas are additive. Stop leaves the
    /// target untouched; successful/forced completion restores the offset.
    /// Loops reuse the path; Yoyo retraces it. Easing remaps overall progress.
    /// Invalid duration becomes 1 second, invalid vector components become
    /// zero, elasticity clamps to 0-1, segment count clamps to 2-65536; warns.
    /// Clearing native finish connections removes restoration. Registered.
    #[func]
    fn punch_scale(
        node: Gd<Node3D>,
        punch: Vector3,
        duration: f64,
        #[opt(default = 10)]
        vibrato: i32,
        #[opt(default = 1.0)]
        elasticity: f64,
        #[opt(default = true)]
        blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node
                .do_punch_scale(punch, duration, vibrato, elasticity, blendable)
                .register(),
        )
    }
    /// Shake using the position-shake lifecycle and input normalization.
    /// Rotation strength is in radians; scale strength is an additive delta.
    /// Camera3D position shakes in its captured relative XY plane (Z ignored).
    /// Generates once per playback, loops reuse, Yoyo retraces, replay regenerates.
    /// Stop leaves the target untouched; successful/forced completion restores.
    /// Invalid inputs warn and normalize; easing follows the global default.
    /// Clearing native finish hooks disables restoration. Already registered.
    #[func]
    fn shake(
        node: Gd<Node3D>,
        duration: f64,
        strength: Vector3,
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
        gd_from_native_tween(
            node
                .do_shake(
                    duration,
                    strength,
                    vibrato,
                    randomness,
                    fade_out,
                    randomness_mode,
                    blendable,
                )
                .register(),
        )
    }
    /// Shake using the position-shake lifecycle and input normalization.
    /// Rotation strength is in radians; scale strength is an additive delta.
    /// Camera3D position shakes in its captured relative XY plane (Z ignored).
    /// Generates once per playback, loops reuse, Yoyo retraces, replay regenerates.
    /// Stop leaves the target untouched; successful/forced completion restores.
    /// Invalid inputs warn and normalize; easing follows the global default.
    /// Clearing native finish hooks disables restoration. Already registered.
    #[func]
    fn shake_rotation(
        node: Gd<Node3D>,
        duration: f64,
        strength: Vector3,
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
        gd_from_native_tween(
            node
                .do_shake_rotation(
                    duration,
                    strength,
                    vibrato,
                    randomness,
                    fade_out,
                    randomness_mode,
                    blendable,
                )
                .register(),
        )
    }
    /// Shake using the position-shake lifecycle and input normalization.
    /// Rotation strength is in radians; scale strength is an additive delta.
    /// Camera3D position shakes in its captured relative XY plane (Z ignored).
    /// Generates once per playback, loops reuse, Yoyo retraces, replay regenerates.
    /// Stop leaves the target untouched; successful/forced completion restores.
    /// Invalid inputs warn and normalize; easing follows the global default.
    /// Clearing native finish hooks disables restoration. Already registered.
    #[func]
    fn shake_scale(
        node: Gd<Node3D>,
        duration: f64,
        strength: Vector3,
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
        gd_from_native_tween(
            node
                .do_shake_scale(
                    duration,
                    strength,
                    vibrato,
                    randomness,
                    fade_out,
                    randomness_mode,
                    blendable,
                )
                .register(),
        )
    }
    /// Local quaternion-composition punch with blending enabled. Radians.
    #[func]
    fn blendable_punch_rotation(
        node: Gd<Node3D>,
        punch: Vector3,
        duration: f64,
        #[opt(default = 10)]
        vibrato: i32,
        #[opt(default = 1.0)]
        elasticity: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node
                .do_blendable_punch_rotation(punch, duration, vibrato, elasticity)
                .register(),
        )
    }
    /// Euler target in radians, YXZ order. Fast chooses shortest angles;
    /// FastBeyond360 preserves winding; WorldAxisAdd/LocalAxisAdd compose
    /// relative rotations in world/object axes. Captures start on first step.
    /// The global variant targets world orientation. Preserves scale.
    /// Invalid values warn and normalize. Already registered float driver.
    #[func]
    fn rotate(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
        #[opt(default = RotateMode::Fast)]
        mode: RotateMode,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_rotate(to, duration, mode).register())
    }
    /// Euler target in radians, YXZ order. Fast chooses shortest angles;
    /// FastBeyond360 preserves winding; WorldAxisAdd/LocalAxisAdd compose
    /// relative rotations in world/object axes. Captures start on first step.
    /// The global variant targets world orientation. Preserves scale.
    /// Invalid values warn and normalize. Already registered float driver.
    #[func]
    fn global_rotate(
        node: Gd<Node3D>,
        to: Vector3,
        duration: f64,
        #[opt(default = RotateMode::Fast)]
        mode: RotateMode,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_global_rotate(to, duration, mode).register())
    }
    /// Shortest-arc quaternion interpolation. Captures start on first step;
    /// preserves scale. Target normalizes; invalid/zero targets warn and
    /// become identity. Ordinary easing applies. Returns a float method driver.
    #[func]
    fn rotate_quaternion(
        node: Gd<Node3D>,
        to: Quaternion,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_rotate_quaternion(to, duration).register())
    }
    /// Shortest-arc quaternion interpolation. Captures start on first step;
    /// preserves scale. Target normalizes; invalid/zero targets warn and
    /// become identity. Ordinary easing applies. Returns a float method driver.
    #[func]
    fn global_rotate_quaternion(
        node: Gd<Node3D>,
        to: Quaternion,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_global_rotate_quaternion(to, duration).register())
    }
    /// Adds successive quaternion rotation deltas in radians. Local deltas
    /// multiply on the right; global deltas on the left. Order matters for
    /// different axes. Preserves other rotation contributions and scale.
    #[func]
    fn blendable_rotation(
        node: Gd<Node3D>,
        by: Vector3,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_blendable_rotation(by, duration).register())
    }
    /// Adds successive quaternion rotation deltas in radians. Local deltas
    /// multiply on the right; global deltas on the left. Order matters for
    /// different axes. Preserves other rotation contributions and scale.
    #[func]
    fn blendable_global_rotation(
        node: Gd<Node3D>,
        by: Vector3,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_blendable_global_rotation(by, duration).register())
    }
    /// Faces a world-space point using Godot -Z forward (or +Z with
    /// use_model_front). look_at captures the destination once; dynamic_look_at
    /// recomputes it as the source moves; look_at_target also follows a moving
    /// node and binds its lifetime/pause state. Axis constraints preserve
    /// unselected initial Euler components every step via shortest Euler routes. Singular direction/up warns and
    /// retains the last valid orientation. Captures start on first step.
    #[func]
    fn look_at(
        node: Gd<Node3D>,
        towards: Vector3,
        duration: f64,
        #[opt(default = Vector3::UP)]
        up: Vector3,
        #[opt(default = RotationAxis::All)]
        axis: RotationAxis,
        #[opt(default = false)]
        use_model_front: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node.do_look_at(towards, duration, up, axis, use_model_front).register(),
        )
    }
    /// Faces a world-space point using Godot -Z forward (or +Z with
    /// use_model_front). look_at captures the destination once; dynamic_look_at
    /// recomputes it as the source moves; look_at_target also follows a moving
    /// node and binds its lifetime/pause state. Axis constraints preserve
    /// unselected initial Euler components every step via shortest Euler routes. Singular direction/up warns and
    /// retains the last valid orientation. Captures start on first step.
    #[func]
    fn dynamic_look_at(
        node: Gd<Node3D>,
        towards: Vector3,
        duration: f64,
        #[opt(default = Vector3::UP)]
        up: Vector3,
        #[opt(default = RotationAxis::All)]
        axis: RotationAxis,
        #[opt(default = false)]
        use_model_front: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node
                .do_dynamic_look_at(towards, duration, up, axis, use_model_front)
                .register(),
        )
    }
    /// Faces a world-space point using Godot -Z forward (or +Z with
    /// use_model_front). look_at captures the destination once; dynamic_look_at
    /// recomputes it as the source moves; look_at_target also follows a moving
    /// node and binds its lifetime/pause state. Axis constraints preserve
    /// unselected initial Euler components every step via shortest Euler routes. Singular direction/up warns and
    /// retains the last valid orientation. Captures start on first step.
    #[func]
    fn look_at_target(
        node: Gd<Node3D>,
        target: Gd<Node3D>,
        duration: f64,
        #[opt(default = Vector3::UP)]
        up: Vector3,
        #[opt(default = RotationAxis::All)]
        axis: RotationAxis,
        #[opt(default = false)]
        use_model_front: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node
                .do_look_at_target(target, duration, up, axis, use_model_front)
                .register(),
        )
    }
    /// Local 3D Archimedean spiral. Captures start on first step. About axis,
    /// travel=duration*speed*progress, angle=frequency*travel, radius=travel
    /// (or expands then contracts), axial displacement=depth*travel. Snapping
    /// rounds position. Invalid parameters warn and normalize. Public Pro
    /// controls are supported; undocumented numerical parity is not asserted.
    #[func]
    fn spiral(
        node: Gd<Node3D>,
        duration: f64,
        #[opt(default = Vector3::BACK)]
        axis: Vector3,
        #[opt(default = SpiralMode::Expand)]
        mode: SpiralMode,
        #[opt(default = 1.0)]
        speed: f64,
        #[opt(default = 10.0)]
        frequency: f64,
        #[opt(default = 0.0)]
        depth: f64,
        #[opt(default = false)]
        snapping: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node
                .do_spiral(duration, axis, mode, speed, frequency, depth, snapping)
                .register(),
        )
    }
}
