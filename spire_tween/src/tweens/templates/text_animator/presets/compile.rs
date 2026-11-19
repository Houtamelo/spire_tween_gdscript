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

trait Finite: Copy {
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
        strength: strength.map(|axis| axis as f64),
        randomness,
        fade_out,
        mode,
    })
}

fn endpoint<T: FromGodot + Finite>(value: &Variant, name: &str) -> Result<T> {
    let value = value
        .try_to_relaxed::<T>()
        .map_err(|error| anyhow::anyhow!("`{name}` must match `kind`: {error}; effect entry skipped."))?;
    check_finite(value, name)?;
    Ok(value)
}

impl CompiledTextEffect {
    pub(crate) fn compile(resource: &Gd<SpireTextEffect>) -> Result<Self> {
        let r = resource.bind();
        check_finite(r.duration, "duration")?;
        check_finite(r.character_phase_offset, "character_phase_offset")?;
        ensure!(r.duration > 0.0, "`duration` must be finite and positive; effect entry skipped.");
        let timing = Timing {
            duration: r.duration,
            ease: r.ease,
            mode: r.loop_mode,
            phase: r.character_phase_offset,
        };
        let uses_pivot = matches!(
            r.kind,
            TextEffectKind::Rotation
                | TextEffectKind::Scale
                | TextEffectKind::PunchRotation
                | TextEffectKind::PunchScale
                | TextEffectKind::ShakeRotation
                | TextEffectKind::ShakeScale
        );
        let pivot = if uses_pivot {
            check_finite(r.pivot, "pivot")?;
            r.pivot
        } else {
            Vector2::ZERO
        };
        let effect = match r.kind {
            TextEffectKind::Color => Effect::Color(endpoint(&r.from, "from")?, endpoint(&r.to, "to")?),
            TextEffectKind::Fade => Effect::Fade(endpoint(&r.from, "from")?, endpoint(&r.to, "to")?),
            TextEffectKind::Offset => Effect::Offset(endpoint(&r.from, "from")?, endpoint(&r.to, "to")?),
            TextEffectKind::Rotation => {
                Effect::Rotation(endpoint(&r.from, "from")?, endpoint(&r.to, "to")?, r.rotate_mode)
            }
            TextEffectKind::Scale => Effect::Scale(endpoint(&r.from, "from")?, endpoint(&r.to, "to")?),
            TextEffectKind::PunchOffset | TextEffectKind::PunchRotation | TextEffectKind::PunchScale => {
                let (channel, strength) = match r.kind {
                    TextEffectKind::PunchRotation => (Channel::Rotation, r.punch),
                    TextEffectKind::PunchScale => (Channel::Scale, Vector3::new(r.punch.x, r.punch.y, 0.0)),
                    _ => (Channel::Offset, Vector3::new(r.punch.x, r.punch.y, 0.0)),
                };
                check_finite(strength, "punch")?;
                check_finite(r.elasticity, "elasticity")?;
                punch(channel, strength, r.duration, r.vibrato, r.elasticity)?
            }
            TextEffectKind::ShakeRotation => {
                check_finite(r.strength, "strength")?;
                check_finite(r.randomness, "randomness")?;
                Effect::Shake3(shake(
                    [r.strength.x, r.strength.y, r.strength.z],
                    r.duration,
                    r.vibrato,
                    r.randomness,
                    r.fade_out,
                    r.randomness_mode,
                )?)
            }
            TextEffectKind::ShakeOffset | TextEffectKind::ShakeScale => {
                check_finite(Vector2::new(r.strength.x, r.strength.y), "strength")?;
                check_finite(r.randomness, "randomness")?;
                let channel = if r.kind == TextEffectKind::ShakeOffset { Channel::Offset } else { Channel::Scale };
                Effect::Shake2(
                    channel,
                    shake(
                        [r.strength.x, r.strength.y],
                        r.duration,
                        r.vibrato,
                        r.randomness,
                        r.fade_out,
                        r.randomness_mode,
                    )?,
                )
            }
        };
        Ok(Self { timing, pivot, effect })
    }
}
