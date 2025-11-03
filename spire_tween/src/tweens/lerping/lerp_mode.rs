use super::*;

#[derive(Debug, Clone)]
pub enum LerpMode<T> {
    Absolute {
        duration: f64,
        from: FromValue<T>,
    },
    SpeedBased {
        speed: f64,
        from: FromValue<T>,
        start_distance: Option<f64>,
        step_sum: f64,
    },
    Relative {
        duration: f64,
        relative_to: T,
        previous_anim_pos: f64,
    },
}

impl<T> LerpMode<T> {
    #[inline]
    pub fn absolute(duration: f64) -> Self {
        LerpMode::Absolute {
            duration,
            from: FromValue::PendingEvaluation,
        }
    }

    #[inline]
    pub fn speed_based(speed: f64) -> Self {
        LerpMode::SpeedBased {
            speed,
            from: FromValue::PendingEvaluation,
            start_distance: None,
            step_sum: 0.,
        }
    }

    #[inline]
    pub fn relative(duration: f64, relative_to: T) -> Self {
        LerpMode::Relative {
            duration,
            relative_to,
            previous_anim_pos: 0.,
        }
    }

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

#[derive(Debug, Clone)]
pub enum FromValue<T> {
    PendingEvaluation,
    AlreadyEvaluated(T),
    Explicit(T),
}

impl<T> FromValue<T> {
    #[inline]
    pub fn set_evaluated(&mut self, value: T) -> &T {
        *self = FromValue::AlreadyEvaluated(value);
        if let FromValue::AlreadyEvaluated(value_ref) = self {
            value_ref
        } else {
            unreachable!()
        }
    }

    #[inline]
    pub fn reset_if_not_explicit(&mut self) {
        if let FromValue::AlreadyEvaluated(_) = self {
            *self = FromValue::PendingEvaluation;
        }
    }

    #[inline]
    pub fn get_or_evaluate(&mut self, evaluator: impl FnOnce() -> T) -> &T {
        match self {
            FromValue::PendingEvaluation => self.set_evaluated(evaluator()),
            FromValue::AlreadyEvaluated(value_ref) => value_ref,
            FromValue::Explicit(value_ref) => value_ref,
        }
    }
}
