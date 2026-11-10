//! Validate authoring resources once and copy their typed fields into render entries.

use std::num::NonZeroUsize;

use super::{
    sample::{Effect, Entry, ShakeSettings, Timing},
    *,
};

trait Finite {
    fn is_finite(self) -> bool;
}

impl Finite for f64 {
    fn is_finite(self) -> bool { f64::is_finite(self) }
}
impl Finite for Vector2 {
    fn is_finite(self) -> bool { Vector2::is_finite(self) }
}
impl Finite for Vector3 {
    fn is_finite(self) -> bool { Vector3::is_finite(self) }
}
impl Finite for Color {
    fn is_finite(self) -> bool { [self.r, self.g, self.b, self.a].into_iter().all(f32::is_finite) }
}

fn check_finite(value: impl Finite, name: &str, warnings: &mut Vec<GString>) -> Option<()> {
    if value.is_finite() {
        Some(())
    } else {
        warnings.push(GString::from(&format!("`{name}` must be finite; effect entry skipped.")));
        None
    }
}

fn segments(vibrato: i32, duration: f64, warnings: &mut Vec<GString>) -> Option<NonZeroUsize> {
    let count = (f64::from(vibrato) * duration).floor();
    if vibrato <= 0 || count > 65536.0 || !count.is_finite() {
        warnings.push("`vibrato` must be positive and produce at most 65536 segments; effect entry skipped.".into());
        return None;
    }
    NonZeroUsize::new(count.max(2.0) as usize)
}

fn punch(
    channel: Channel,
    strength: Vector3,
    duration: f64,
    vibrato: i32,
    elasticity: f64,
    warnings: &mut Vec<GString>,
) -> Option<Effect> {
    let segments = segments(vibrato, duration, warnings)?;
    if !(0.0..=1.0).contains(&elasticity) {
        warnings.push("`elasticity` must be between 0 and 1; effect entry skipped.".into());
        return None;
    }
    Some(Effect::Punch(channel, strength, segments, elasticity))
}

fn shake<const D: usize>(
    strength: [real; D],
    duration: f64,
    vibrato: i32,
    randomness: f64,
    fade_out: bool,
    mode: ShakeRandomnessMode,
    warnings: &mut Vec<GString>,
) -> Option<ShakeSettings<D>> {
    let segments = segments(vibrato, duration, warnings)?;
    if !(0.0..=180.0).contains(&randomness) {
        warnings.push("`randomness` must be between 0 and 180; effect entry skipped.".into());
        return None;
    }
    if strength.iter().any(|&axis| axis < 0.0) {
        warnings.push("`strength` must be non-negative; effect entry skipped.".into());
        return None;
    }
    Some(ShakeSettings {
        segments,
        strength: strength.map(|axis| axis as f64),
        randomness,
        fade_out,
        mode,
    })
}

impl Entry {
    pub(super) fn compile(resource: &Gd<Resource>, warnings: &mut Vec<GString>) -> Option<Self> {
        // Each branch binds a concrete resource. Renaming a class or field is
        // checked by Rust rather than inferred from class-name substrings.
        macro_rules! compile {
            ($class:ty, [$($field:ident),*], |$r:ident| $body:expr) => {
                if let Ok(resource) = resource.clone().try_cast::<$class>() {
                    let $r = resource.bind();
                    check_finite($r.duration, "duration", warnings)?;
                    check_finite($r.character_phase_offset, "character_phase_offset", warnings)?;
                    $(check_finite($r.$field, stringify!($field), warnings)?;)*
                    if $r.duration <= 0.0 {
                        warnings.push("`duration` must be finite and positive; effect entry skipped.".into());
                        return None;
                    }
                    let timing = Timing {
                        duration: $r.duration,
                        ease: $r.ease,
                        mode: $r.loop_mode,
                        phase: $r.character_phase_offset,
                    };
                    let (effect, pivot) = $body;
                    return Some(Self::new(timing, pivot, effect));
                }
            };
        }

        compile!(SpireTextColorEffect, [from, to], |r| (Effect::Color(r.from, r.to), Vector2::ZERO));
        compile!(SpireTextFadeEffect, [from, to], |r| (Effect::Fade(r.from, r.to), Vector2::ZERO));
        compile!(SpireTextOffsetEffect, [from, to], |r| (Effect::Offset(r.from, r.to), Vector2::ZERO));
        compile!(SpireTextRotationEffect, [from, to, pivot], |r| (
            Effect::Rotation(r.from, r.to, r.rotate_mode),
            r.pivot
        ));
        compile!(SpireTextScaleEffect, [from, to, pivot], |r| (Effect::Scale(r.from, r.to), r.pivot));
        compile!(SpireTextPunchOffsetEffect, [punch, elasticity], |r| (
            punch(
                Channel::Offset,
                Vector3::new(r.punch.x, r.punch.y, 0.0),
                r.duration,
                r.vibrato,
                r.elasticity,
                warnings
            )?,
            Vector2::ZERO
        ));
        compile!(SpireTextPunchRotationEffect, [punch, elasticity, pivot], |r| (
            punch(Channel::Rotation, r.punch, r.duration, r.vibrato, r.elasticity, warnings)?,
            r.pivot
        ));
        compile!(SpireTextPunchScaleEffect, [punch, elasticity, pivot], |r| (
            punch(
                Channel::Scale,
                Vector3::new(r.punch.x, r.punch.y, 0.0),
                r.duration,
                r.vibrato,
                r.elasticity,
                warnings
            )?,
            r.pivot
        ));
        compile!(SpireTextShakeOffsetEffect, [strength, randomness], |r| (
            Effect::Shake2(
                Channel::Offset,
                shake(
                    [r.strength.x, r.strength.y],
                    r.duration,
                    r.vibrato,
                    r.randomness,
                    r.fade_out,
                    r.randomness_mode,
                    warnings
                )?,
                HashMap::default()
            ),
            Vector2::ZERO
        ));
        compile!(SpireTextShakeRotationEffect, [strength, randomness, pivot], |r| (
            Effect::Shake3(
                shake(
                    [r.strength.x, r.strength.y, r.strength.z],
                    r.duration,
                    r.vibrato,
                    r.randomness,
                    r.fade_out,
                    r.randomness_mode,
                    warnings
                )?,
                HashMap::default()
            ),
            r.pivot
        ));
        compile!(SpireTextShakeScaleEffect, [strength, randomness, pivot], |r| (
            Effect::Shake2(
                Channel::Scale,
                shake(
                    [r.strength.x, r.strength.y],
                    r.duration,
                    r.vibrato,
                    r.randomness,
                    r.fade_out,
                    r.randomness_mode,
                    warnings
                )?,
                HashMap::default()
            ),
            r.pivot
        ));

        warnings.push(GString::from(&format!(
            "`effects` contains unsupported resource `{}`; entry skipped.",
            resource.get_class()
        )));
        None
    }
}
