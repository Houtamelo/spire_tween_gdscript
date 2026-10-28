use super::*;

impl ILerpable for f64 {
    type BasicLerper = ();
    type Lerper = ();
}

impl BasicLerp<f64> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &f64, to: &f64, t: f64) -> f64 { from + (to - from) * t }
}

impl SpireLerp<f64> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &f64, previous_calc: &f64, new_calc: &f64) -> f64 {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(&mut self, from: &f64, to: &f64, speed: f64, step: f64) -> (f64, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

        let remaining_distance = f64::abs(to - from);
        let abs_step = f64::min(remaining_distance, max_step);

        let unused_time = (max_step - abs_step) / speed;
        let value = f64::move_towards(*from, *to, abs_step);

        let step_result = if max_step >= remaining_distance {
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
    fn spire_distance(&mut self, from: &f64, to: &f64) -> f64 { f64::abs(to - from) }
}

impl MoveTowards for f64 {
    #[inline]
    fn move_towards(self, to: Self, abs_move: Self) -> Self {
        if self < to { Self::min(self + abs_move, to) } else { Self::max(self - abs_move, to) }
    }
}

// Impls for f32 are used just as helpers to other impls, we do not have tweens that actually hold f32 values.
// This is why `ILerpable` is not implemented for f32.
impl MoveTowards for f32 {
    #[inline]
    fn move_towards(self, to: Self, abs_move: Self) -> Self {
        if self < to { Self::min(self + abs_move, to) } else { Self::max(self - abs_move, to) }
    }
}

impl BasicLerp<f32> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &f32, to: &f32, t: f64) -> f32 {
        let t = t as f32;
        from + (to - from) * t
    }
}

impl SpireLerp<f32> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &f32, previous_calc: &f32, new_calc: &f32) -> f32 {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(&mut self, from: &f32, to: &f32, speed: f64, step: f64) -> (f32, StepResult) {
        let speed = speed as f32;
        let step = step as f32;

        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

        let remaining_distance = f32::abs(to - from);
        let abs_step = f32::min(remaining_distance, max_step);

        let unused_time = (max_step - abs_step) / speed;
        let value = f32::move_towards(*from, *to, abs_step);

        let step_result = if max_step >= remaining_distance {
            StepResult::Finished {
                excess_time: unused_time as f64,
            }
        } else {
            StepResult::Unfinished {
                accumulated_time: unused_time as f64,
            }
        };

        (value, step_result)
    }
    #[inline]
    fn spire_distance(&mut self, from: &f32, to: &f32) -> f64 { f32::abs(to - from) as f64 }
}
