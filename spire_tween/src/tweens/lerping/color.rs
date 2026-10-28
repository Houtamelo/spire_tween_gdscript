use super::*;

impl ILerpable for Color {
    type BasicLerper = ();
    type Lerper = ();
}

impl BasicLerp<Color> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &Color, to: &Color, t: f64) -> Color { Color::lerp(*from, *to, t) }
}

impl SpireLerp<Color> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &Color, previous_calc: &Color, new_calc: &Color) -> Color {
        *present_at_obj + (*new_calc - *previous_calc)
    }

    fn spire_step(&mut self, from: &Color, to: &Color, speed: f64, step: f64) -> (Color, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

        let from_glam = glam::Vec4::new(from.r, from.g, from.b, from.a);
        let to_glam = glam::Vec4::new(to.r, to.g, to.b, to.a);

        let max_distance = from_glam.distance(to_glam);

        let abs_step = f32::min(max_distance, max_step as f32);

        let unused_time = (max_step - abs_step as f64) / speed;

        let value = {
            let from_glam = glam::Vec4::new(from.r, from.g, from.b, from.a);
            let to_glam = glam::Vec4::new(to.r, to.g, to.b, to.a);
            let result_glam = from_glam.move_towards(to_glam, abs_step);
            Color::from_rgba(result_glam.x, result_glam.y, result_glam.z, result_glam.w)
        };

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
    fn spire_distance(&mut self, from: &Color, to: &Color) -> f64 {
        let from_glam = glam::Vec4::new(from.r, from.g, from.b, from.a);
        let to_glam = glam::Vec4::new(to.r, to.g, to.b, to.a);
        from_glam.distance(to_glam) as f64
    }
}
