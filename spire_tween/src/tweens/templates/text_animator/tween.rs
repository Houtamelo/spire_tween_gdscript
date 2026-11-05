use super::*;

impl TextAnimator {
    fn progress(
        &self,
        index: i64,
        duration: f64,
        mut update: impl FnMut(f64) + 'static,
    ) -> SpireTween<LerpMethodData<f64>> {
        let callable = Callable::from_fn("spire_character_update", move |args| {
            update(args[0].to::<f64>());
            Variant::nil()
        });
        self.checked(index, SpireTween::<LerpMethodData<f64>>::new(callable, 0.0, 1.0, duration_or_default(duration)))
    }
    fn checked(&self, index: i64, mut tween: SpireTween<LerpMethodData<f64>>) -> SpireTween<LerpMethodData<f64>> {
        tween.t.validation = Some(self.validation(index));
        if let Some(label) = &self.state.label
            && label.is_instance_valid()
        {
            tween.t.owner = Some(ObjectOrNode::from_unchecked_object(label.clone().upcast()));
        }
        if !self.check(index) {
            tween.stop();
        }
        tween
    }

    /// Tweens the additional pixel offset. Captures the start on first processing.
    pub fn do_offset_char(&self, index: i64, to: Vector2, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        let to = clean_vec3(Vector3::new(to.x, to.y, 0.0), false);
        let to = Vector2::new(to.x, to.y);
        let animator = self.clone();
        let mut from = None;
        self.progress(index, duration, move |p| {
            let from = *from.get_or_insert_with(|| animator.get_char_offset(index));
            animator.set_char_offset(index, from.lerp(to, p as real));
        })
    }
    /// Tweens per-axis scale multipliers. Negative values mirror the glyph.
    pub fn do_scale_char(&self, index: i64, to: Vector2, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        let to = clean_vec3(Vector3::new(to.x, to.y, 0.0), false);
        let to = Vector2::new(to.x, to.y);
        let animator = self.clone();
        let mut from = None;
        self.progress(index, duration, move |p| {
            let from = *from.get_or_insert_with(|| animator.get_char_scale(index));
            animator.set_char_scale(index, from.lerp(to, p as real));
        })
    }
    /// Tweens fill RGBA. Target alpha clamps to `[0,1]`; RGB permits HDR values. Drawing
    /// captures the effective initial BBCode color; before first draw uses theme.
    pub fn do_color_char(&self, index: i64, to: Color, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        let to = Color::from_rgba(
            finite(to.r as f64, 1.0, "red") as f32,
            finite(to.g as f64, 1.0, "green") as f32,
            finite(to.b as f64, 1.0, "blue") as f32,
            unit(to.a as f64, 1.0, "alpha") as f32,
        );
        let animator = self.clone();
        let mut from = None;
        self.progress(index, duration, move |p| {
            let from = *from.get_or_insert_with(|| animator.get_char_color(index));
            // Normalize caller inputs, not values produced by legitimate easing
            // overshoot or Incremental loops. Match ordinary color interpolation.
            animator.write(index, |c, stamp| c.set_color(from.lerp(to, p), stamp));
        })
    }
    /// Tweens toward an alpha target in `[0,1]`, preserving RGB. Easing may
    /// overshoot that range. Invisible characters retain indices.
    pub fn do_fade_char(&self, index: i64, to: f64, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        let to = unit(to, 1.0, "alpha");
        let animator = self.clone();
        let mut from = None;
        self.progress(index, duration, move |p| {
            let from = *from.get_or_insert_with(|| animator.get_char_alpha(index));
            animator.write(index, |c, stamp| {
                c.alpha = Value {
                    value: Some(from + (to - from) * p),
                    stamp,
                }
            });
        })
    }
    /// Tweens a YXZ Euler rotation in radians, projected orthographically. Fast
    /// uses the shortest per-axis angular delta; FastBeyond360 preserves turns.
    /// WorldAxisAdd/LocalAxisAdd compose on the left/right of the captured rotation.
    pub fn do_rotate_char(
        &self,
        index: i64,
        to: Vector3,
        duration: f64,
        mode: RotateMode,
    ) -> SpireTween<LerpMethodData<f64>> {
        let to = clean_vec3(to, false);
        let animator = self.clone();
        let mut from = None;
        self.progress(index, duration, move |p| {
            let from = *from.get_or_insert_with(|| animator.get_char_rotation(index));
            let value = match mode {
                RotateMode::Fast | RotateMode::FastBeyond360 => {
                    let mut delta = to - from;
                    if mode == RotateMode::Fast {
                        for axis in [
                            godot::builtin::Vector3Axis::X,
                            godot::builtin::Vector3Axis::Y,
                            godot::builtin::Vector3Axis::Z,
                        ] {
                            delta[axis] = (delta[axis] + std::f64::consts::PI as real)
                                .rem_euclid(std::f64::consts::TAU as real)
                                - std::f64::consts::PI as real;
                        }
                    }
                    from + delta * p as real
                }
                RotateMode::WorldAxisAdd => {
                    (Quaternion::from_euler(to * p as real) * Quaternion::from_euler(from)).get_euler()
                }
                RotateMode::LocalAxisAdd => {
                    (Quaternion::from_euler(from) * Quaternion::from_euler(to * p as real)).get_euler()
                }
            };
            animator.set_char_rotation(index, value);
        })
    }

