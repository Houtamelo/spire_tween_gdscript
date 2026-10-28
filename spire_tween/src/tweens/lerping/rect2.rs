use super::*;

impl ILerpable for Rect2 {
    type BasicLerper = ();
    type Lerper = ();
}

impl BasicLerp<Rect2> for () {
    fn spire_lerp(&mut self, from: &Rect2, to: &Rect2, t: f64) -> Rect2 {
        Rect2::new(from.position.lerp(to.position, t as real), from.size.lerp(to.size, t as real))
    }
}

impl SpireLerp<Rect2> for () {
    fn add_relative(&mut self, current: &Rect2, previous: &Rect2, next: &Rect2) -> Rect2 {
        Rect2::new(current.position + (next.position - previous.position), current.size + (next.size - previous.size))
    }

    fn spire_step(&mut self, from: &Rect2, to: &Rect2, speed: f64, step: f64) -> (Rect2, StepResult) {
        let (t, result) = step_ratio(self.spire_distance(from, to), speed, step);
        (if t == 1.0 { *to } else { self.spire_lerp(from, to, t) }, result)
    }

    fn spire_distance(&mut self, from: &Rect2, to: &Rect2) -> f64 {
        let from = Vector4::new(from.position.x, from.position.y, from.size.x, from.size.y);
        let to = Vector4::new(to.position.x, to.position.y, to.size.x, to.size.y);
        from.distance_to(to) as f64
    }
}
