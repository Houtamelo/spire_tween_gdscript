use super::*;

impl ILerpable for GString {
    type BasicLerper = ();
    type Lerper = ();
}

impl BasicLerp<GString> for () {
    fn spire_lerp(&mut self, from: &GString, to: &GString, t: f64) -> GString {
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

        GString::from(result.into_iter().take(new_len).collect::<String>().as_str())
    }
}

impl SpireLerp<GString> for () {
    fn add_relative(&mut self, present_at_obj: &GString, prev_calc: &GString, next_calc: &GString) -> GString {
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

        GString::from(result.as_str())
    }

    fn spire_step(&mut self, from: &GString, to: &GString, speed: f64, step: f64) -> (GString, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (from.clone(), StepResult::Unfinished { accumulated_time: 0. });
        }

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

        (GString::from(value.as_str()), step_result)
    }

    fn spire_distance(&mut self, from: &GString, to: &GString) -> f64 {
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
}
