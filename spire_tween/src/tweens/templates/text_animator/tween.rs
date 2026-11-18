use super::{
    presets::{Effect, PunchSettings, ShakeSettings},
    *,
};

pub(super) fn rotation_at(from: Vector3, to: Vector3, p: f64, mode: RotateMode) -> Vector3 {
    match mode {
        RotateMode::Fast | RotateMode::FastBeyond360 => {
            let mut delta = to - from;
            if mode == RotateMode::Fast {
                for axis in [Vector3Axis::X, Vector3Axis::Y, Vector3Axis::Z] {
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

fn punch(channel: Channel, strength: Vector3, duration: f64, vibrato: i32, elasticity: f64) -> Effect {
    Effect::Punch(channel, PunchSettings {
        strength:   clean_vec3(strength, false),
        segments:   segment_count(vibrato, duration),
        elasticity: unit(elasticity, 1.0, "elasticity"),
    })
}

fn shake(
    channel: Channel,
    strength: Vector3,
    duration: f64,
    vibrato: i32,
    randomness: real,
    fade_out: bool,
    mode: ShakeRandomnessMode,
) -> Effect {
    let strength = clean_vec3(strength, true);
    let settings = ShakeSettings {
        strength: [strength.x as f64, strength.y as f64, strength.z as f64],
        segments: segment_count(vibrato, duration),
        randomness: angle(randomness),
        fade_out,
        mode,
    };
    match channel {
        Channel::Rotation => Effect::Shake3(settings),
        _ => Effect::Shake2(channel, ShakeSettings {
            strength: [settings.strength[0], settings.strength[1]],
            segments: settings.segments,
            randomness: settings.randomness,
            fade_out,
            mode,
        }),
    }
}

macro_rules! character_builders {
    ($(fn $single:ident => $range:ident($($arg:ident: $ty:ty),*) using $duration:ident $body:block)*) => {
        impl SpireRichTextLabel {
            $(
                /// Animate one logical character using a Char tween.
                /// Missing indices ignore writes while playback continues.
                pub fn $single(this: Gd<Self>, index: i64, $($arg: $ty),*) -> SpireTween<CharTweenData> {
                    Self::$range(this, index, index, $($arg),*, EaseKind::Basic(Ease::Linear))
                }
                /// Animate an inclusive range; `last = -1` selects through the text end.
                /// `duration` is seconds per character per leg. Total range duration is
                /// the latest character completion, including its start delay and loops.
                /// The EaseKind supplies finite, non-negative start-delay multiples of duration.
                pub fn $range(this: Gd<Self>, first: i64, last: i64, $($arg: $ty),*, delay_curve: EaseKind) -> SpireTween<CharTweenData> {
                    let $duration = duration_or_default($duration);
                    let (effect, blendable) = $body;
                    CharTweenData::build(this, first, last, $duration, delay_curve, effect, blendable)
                }
            )*
        }
        /// Unregistered native Char tween builders for a label and its Gd handle.
        /// Duration is per character per leg; a range finishes with its last character.
        /// Curve samples are snapshotted during construction. Start values are
        /// captured separately when each character's delay elapses.
        pub trait SpireDoRichTextCharacters {
            $(
                fn $single(&self, index: i64, $($arg: $ty),*) -> SpireTween<CharTweenData>;
                fn $range(&self, first: i64, last: i64, $($arg: $ty),*, delay_curve: EaseKind) -> SpireTween<CharTweenData>;
            )*
        }
        impl SpireDoRichTextCharacters for Gd<SpireRichTextLabel> {
            $(
                fn $single(&self, index: i64, $($arg: $ty),*) -> SpireTween<CharTweenData> {
                    SpireRichTextLabel::$single(self.clone(), index, $($arg),*)
                }
                fn $range(&self, first: i64, last: i64, $($arg: $ty),*, delay_curve: EaseKind) -> SpireTween<CharTweenData> {
                    SpireRichTextLabel::$range(self.clone(), first, last, $($arg),*, delay_curve)
                }
            )*
        }
        impl SpireDoRichTextCharacters for SpireRichTextLabel {
            $(
                fn $single(&self, index: i64, $($arg: $ty),*) -> SpireTween<CharTweenData> {
                    Self::$single(self.to_gd(), index, $($arg),*)
                }
                fn $range(&self, first: i64, last: i64, $($arg: $ty),*, delay_curve: EaseKind) -> SpireTween<CharTweenData> {
                    Self::$range(self.to_gd(), first, last, $($arg),*, delay_curve)
                }
            )*
        }
    }
}

character_builders! {
    fn do_offset_char => do_offset_chars(to: Vector2, duration: f64) using duration {
        (Effect::Offset(Vector2::ZERO, clean_vec2(to, false)), false)
    }
    fn do_scale_char => do_scale_chars(to: Vector2, duration: f64) using duration {
        (Effect::Scale(Vector2::ONE, clean_vec2(to, false)), false)
    }
    fn do_color_char => do_color_chars(to: Color, duration: f64) using duration {
        (Effect::Color(Color::WHITE, clean_color(to)), false)
    }
    fn do_fade_char => do_fade_chars(to: f64, duration: f64) using duration {
        (Effect::Fade(1.0, unit(to, 1.0, "alpha")), false)
    }
    fn do_rotate_char => do_rotate_chars(to: Vector3, duration: f64, mode: RotateMode) using duration {
        (Effect::Rotation(Vector3::ZERO, clean_vec3(to, false), mode), false)
    }
    fn do_punch_char_offset => do_punch_chars_offset(punch_value: Vector2, duration: f64, vibrato: i32, elasticity: f64, blendable: bool) using duration {
        (punch(Channel::Offset, Vector3::new(punch_value.x, punch_value.y, 0.0), duration, vibrato, elasticity), blendable)
    }
    fn do_punch_char_rotation => do_punch_chars_rotation(punch_value: Vector3, duration: f64, vibrato: i32, elasticity: f64, blendable: bool) using duration {
        (punch(Channel::Rotation, punch_value, duration, vibrato, elasticity), blendable)
    }
    fn do_punch_char_scale => do_punch_chars_scale(punch_value: Vector2, duration: f64, vibrato: i32, elasticity: f64, blendable: bool) using duration {
        (punch(Channel::Scale, Vector3::new(punch_value.x, punch_value.y, 0.0), duration, vibrato, elasticity), blendable)
    }
    fn do_shake_char_offset => do_shake_chars_offset(duration: f64, strength: Vector2, vibrato: i32, randomness: real, fade_out: bool, mode: ShakeRandomnessMode, blendable: bool) using duration {
        (shake(Channel::Offset, Vector3::new(strength.x, strength.y, 0.0), duration, vibrato, randomness, fade_out, mode), blendable)
    }
    fn do_shake_char_rotation => do_shake_chars_rotation(duration: f64, strength: Vector3, vibrato: i32, randomness: real, fade_out: bool, mode: ShakeRandomnessMode, blendable: bool) using duration {
        (shake(Channel::Rotation, strength, duration, vibrato, randomness, fade_out, mode), blendable)
    }
    fn do_shake_char_scale => do_shake_chars_scale(duration: f64, strength: Vector2, vibrato: i32, randomness: real, fade_out: bool, mode: ShakeRandomnessMode, blendable: bool) using duration {
        (shake(Channel::Scale, Vector3::new(strength.x, strength.y, 0.0), duration, vibrato, randomness, fade_out, mode), blendable)
    }
}
