use std::fmt::Debug;

use crate::internal_prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum StepResult {
    Unfinished { accumulated_time: f64 },
    Finished { excess_time: f64 },
}

#[derive(Debug, Clone)]
pub enum LerpMode<T> {
    Absolute { duration: f64, from: Option<T> },
    SpeedBased { speed: f64, t_sum: f64 },
    Relative { duration: f64, origin: T },
}

pub trait SpireLerp: Sized + Clone + FromGodot + ToGodot {
    fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self;
    fn add_relative(current_at_obj: &Self, previous_relative: &Self, new_relative: &Self) -> Self;
    fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult);
    fn spire_distance(from: &Self, to: &Self) -> f64;

    fn relative_origin() -> Self;
}

trait MoveTowards {
    fn move_towards(from: Self, to: Self, abs_move: Self) -> Self;
}

impl MoveTowards for f32 {
    fn move_towards(from: Self, to: Self, abs_move: Self) -> Self {
        if from < to {
            Self::min(from + abs_move, to)
        } else {
            Self::max(from - abs_move, to)
        }
    }
}

impl MoveTowards for f64 {
    fn move_towards(from: Self, to: Self, abs_move: Self) -> Self {
        if from < to {
            Self::min(from + abs_move, to)
        } else {
            Self::max(from - abs_move, to)
        }
    }
}

impl MoveTowards for i32 {
    fn move_towards(from: Self, to: Self, abs_move: Self) -> Self {
        if from < to {
            Self::min(from + abs_move, to)
        } else {
            Self::max(from - abs_move, to)
        }
    }
}

impl MoveTowards for i64 {
    fn move_towards(from: Self, to: Self, abs_move: Self) -> Self {
        if from < to {
            Self::min(from + abs_move, to)
        } else {
            Self::max(from - abs_move, to)
        }
    }
}

impl SpireLerp for i32 {
    fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
        (*from as f64 + (to - from) as f64 * t).round() as i32
    }

    fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
        let max_step = speed * t;

        let remaining_distance = i32::abs(to - from);
        let abs_step = i32::min(remaining_distance, max_step.floor() as i32);

        let unused_time = (max_step - abs_step as f64) / speed;
        let value = i32::move_towards(*from, *to, abs_step);

        let step_result = if max_step >= remaining_distance as f64 {
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

    fn spire_distance(from: &Self, to: &Self) -> f64 { i32::abs(to - from) as f64 }

    fn relative_origin() -> Self { 0 }
}

impl SpireLerp for i64 {
    fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
        (*from as f64 + (to - from) as f64 * t).round() as i64
    }

    fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
        let max_step = speed * t;

        let remaining_distance = i64::abs(to - from);
        let abs_step = i64::min(remaining_distance, max_step.floor() as i64);

        let unused_time = (max_step - abs_step as f64) / speed;
        let value = i64::move_towards(*from, *to, abs_step);

        let step_result = if max_step >= remaining_distance as f64 {
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

    fn spire_distance(from: &Self, to: &Self) -> f64 { i64::abs(to - from) as f64 }

    fn relative_origin() -> Self { 0 }
}

impl SpireLerp for f32 {
    fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
        let t = t as f32;
        from + (to - from) * t
    }

    fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
        let speed = speed as f32;
        let t = t as f32;

        let max_step = speed * t;

        let remaining_distance = f32::abs(to - from);
        let abs_step = f32::min(remaining_distance, max_step);

        let unused_time = (max_step - abs_step) / speed;
        let value = f32::move_towards(*from, *to, abs_step);

        let step_result = if max_step >= remaining_distance {
            StepResult::Finished {
                excess_time: unused_time as f64,
            }
        } else {
            StepResult::Unfinished {
                accumulated_time: unused_time as f64,
            }
        };

        (value, step_result)
    }

    fn spire_distance(from: &Self, to: &Self) -> f64 { f32::abs(to - from) as f64 }

    fn relative_origin() -> Self { 0. }
}

impl SpireLerp for f64 {
    fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self { from + (to - from) * t }

    fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
        let max_step = speed * t;

        let remaining_distance = f64::abs(to - from);
        let abs_step = f64::min(remaining_distance, max_step);

        let unused_time = (max_step - abs_step) / speed;
        let value = f64::move_towards(*from, *to, abs_step);

        let step_result = if max_step >= remaining_distance {
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

    fn spire_distance(from: &Self, to: &Self) -> f64 { f64::abs(to - from) }

    fn relative_origin() -> Self { 0. }
}

impl SpireLerp for GString {
    fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
        let from = from.to_string();
        let to = to.to_string();

        let t = f64::clamp(t, 0.0, 1.0);

