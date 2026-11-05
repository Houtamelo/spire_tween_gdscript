use std::rc::Rc;

use super::*;

/// Values with an invertible additive contribution. Quaternion contributions
/// compose on the right; their order matters, unlike vector addition.
pub(crate) trait EffectValue: Copy + 'static {
    const ZERO: Self;
    fn replace(current: Self, previous: Self, next: Self) -> Self;
    fn offset(initial: Self, next: Self) -> Self { Self::replace(initial, Self::ZERO, next) }
}
macro_rules! additive {
    ($($ty:ty),*) => {$(impl EffectValue for $ty {
        const ZERO: Self = <$ty>::ZERO;
        fn replace(current: Self, previous: Self, next: Self) -> Self { current - previous + next }
    })*};
}
additive!(Vector2, Vector3);
impl EffectValue for f64 {
    const ZERO: Self = 0.0;
    fn replace(current: Self, previous: Self, next: Self) -> Self { current - previous + next }
}
impl EffectValue for Quaternion {
    const ZERO: Self = Quaternion::IDENTITY;
    fn replace(current: Self, previous: Self, next: Self) -> Self { (current * previous.inverse() * next).normalized() }
}

struct EffectState<V> {
    initial: Option<V>,
    previous: V,
    generated: bool,
    epoch: u64,
}

pub(crate) fn duration_or_default(duration: f64) -> f64 {
    if duration.is_finite() && duration > 0.0 {
        duration
    } else {
        godot_warn!("[spire effect] duration must be finite and positive, got {duration}; using 1 second.");
        1.0
    }
}

pub(crate) fn segment_count(vibrato: i32, duration: f64) -> std::num::NonZeroUsize {
    let requested = (f64::from(vibrato) * duration).floor();
    let value = requested.clamp(2.0, 65536.0) as usize;
    if vibrato <= 0 || !requested.is_finite() || requested > 65536.0 {
        godot_warn!("[spire effect] invalid segment count {requested}; using {value} (allowed 2 to 65536).");
    }
    std::num::NonZeroUsize::new(value).unwrap_or(std::num::NonZeroUsize::MIN.saturating_add(1))
}

pub(crate) fn finite(value: f64, fallback: f64, name: &str) -> f64 {
    if value.is_finite() {
        value
    } else {
        godot_warn!("[spire template] {name} must be finite, got {value}; using {fallback}.");
        fallback
    }
}

pub(crate) fn unit(value: f64, fallback: f64, name: &str) -> f64 {
    let value = finite(value, fallback, name);
    if !(0.0..=1.0).contains(&value) {
        godot_warn!("[spire template] {name} must be in [0, 1], got {value}; clamping.");
    }
    value.clamp(0.0, 1.0)
}

/// Shares only template state, never a pointer to the processing tween.
pub(crate) fn effect<V: EffectValue>(
    owner: Option<Gd<Object>>,
    getter: impl Fn() -> V + 'static,
    setter: impl FnMut(V) + 'static,
    sample: impl FnMut(f64, bool) -> V + 'static,
    duration: f64,
    blendable: bool,
) -> SpireTween<LerpMethodData<f64>> {
    effect_with_epoch(owner, getter, setter, sample, duration, blendable, None)
}

// A rebuilt target has already discarded the previous contribution. Reset the
// effect's bookkeeping before sampling/restoring it, without changing its clock.
pub(crate) fn effect_with_epoch<V: EffectValue>(
    owner: Option<Gd<Object>>,
    getter: impl Fn() -> V + 'static,
    setter: impl FnMut(V) + 'static,
    mut sample: impl FnMut(f64, bool) -> V + 'static,
    duration: f64,
    blendable: bool,
    epoch: Option<Rc<dyn Fn() -> u64>>,
) -> SpireTween<LerpMethodData<f64>> {
    let state = RcPtr::new(EffectState {
        initial: None,
        previous: V::ZERO,
        generated: false,
        epoch: 0,
    });
    let getter = Rc::new(getter);
    let setter = RcPtr::new(setter);
    let callable = Callable::from_fn("spire_effect", {
        let mut state = state.clone();
        let getter = getter.clone();
        let mut setter = setter.clone();
        let epoch = epoch.clone();
        move |args| {
            let current_epoch = epoch.as_ref().map_or(0, |read| read());
            if state.epoch != current_epoch {
                state.initial = None;
                state.previous = V::ZERO;
                state.generated = false;
                state.epoch = current_epoch;
            }
            let current = getter();
            let initial = *state.initial.get_or_insert(current);
            let next = sample(args[0].to::<f64>().rem_euclid(1.0), !state.generated);
            let value = if blendable { V::replace(current, state.previous, next) } else { V::offset(initial, next) };
            state.generated = true;
            state.previous = next;
            setter(value);
            Variant::nil()
        }
    });
    let mut tween = SpireTween::<LerpMethodData<f64>>::new(callable, 0.0, 1.0, duration_or_default(duration));
    tween.t.owner = owner.clone().map(ObjectOrNode::from_unchecked_object);
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
            let mut setter = setter;
            move || {
                if owner.as_ref().is_some_and(|owner| !owner.is_instance_valid()) {
                    return;
                }
                let current = if blendable || epoch.is_some() { getter() } else { V::ZERO };
                if epoch.as_ref().is_some_and(|read| read() != state.epoch) {
                    state.initial = None;
                    state.previous = V::ZERO;
                    return;
                }
                let value = if blendable { Some(V::replace(current, state.previous, V::ZERO)) } else { state.initial };
                state.previous = V::ZERO;
                if let Some(value) = value {
                    setter(value);
                }
            }
        },
        SpireFlags::empty(),
    );
    tween
}
