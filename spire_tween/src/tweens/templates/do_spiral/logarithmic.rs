use super::*;

#[inline]
pub fn point(t: f32, scale: Vector2, shear: f32, growth: Vector2) -> Vector2 {
    Vector2 {
        x: scale.x * t * f32::exp(t * growth.x) * f32::cos(t + shear),
        y: scale.y * t * f32::exp(t * growth.y) * f32::sin(t),
    }
}

#[inline]
pub fn speed_fn(scale: Vector2, shear: f32, growth: Vector2) -> impl Fn(f32) -> f32 {
    move |t: f32| -> f32 {
        let t_gx = t * growth.x;
        let t_gy = t * growth.y;
        let exp_gx = f32::exp(t_gx);
        let exp_gy = f32::exp(t_gy);
        let (sin_t_shear, cos_t_shear) = f32::sin_cos(t + shear);
        let (sin_t, cos_t) = f32::sin_cos(t);

        let dx = scale.x * exp_gx * (cos_t_shear * (1.0 + t_gx) - t * sin_t_shear);
        let dy = scale.y * exp_gy * (sin_t * (1.0 + t_gy) + t * cos_t);
        f32::sqrt(dx * dx + dy * dy)
    }
}

#[cfg(test)]
mod benches {
    extern crate test;

    use godot::builtin::Vector2;
    use rand::Rng;
    use test::Bencher;

    use super::{angle_from_arc_length, arc_length};

    #[bench]
    fn bench_arc_length_between_random(b: &mut Bencher) {
        let mut rng = rand::rng();

        let inputs = (0..1000)
            .map(|_| {
                let from = rng.random_range(-8.0 * std::f32::consts::PI..=8.0 * std::f32::consts::PI);
                let to = rng.random_range(-8.0 * std::f32::consts::PI..=8.0 * std::f32::consts::PI);
                let scale = Vector2 {
                    x: rng.random_range(-128.0..=128.0),
                    y: rng.random_range(-128.0..=128.0),
                };
                let shear = rng.random_range(-4.0..=4.0);
                let growth = Vector2 {
                    x: rng.random_range(0.0..=1.0),
                    y: rng.random_range(0.0..=1.0),
                };

                (from, to, scale, shear, growth)
            })
            .collect::<Vec<_>>();

        let mut inputs = inputs.iter().cycle();

        b.iter(|| {
            let (from, to, scale, shear, growth) = unsafe { *inputs.next().unwrap_unchecked() };
            test::black_box(arc_length(from, to, super::speed_fn(scale, shear, growth)))
        });
    }

    #[bench]
    fn bench_angle_from_length_random(b: &mut Bencher) {
        let mut rng = rand::rng();

        let inputs = (0..1000)
            .map(|_| {
                let from = rng.random_range(-8.0 * std::f32::consts::PI..=8.0 * std::f32::consts::PI);
                let len = rng.random_range(0.0..=1024.0);
                let scale = Vector2 {
                    x: rng.random_range(-128.0..=128.0),
                    y: rng.random_range(-128.0..=128.0),
                };
                let shear = rng.random_range(-4.0..=4.0);
                let growth = Vector2 {
                    x: rng.random_range(0.0..=1.0),
                    y: rng.random_range(0.0..=1.0),
                };

                (from, len, scale, shear, growth)
            })
            .collect::<Vec<_>>();

        let mut inputs = inputs.iter().cycle();

        b.iter(|| {
            let (from, len, scale, shear, growth) = unsafe { *inputs.next().unwrap_unchecked() };
            test::black_box(angle_from_arc_length(from, len, super::speed_fn(scale, shear, growth)))
        });
    }
}
