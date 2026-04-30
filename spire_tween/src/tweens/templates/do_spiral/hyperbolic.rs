use super::*;

#[inline]
pub fn point(t: f32, scale: Vector2, shear: f32) -> Vector2 {
    let non_zero_theta = if t.abs() > 1e-6 { t } else { 1e-6 };
    let r = scale / non_zero_theta;

    Vector2 {
        x: r.x * f32::cos(t + shear),
        y: r.y * f32::sin(t),
    }
}

#[inline]
pub fn speed_fn(scale: Vector2, shear: f32) -> impl Fn(f32) -> f32 {
    move |t: f32| {
        let non_zero_theta = if t.abs() > 1e-6 { t } else { 1e-6 };
        let t_sq = non_zero_theta * non_zero_theta;

        let (sin_t_shear, cos_t_shear) = f32::sin_cos(non_zero_theta + shear);
        let (sin_t, cos_t) = f32::sin_cos(non_zero_theta);

        let r_x = scale.x / non_zero_theta;
        let r_y = scale.y / non_zero_theta;
        let dr_x = -scale.x / t_sq;
        let dr_y = -scale.y / t_sq;

        let dx = dr_x * cos_t_shear - r_x * sin_t_shear;
        let dy = dr_y * sin_t + r_y * cos_t;

        f32::sqrt(dx * dx + dy * dy)
    }
}

#[cfg(test)]
mod benches {
    extern crate test;

    use std::f32::consts::PI;

    use godot::builtin::Vector2;
    use rand::Rng;
    use test::Bencher;

    use super::*;

    #[bench]
    fn bench_arc_length_between_random(b: &mut Bencher) {
        let mut rng = rand::rng();

        let inputs = (0..1000)
            .map(|_| {
                let from = rng.random_range(-8.0 * PI..=8.0 * PI);
                let to = rng.random_range(-8.0 * PI..=8.0 * PI);
                let scale = Vector2 {
                    x: rng.random_range(-128.0..=128.0),
                    y: rng.random_range(-128.0..=128.0),
                };
                let shear = rng.random_range(-4.0..=4.0);

                (from, to, scale, shear)
            })
            .collect::<Vec<_>>();

        let mut inputs = inputs.iter().cycle();

        b.iter(|| {
            let (from, to, scale, shear) = unsafe { *inputs.next().unwrap_unchecked() };
            test::black_box(arc_length(from, to, speed_fn(scale, shear)))
        });
    }

    #[bench]
    fn bench_angle_from_length_random(b: &mut Bencher) {
        let mut rng = rand::rng();

        let inputs = (0..1000)
            .map(|_| {
                let from = rng.random_range(-8.0 * PI..=8.0 * PI);
                let len = rng.random_range(0.0..=1024.0);
                let scale = Vector2 {
                    x: rng.random_range(-128.0..=128.0),
                    y: rng.random_range(-128.0..=128.0),
                };
                let shear = rng.random_range(-4.0..=4.0);

                (from, len, scale, shear)
            })
            .collect::<Vec<_>>();

        let mut inputs = inputs.iter().cycle();

        b.iter(|| {
            let (from, len, scale, shear) = unsafe { *inputs.next().unwrap_unchecked() };
            test::black_box(angle_from_arc_length(from, len, speed_fn(scale, shear)))
        });
    }
}
