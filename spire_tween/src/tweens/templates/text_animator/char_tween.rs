//! Batched character playback. Duration is per character; completion belongs to the range.

use anyhow::{Result, ensure};

use super::{
    super::shake_path::ShakePath,
    presets::{Effect, ShakeSettings},
    *,
};

/// Runtime data for a character range, owned by one [`SpireTween`].
/// Each character animates for `character_duration` seconds per leg. The total
/// duration includes the latest start delay and every requested leg.
pub struct CharTweenData {
    target: Gd<SpireRichTextLabel>,
    effect: Effect,
    character_duration: f64,
    ease: EaseKind,
    blendable: bool,
    characters: Vec<CharacterPlayback>,
}

#[derive(Clone, Copy)]
enum Initial {
    Color(Color),
    Alpha(f64),
    Vector2(Vector2),
    Vector3(Vector3),
}

struct CharacterPlayback {
    index: i64,
    delay: f64,
    remaining_delay: f64,
    time: f64,
    legs: i64,
    started: bool,
    finished: bool,
    sample: Option<f64>,
    initial: Option<Initial>,
    previous: Vector3,
    generated: bool,
    epoch: u64,
    path: Option<ShakePath<3>>,
}

impl CharacterPlayback {
    fn new(index: i64, delay: f64) -> Self {
        Self {
            index,
            delay,
            remaining_delay: delay,
            time: 0.0,
            legs: 0,
            started: false,
            finished: false,
            sample: None,
            initial: None,
            previous: Vector3::ZERO,
            generated: false,
            epoch: 0,
            path: None,
        }
    }

    fn restart(&mut self) {
        self.remaining_delay = self.delay;
        self.time = 0.0;
        self.legs = 0;
        self.started = false;
        self.finished = false;
        self.sample = None;
        self.generated = false;
    }

    fn apply(&mut self, renderer: &mut SpireCharRenderer, effect: Effect, blendable: bool) {
        let Some(p) = self.sample.take() else { return };
        if !renderer.is_valid_char(self.index) {
            return;
        }
        if effect.impulse_channel().is_some() && self.epoch != renderer.epoch {
            self.initial = None;
            self.previous = Vector3::ZERO;
            self.generated = false;
            self.epoch = renderer.epoch;
        }
        let index = self.index;
        let initial = *self.initial.get_or_insert_with(|| match effect {
            Effect::Color(..) => Initial::Color(renderer.get_char_color(index)),
            Effect::Fade(..) => Initial::Alpha(renderer.get_char_alpha(index)),
            Effect::Offset(..) => Initial::Vector2(renderer.get_char_offset(index)),
            Effect::Scale(..) => Initial::Vector2(renderer.get_char_scale(index)),
            Effect::Rotation(..) => Initial::Vector3(renderer.get_char_rotation(index)),
            _ => Initial::Vector3(read_channel(renderer, index, effect.impulse_channel().unwrap())),
        });
        match (effect, initial) {
            (Effect::Color(_, to), Initial::Color(from)) => {
                renderer.write(index, |c, stamp| c.set_color(from.lerp(to, p), stamp));
            }
            (Effect::Fade(_, to), Initial::Alpha(from)) => {
                renderer.write(index, |c, stamp| c.alpha.set(Some(from + (to - from) * p), stamp));
            }
            (Effect::Offset(_, to), Initial::Vector2(from)) => {
                renderer.set_char_offset(index, from.lerp(to, p as real))
            }
            (Effect::Scale(_, to), Initial::Vector2(from)) => renderer.set_char_scale(index, from.lerp(to, p as real)),
            (Effect::Rotation(_, to, mode), Initial::Vector3(from)) => {
                renderer.set_char_rotation(index, tween::rotation_at(from, to, p, mode));
            }
            (_, Initial::Vector3(from)) => {
                let channel = effect.impulse_channel().expect("impulse initial value");
                let periodic = p.rem_euclid(1.0);
                let next = match effect {
                    Effect::Punch(_, settings) => {
                        settings.strength * punch_sample(periodic, settings.segments.get(), settings.elasticity) as real
                    }
                    Effect::Shake2(_, settings) => self.shake(settings.into_3d(), periodic),
                    Effect::Shake3(settings) => self.shake(settings, periodic),
                    _ => unreachable!("basic effect initial value has its matching type"),
                };
                let value =
                    if blendable { read_channel(renderer, index, channel) - self.previous + next } else { from + next };
                self.previous = next;
                self.generated = true;
                write_channel(renderer, index, channel, value);
                if self.finished {
                    self.cleanup(renderer, channel, blendable);
                }
            }
            _ => unreachable!("initial value is captured from this effect"),
        }
    }

