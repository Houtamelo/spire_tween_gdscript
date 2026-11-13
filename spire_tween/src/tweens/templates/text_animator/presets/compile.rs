//! Validate authoring resources once and copy their typed fields into render entries.

use std::num::NonZeroUsize;

use anyhow::{Result, ensure};

use super::{
    sample::{Effect, PunchSettings, ShakeSettings, Timing},
    *,
};

/// Validated snapshot shared by continuous rendering and Blueprint tween construction.
#[derive(Clone, Copy)]
pub(crate) struct CompiledTextEffect {
    pub(super) timing: Timing,
    pub(super) pivot:  Vector2,
    pub(super) effect: Effect,
}

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

fn check_finite(value: impl Finite, name: &str) -> Result<()> {
    ensure!(value.is_finite(), "`{name}` must be finite; effect entry skipped.");
    Ok(())
}

fn segments(vibrato: i32, duration: f64) -> Result<NonZeroUsize> {
    let count = (f64::from(vibrato) * duration).floor();
    ensure!(
        vibrato > 0 && count <= 65536.0 && count.is_finite(),
        "`vibrato` must be positive and produce at most 65536 segments; effect entry skipped."
    );
    Ok(NonZeroUsize::new(count.max(2.0) as usize).expect("segment count is clamped to at least two"))
}

fn punch(channel: Channel, strength: Vector3, duration: f64, vibrato: i32, elasticity: f64) -> Result<Effect> {
    let segments = segments(vibrato, duration)?;
    ensure!((0.0..=1.0).contains(&elasticity), "`elasticity` must be between 0 and 1; effect entry skipped.");
    Ok(Effect::Punch(channel, PunchSettings {
        strength,
        segments,
        vibrato,
        elasticity,
    }))
}

fn shake<const D: usize>(
    strength: [real; D],
    duration: f64,
    vibrato: i32,
    randomness: f64,
    fade_out: bool,
    mode: ShakeRandomnessMode,
) -> Result<ShakeSettings<D>> {
    let segments = segments(vibrato, duration)?;
    ensure!((0.0..=180.0).contains(&randomness), "`randomness` must be between 0 and 180; effect entry skipped.");
    ensure!(!strength.iter().any(|&axis| axis < 0.0), "`strength` must be non-negative; effect entry skipped.");
    Ok(ShakeSettings {
        segments,
        vibrato,
        strength: strength.map(|axis| axis as f64),
        randomness,
        fade_out,
        mode,
    })
}

impl CompiledTextEffect {
    pub(crate) fn compile(resource: &Gd<Resource>, field: &str) -> Result<Self> {
        // Each branch binds a concrete resource. Renaming a class or field is
        // checked by Rust rather than inferred from class-name substrings.
        macro_rules! compile {
            ($bind:ident, [$($field:ident),*], |$r:ident| $body:expr) => {{
                let $r = $bind.bind();
                check_finite($r.duration, "duration")?;
                check_finite($r.character_phase_offset, "character_phase_offset")?;
                $(check_finite($r.$field, stringify!($field))?;)*
                ensure!($r.duration > 0.0, "`duration` must be finite and positive; effect entry skipped.");

                let timing = Timing {
                    duration: $r.duration,
                    ease: $r.ease,
                    mode: $r.loop_mode,
                    phase: $r.character_phase_offset,
                };

                let (effect, pivot) = $body;
                return Ok(Self { timing, pivot, effect });
            }};
        }

        match_class! { resource.clone(),
            color @ SpireTextColorEffect => compile!(color, [from, to], |r| (Effect::Color(r.from, r.to), Vector2::ZERO)),
            fade @ SpireTextFadeEffect => compile!(fade, [from, to], |r| (Effect::Fade(r.from, r.to), Vector2::ZERO)),
            offset @ SpireTextOffsetEffect => compile!(offset, [from, to], |r| (Effect::Offset(r.from, r.to), Vector2::ZERO)),
            rotation @ SpireTextRotationEffect => compile!(rotation, [from, to, pivot], |r| (Effect::Rotation(r.from, r.to, r.rotate_mode), r.pivot)),
            scale @ SpireTextScaleEffect => compile!(scale, [from, to, pivot], |r| (Effect::Scale(r.from, r.to), r.pivot)),
            punch_offset @ SpireTextPunchOffsetEffect => compile!(punch_offset, [punch, elasticity], |r| (
                    punch(
                        Channel::Offset,
                        Vector3::new(r.punch.x, r.punch.y, 0.0),
                        r.duration,
                        r.vibrato,
                        r.elasticity
                    )?,
                    Vector2::ZERO
                )),
            punch_rotation @ SpireTextPunchRotationEffect => compile!(punch_rotation, [punch, elasticity, pivot], |r| (
                    punch(Channel::Rotation, r.punch, r.duration, r.vibrato, r.elasticity)?,
                    r.pivot
                )),
            punch_scale @ SpireTextPunchScaleEffect => compile!(punch_scale, [punch, elasticity, pivot], |r| (
                    punch(
                        Channel::Scale,
                        Vector3::new(r.punch.x, r.punch.y, 0.0),
                        r.duration,
                        r.vibrato,
                        r.elasticity
                    )?,
                    r.pivot
                )),
            shake_offset @ SpireTextShakeOffsetEffect => compile!(shake_offset, [strength, randomness], |r| (
                    Effect::Shake2(
                        Channel::Offset,
                        shake(
                            [r.strength.x, r.strength.y],
                            r.duration,
                            r.vibrato,
                            r.randomness,
                            r.fade_out,
                            r.randomness_mode
                        )?
                    ),
                    Vector2::ZERO
                )),
            shake_rotation @ SpireTextShakeRotationEffect => compile!(shake_rotation, [strength, randomness, pivot], |r| (
                    Effect::Shake3(shake(
                        [r.strength.x, r.strength.y, r.strength.z],
                        r.duration,
                        r.vibrato,
                        r.randomness,
                        r.fade_out,
                        r.randomness_mode
                    )?),
                    r.pivot
                )),
            shake_scale @ SpireTextShakeScaleEffect => compile!(shake_scale, [strength, randomness, pivot], |r| (
                    Effect::Shake2(
                        Channel::Scale,
                        shake(
                            [r.strength.x, r.strength.y],
                            r.duration,
                            r.vibrato,
                            r.randomness,
                            r.fade_out,
                            r.randomness_mode
                        )?
                    ),
                    r.pivot
                )),
        }

        bail!("`{field}` contains unsupported resource `{}`; entry skipped.", resource.get_class())
    }
}
