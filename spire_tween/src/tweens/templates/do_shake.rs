use std::num::NonZeroUsize;

use godot::obj::bounds::DeclEngine;
use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};

use super::{shake_path::ShakePath, *};

const MIN_SEGMENTS: NonZeroUsize = NonZeroUsize::MIN.saturating_add(1);
const MAX_SEGMENTS: usize = 65_536;

/// Linearly interpolated local-position shake for `Node2D` and its subclasses.
///
/// Uses a planar sampler implemented from the shake mathematical specification.
/// Its timing, direction distribution and amplitude rules match the intended
/// DOTween-compatible behavior without translating its source implementation.
/// `strength` selects the per-axis shaping; waypoint magnitude is its length,
/// so components are not hard bounds after DOTween's renormalization.
/// `vibrato * duration`, truncated and clamped to `[2, 65536]`, is the segment count.
/// Nonpositive vibrato and counts above the upper bound emit warnings. Short
/// animations with positive vibrato use the minimum of two without a warning.
/// `randomness` is an angle in degrees in `[0, 180]`; see [`ShakeRandomnessMode`].
///
/// `fade_out` decreases waypoint magnitude and increases segment durations;
/// disabling it uses constant amplitude and equal segment durations. Both end
/// at zero offset. The returned method tween follows the global default easing
/// unless explicitly overridden. Easing remaps overall path progress.
/// `blendable` applies `current - previous_offset + next_offset`, preserving
/// additive movement and other blendable shakes. Absolute property writers can
/// discard the tracked contribution. When false, the start is captured on first
/// processing and each write is `start + offset`, including on replay.
///
/// Generates one path per playback. Restart and Incremental repeat that path
/// without drift; Yoyo retraces it. Samples the method value modulo one, so
/// easing overshoot wraps around the path. Keep the method's range at `0 -> 1`.
/// Replaying after stop generates a new path; pause/resume keeps the current one.
/// `stop()` leaves the node untouched. An immediate `finished` connection restores
/// the zero offset on completion, including `force_complete()`, before later
/// finish handlers. Restoration is not guaranteed before `stopped` handlers.
/// Calling [`SpireTween::finished_clear_connections`] disables cleanup;
/// [`SpireTween::stopped_clear_connections`] disables path invalidation for replay.
/// Freeing the owner cancels without invoking the position accessors.
///
/// Returns an unregistered builder. Invalid parameters emit warnings:
/// nonpositive/nonfinite duration becomes 1 second; negative or nonfinite
/// strength components become zero; finite randomness is clamped to `[0, 180]`,
/// and nonfinite randomness becomes 90 degrees.
/// Rust passes every option explicitly; GDScript defaults `vibrato` to 10,
/// `randomness` to 90, `fade_out` and `blendable` to true, and mode to Full.
pub trait DoShakeNode2D<Marker = ()> {
    fn do_shake(
        &self,
        duration: f64,
        strength: Vector2,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        randomness_mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
}

impl<T: Inherits<Node2D> + Inherits<Object>> DoShakeNode2D<()> for Gd<T> {
    fn do_shake(
        &self,
        duration: f64,
        strength: Vector2,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        randomness_mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        do_shake(
            self.clone().upcast::<Node2D>(),
            Node2D::get_position,
            Node2D::set_position,
            duration,
            strength,
            vibrato,
            randomness,
            fade_out,
            randomness_mode,
            blendable,
        )
    }
}

/// [`DoShakeNode2D`] for `Control` nodes. Same arguments, loop, blending,
/// completion and parameter normalization semantics. Container layout can overwrite position.
pub trait DoShakeControl<Marker = ()> {
    fn do_shake(
        &self,
        duration: f64,
        strength: Vector2,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        randomness_mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
}

impl<T: Inherits<Control> + Inherits<Object>> DoShakeControl<()> for Gd<T> {
    fn do_shake(
        &self,
        duration: f64,
        strength: Vector2,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        randomness_mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        do_shake(
            self.clone().upcast::<Control>(),
            Control::get_position,
            Control::set_position,
            duration,
            strength,
            vibrato,
            randomness,
            fade_out,
            randomness_mode,
            blendable,
        )
    }
}

#[derive(Default)]
struct ShakeState {
    generated: bool,
    initial_position: Option<Vector2>,
    previous_offset: Vector2,
}

fn do_shake<T: GodotClass<Declarer = DeclEngine> + Inherits<Object>>(
    node: Gd<T>,
    get_pos: fn(&T) -> Vector2,
    set_pos: fn(&mut T, Vector2),
    duration: f64,
    strength: Vector2,
    vibrato: i32,
    randomness: real,
    fade_out: bool,
    randomness_mode: ShakeRandomnessMode,
    blendable: bool,
) -> SpireTween<LerpMethodData<f64>> {
    let rng = Xoshiro256PlusPlus::try_from_os_rng().unwrap_or_else(|_| {
        godot_warn!("[do_shake] Failed to acquire OS randomness, falling back to fixed seed.");
        Xoshiro256PlusPlus::seed_from_u64(1337)
    });
    do_shake_internal(
        node,
        get_pos,
        set_pos,
        duration,
        strength,
        vibrato,
        randomness,
        fade_out,
        randomness_mode,
        blendable,
        rng,
    )
}

// Kept inside this private module; only integration-test builds re-export it.
#[doc(hidden)]
pub fn do_shake_internal<T: GodotClass<Declarer = DeclEngine> + Inherits<Object>>(
    node: Gd<T>,
    get_pos: fn(&T) -> Vector2,
    set_pos: fn(&mut T, Vector2),
    duration: f64,
    strength: Vector2,
    vibrato: i32,
    randomness: real,
    fade_out: bool,
    randomness_mode: ShakeRandomnessMode,
    blendable: bool,
    mut rng: Xoshiro256PlusPlus,
) -> SpireTween<LerpMethodData<f64>> {
    let duration = if duration.is_finite() && duration > 0.0 {
        duration
    } else {
        godot_warn!("[do_shake] duration must be finite and positive; using 1 second instead of {duration}.");
        1.0
    };

    let strength = if strength.is_finite() && strength.x >= 0.0 && strength.y >= 0.0 {
        strength
    } else {
        godot_warn!(
            "[do_shake] strength must be finite and nonnegative; replacing invalid components of {strength} with zero."
        );
        Vector2::new(
            if strength.x.is_finite() { strength.x.max(0.0) } else { 0.0 },
            if strength.y.is_finite() { strength.y.max(0.0) } else { 0.0 },
        )
    };

    let randomness = if randomness.is_finite() && (0.0..=180.0).contains(&randomness) {
        randomness
    } else {
        let corrected = if randomness.is_finite() { randomness.clamp(0.0, 180.0) } else { 90.0 };
        godot_warn!(
            "[do_shake] randomness must be between 0 and 180 degrees; using {corrected} instead of {randomness}."
        );
        corrected
    };

    let owner = node.clone().upcast::<Object>();
    let requested_segments = (f64::from(vibrato) * duration).floor();
    let segment_count = requested_segments.clamp(MIN_SEGMENTS.get() as f64, MAX_SEGMENTS as f64) as usize;
    let segments = NonZeroUsize::new(segment_count).unwrap_or(MIN_SEGMENTS);
    if vibrato <= 0 || !requested_segments.is_finite() || requested_segments > MAX_SEGMENTS as f64 {
        godot_warn!(
            "[do_shake] invalid segment count {requested_segments} from vibrato {vibrato}; using {segments} (allowed \
             2 to {MAX_SEGMENTS})."
        );
    }

    let mut path =
        ShakePath::new(segments, [strength.x as f64, strength.y as f64], randomness as f64, fade_out, randomness_mode);
    let state = RcPtr::new(ShakeState::default());

    let callable = Callable::from_fn("do_shake", {
        let mut node = node.clone();
        let mut state = state.clone();

        move |args| {
            let current = get_pos(&node);
            let position = {
                let initial = *state.initial_position.get_or_insert(current);
                if !state.generated {
                    path.regenerate(&mut rng);
                    state.generated = true;
                }

                let phase = args[0].to::<f64>().rem_euclid(1.0);
                let offset = path.sample(phase);
                let offset = Vector2::new(offset[0] as real, offset[1] as real);
                let position = if blendable { current - state.previous_offset + offset } else { initial + offset };
                state.previous_offset = offset;
                position
            };
            set_pos(&mut node, position);
            Variant::nil()
        }
    });

    // Let the ordinary method tween easing remap overall path progress.
    let mut tween = SpireTween::<LerpMethodData<f64>>::new(callable, 0.0, 1.0, duration);
    tween.t.owner = Some(ObjectOrNode::from_unchecked_object(owner));

    tween.stopped_connect(
        {
            let mut state = state.clone();
            move || state.generated = false
        },
        SpireFlags::empty(),
    );

    tween.finished_connect(
        {
            let mut state = state;
            let mut node = node;

            move || {
                if !node.is_instance_valid() {
                    return;
                }

                let position = {
                    let position =
                        if blendable { Some(get_pos(&node) - state.previous_offset) } else { state.initial_position };

                    state.previous_offset = Vector2::ZERO;
                    position
                };

                if let Some(position) = position {
                    set_pos(&mut node, position);
                }
            }
        },
        SpireFlags::empty(),
    );

    tween
}