    fn shake(&mut self, settings: ShakeSettings<3>, progress: f64) -> Vector3 {
        if !self.generated {
            self.path = Some(settings.path(rand::random()));
        }
        let value = self
            .path
            .as_ref()
            .expect("shake path generated before sampling")
            .sample(progress);
        Vector3::new(value[0] as real, value[1] as real, value[2] as real)
    }

    fn cleanup(&mut self, renderer: &mut SpireCharRenderer, channel: Channel, blendable: bool) {
        if self.epoch != renderer.epoch || !self.generated || !renderer.is_valid_char(self.index) {
            return;
        }
        let value = if blendable {
            read_channel(renderer, self.index, channel) - self.previous
        } else if let Some(Initial::Vector3(initial)) = self.initial {
            initial
        } else {
            return
        };
        write_channel(renderer, self.index, channel, value);
        self.previous = Vector3::ZERO;
    }
}

fn read_channel(renderer: &SpireCharRenderer, index: i64, channel: Channel) -> Vector3 {
    match channel {
        Channel::Offset => {
            let v = renderer.get_char_offset(index);
            Vector3::new(v.x, v.y, 0.0)
        }
        Channel::Scale => {
            let v = renderer.get_char_scale(index);
            Vector3::new(v.x, v.y, 0.0)
        }
        Channel::Rotation => renderer.get_char_rotation(index),
    }
}

fn write_channel(renderer: &mut SpireCharRenderer, index: i64, channel: Channel, value: Vector3) {
    match channel {
        Channel::Offset => renderer.set_char_offset(index, Vector2::new(value.x, value.y)),
        Channel::Scale => renderer.set_char_scale(index, Vector2::new(value.x, value.y)),
        Channel::Rotation => renderer.set_char_rotation(index, value),
    }
}

impl Effect {
    pub(super) fn impulse_channel(self) -> Option<Channel> {
        match self {
            Self::Punch(channel, _) | Self::Shake2(channel, _) => Some(channel),
            Self::Shake3(_) => Some(Channel::Rotation),
            _ => None,
        }
    }
}

impl ShakeSettings<2> {
    fn into_3d(self) -> ShakeSettings<3> {
        ShakeSettings {
            strength: [self.strength[0], self.strength[1], 0.0],
            segments: self.segments,
            randomness: self.randomness,
            fade_out: self.fade_out,
            mode: self.mode,
        }
    }
}

/// Validate and snapshot the curve at the selected indices. The curve supplies
/// non-negative start delays in multiples of each character's duration.
pub(crate) fn resolve_char_start_delays(first: i64, last: i64, duration: f64, curve: &EaseKind) -> Result<Vec<f64>> {
    ensure!(
        first == last || first >= 0 && last >= first,
        "`first` and `last` must form a non-empty range."
    );
    ensure!(duration.is_finite() && duration > 0.0, "`duration` must be finite and positive.");
    let count = last
        .checked_sub(first)
        .and_then(|n| n.checked_add(1))
        .and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| anyhow::anyhow!("`first` and `last` must form a representable range."))?;
    let mut delays = Vec::new();
    delays.try_reserve_exact(count)?;
    for ordinal in 0..count {
        let x = if count == 1 { 0.0 } else { ordinal as f64 / (count - 1) as f64 };
        // Start times are sampled once: retain authored Curve points exactly
        // instead of approximating peaks through EaseKind's per-frame bake.
        let factor = match curve {
            EaseKind::GodotCurve(curve) => curve.sample(x as f32) as f64,
            _ => curve.sample(x),
        };
        let delay = factor * duration;
        ensure!(
            factor.is_finite() && factor >= 0.0 && delay.is_finite() && (delay + duration).is_finite(),
            "`delay_curve` samples must be finite and non-negative and produce finite completion times."
        );
        delays.push(delay);
    }
    Ok(delays)
}

