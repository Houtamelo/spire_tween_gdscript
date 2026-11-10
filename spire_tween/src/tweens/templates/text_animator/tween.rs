use super::*;

pub(super) fn rotation_at(from: Vector3, to: Vector3, p: f64, mode: RotateMode) -> Vector3 {
    match mode {
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
        RotateMode::WorldAxisAdd => (Quaternion::from_euler(to * p as real) * Quaternion::from_euler(from)).get_euler(),
        RotateMode::LocalAxisAdd => (Quaternion::from_euler(from) * Quaternion::from_euler(to * p as real)).get_euler(),
    }
}

impl SpireRichTextLabel {
    fn progress(
        this: Gd<Self>,
        index: i64,
        duration: f64,
        mut update: impl FnMut(&mut SpireCharRenderer, f64) + 'static,
    ) -> SpireTween<LerpMethodData<f64>> {
        let mut target = this.clone();
        let callable = this.linked_callable("spire_character_update", move |args| {
            let mut effect = target.bind().char_renderer.clone();
            {
                let mut state = effect.bind_mut();
                if !state.is_valid_char(index) {
                    return Variant::nil();
                }
                update(&mut state, args[0].to::<f64>());
            }
            target.queue_redraw();
            Variant::nil()
        });
        SpireTween::<LerpMethodData<f64>>::new(callable, 0.0, 1.0, duration_or_default(duration))
    }

    /// Tweens the additional pixel offset. Captures the start on first processing.
    pub fn do_offset_char(this: Gd<Self>, index: i64, to: Vector2, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        let to = clean_vec2(to, false);
        let mut from = None;
        Self::progress(this, index, duration, move |effect, p| {
            let from = *from.get_or_insert_with(|| effect.get_char_offset(index));
            effect.set_char_offset(index, from.lerp(to, p as real));
        })
    }
    /// Tweens per-axis scale multipliers. Negative values mirror the glyph.
    pub fn do_scale_char(this: Gd<Self>, index: i64, to: Vector2, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        let to = clean_vec2(to, false);
        let mut from = None;
        Self::progress(this, index, duration, move |effect, p| {
            let from = *from.get_or_insert_with(|| effect.get_char_scale(index));
            effect.set_char_scale(index, from.lerp(to, p as real));
        })
    }
    /// Tweens fill RGBA. Target alpha clamps to `[0,1]`; RGB permits HDR values. Drawing
    /// captures the effective initial BBCode color; before first draw uses theme.
    pub fn do_color_char(this: Gd<Self>, index: i64, to: Color, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        let to = clean_color(to);
        let mut from = None;
        Self::progress(this, index, duration, move |effect, p| {
            let from = *from.get_or_insert_with(|| effect.get_char_color(index));
            // Normalize caller inputs, not values produced by legitimate easing
            // overshoot or Incremental loops. Match ordinary color interpolation.
            effect.write(index, |c, stamp| c.set_color(from.lerp(to, p), stamp));
        })
    }
    /// Tweens toward an alpha target in `[0,1]`, preserving RGB. Easing may
    /// overshoot that range. Invisible characters retain indices.
    pub fn do_fade_char(this: Gd<Self>, index: i64, to: f64, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        let to = unit(to, 1.0, "alpha");
        let mut from = None;
        Self::progress(this, index, duration, move |effect, p| {
            let from = *from.get_or_insert_with(|| effect.get_char_alpha(index));
            effect.write(index, |c, stamp| c.alpha.set(Some(from + (to - from) * p), stamp));
        })
    }
    /// Tweens a YXZ Euler rotation in radians, projected orthographically. Fast
    /// uses the shortest per-axis angular delta; FastBeyond360 preserves turns.
    /// WorldAxisAdd/LocalAxisAdd compose on the left/right of the captured rotation.
    pub fn do_rotate_char(
        this: Gd<Self>,
        index: i64,
        to: Vector3,
        duration: f64,
        mode: RotateMode,
    ) -> SpireTween<LerpMethodData<f64>> {
        let to = clean_vec3(to, false);
        let mut from = None;
        Self::progress(this, index, duration, move |effect, p| {
            let from = *from.get_or_insert_with(|| effect.get_char_rotation(index));
            effect.set_char_rotation(index, rotation_at(from, to, p, mode));
        })
    }

    fn accessors(
        label_id: InstanceId,
        index: i64,
        channel: Channel,
    ) -> (impl Fn() -> Vector3 + 'static, impl FnMut(Vector3) + 'static) {
        (
            move || {
                let Ok(label) = Gd::<Self>::try_from_instance_id(label_id) else { return Vector3::ZERO };
                let effect = label.bind().char_renderer.clone();
                let state = effect.bind();
                match channel {
                    Channel::Offset => {
                        let v = state.get_char_offset(index);
                        Vector3::new(v.x, v.y, 0.0)
                    }
                    Channel::Scale => {
                        let v = state.get_char_scale(index);
                        Vector3::new(v.x, v.y, 0.0)
                    }
                    Channel::Rotation => state.get_char_rotation(index),
                }
            },
            move |value: Vector3| {
                let Ok(label) = Gd::<Self>::try_from_instance_id(label_id) else { return };
                if !Self::is_valid_char(label.clone(), index) {
                    return;
                }
                match channel {
                    Channel::Offset => Self::set_char_offset(label, index, Vector2::new(value.x, value.y)),
                    Channel::Scale => Self::set_char_scale(label, index, Vector2::new(value.x, value.y)),
                    Channel::Rotation => Self::set_char_rotation(label, index, value),
                }
            },
        )
    }

