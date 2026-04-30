use std::f32::consts::PI;

use godot::obj::bounds::DeclEngine;
use rand::Rng;
use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};

use super::*;

/// Shakes a `Node2D`'s local position within a ring (`radius_min..radius_max`).
/// `vibratio`: 0.0 = random, 1.0 = opposite-point bounce. Returns to origin when done.
pub trait DoShakeNode2D<Marker = ()> {
    fn do_shake(
        &self,
        radius_min: real,
        radius_max: real,
        vibratio: real,
        frequency: f64,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>>;
}

impl<T: Inherits<Node2D> + Inherits<Object>> DoShakeNode2D<()> for Gd<T> {
    fn do_shake(
        &self,
        radius_min: real,
        radius_max: real,
        vibratio: real,
        frequency: f64,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>> {
        let node = self.clone().upcast::<Node2D>();
        do_shake(
            node,
            Node2D::get_position,
            Node2D::set_position,
            radius_min,
            radius_max,
            vibratio,
            frequency,
            duration,
        )
    }
}

/// Same as `DoShakeNode2D` but for `Control` nodes.
pub trait DoShakeControl<Marker = ()> {
    fn do_shake(
        &self,
        radius_min: real,
        radius_max: real,
        vibratio: real,
        frequency: f64,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>>;
}

impl<T: Inherits<Control> + Inherits<Object>> DoShakeControl<()> for Gd<T> {
    fn do_shake(
        &self,
        radius_min: real,
        radius_max: real,
        vibratio: real,
        frequency: f64,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>> {
        let node = self.clone().upcast::<Control>();
        do_shake(
            node,
            Control::get_position,
            Control::set_position,
            radius_min,
            radius_max,
            vibratio,
            frequency,
            duration,
        )
    }
}

fn do_shake<T: GodotClass<Declarer = DeclEngine>>(
    mut node: Gd<T>,
    get_pos: fn(&T) -> Vector2,
    set_pos: fn(&mut T, Vector2),
    radius_min: real,
    radius_max: real,
    vibratio: real,
    frequency: f64,
    duration: f64,
) -> SpireTween<LerpMethodData<f64>> {
    let amplitude_factor = 1.0 - vibratio.clamp(0.0, 1.0);

    let radius_range = if radius_min < radius_max { radius_min..=radius_max } else { radius_max..=radius_min };
    let radius_avg = (radius_min + radius_max) * 0.5;
    let radius_avg_x2 = radius_avg * 2.0;

    let update_interval = 1.0 / frequency;
    let (angle_close_weight, angle_far_weight) =
        if amplitude_factor <= 0.5 { (amplitude_factor * 2.0, 0.0) } else { (1.0, amplitude_factor * 2.0 - 1.0) };

    let angle_vibration_range = {
        let vibration_amplitude = PI * amplitude_factor;
        -vibration_amplitude..=vibration_amplitude
    };

    let mut rng = Xoshiro256PlusPlus::try_from_os_rng().ok().unwrap_or_else(|| {
        godot_error!("[do_shake] Failed to acquire OS randomness, falling back to fixed seed.");
        Xoshiro256PlusPlus::seed_from_u64(1337)
    });

    let mut prev_angle = rng.random_range(0.0..=(2.0 * PI));
    let mut prev_radius = rng.random_range(radius_range);
    let mut prev_update = 0.0;
    let mut prev_offset = Vector2 {
        x: prev_radius * prev_angle.cos(),
        y: prev_radius * prev_angle.sin(),
    };

    let callable = Callable::from_fn("do_shake", move |args| {
        let Some(time) = args
            .first()
            .and_then(|v| v.try_to_relaxed::<f64>().ok())
        else {
            godot_error!("[do_shake] BUG: Expected `time(f64)` argument.");
            return Variant::nil();
        };

        // When the tween ends, return to the original(relative) position.
        if time.approx_eq(&duration) {
            let curr_pos = get_pos(&node);
            let final_pos = curr_pos - prev_offset;
            set_pos(&mut node, final_pos); // LOCAL!
            return Variant::nil();
        }

        if time - prev_update < update_interval {
            return Variant::nil();
        }

        let next_angle = {
            let prev_angle_opposite = prev_angle + PI;
            let angle_offset = rng.random_range(angle_vibration_range.clone());
            prev_angle_opposite + angle_offset
        };

        let next_radius = {
            let prev_radius_opposite = radius_avg_x2 - prev_radius;
            let next_radius_range = match (amplitude_factor <= 0.5, prev_radius_opposite < radius_avg) {
                (true, true) => {
                    let range_min = real::lerp(prev_radius_opposite, radius_min, angle_close_weight);
                    let range_max = real::lerp(prev_radius_opposite, radius_avg, angle_close_weight);
                    range_min..=range_max
                }
                (true, false) => {
                    let range_min = real::lerp(prev_radius_opposite, radius_avg, angle_close_weight);
                    let range_max = real::lerp(prev_radius_opposite, radius_max, angle_close_weight);
                    range_min..=range_max
                }
                (false, true) => {
                    let range_max = real::lerp(radius_avg, radius_max, angle_far_weight);
                    radius_min..=range_max
                }
                (false, false) => {
                    let range_min = real::lerp(radius_min, radius_avg, angle_far_weight);
                    range_min..=radius_max
                }
            };

            rng.random_range(next_radius_range)
        };

        let next_offset = Vector2 {
            x: next_radius * next_angle.cos(),
            y: next_radius * next_angle.sin(),
        };

        let curr_pos = get_pos(&node);
        let next_pos = curr_pos - prev_offset + next_offset;
        set_pos(&mut node, next_pos); // LOCAL!

        prev_update = time;
        prev_angle = next_angle;
        prev_radius = next_radius;
        prev_offset = next_offset;

        Variant::nil()
    });

    SpireTween::<LerpMethodData<f64>>::new(callable, 0.0, duration, duration)
}
