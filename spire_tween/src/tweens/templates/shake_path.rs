//! Planar shake equations with no engine runtime calls.

use std::num::NonZeroUsize;

use rand::Rng;

use crate::enums::ShakeRandomnessMode;

pub(crate) struct ShakePath<const D: usize = 2> {
    segments: NonZeroUsize,
    strength: [f64; D],
    randomness_degrees: f64,
    fade_out: bool,
    mode: ShakeRandomnessMode,
    waypoints: Vec<[f64; D]>,
}

impl<const D: usize> ShakePath<D> {
    pub(crate) fn new(
        segments: NonZeroUsize,
        strength: [f64; D],
        randomness_degrees: f64,
        fade_out: bool,
        mode: ShakeRandomnessMode,
    ) -> Self {
        Self {
            segments,
            strength,
            randomness_degrees,
            fade_out,
            mode,
            waypoints: Vec::new(),
        }
    }

    pub(crate) fn regenerate(&mut self, rng: &mut impl Rng) {
        let segments = self.segments.get();
        let waypoint_count = segments + 1;
        if self.waypoints.len() != waypoint_count {
            self.waypoints.resize(waypoint_count, [0.0; D]);
        }
        self.waypoints.fill([0.0; D]);

        let strength_magnitude = self
            .strength
            .iter()
            .fold(0.0_f64, |length, component| length.hypot(*component));
        let randomness_radians = self.randomness_degrees.to_radians();
        let mut theta = rng.random::<f64>() * (2.0 * std::f64::consts::PI);

        for k in 1..segments {
            if k > 1 {
                theta = theta - std::f64::consts::PI + randomness_offset(rng.random(), randomness_radians, self.mode);
            }

            let phi = randomness_offset(rng.random(), randomness_radians, self.mode);
            let amplitude = if self.fade_out {
                strength_magnitude * (1.0 - (k - 1) as f64 / segments as f64)
            } else {
                strength_magnitude
            };

            if strength_magnitude > 0.0 {
                self.waypoints[k] =
                    shaped_waypoint(amplitude, theta, phi, self.strength.map(|s| s / strength_magnitude));
            }
        }
    }

    pub(crate) fn sample(&self, progress: f64) -> [f64; D] {
        if !(progress > 0.0 && progress < 1.0) {
            return [0.0; D];
        }

        // Find the first destination knot whose time is at or after progress.
        // Knot times are strictly increasing, so this lower-bound search is
        // logarithmic without needing a second time buffer.
        let mut lower = 0;
        let mut upper = self.segments.get();
        while lower < upper {
            let middle = lower + (upper - lower) / 2;
            if self.knot_time(middle) < progress {
                lower = middle + 1;
            } else {
                upper = middle;
            }
        }

        let end_knot = lower;
        let start_knot = end_knot - 1;
        let start_time = self.knot_time(start_knot);
        let end_time = self.knot_time(end_knot);
        let fraction = (progress - start_time) / (end_time - start_time);
        let start = self.waypoints[start_knot];
        let end = self.waypoints[end_knot];
        if fraction <= 0.0 {
            return start;
        }
        if fraction >= 1.0 {
            return end;
        }
        std::array::from_fn(|i| start[i] + (end[i] - start[i]) * fraction)
    }

    fn knot_time(&self, knot: usize) -> f64 {
        let n = self.segments.get() as f64;
        let k = knot as f64;
        if self.fade_out { k * (k + 1.0) / (n * (n + 1.0)) } else { k / n }
    }
}

fn randomness_offset(unit: f64, radians: f64, mode: ShakeRandomnessMode) -> f64 {
    match mode {
        ShakeRandomnessMode::Full => (2.0 * unit - 1.0) * radians,
        ShakeRandomnessMode::Harmonic => unit * radians,
    }
}

