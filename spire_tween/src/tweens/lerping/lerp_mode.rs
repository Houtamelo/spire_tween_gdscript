use super::*;

/// Selects how a property tween computes its per-tick target value.
///
/// Switch between modes via the `as_*` builders / `set_*` methods on
/// [`SpireTween<LerpPropertyData<T>>`]. New tweens default to [`Self::Absolute`].
#[derive(Debug, Clone)]
pub enum LerpMode<T> {
    /// Default mode. Fixed duration. The current value each tick is
    /// `from + (to - from) * ease(elapsed / duration)`.
    ///
    /// `from` is lazily read from the property on the first tick unless explicitly
    /// set via [`SpireTween::<LerpPropertyData<T>>::set_begin_value`] /
    /// [`begin_from`](SpireTween::<LerpPropertyData<T>>::begin_from).
    Absolute {
        duration: f64,
        from: FromValue<T>,
    },
    /// Constant speed (units/second) instead of fixed duration. Each tick "bumps" the
    /// current value toward `to` by `speed * delta_time`. Composes well with other
    /// forces affecting the same property; never overshoots.
    ///
    /// `start_distance` is cached on the first tick to compute progress for easing;
    /// `step_sum` is the integer-stepping fuel reservoir (see [`SpireLerp::spire_step`]).
    SpeedBased {
        speed: f64,
        from: FromValue<T>,
        start_distance: Option<f64>,
        step_sum: f64,
    },
    /// Each tick adds an offset (the delta between this tick's lerp point and the
    /// previous tick's lerp point) to the property's *current* value. Blends with
    /// external forces and with other relative tweens on the same property.
    ///
    /// `relative_to` is the conceptual zero — passed at construction as
    /// `T::default()` by [`SpireTween::<LerpPropertyData<T>>::as_relative`].
    /// `previous_anim_pos` tracks the previous frame's eased progress.
    Relative {
        duration: f64,
        relative_to: T,
        previous_anim_pos: f64,
    },
}

impl<T> LerpMode<T> {
    /// Builds an [`Absolute`](Self::Absolute) mode with `from` left as
    /// [`FromValue::PendingEvaluation`] (read from the property on first tick).
    #[inline]
    pub fn absolute(duration: f64) -> Self {
        LerpMode::Absolute {
            duration,
            from: FromValue::PendingEvaluation,
        }
    }

    /// Builds a [`SpeedBased`](Self::SpeedBased) mode with no cached start distance
    /// and an empty fuel reservoir.
    #[inline]
    pub fn speed_based(speed: f64) -> Self {
        LerpMode::SpeedBased {
            speed,
            from: FromValue::PendingEvaluation,
            start_distance: None,
            step_sum: 0.,
        }
    }

    /// Builds a [`Relative`](Self::Relative) mode anchored at `relative_to` with
    /// `previous_anim_pos = 0`.
    #[inline]
    pub fn relative(duration: f64, relative_to: T) -> Self {
        LerpMode::Relative {
            duration,
            relative_to,
            previous_anim_pos: 0.,
        }
    }

    /// Resets per-tick caches so the next play behaves like a fresh start:
    /// re-evaluates `from` (when not [`FromValue::Explicit`]), clears
    /// `start_distance` / `step_sum` (speed-based), zeroes `previous_anim_pos`
    /// (relative). Called automatically by `play()` on a stopped tween, and on each
    /// loop in [`LoopMode::Restart`].
    #[inline]
    pub fn reset_state(&mut self) {
        match self {
            LerpMode::Absolute { duration: _, from } => {
                from.reset_if_not_explicit();
            }
            LerpMode::SpeedBased {
                speed: _,
                from,
                start_distance,
                step_sum,
            } => {
                from.reset_if_not_explicit();
                *start_distance = None;
                *step_sum = 0.;
            }
            LerpMode::Relative {
                duration: _,
                relative_to: _,
                previous_anim_pos,
            } => {
                *previous_anim_pos = 0.;
            }
        }
    }
}

/// Tracks the starting value of a property tween across its lifetime.
///
/// Used inside [`LerpMode::Absolute`] and [`LerpMode::SpeedBased`] to model the
/// "lazy first read" behavior: by default a tween reads `from` off the property the
/// first time it ticks, but the user can override that with an explicit value.
#[derive(Debug, Clone)]
pub enum FromValue<T> {
    /// No starting value yet — will be read from the property on the first tick.
    PendingEvaluation,
    /// Cached from a previous [`PendingEvaluation`](Self::PendingEvaluation) read.
    /// Reset back to `PendingEvaluation` when the tween restarts (so each
    /// [`LoopMode::Restart`] loop re-reads the property).
    AlreadyEvaluated(T),
    /// Explicitly set by the user (e.g. via
    /// [`SpireTween::<LerpPropertyData<T>>::set_begin_value`] /
    /// [`begin_from`](SpireTween::<LerpPropertyData<T>>::begin_from)). Survives
    /// restarts.
    Explicit(T),
}

impl<T> FromValue<T> {
    /// Caches `value`, transitioning to [`AlreadyEvaluated`](Self::AlreadyEvaluated),
    /// and returns a reference to the cached value.
    #[inline]
    pub fn set_evaluated(&mut self, value: T) -> &T {
        *self = FromValue::AlreadyEvaluated(value);
        if let FromValue::AlreadyEvaluated(value_ref) = self { value_ref } else { unreachable!() }
    }

    /// Reverts an [`AlreadyEvaluated`](Self::AlreadyEvaluated) cache back to
    /// [`PendingEvaluation`](Self::PendingEvaluation). Leaves
    /// [`Explicit`](Self::Explicit) values untouched. Called between loops in
    /// [`LoopMode::Restart`] to force a fresh read of the property.
    #[inline]
    pub fn reset_if_not_explicit(&mut self) {
        if let FromValue::AlreadyEvaluated(_) = self {
            *self = FromValue::PendingEvaluation;
        }
    }

    /// Returns the cached value if there is one, otherwise invokes `evaluator`,
    /// caches its result, and returns a reference to it.
    #[inline]
    pub fn get_or_evaluate(&mut self, evaluator: impl FnOnce() -> T) -> &T {
        match self {
            FromValue::PendingEvaluation => self.set_evaluated(evaluator()),
            FromValue::AlreadyEvaluated(value_ref) => value_ref,
            FromValue::Explicit(value_ref) => value_ref,
        }
    }
}
