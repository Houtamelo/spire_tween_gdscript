use std::num::NonZeroUsize;

use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};

use super::{
    super::super::{punch::punch_sample, shake_path::ShakePath},
    *,
};

#[derive(Clone, Copy)]
pub(super) struct Timing {
    pub duration: f64,
    pub ease: Ease,
    pub mode: LoopMode,
    pub phase: f64,
}

impl Timing {
    fn progress(self, elapsed: f64, index: i32) -> (f64, f64) {
        let time = (elapsed + f64::from(index) * self.phase) / self.duration;
        let cycle = time.floor();
        let mut p = time.rem_euclid(1.0);
        if self.mode == LoopMode::Yoyo && cycle.rem_euclid(2.0) >= 1.0 {
            p = 1.0 - p;
        }

        let p = self.ease.sample(p);
        let x = if self.mode == LoopMode::Incremental { cycle + p } else { p };

        (p, x)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Effect {
    Color(Color, Color),
    Fade(f64, f64),
    Offset(Vector2, Vector2),
    Rotation(Vector3, Vector3, RotateMode),
    Scale(Vector2, Vector2),
    Punch(Channel, PunchSettings),
    Shake2(Channel, ShakeSettings<2>),
    Shake3(ShakeSettings<3>),
}

#[derive(Clone, Copy)]
pub(crate) struct PunchSettings {
    pub strength: Vector3,
    pub segments: NonZeroUsize,
    pub elasticity: f64,
}

#[derive(Clone, Copy)]
pub(crate) struct ShakeSettings<const D: usize> {
    pub segments: NonZeroUsize,
    pub strength: [f64; D],
    pub randomness: f64,
    pub fade_out: bool,
    pub mode: ShakeRandomnessMode,
}

impl<const D: usize> ShakeSettings<D> {
    pub(crate) fn path(&self, seed: u64) -> ShakePath<D> {
        let mut path = ShakePath::new(self.segments, self.strength, self.randomness, self.fade_out, self.mode);
        path.regenerate(&mut Xoshiro256PlusPlus::seed_from_u64(seed));
        path
    }
}

pub(super) struct Entry {
    config:  CompiledTextEffect,
    origins: HashMap<(i64, i32), (u64, Vector2)>,
    paths2:  HashMap<i32, ShakePath<2>>,
    paths3:  HashMap<i32, ShakePath<3>>,
}

impl Entry {
    pub(super) fn reset_origins(&mut self) { self.origins.clear(); }

    pub(super) fn new(config: CompiledTextEffect) -> Self {
        Self {
            config,
            origins: HashMap::default(),
            paths2: HashMap::default(),
            paths3: HashMap::default(),
        }
    }

    pub(super) fn apply(&mut self, fx: &mut Gd<CharFxTransform>, frame: u64, seed: u64) {
        let (periodic, progress) = self
            .config
            .timing
            .progress(fx.get_elapsed_time(), fx.get_relative_index());
        let mut offset = Vector2::ZERO;
        let mut rotation = Vector3::ZERO;
        let mut scale = Vector2::ONE;
        let mut assign = |channel, value: Vector3| match channel {
            Channel::Offset => offset = Vector2::new(value.x, value.y),
            Channel::Rotation => rotation = value,
            Channel::Scale => scale += Vector2::new(value.x, value.y),
        };
        match &self.config.effect {
            Effect::Color(from, to) => {
                let color = from.lerp(*to, progress);
                let mut incoming = fx.get_color();
                if !fx.is_outline() {
                    incoming = color;
                }
                incoming.a = color.a;
                fx.set_color(incoming);
                return;
            }
            Effect::Fade(from, to) => {
                let mut color = fx.get_color();
                color.a = (*from + (*to - *from) * progress) as real;
                fx.set_color(color);
                return;
            }
            Effect::Offset(from, to) => offset = from.lerp(*to, progress as real),
            Effect::Scale(from, to) => scale = from.lerp(*to, progress as real),
            Effect::Rotation(from, to, mode) => {
                rotation = super::super::tween::rotation_at(*from, *to, progress, *mode);
            }
            Effect::Punch(channel, settings) => assign(
                *channel,
                settings.strength * punch_sample(periodic, settings.segments.get(), settings.elasticity) as real,
            ),
            Effect::Shake2(channel, settings) => {
                let key = fx.get_range().x;
                let path = self
                    .paths2
                    .entry(key)
                    .or_insert_with(|| settings.path(seed.wrapping_add(key as u64)));
                let value = path.sample(periodic);
                assign(*channel, Vector3::new(value[0] as real, value[1] as real, 0.0));
            }
            Effect::Shake3(settings) => {
                let key = fx.get_range().x;
                let path = self
                    .paths3
                    .entry(key)
                    .or_insert_with(|| settings.path(seed.wrapping_add(key as u64)));
                let value = path.sample(periodic);
                rotation = Vector3::new(value[0] as real, value[1] as real, value[2] as real);
            }
        }
        let incoming = fx.get_transform();
        // One adapter can render nested occurrences of its own tag. Their
        // CharFXTransform objects identify distinct scopes even for the same glyph.
        let key = (fx.instance_id().to_i64(), fx.get_range().x);
        let origin = self.origins.entry(key).or_insert((frame, incoming.origin));
        if fx.get_glyph_count() > 0 || origin.0 != frame {
            *origin = (frame, incoming.origin);
        }
        let pivot = render::cluster_pivot(incoming, self.config.pivot, origin.1);
        fx.set_transform(incoming * render::projected_transform(offset, rotation, scale, pivot));
    }
}
