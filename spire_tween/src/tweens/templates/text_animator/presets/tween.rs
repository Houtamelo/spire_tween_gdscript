//! Ordinary character tweens built from the same validated resources as preset rendering.

use super::{sample::Effect, *};

impl CompiledTextEffect {
    fn impulse_channel(&self) -> Option<Channel> {
        match self.effect {
            Effect::Punch(channel, _) | Effect::Shake2(channel, _) => Some(channel),
            Effect::Shake3(_) => Some(Channel::Rotation),
            _ => None,
        }
    }

    pub(crate) fn impulse_start_default(&self) -> Option<Variant> {
        Some(match self.impulse_channel()? {
            Channel::Offset => Vector2::ZERO.to_variant(),
            Channel::Scale => Vector2::ONE.to_variant(),
            Channel::Rotation => Vector3::ZERO.to_variant(),
        })
    }

    pub(crate) fn loop_mode(&self) -> LoopMode { self.timing.mode }

    /// Called when the step is reached, before any per-character start delays.
    pub(crate) fn initialize(
        &self,
        label: Gd<SpireRichTextLabel>,
        index: i64,
        start_enabled: bool,
        impulse_start: &Variant,
    ) {
        if matches!(self.effect, Effect::Rotation(..) | Effect::Scale(..))
            || matches!(self.impulse_channel(), Some(Channel::Rotation | Channel::Scale))
        {
            SpireRichTextLabel::set_char_pivot(label.clone(), index, self.pivot);
        }
        if !start_enabled {
            return;
        }
        match self.effect {
            Effect::Color(from, _) => SpireRichTextLabel::set_char_color(label, index, from),
            Effect::Fade(from, _) => SpireRichTextLabel::set_char_alpha(label, index, from),
            Effect::Offset(from, _) => SpireRichTextLabel::set_char_offset(label, index, from),
            Effect::Scale(from, _) => SpireRichTextLabel::set_char_scale(label, index, from),
            Effect::Rotation(from, ..) => SpireRichTextLabel::set_char_rotation(label, index, from),
            _ => match self.impulse_channel().expect("impulse effect") {
                Channel::Offset => SpireRichTextLabel::set_char_offset(label, index, impulse_start.to()),
                Channel::Scale => SpireRichTextLabel::set_char_scale(label, index, impulse_start.to()),
                Channel::Rotation => SpireRichTextLabel::set_char_rotation(label, index, impulse_start.to()),
            },
        }
    }

    pub(crate) fn tween(
        &self,
        label: Gd<SpireRichTextLabel>,
        index: i64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let duration = self.timing.duration;
        let tween = match self.effect {
            Effect::Color(_, to) => label.do_color_char(index, to, duration),
            Effect::Fade(_, to) => label.do_fade_char(index, to, duration),
            Effect::Offset(_, to) => label.do_offset_char(index, to, duration),
            Effect::Scale(_, to) => label.do_scale_char(index, to, duration),
            Effect::Rotation(_, to, mode) => label.do_rotate_char(index, to, duration, mode),
            Effect::Punch(channel, settings) => {
                let strength = settings.strength;
                match channel {
                    Channel::Offset => label.do_punch_char_offset(
                        index,
                        Vector2::new(strength.x, strength.y),
                        duration,
                        settings.vibrato,
                        settings.elasticity,
                        blendable,
                    ),
                    Channel::Scale => label.do_punch_char_scale(
                        index,
                        Vector2::new(strength.x, strength.y),
                        duration,
                        settings.vibrato,
                        settings.elasticity,
                        blendable,
                    ),
                    Channel::Rotation => label.do_punch_char_rotation(
                        index,
                        strength,
                        duration,
                        settings.vibrato,
                        settings.elasticity,
                        blendable,
                    ),
                }
            }
            Effect::Shake2(channel, settings) => {
                let strength = Vector2::new(settings.strength[0] as real, settings.strength[1] as real);
                match channel {
                    Channel::Offset => label.do_shake_char_offset(
                        index,
                        duration,
                        strength,
                        settings.vibrato,
                        settings.randomness as real,
                        settings.fade_out,
                        settings.mode,
                        blendable,
                    ),
                    Channel::Scale => label.do_shake_char_scale(
                        index,
                        duration,
                        strength,
                        settings.vibrato,
                        settings.randomness as real,
                        settings.fade_out,
                        settings.mode,
                        blendable,
                    ),
                    Channel::Rotation => unreachable!("rotation uses the three-axis shake settings"),
                }
            }
            Effect::Shake3(settings) => label.do_shake_char_rotation(
                index,
                duration,
                Vector3::new(settings.strength[0] as real, settings.strength[1] as real, settings.strength[2] as real),
                settings.vibrato,
                settings.randomness as real,
                settings.fade_out,
                settings.mode,
                blendable,
            ),
        };
        tween.with_ease(EaseKind::Basic(self.timing.ease))
    }
}
