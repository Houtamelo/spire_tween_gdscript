use super::*;

mod archimedean;
mod fermat;
mod hyperbolic;
mod logarithmic;

/// Moves a `Node2D` along a spiral path. Angles in radians, movement is local.
/// Speed is arc-length-based (except hyperbolic, which uses angular speed).
pub trait DoSpiral<Marker = ()> {
    fn do_spiral(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        scale: Vector2,
        duration: f64,
        rotation: f32,
        shear: f32,
        mode: Spiral,
        log_growth: Vector2,
    ) -> SpireTween<LerpMethodData<f64>>;
}

impl<T: Inherits<Node2D> + Inherits<Object>> DoSpiral<()> for Gd<T> {
    fn do_spiral(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        scale: Vector2,
        duration: f64,
        rotation: f32,
        shear: f32,
        mode: Spiral,
        log_growth: Vector2,
    ) -> SpireTween<LerpMethodData<f64>> {
        let mut node = self.clone().upcast::<Node2D>();

        let total_len = match mode {
            Spiral::Logarithmic => arc_length(from_angle, to_angle, logarithmic::speed_fn(scale, shear, log_growth)),
            Spiral::Archimedean => arc_length(from_angle, to_angle, archimedean::speed_fn(scale, shear)),
            Spiral::Fermat => arc_length(from_angle, to_angle, fermat::speed_fn(scale, shear)),

            // Hyperbolic arc lengths approach infinity near t=0, so we lerp the angle instead of the arc length
            Spiral::Hyperbolic => {
                let callable = Callable::from_fn("do_spiral", move |args| {
                    let Some(t) = args.first().and_then(|v| v.try_to_relaxed::<f64>().ok()) else {
                        godot_error!("[do_spiral] BUG: Expected `angle(f64)` argument.");
                        return Variant::nil();
                    };

                    // NOT GLOBAL
                    let next_p = hyperbolic::point(t as f32, scale, shear);
                    node.set_position(center + next_p.rotated(rotation));
                    Variant::nil()
                });

                return SpireTween::<LerpMethodData<f64>>::new(callable, from_angle as f64, to_angle as f64, duration);
            }
        } as f64;

        let mut prev_len = 0.;
        let mut prev_t = from_angle;

        let callable = Callable::from_fn("do_spiral", move |args| {
            let Some(next_len) = args
                .first()
                .and_then(|v| v.try_to_relaxed::<f64>().ok())
            else {
                godot_error!("[do_spiral] BUG: Expected `length(f64)` argument.");
                return Variant::nil();
            };

            if next_len.approx_eq(&total_len) {
                let next_p = match mode {
                    Spiral::Logarithmic => logarithmic::point(to_angle, scale, shear, log_growth),
                    Spiral::Archimedean => archimedean::point(to_angle, scale, shear),
                    Spiral::Hyperbolic => hyperbolic::point(to_angle, scale, shear),
                    Spiral::Fermat => fermat::point(to_angle, scale, shear),
                };

                prev_t = to_angle;
                prev_len = total_len;

                // NOT GLOBAL
                node.set_position(center + next_p.rotated(rotation));
                return Variant::nil();
            }

            let len_inc = (next_len - prev_len) as f32;

            let (next_t, next_p) = match mode {
                Spiral::Archimedean => {
                    let angle = angle_from_arc_length(prev_t, len_inc, archimedean::speed_fn(scale, shear));
                    let point = archimedean::point(angle, scale, shear);
                    (angle, point)
                }
                Spiral::Logarithmic => {
                    let angle = angle_from_arc_length(prev_t, len_inc, logarithmic::speed_fn(scale, shear, log_growth));
                    let point = logarithmic::point(angle, scale, shear, log_growth);
                    (angle, point)
                }
                Spiral::Hyperbolic => {
                    let angle = angle_from_arc_length(prev_t, len_inc, hyperbolic::speed_fn(scale, shear));
                    let point = hyperbolic::point(angle, scale, shear);
                    (angle, point)
                }
                Spiral::Fermat => {
                    let angle = angle_from_arc_length(prev_t, len_inc, fermat::speed_fn(scale, shear));
                    let point = fermat::point(angle, scale, shear);
                    (angle, point)
                }
            };

            prev_t = next_t;
            prev_len = next_len;

            // NOT GLOBAL
            node.set_position(center + next_p.rotated(rotation));
            Variant::nil()
        });

        SpireTween::<LerpMethodData<f64>>::new(callable, 0.0, total_len, duration)
    }
}

#[inline]
fn arc_length(t1: f32, t2: f32, speed_fn: impl Fn(f32) -> f32) -> f32 {
    const N: usize = 60;
    let h = (t2 - t1) / (N as f32);

    let mut sum = speed_fn(t1) + speed_fn(t2);

    for i in 1..N {
        let t = t1 + i as f32 * h;
        let coefficient = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += coefficient * speed_fn(t);
    }

    (h / 3.0) * sum
}

#[inline]
fn angle_from_arc_length(t1: f32, target_length: f32, speed_fn: impl Fn(f32) -> f32) -> f32 {
    const STEP_SIZE: f32 = 0.0025; // Adaptive step
    const MAX_STEPS: usize = 40000;

    let direction = target_length.signum();
    let target_length = target_length.abs();

    // Adaptive step size based on current speed
    let mut t = t1;
    let mut accumulated = 0.0;
    let mut prev_speed = speed_fn(t);

    for _ in 0..MAX_STEPS {
        let current_speed = speed_fn(t);
        let avg_speed = (prev_speed + current_speed) * 0.5;

        // Adaptive step: smaller steps when speed changes rapidly
        let speed_change = (current_speed - prev_speed).abs();
        let adaptive_step = STEP_SIZE / (1.0 + speed_change);

        // Trapezoidal rule increment
        let step_length = avg_speed * adaptive_step;

        if accumulated + step_length >= target_length {
            // Linear interpolation for the final step
            let remaining = target_length - accumulated;
            let final_dt = remaining / avg_speed.max(1e-6);
            return t + direction * final_dt;
        }

        accumulated += step_length;
        t += direction * adaptive_step;
        prev_speed = current_speed;
    }

    t
}
