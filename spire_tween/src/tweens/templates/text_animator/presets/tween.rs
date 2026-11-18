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

    pub(crate) fn duration(&self) -> f64 { self.timing.duration }

    pub(crate) fn range_tween(
        &self,
        label: Gd<SpireRichTextLabel>,
        first: i64,
        last: i64,
        delay_curve: EaseKind,
        blendable: bool,
    ) -> SpireTween<CharTweenData> {
        CharTweenData::build(label, first, last, self.timing.duration, delay_curve, self.effect, blendable)
            .with_ease(EaseKind::Basic(self.timing.ease))
    }
}
