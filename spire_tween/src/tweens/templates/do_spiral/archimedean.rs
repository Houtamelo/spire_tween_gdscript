use super::*;

#[inline]
pub fn point(t: f32, scale: Vector2, shear: f32) -> Vector2 {
    let x = scale.x * t * f32::cos(t + shear);
    let y = scale.y * t * f32::sin(t);
    Vector2::new(x, y)
}

#[inline]
pub fn speed_fn(scale: Vector2, shear: f32) -> impl Fn(f32) -> f32 {
    move |t: f32| -> f32 {
        let (sin_t_shear, cos_t_shear) = f32::sin_cos(t + shear);
        let (sin_t, cos_t) = f32::sin_cos(t);

        let dx = scale.x * (cos_t_shear - t * sin_t_shear);
        let dy = scale.y * (sin_t + t * cos_t);
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
