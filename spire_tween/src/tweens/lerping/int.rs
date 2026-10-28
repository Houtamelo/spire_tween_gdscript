use super::*;

impl ILerpable for i64 {
    type BasicLerper = ();
    type Lerper = ();
}

impl MoveTowards for i64 {
    #[inline]
    fn move_towards(self, to: Self, abs_move: Self) -> Self {
        if self < to { Self::min(self + abs_move, to) } else { Self::max(self - abs_move, to) }
    }
}

impl BasicLerp<i64> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &i64, to: &i64, t: f64) -> i64 {
        (*from as f64 + (to - from) as f64 * t).round() as i64
    }
}

impl SpireLerp<i64> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &i64, previous_calc: &i64, new_calc: &i64) -> i64 {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(&mut self, from: &i64, to: &i64, speed: f64, step: f64) -> (i64, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

        let remaining_distance = i64::abs(to - from);
        let abs_step = i64::min(remaining_distance, max_step.floor() as i64);

        let unused_time = (max_step - abs_step as f64) / speed;
        let value = i64::move_towards(*from, *to, abs_step);

        let step_result = if max_step >= remaining_distance as f64 {
            StepResult::Finished {
                excess_time: unused_time,
            }
        } else {
            StepResult::Unfinished {
                accumulated_time: unused_time,
            }
        };

        (value, step_result)
    }
    #[inline]
    fn spire_distance(&mut self, from: &i64, to: &i64) -> f64 { i64::abs(to - from) as f64 }
}

// Impls for i32 are used just as helpers to other impls, we do not have tweens that actually hold i32 values.
// This is why `ILerpable` is not implemented for i32.
impl MoveTowards for i32 {
    #[inline]
    fn move_towards(self, to: Self, abs_move: Self) -> Self {
        if self < to { Self::min(self + abs_move, to) } else { Self::max(self - abs_move, to) }
    }
}

impl BasicLerp<i32> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &i32, to: &i32, t: f64) -> i32 {
        (*from as f64 + (to - from) as f64 * t).round() as i32
    }
}

impl SpireLerp<i32> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &i32, previous_calc: &i32, new_calc: &i32) -> i32 {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(&mut self, from: &i32, to: &i32, speed: f64, step: f64) -> (i32, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

        let remaining_distance = i32::abs(to - from);
        let abs_step = i32::min(remaining_distance, max_step.floor() as i32);

        let unused_time = (max_step - abs_step as f64) / speed;
        let value = i32::move_towards(*from, *to, abs_step);

        let step_result = if max_step >= remaining_distance as f64 {
            StepResult::Finished {
                excess_time: unused_time,
            }
        } else {
            StepResult::Unfinished {
                accumulated_time: unused_time,
            }
        };

        (value, step_result)
    }
    #[inline]
    fn spire_distance(&mut self, from: &i32, to: &i32) -> f64 { i32::abs(to - from) as f64 }
}