    fn accessors(
        &self,
        index: i64,
        channel: Channel,
    ) -> (impl Fn() -> Vector3 + 'static, impl FnMut(Vector3) + 'static) {
        let read = self.clone();
        let write = self.clone();
        (
            move || match channel {
                Channel::Offset => {
                    let v = read.get_char_offset(index);
                    Vector3::new(v.x, v.y, 0.0)
                }
                Channel::Scale => {
                    let v = read.get_char_scale(index);
                    Vector3::new(v.x, v.y, 0.0)
                }
                Channel::Rotation => read.get_char_rotation(index),
            },
            move |value: Vector3| {
                // Completion callbacks may follow user stop handlers which edit
                // text or free the label. Never write to a disappeared target.
                if !write.is_valid_char(index) {
                    return;
                }
                match channel {
                    Channel::Offset => write.set_char_offset(index, Vector2::new(value.x, value.y)),
                    Channel::Scale => write.set_char_scale(index, Vector2::new(value.x, value.y)),
                    Channel::Rotation => write.set_char_rotation(index, value),
                }
            },
        )
    }
    fn punch(
        &self,
        index: i64,
        channel: Channel,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let (get, set) = self.accessors(index, channel);
        let punch = clean_vec3(punch, false);
        let duration = duration_or_default(duration);
        let segments = segment_count(vibrato, duration).get();
        let elasticity = unit(elasticity, 1.0, "elasticity");
        let sample = move |p, _| punch * super::super::punch::punch_sample(p, segments, elasticity) as real;
        let a = self.clone();
        self.checked(
            index,
            effect_with_epoch(
                None,
                get,
                set,
                sample,
                duration,
                blendable,
                Some(std::rc::Rc::new(move || a.state.epoch)),
            ),
        )
    }
    fn shake(
        &self,
        index: i64,
        channel: Channel,
        duration: f64,
        strength: Vector3,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let (get, set) = self.accessors(index, channel);
        let a = self.clone();
        self.checked(
            index,
            super::super::shake_variants::shake_3d_with_epoch(
                None,
                get,
                set,
                |v| v,
                strength,
                duration,
                vibrato,
                randomness,
                fade_out,
                mode,
                blendable,
                Some(std::rc::Rc::new(move || a.state.epoch)),
            ),
        )
    }

    /// Punches the pixel offset. Uses the existing vector punch math and cleanup.
    pub fn do_punch_char_offset(
        &self,
        index: i64,
        punch: Vector2,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        self.punch(
            index,
            Channel::Offset,
            Vector3::new(punch.x, punch.y, 0.0),
            duration,
            vibrato,
            elasticity,
            blendable,
        )
    }
    /// Punches Euler angles in radians, independently per axis.
    pub fn do_punch_char_rotation(
        &self,
        index: i64,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        self.punch(index, Channel::Rotation, punch, duration, vibrato, elasticity, blendable)
    }
    /// Punches scale with an additive per-axis delta.
    pub fn do_punch_char_scale(
        &self,
        index: i64,
        punch: Vector2,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        self.punch(index, Channel::Scale, Vector3::new(punch.x, punch.y, 0.0), duration, vibrato, elasticity, blendable)
    }
    /// Shakes the pixel offset using the existing interpolated waypoint sampler.
    pub fn do_shake_char_offset(
        &self,
        index: i64,
        duration: f64,
        strength: Vector2,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        self.shake(
            index,
            Channel::Offset,
            duration,
            Vector3::new(strength.x, strength.y, 0.0),
            vibrato,
            randomness,
            fade_out,
            mode,
            blendable,
        )
    }
    /// Shakes Euler angles with per-axis strengths in radians.
    pub fn do_shake_char_rotation(
        &self,
        index: i64,
        duration: f64,
        strength: Vector3,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        self.shake(index, Channel::Rotation, duration, strength, vibrato, randomness, fade_out, mode, blendable)
    }
    /// Shakes scale with additive per-axis strengths.
    pub fn do_shake_char_scale(
        &self,
        index: i64,
        duration: f64,
        strength: Vector2,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        self.shake(
            index,
            Channel::Scale,
            duration,
            Vector3::new(strength.x, strength.y, 0.0),
            vibrato,
            randomness,
            fade_out,
            mode,
            blendable,
        )
    }
}

#[derive(Clone, Copy)]
enum Channel {
    Offset,
    Rotation,
    Scale,
}
