use super::*;

impl ILerpable for GString {
    type BasicLerper = ();
    type Lerper = StringLerper;
}

impl BasicLerp<GString> for () {
    fn spire_lerp(&mut self, from: &GString, to: &GString, t: f64) -> GString {
        let from = from.chars();
        let to = to.chars();

        let t = f64::clamp(t, 0.0, 1.0);

        let from_len = from.len() as i64;
        let to_len = to.len() as i64;
        let new_len_raw = from_len + ((to_len - from_len) as f64 * t).round() as i64;
        let new_len = i64::abs(new_len_raw) as usize;

        let chars_to_take = usize::min((to_len as f64 * t).round() as usize, to_len as usize);
        let result: SmallVec<[char; 128]> = to[..chars_to_take]
            .iter()
            .chain(from.iter().skip(chars_to_take))
            .take(new_len)
            .copied()
            .collect();
        GString::from(result.as_slice())
    }
}

impl SpireLerp<GString> for () {
    fn add_relative(&mut self, present_at_obj: &GString, prev_calc: &GString, next_calc: &GString) -> GString {
        let previous_calc = prev_calc.chars();
        let next_calc = next_calc.chars();

        let mut result = SmallVec::<[char; 128]>::from_slice(present_at_obj.chars());

        let prev_len = previous_calc.len();
        let next_len = next_calc.len();

        if prev_len < next_len {
            result.extend_from_slice(&next_calc[prev_len..]);
        } else {
            result.truncate(result.len().saturating_sub(prev_len - next_len));
        }

        GString::from(result.as_slice())
    }

    fn spire_step(&mut self, from: &GString, to: &GString, speed: f64, step: f64) -> (GString, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (from.clone(), StepResult::Unfinished { accumulated_time: 0. });
        }

        let abs_step = max_step.floor() as i64;
        let unused_time = (max_step - abs_step as f64) / speed;

        let from_chars = from.chars();
        let to_chars = to.chars();
        let from_len = from_chars.len();
        let to_len = to_chars.len();
        let mut result = SmallVec::<[char; 128]>::from_slice(from_chars);

        let mut remaining = abs_step;

        for (idx, &char) in to_chars.iter().enumerate() {
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
        let step_result = if result.as_slice() == to_chars {
            StepResult::Finished {
                excess_time: final_unused_time,
            }
        } else {
            StepResult::Unfinished {
                accumulated_time: final_unused_time,
            }
        };

        (GString::from(result.as_slice()), step_result)
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