impl CharTweenData {
    pub(super) fn build(
        target: Gd<SpireRichTextLabel>,
        first: i64,
        last: i64,
        duration: f64,
        curve: EaseKind,
        effect: Effect,
        blendable: bool,
    ) -> SpireTween<Self> {
        let duration = duration_or_default(duration);
        let last = if last == -1 && first != -1 {
            target.upcast_ref::<RichTextLabel>().get_total_character_count() as i64 - 1
        } else {
            last
        };
        let result = (|| -> Result<Vec<f64>> {
            if first != last {
                ensure!(
                    last < target.upcast_ref::<RichTextLabel>().get_total_character_count() as i64,
                    "`last` must select an existing character or be -1."
                );
            }
            resolve_char_start_delays(first, last, duration, &curve)
        })();
        let characters = match result {
            Ok(delays) => delays
                .into_iter()
                .enumerate()
                .map(|(ordinal, delay)| CharacterPlayback::new(first + ordinal as i64, delay))
                .collect(),
            Err(error) => {
                godot_warn!("[spire characters] {error}");
                Vec::new()
            }
        };
        SpireTween::new_with_data(Self {
            target,
            effect,
            character_duration: duration,
            ease: EaseKind::default(),
            blendable,
            characters,
        })
    }
}

impl SpireTween<CharTweenData> {
    /// Seconds per animation leg for each character, excluding its start delay.
    pub fn get_character_duration(&self) -> f64 { self.t.character_duration }
    /// Total nominal range duration, including all start delays and requested
    /// legs. Returns infinity for infinite looping. Pauses and deliberate
    /// one-loop-per-update stalls can extend wall-clock completion.
    pub fn get_duration(&self) -> f64 {
        if self.loop_max < 0 {
            return f64::INFINITY;
        }
        self.t
            .characters
            .iter()
            .map(|c| c.delay + self.t.character_duration * self.loop_max.max(1) as f64)
            .fold(0.0, f64::max)
    }
    pub fn get_ease(&self) -> &EaseKind { &self.t.ease }
    pub fn set_ease(&mut self, ease: EaseKind) { self.t.ease = ease; }
    pub fn with_ease(mut self, ease: EaseKind) -> Self {
        self.set_ease(ease);
        self
    }
    pub fn get_owner(&self) -> &Gd<SpireRichTextLabel> { &self.t.target }
}