    fn character_epoch(label_id: InstanceId) -> u64 {
        let Ok(label) = Gd::<Self>::try_from_instance_id(label_id) else { return 0 };
        let effect = label.bind().char_renderer.clone();
        effect.bind().epoch
    }

    fn punch(
        this: Gd<Self>,
        index: i64,
        channel: Channel,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let label_id = this.instance_id();
        let (get, set) = Self::accessors(label_id, index, channel);
        let punch = clean_vec3(punch, false);
        let duration = duration_or_default(duration);
        let segments = segment_count(vibrato, duration).get();
        let elasticity = unit(elasticity, 1.0, "elasticity");
        let sample = move |p, _| punch * punch_sample(p, segments, elasticity) as real;
        effect_with_epoch(
            Some(this.upcast()),
            get,
            set,
            sample,
            duration,
            blendable,
            Some(std::rc::Rc::new(move || Self::character_epoch(label_id))),
        )
    }

    fn shake(
        this: Gd<Self>,
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
        let label_id = this.instance_id();
        let (get, set) = Self::accessors(label_id, index, channel);
        shake_3d_with_epoch(
            Some(this.upcast()),
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
            Some(std::rc::Rc::new(move || Self::character_epoch(label_id))),
        )
    }

    /// Punches the pixel offset. Uses the existing vector punch math and cleanup.
    pub fn do_punch_char_offset(
        this: Gd<Self>,
        index: i64,
        punch: Vector2,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        Self::punch(
            this,
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
        this: Gd<Self>,
        index: i64,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        Self::punch(this, index, Channel::Rotation, punch, duration, vibrato, elasticity, blendable)
    }
    /// Punches scale with an additive per-axis delta.
    pub fn do_punch_char_scale(
        this: Gd<Self>,
        index: i64,
        punch: Vector2,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        Self::punch(
            this,
            index,
            Channel::Scale,
            Vector3::new(punch.x, punch.y, 0.0),
            duration,
            vibrato,
            elasticity,
            blendable,
        )
    }
    /// Shakes the pixel offset using the existing interpolated waypoint sampler.
    pub fn do_shake_char_offset(
        this: Gd<Self>,
        index: i64,
        duration: f64,
        strength: Vector2,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        Self::shake(
            this,
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
        this: Gd<Self>,
        index: i64,
        duration: f64,
        strength: Vector3,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        Self::shake(this, index, Channel::Rotation, duration, strength, vibrato, randomness, fade_out, mode, blendable)
    }
    /// Shakes scale with additive per-axis strengths.
    pub fn do_shake_char_scale(
        this: Gd<Self>,
        index: i64,
        duration: f64,
        strength: Vector2,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        Self::shake(
            this,
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

macro_rules! character_builders {
    ($(fn $name:ident($($arg:ident: $ty:ty),*);)*) => {
        /// Unregistered native character builders for a label and its Gd handle.
        /// Construction does not mutate or reborrow the label. Missing-index writes are ignored.
        pub trait SpireDoRichTextCharacters {
            $(fn $name(&self, $($arg: $ty),*) -> SpireTween<LerpMethodData<f64>>;)*
        }
        impl SpireDoRichTextCharacters for Gd<SpireRichTextLabel> {
            $(fn $name(&self, $($arg: $ty),*) -> SpireTween<LerpMethodData<f64>> {
                SpireRichTextLabel::$name(self.clone(), $($arg),*)
            })*
        }
        impl SpireDoRichTextCharacters for SpireRichTextLabel {
            $(fn $name(&self, $($arg: $ty),*) -> SpireTween<LerpMethodData<f64>> {
                Self::$name(self.to_gd(), $($arg),*)
            })*
        }
    }
}

character_builders! {
    fn do_offset_char(index: i64, to: Vector2, duration: f64);
    fn do_scale_char(index: i64, to: Vector2, duration: f64);
    fn do_color_char(index: i64, to: Color, duration: f64);
    fn do_fade_char(index: i64, to: f64, duration: f64);
    fn do_rotate_char(index: i64, to: Vector3, duration: f64, mode: RotateMode);
    fn do_punch_char_offset(index: i64, punch: Vector2, duration: f64, vibrato: i32, elasticity: f64, blendable: bool);
    fn do_punch_char_rotation(index: i64, punch: Vector3, duration: f64, vibrato: i32, elasticity: f64, blendable: bool);
    fn do_punch_char_scale(index: i64, punch: Vector2, duration: f64, vibrato: i32, elasticity: f64, blendable: bool);
    fn do_shake_char_offset(index: i64, duration: f64, strength: Vector2, vibrato: i32, randomness: real, fade_out: bool, mode: ShakeRandomnessMode, blendable: bool);
    fn do_shake_char_rotation(index: i64, duration: f64, strength: Vector3, vibrato: i32, randomness: real, fade_out: bool, mode: ShakeRandomnessMode, blendable: bool);
    fn do_shake_char_scale(index: i64, duration: f64, strength: Vector2, vibrato: i32, randomness: real, fade_out: bool, mode: ShakeRandomnessMode, blendable: bool);
}