fn shaped_waypoint<const D: usize>(amplitude: f64, theta: f64, phi: f64, shaping: [f64; D]) -> [f64; D] {
    let cos_theta = theta.cos();
    let x = shaping[0] * cos_theta * phi.cos();
    let y = theta.sin();
    let z = cos_theta * phi.sin();
    let d = (x * x + y * y + z * z).sqrt();
    let h = if d <= shaping[1] { 1.0 } else { shaping[1] / d };
    let y_shaped = y * h;
    let z_shaped = if D == 3 {
        let d = (x * x + y_shaped * y_shaped + z * z).sqrt();
        -z * if d <= shaping[2] { 1.0 } else { shaping[2] / d }
    } else {
        0.0
    };
    let length = if D == 3 { x.hypot(y_shaped).hypot(z_shaped) } else { x.hypot(y_shaped) };
    if length > 0.0 { std::array::from_fn(|i| amplitude * [x, y_shaped, z_shaped][i] / length) } else { [0.0; D] }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use rand::Rng;
    use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};

    use super::{ShakePath, ShakeRandomnessMode};

    fn seeded_rng() -> Xoshiro256PlusPlus { Xoshiro256PlusPlus::seed_from_u64(12345) }

    #[test]
    fn spatial_waypoints_match_independently_evaluated_equations() {
        let mut path =
            ShakePath::new(NonZeroUsize::new(4).unwrap(), [3.0, 4.0, 5.0], 90.0, true, ShakeRandomnessMode::Full);
        path.regenerate(&mut seeded_rng());
        // Calculated from the spherical direction, sequential component shaping,
        // and normalization equations, using the documented seed's unit draws.
        for (time, expected) in [
            (0.1, [-2.3439257194279484, -2.1008340862112944, -6.331864525083977]),
            (0.3, [0.9595328260297309, -3.699277883840274, 3.6769062938701094]),
            (0.6, [0.2694263853910813, 3.482950640944519, -0.5444853125652347]),
        ] {
            for (a, b) in path.sample(time).into_iter().zip(expected) {
                assert!((a - b).abs() < 1e-12);
            }
        }
    }

    fn assert_close(actual: [f64; 2], expected: [f64; 2]) {
        for (actual, expected) in actual.into_iter().zip(expected) {
            assert!((actual - expected).abs() < 1.0e-12, "{actual} != {expected}");
        }
    }

    #[test]
    fn zero_strength_is_zero_and_consumes_the_defined_draws() {
        let mut path =
            ShakePath::new(NonZeroUsize::new(4).unwrap(), [0.0, 0.0], 180.0, false, ShakeRandomnessMode::Full);
        let mut rng = seeded_rng();
        let mut expected_rng = rng.clone();
        for _ in 0..1 + 1 + 2 * (4 - 2) {
            let _ = expected_rng.random::<f64>();
        }
        path.regenerate(&mut rng);

        assert_eq!(rng.random::<u64>(), expected_rng.random::<u64>());
        assert_eq!(path.sample(0.2), [0.0, 0.0]);
        assert_eq!(path.sample(0.8), [0.0, 0.0]);
    }

    #[test]
    fn zero_randomness_alternates_equal_waypoints_and_interpolates() {
        let mut path = ShakePath::new(NonZeroUsize::new(4).unwrap(), [3.0, 4.0], 0.0, false, ShakeRandomnessMode::Full);
        path.regenerate(&mut seeded_rng());
        let peak = path.sample(0.25);
        assert_close([peak[0].hypot(peak[1]), 0.0], [5.0, 0.0]);
        assert_close(path.sample(0.125), [peak[0] * 0.5, peak[1] * 0.5]);
        assert_close(path.sample(0.375), [0.0, 0.0]);
        assert_close(path.sample(0.5), [-peak[0], -peak[1]]);
        assert_close(path.sample(0.625), [0.0, 0.0]);
    }

    #[test]
    fn fading_uses_the_quadratic_times_and_decreasing_magnitudes() {
        let mut path = ShakePath::new(NonZeroUsize::new(4).unwrap(), [3.0, 4.0], 0.0, true, ShakeRandomnessMode::Full);
        path.regenerate(&mut seeded_rng());
        let peak = path.sample(0.1);
        assert_close([peak[0].hypot(peak[1]), 0.0], [5.0, 0.0]);
        assert_close(path.sample(0.3), [peak[0] * -0.75, peak[1] * -0.75]);
        assert_close(path.sample(0.6), [peak[0] * 0.5, peak[1] * 0.5]);
    }

    #[test]
    fn each_disabled_axis_stays_on_the_other_axis() {
        let mut vertical =
            ShakePath::new(NonZeroUsize::new(2).unwrap(), [0.0, 4.0], 0.0, false, ShakeRandomnessMode::Full);
        vertical.regenerate(&mut seeded_rng());
        assert_close(vertical.sample(0.5), [0.0, -4.0]);

        let mut horizontal =
            ShakePath::new(NonZeroUsize::new(2).unwrap(), [3.0, 0.0], 0.0, false, ShakeRandomnessMode::Full);
        horizontal.regenerate(&mut seeded_rng());
        assert_close(horizontal.sample(0.5), [-3.0, 0.0]);
    }

    #[test]
    fn boundaries_are_exact_zero_and_sampling_is_stateless() {
        let mut path =
            ShakePath::new(NonZeroUsize::new(3).unwrap(), [1.0, 2.0], 20.0, false, ShakeRandomnessMode::Harmonic);
        path.regenerate(&mut seeded_rng());

        assert_eq!(path.sample(-1.0), [0.0, 0.0]);
        assert_eq!(path.sample(0.0), [0.0, 0.0]);
        assert_eq!(path.sample(1.0), [0.0, 0.0]);
        assert_eq!(path.sample(2.0), [0.0, 0.0]);
        let forward_a = path.sample(0.2);
        let forward_b = path.sample(0.8);
        assert_close(path.sample(0.8), forward_b);
        assert_close(path.sample(0.2), forward_a);
    }

    #[test]
    fn regeneration_reuses_the_waypoint_buffer() {
        let mut path = ShakePath::new(NonZeroUsize::new(3).unwrap(), [1.0, 2.0], 0.0, false, ShakeRandomnessMode::Full);
        let mut rng = seeded_rng();
        path.regenerate(&mut rng);
        let capacity = path.waypoints.capacity();
        let first = path.sample(1.0 / 6.0);

        path.regenerate(&mut rng);
        assert_eq!(path.waypoints.capacity(), capacity);
        assert_ne!(path.sample(1.0 / 6.0), first);
    }

    #[test]
    fn lookup_handles_knot_boundaries_and_interiors_for_both_timing_modes() {
        fn knot_time(knot: usize, segments: usize, fade_out: bool) -> f64 {
            let n = segments as f64;
            let k = knot as f64;
            if fade_out { k * (k + 1.0) / (n * (n + 1.0)) } else { k / n }
        }

        fn expected_waypoint(knot: usize, segments: usize, fade_out: bool) -> [f64; 2] {
            if knot == 0 || knot == segments {
                [0.0, 0.0]
            } else {
                let amplitude = if fade_out { 1.0 - (knot - 1) as f64 / segments as f64 } else { 1.0 };
                [if knot % 2 == 1 { -amplitude } else { amplitude }, 0.0]
            }
        }

        for segments in [1, 2, 3, 7] {
            for fade_out in [false, true] {
                let mut path = ShakePath::new(
                    NonZeroUsize::new(segments).unwrap(),
                    [1.0, 0.0],
                    0.0,
                    fade_out,
                    ShakeRandomnessMode::Full,
                );
                path.regenerate(&mut seeded_rng());

                for knot in 0..=segments {
                    let progress = knot_time(knot, segments, fade_out);
                    let actual = path.sample(progress);
                    let expected = expected_waypoint(knot, segments, fade_out);
                    assert_close(actual, expected);
                }

                for segment in 0..segments {
                    let start_time = knot_time(segment, segments, fade_out);
                    let end_time = knot_time(segment + 1, segments, fade_out);
                    let start = expected_waypoint(segment, segments, fade_out);
                    let end = expected_waypoint(segment + 1, segments, fade_out);
                    assert_close(path.sample((start_time + end_time) / 2.0), [
                        (start[0] + end[0]) / 2.0,
                        (start[1] + end[1]) / 2.0,
                    ]);
                }
            }
        }
    }
}