        let from_len = from.chars().count() as i64;
        let to_len = to.chars().count() as i64;
        let new_len_raw = from_len + ((to_len - from_len) as f64 * t).round() as i64;
        let new_len = i64::abs(new_len_raw) as usize;

        let mut result = from.chars().collect::<Vec<_>>();
        let chars_to_take = usize::min((to_len as f64 * t).round() as usize, to_len as usize);
        let taken_chars = to.chars().take(chars_to_take).enumerate();
        for (index, char) in taken_chars {
            if result.len() > index {
                result[index] = char;
            } else {
                result.push(char);
            }
        }

        result.into_iter().take(new_len).collect::<String>().into()
    }

    fn add_relative(present_at_obj: &Self, prev_calc: &Self, next_calc: &Self) -> Self {
        let previous_calc = prev_calc.chars();
        let next_calc = next_calc.chars();

        let mut result = present_at_obj.to_string();

        let prev_len = previous_calc.len();
        let next_len = next_calc.len();

        if prev_len < next_len {
            result.extend(&next_calc[prev_len..]);
        } else {
            for _ in 0..(prev_len - next_len) {
                result.pop();
            }
        }

        result.into()
    }

    fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
        let max_step = speed * t;
        let abs_step = max_step.floor() as i64;
        let unused_time = (max_step - abs_step as f64) / speed;

        let from_len = from.len();
        let to_len = to.len();

        let to_str = to.to_string();

        let mut result = from.to_string().chars().collect::<Vec<_>>();

        let mut remaining = abs_step;

        for (idx, char) in to_str.chars().enumerate() {
            if remaining <= 0 {
                break;
            }

            if idx >= from_len {
                result.push(char);
                remaining -= 1;
            } else if result[idx] != char {
                result[idx] = char;
                remaining -= 1;
            }
        }

        let mut char_delta = from_len as i64 - to_len as i64;
        while char_delta > 0 && remaining > 0 {
            result.pop();
            char_delta -= 1;
            remaining -= 1;
        }

        let final_unused_time = unused_time + (remaining as f64 / speed);
        let value = result.into_iter().collect::<String>();

        let step_result = if value == to_str {
            StepResult::Finished {
                excess_time: final_unused_time,
            }
        } else {
            StepResult::Unfinished {
                accumulated_time: final_unused_time,
            }
        };

        (value.into(), step_result)
    }

    fn spire_distance(from: &Self, to: &Self) -> f64 {
        let mut distance = 0;
        let from_len = from.len();
        let to_len = to.len();

        let from_chars = from.chars();
        let to_chars = to.chars();

        for i in 0..usize::min(from_len, to_len) {
            if from_chars[i] != to_chars[i] {
                distance += 1;
            }
        }

        let count_abs = usize::abs_diff(from_len, to_len);
        (distance + count_abs) as f64
    }

    fn relative_origin() -> Self { GString::new() }
}

impl SpireLerp for Color {
    fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self { Color::lerp(*from, *to, t) }

    fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
        *present_at_obj + (*new_calc - *previous_calc)
    }

    fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
        let max_step = speed * t;

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

    fn spire_distance(from: &Self, to: &Self) -> f64 {
        let from_glam = glam::Vec4::new(from.r, from.g, from.b, from.a);
        let to_glam = glam::Vec4::new(to.r, to.g, to.b, to.a);
        from_glam.distance(to_glam) as f64
    }

    fn relative_origin() -> Self { Color::from_rgba(0., 0., 0., 0.) }
}

impl SpireLerp for Vector2 {
    fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
        let from = *from;
        let to = *to;

        Vector2::lerp(from, to, t as f32)
    }

    fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
        *present_at_obj + (*new_calc - *previous_calc)
    }

    fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
        let max_step = speed * t;
        let max_distance = from.distance_to(*to);

        let abs_step = f32::min(max_distance, max_step as f32);

        let unused_time = (max_step - abs_step as f64) / speed;
        let value = Vector2::move_toward(*from, *to, abs_step);

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

    fn spire_distance(from: &Self, to: &Self) -> f64 { from.distance_to(*to) as f64 }

    fn relative_origin() -> Self { Vector2::new(0., 0.) }
}

impl SpireLerp for Vector3 {
    fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
        let from = *from;
        let to = *to;

        Vector3::lerp(from, to, t as f32)
    }

    fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
        *present_at_obj + (*new_calc - *previous_calc)
    }

    fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
        let max_step = speed * t;
        let max_distance = from.distance_to(*to);

        let abs_step = f32::min(max_distance, max_step as f32);

        let unused_time = (max_step - abs_step as f64) / speed;

        let value = Vector3::move_toward(*from, *to, abs_step);

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

    fn spire_distance(from: &Self, to: &Self) -> f64 { from.distance_to(*to) as f64 }

    fn relative_origin() -> Self { Vector3::new(0., 0., 0.) }
}
