use super::*;

impl ILerpable for Quaternion {
    type BasicLerper = ();
    type Lerper = ();
}

fn normalize_quaternion(value: Quaternion) -> Quaternion {
    let length = (value.x as f64)
        .hypot(value.y as f64)
        .hypot(value.z as f64)
        .hypot(value.w as f64);
    if value.is_finite() && length > 0.0 {
        Quaternion::new(
            (value.x as f64 / length) as real,
            (value.y as f64 / length) as real,
            (value.z as f64 / length) as real,
            (value.w as f64 / length) as real,
        )
    } else {
        godot_warn!("[spire quaternion] expected a finite, nonzero quaternion, got {value:?}; using identity.");
        Quaternion::IDENTITY
    }
}

// The principal relative rotation has a nonnegative scalar component. Using
// atan2 rather than acos(dot) retains small angular distances near identity.
fn shortest_rotation(from: Quaternion, to: Quaternion) -> Quaternion {
    let delta = normalize_quaternion(from.inverse() * to);
    if delta.w < 0.0 { -delta } else { delta }
}

fn half_angle(value: Quaternion) -> (f64, f64) {
    let sin = (value.x as f64).hypot(value.y as f64).hypot(value.z as f64);
    (sin.atan2(value.w as f64), sin)
}

impl BasicLerp<Quaternion> for () {
    fn spire_lerp(&mut self, from: &Quaternion, to: &Quaternion, t: f64) -> Quaternion {
        let from = normalize_quaternion(*from);
        let to = normalize_quaternion(*to);
        let delta = shortest_rotation(from, to);
        let (angle, sin) = half_angle(delta);
        if sin == 0.0 {
            return from;
        }
        // Raising the relative unit rotation to t is shortest-arc slerp,
        // including extrapolation for easing overshoot and Incremental loops.
        let (sin_t, cos_t) = (angle * t).sin_cos();
        let scale = sin_t / sin;
        let rotation = Quaternion::new(
            (delta.x as f64 * scale) as real,
            (delta.y as f64 * scale) as real,
            (delta.z as f64 * scale) as real,
            cos_t as real,
        );
        normalize_quaternion(from * rotation)
    }
}

impl SpireLerp<Quaternion> for () {
    fn prepare_completion_value(&mut self, value: Quaternion) -> Quaternion { normalize_quaternion(value) }

    fn add_relative(&mut self, current: &Quaternion, previous: &Quaternion, next: &Quaternion) -> Quaternion {
        normalize_quaternion(
            normalize_quaternion(*current) * normalize_quaternion(*previous).inverse() * normalize_quaternion(*next),
        )
    }

    fn spire_step(&mut self, from: &Quaternion, to: &Quaternion, speed: f64, step: f64) -> (Quaternion, StepResult) {
        let from = normalize_quaternion(*from);
        let to = normalize_quaternion(*to);
        let (t, result) = step_ratio(self.spire_distance(&from, &to), speed, step);
        (if t == 1.0 { to } else { self.spire_lerp(&from, &to, t) }, result)
    }

    fn spire_distance(&mut self, from: &Quaternion, to: &Quaternion) -> f64 {
        let delta = shortest_rotation(normalize_quaternion(*from), normalize_quaternion(*to));
        2.0 * half_angle(delta).0
    }
}
