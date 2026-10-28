use super::*;

impl ILerpable for Vector3 {
    type BasicLerper = ();
    type Lerper = ();
}

impl BasicLerp<Vector3> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &Vector3, to: &Vector3, t: f64) -> Vector3 {
        let from = *from;
        let to = *to;
        Vector3::lerp(from, to, t as f32)
    }
}

impl SpireLerp<Vector3> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &Vector3, previous_calc: &Vector3, new_calc: &Vector3) -> Vector3 {
        *present_at_obj + (*new_calc - *previous_calc)
    }

    fn spire_step(&mut self, from: &Vector3, to: &Vector3, speed: f64, step: f64) -> (Vector3, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

        let max_distance = from.distance_to(*to);
        let abs_step = f32::min(max_distance, max_step as f32);
        let unused_time = (max_step - abs_step as f64) / speed;
        let value = Vector3::move_toward(*from, *to, abs_step);

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
    fn spire_distance(&mut self, from: &Vector3, to: &Vector3) -> f64 { from.distance_to(*to) as f64 }
}
