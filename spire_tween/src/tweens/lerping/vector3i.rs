use super::*;

impl ILerpable for Vector3i {
    type BasicLerper = ();
    type Lerper = ();
}

impl BasicLerp<Vector3i> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &Vector3i, to: &Vector3i, weight: f64) -> Vector3i {
        Vector3i {
            x: self.spire_lerp(&from.x, &to.x, weight),
            y: self.spire_lerp(&from.y, &to.y, weight),
            z: self.spire_lerp(&from.z, &to.z, weight),
        }
    }
}

impl SpireLerp<Vector3i> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &Vector3i, previous_calc: &Vector3i, new_calc: &Vector3i) -> Vector3i {
        *present_at_obj + *new_calc - *previous_calc
    }

    fn spire_step(&mut self, from: &Vector3i, to: &Vector3i, speed: f64, step: f64) -> (Vector3i, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

        let remaining_distance = Self::spire_distance(self, from, to);
        let abs_step = f64::min(remaining_distance, max_step);

        #[allow(deprecated)]
        let value = {
            let from_float = from.cast_float();
            let to_float = to.cast_float();
            let result_float = Vector3::move_toward(from_float, to_float, abs_step as f32);
            Vector3i {
                x: result_float.x.floor() as i32,
                y: result_float.y.floor() as i32,
                z: result_float.z.floor() as i32,
            }
        };

        let actual_distance_moved = Self::spire_distance(self, from, &value);
        let unused_time = (max_step - actual_distance_moved) / speed;

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
    fn spire_distance(&mut self, from: &Vector3i, to: &Vector3i) -> f64 {
        let dx = (to.x - from.x) as f64;
        let dy = (to.y - from.y) as f64;
        let dz = (to.z - from.z) as f64;
        f64::sqrt(dx * dx + dy * dy + dz * dz)
    }
}
