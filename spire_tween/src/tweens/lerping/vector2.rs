use super::*;

impl ILerpable for Vector2 {
    type BasicLerper = ();
    type Lerper = ();
}

impl BasicLerp<Vector2> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &Vector2, to: &Vector2, weight: f64) -> Vector2 {
        let from = *from;
        let to = *to;
        Vector2::lerp(from, to, weight as f32)
    }
}

impl SpireLerp<Vector2> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &Vector2, previous_calc: &Vector2, new_calc: &Vector2) -> Vector2 {
        *present_at_obj + (*new_calc - *previous_calc)
    }

    fn spire_step(&mut self, from: &Vector2, to: &Vector2, speed: f64, step: f64) -> (Vector2, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

        let max_distance = from.distance_to(*to);
        let abs_step = f32::min(max_distance, max_step as f32);

        let unused_time = (max_step - abs_step as f64) / speed;
        let value = Vector2::move_toward(*from, *to, abs_step);

        let step_result = if abs_step >= max_distance {
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
    fn spire_distance(&mut self, from: &Vector2, to: &Vector2) -> f64 { from.distance_to(*to) as f64 }
}