impl SpireTweener for SpireTween<CharTweenData> {
    fn play_internal(&mut self) {
        if self.state == State::Stopped {
            self.reset_counters();
            for character in &mut self.t.characters {
                character.restart();
            }
        }
    }
    fn pause_internal(&mut self) {}
    fn stop_internal(&mut self) {
        for character in &mut self.t.characters {
            character.generated = false;
        }
    }
    fn force_complete_internal(&mut self) -> bool {
        if !self.t.target.is_instance_valid() || self.t.characters.is_empty() {
            return false;
        }
        for character in &mut self.t.characters {
            if character.finished {
                continue;
            }
            let leg = if self.loop_max < 0 { character.legs } else { self.loop_max.max(1) - 1 };
            character.sample = Some(calc_animation_position(
                self.t.character_duration,
                self.t.character_duration,
                leg,
                self.loop_mode,
                &self.t.ease,
            ));
        }
        if !self.t.target.is_instance_valid() {
            return false;
        }
        let mut renderer = self.t.target.bind().char_renderer.clone();
        {
            let mut renderer = renderer.bind_mut();
            for character in &mut self.t.characters {
                if character.finished {
                    continue;
                }
                character.finished = true;
                character.apply(&mut renderer, self.t.effect, self.t.blendable);
            }
        }
        self.t.target.queue_redraw();
        true
    }
    fn process_internal(&mut self, delta_time: f64, _is_tree_paused: bool) -> AdvanceTimeResult {
        if !self.t.target.is_instance_valid() || self.t.characters.is_empty() {
            return AdvanceTimeResult::ShouldDespawn;
        }
        let owner = ObjectOrNode::Node(self.t.target.clone().upcast());
        match self.check_owner_validity_and_pause_mode(&owner) {
            NodesValidityResult::CanProcess => {}
            NodesValidityResult::DontProcess => return AdvanceTimeResult::Paused,
            NodesValidityResult::SomeNodesDead => return AdvanceTimeResult::ShouldDespawn,
        }
        let Some(step) = self.handle_time_step(delta_time) else { return AdvanceTimeResult::Playing };
        // A previous update may have carried excess across a loop boundary.
        // Preserve it even when this update supplies zero new time.
        let mut shared_excess = f64::INFINITY;
        for character in &mut self.t.characters {
            character.sample = None;
            if character.finished && (self.loop_max < 0 || character.legs < self.loop_max.max(1)) {
                character.finished = false;
            }
            if character.finished {
                continue;
            }
            let local_step = if !character.started {
                character.remaining_delay -= step;
                if character.remaining_delay > 0.0 {
                    continue;
                }
                character.started = true;
                -character.remaining_delay
            } else {
                step
            };
            character.time += local_step;
            character.sample = Some(calc_animation_position(
                self.t.character_duration,
                character.time,
                character.legs,
                self.loop_mode,
                &self.t.ease,
            ));
            if character.time >= self.t.character_duration {
                character.legs += 1;
                let excess = character.time - self.t.character_duration;
                if self.loop_max >= 0 && character.legs >= self.loop_max.max(1) {
                    character.finished = true;
                    shared_excess = shared_excess.min(excess);
                }
                character.time = excess;
            }
        }
        // Callable easing runs before borrowing the renderer, just as method
        // tweens evaluate easing before invoking their update callback.
        if !self.t.target.is_instance_valid() {
            return AdvanceTimeResult::ShouldDespawn;
        }
        let mut renderer = self.t.target.bind().char_renderer.clone();
        {
            let mut renderer = renderer.bind_mut();
            for character in &mut self.t.characters {
                character.apply(&mut renderer, self.t.effect, self.t.blendable);
            }
        }
        self.t.target.queue_redraw();
        let legs = self.t.characters.iter().map(|c| c.legs).min().unwrap_or(0);
        let finished = self.t.characters.iter().all(|c| c.finished);
        if legs > self.loop_counter {
            self.handle_loop_finished(if finished { shared_excess } else { 0.0 });
        }
        if finished {
            AdvanceTimeResult::Completed {
                unscaled_excess_time: unscale_time(shared_excess, self.speed_scale),
            }
        } else {
            AdvanceTimeResult::Playing
        }
    }
}

/// A character range with one tween handle. Constructor `duration` is seconds
/// per character per leg; [method get_duration] reports the whole range duration.
/// Each character's initial delay applies once, preserving continuous travelling
/// loops. [signal loop_finished] fires when all characters complete that leg.
#[derive(GodotClass)]
#[class(base = RefCounted, no_init)]
pub struct SpireCharTween {
    pub base:  Base<RefCounted>,
    pub inner: UnsafeCell<TweenPtr<CharTweenData>>,
}

define_base_gd_methods! { SpireCharTween, SpireTween<CharTweenData>, CharTweenData }

#[godot_api(secondary)]
impl SpireCharTween {
    /// Seconds per animation leg for each character, excluding its start delay.
    #[func]
    pub fn get_character_duration(&self) -> f64 { self.to_ref().get_character_duration() }
    /// Total nominal range duration, including character start delays and all
    /// requested legs. Infinite looping returns infinity.
    #[func]
    pub fn get_duration(&self) -> f64 { self.to_ref().get_duration() }
    #[func]
    pub fn get_ease(&self) -> EaseKind { self.to_ref().get_ease().clone() }
    /// Set easing for each character after its individual start delay.
    #[func(gd_self)]
    pub fn set_ease(this: Gd<Self>, easing: Ease) -> Gd<Self> {
        this.bind().to_mut().set_ease(EaseKind::Basic(easing));
        this
    }
    /// Set each character's temporal easing curve. This is independent of the
    /// spatial curve that determines character start delays.
    #[func(gd_self)]
    pub fn set_ease_curve(this: Gd<Self>, curve: Option<Gd<Curve>>) -> Gd<Self> {
        this.bind()
            .to_mut()
            .set_ease(curve.map_or_else(EaseKind::default, EaseKind::GodotCurve));
        this
    }
    #[func(gd_self)]
    pub fn set_ease_func(this: Gd<Self>, callable: Callable) -> Gd<Self> {
        this.bind().to_mut().set_ease(EaseKind::Callable(callable));
        this
    }
}
