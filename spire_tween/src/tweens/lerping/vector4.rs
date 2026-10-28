use super::*;

impl ILerpable for Vector4 {
    type BasicLerper = ();
    type Lerper = ();
}

impl BasicLerp<Vector4> for () {
    fn spire_lerp(&mut self, from: &Vector4, to: &Vector4, t: f64) -> Vector4 { from.lerp(*to, t as real) }
}

impl SpireLerp<Vector4> for () {
    fn add_relative(&mut self, current: &Vector4, previous: &Vector4, next: &Vector4) -> Vector4 {
        *current + (*next - *previous)
    }

    fn spire_step(&mut self, from: &Vector4, to: &Vector4, speed: f64, step: f64) -> (Vector4, StepResult) {
        let (t, result) = step_ratio(from.distance_to(*to) as f64, speed, step);
        (if t == 1.0 { *to } else { self.spire_lerp(from, to, t) }, result)
    }

    fn spire_distance(&mut self, from: &Vector4, to: &Vector4) -> f64 { from.distance_to(*to) as f64 }
}
