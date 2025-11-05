use super::*;

pub trait BasicLerp<T> {
    fn spire_lerp(&mut self, from: &T, to: &T, t: f64) -> T;
}

pub trait SpireLerp<T>: BasicLerp<T> {
    fn add_relative(&mut self, current_at_obj: &T, previous_relative: &T, new_relative: &T) -> T;
    fn spire_step(&mut self, from: &T, to: &T, speed: f64, step: f64) -> (T, StepResult);
    fn spire_distance(&mut self, from: &T, to: &T) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub enum StepResult {
    Unfinished { accumulated_time: f64 },
    Finished { excess_time: f64 },
}

impl BasicLerp<i32> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &i32, to: &i32, t: f64) -> i32 {
        (*from as f64 + (to - from) as f64 * t).round() as i32
    }
}

impl SpireLerp<i32> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &i32, previous_calc: &i32, new_calc: &i32) -> i32 {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(&mut self, from: &i32, to: &i32, speed: f64, step: f64) -> (i32, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

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
    #[inline]
    fn spire_distance(&mut self, from: &i32, to: &i32) -> f64 { i32::abs(to - from) as f64 }
}

impl BasicLerp<i64> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &i64, to: &i64, t: f64) -> i64 {
        (*from as f64 + (to - from) as f64 * t).round() as i64
    }
}

impl SpireLerp<i64> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &i64, previous_calc: &i64, new_calc: &i64) -> i64 {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(&mut self, from: &i64, to: &i64, speed: f64, step: f64) -> (i64, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

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
    #[inline]
    fn spire_distance(&mut self, from: &i64, to: &i64) -> f64 { i64::abs(to - from) as f64 }
}

impl BasicLerp<f32> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &f32, to: &f32, t: f64) -> f32 {
        let t = t as f32;
        from + (to - from) * t
    }
}

impl SpireLerp<f32> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &f32, previous_calc: &f32, new_calc: &f32) -> f32 {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(&mut self, from: &f32, to: &f32, speed: f64, step: f64) -> (f32, StepResult) {
        let speed = speed as f32;
        let step = step as f32;

        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

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
    #[inline]
    fn spire_distance(&mut self, from: &f32, to: &f32) -> f64 { f32::abs(to - from) as f64 }
}

impl BasicLerp<f64> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &f64, to: &f64, t: f64) -> f64 { from + (to - from) * t }
}

impl SpireLerp<f64> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &f64, previous_calc: &f64, new_calc: &f64) -> f64 {
        present_at_obj + new_calc - previous_calc
    }

    fn spire_step(&mut self, from: &f64, to: &f64, speed: f64, step: f64) -> (f64, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

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
    #[inline]
    fn spire_distance(&mut self, from: &f64, to: &f64) -> f64 { f64::abs(to - from) }
}

impl BasicLerp<Vector2> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &Vector2, to: &Vector2, weight: f64) -> Vector2 {
        let from = *from;
        let to = *to;
        Vector2::lerp(from, to, weight as f32)
    }
}

impl SpireLerp<Vector2> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &Vector2, previous_calc: &Vector2, new_calc: &Vector2) -> Vector2 {
        *present_at_obj + (*new_calc - *previous_calc)
    }

    fn spire_step(&mut self, from: &Vector2, to: &Vector2, speed: f64, step: f64) -> (Vector2, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

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
    #[inline]
    fn spire_distance(&mut self, from: &Vector2, to: &Vector2) -> f64 { from.distance_to(*to) as f64 }
}

impl BasicLerp<Vector2i> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &Vector2i, to: &Vector2i, weight: f64) -> Vector2i {
        Vector2i {
            x: self.spire_lerp(&from.x, &to.x, weight),
            y: self.spire_lerp(&from.y, &to.y, weight),
        }
    }
}

impl SpireLerp<Vector2i> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &Vector2i, previous_calc: &Vector2i, new_calc: &Vector2i) -> Vector2i {
        *present_at_obj + *new_calc - *previous_calc
    }

    fn spire_step(&mut self, from: &Vector2i, to: &Vector2i, speed: f64, step: f64) -> (Vector2i, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

        let remaining_distance = Self::spire_distance(self, from, to);
        let abs_step = f64::min(remaining_distance, max_step);

        let value = {
            let from_float = from.cast_float();
            let to_float = to.cast_float();
            let result_float = Vector2::move_toward(from_float, to_float, abs_step as f32);
            Vector2i {
                x: result_float.x.floor() as i32,
                y: result_float.y.floor() as i32,
            }
        };

        let actual_distance_moved = Self::spire_distance(self, from, &value);
        let unused_time = (max_step - actual_distance_moved) / speed;

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

    #[inline]
    fn spire_distance(&mut self, from: &Vector2i, to: &Vector2i) -> f64 {
        let dx = (to.x - from.x) as f64;
        let dy = (to.y - from.y) as f64;
        f64::sqrt(dx * dx + dy * dy)
    }
}

impl BasicLerp<Vector3> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &Vector3, to: &Vector3, t: f64) -> Vector3 {
        let from = *from;
        let to = *to;
        Vector3::lerp(from, to, t as f32)
    }
}

impl SpireLerp<Vector3> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &Vector3, previous_calc: &Vector3, new_calc: &Vector3) -> Vector3 {
        *present_at_obj + (*new_calc - *previous_calc)
    }

    fn spire_step(&mut self, from: &Vector3, to: &Vector3, speed: f64, step: f64) -> (Vector3, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

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

    #[inline]
    fn spire_distance(&mut self, from: &Vector3, to: &Vector3) -> f64 { from.distance_to(*to) as f64 }
}

impl BasicLerp<Vector3i> for () {
    #[inline]
    fn spire_lerp(&mut self, from: &Vector3i, to: &Vector3i, weight: f64) -> Vector3i {
        Vector3i {
            x: self.spire_lerp(&from.x, &to.x, weight),
            y: self.spire_lerp(&from.y, &to.y, weight),
            z: self.spire_lerp(&from.z, &to.z, weight),
        }
    }
}

impl SpireLerp<Vector3i> for () {
    #[inline]
    fn add_relative(&mut self, present_at_obj: &Vector3i, previous_calc: &Vector3i, new_calc: &Vector3i) -> Vector3i {
        *present_at_obj + *new_calc - *previous_calc
    }

    fn spire_step(&mut self, from: &Vector3i, to: &Vector3i, speed: f64, step: f64) -> (Vector3i, StepResult) {
        let max_step = speed * step;
        if max_step.is_zero_approx() {
            return (*from, StepResult::Unfinished { accumulated_time: 0. });
        }

        let remaining_distance = Self::spire_distance(self, from, to);
        let abs_step = f64::min(remaining_distance, max_step);

        let value = {
            let from_float = from.cast_float();
            let to_float = to.cast_float();
            let result_float = Vector3::move_toward(from_float, to_float, abs_step as f32);
            Vector3i {
                x: result_float.x.floor() as i32,
                y: result_float.y.floor() as i32,
                z: result_float.z.floor() as i32,
            }
        };

        let actual_distance_moved = Self::spire_distance(self, from, &value);
        let unused_time = (max_step - actual_distance_moved) / speed;

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

    #[inline]
    fn spire_distance(&mut self, from: &Vector3i, to: &Vector3i) -> f64 {
        let dx = (to.x - from.x) as f64;
        let dy = (to.y - from.y) as f64;
        let dz = (to.z - from.z) as f64;
        f64::sqrt(dx * dx + dy * dy + dz * dz)
    }
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

#[derive(Default)]
pub struct CustomBasicLerper {
    pub lerp_fn: Option<Callable>,
    inferred_ty: InferredType,
}

impl CustomBasicLerper {
    pub fn new(lerp_fn: Callable) -> Self {
        Self {
            lerp_fn: Some(lerp_fn),
            inferred_ty: Default::default(),
        }
    }
}

pub struct CustomLerper {
    pub base: CustomBasicLerper,
    pub relative_fn: Callable,
    pub step_fn: Callable,
    pub distance_fn: Callable,
}

impl CustomLerper {
    pub fn new(lerp_fn: Callable, relative_fn: Callable, step_fn: Callable, distance_fn: Callable) -> Self {
        Self {
            base: CustomBasicLerper {
                lerp_fn: Some(lerp_fn),
                inferred_ty: Default::default(),
            },
            relative_fn,
            step_fn,
            distance_fn,
        }
    }
}

#[derive(Default, Clone, Copy)]
enum InferredType {
    #[default]
    None,
    Int,
    Float,
    Color,
    Vector2,
    Vector3,
    GString,
}

impl Default for CustomLerper {
    fn default() -> Self {
        Self {
            base: Default::default(),
            relative_fn: Callable::invalid(),
            step_fn: Callable::invalid(),
            distance_fn: Callable::invalid(),
        }
    }
}

impl BasicLerp<Variant> for Callable {
    #[inline]
    fn spire_lerp(&mut self, from: &Variant, to: &Variant, weight: f64) -> Variant {
        self.call(&[from.clone(), to.clone(), weight.to_variant()])
    }
}

impl CustomBasicLerper {
    fn assimilate_ty_inference(&mut self, var: &Variant) {
        match self.inferred_ty {
            InferredType::None => {
                self.inferred_ty = match var.get_type() {
                    VariantType::INT => InferredType::Int,
                    VariantType::FLOAT => InferredType::Float,
                    VariantType::VECTOR2 => InferredType::Vector2,
                    VariantType::VECTOR3 => InferredType::Vector3,
                    VariantType::COLOR => InferredType::Color,
                    VariantType::STRING | VariantType::STRING_NAME | VariantType::NODE_PATH => InferredType::GString,
                    _ => InferredType::None,
                };
            }
            InferredType::Int => {
                // Godot allows user to pass integers as floats, so it's possible that the user previously provided
                // an integer for something that's supposed to be a float.
                if var.get_type() == VariantType::FLOAT {
                    self.inferred_ty = InferredType::Float;
                }
            }
            | InferredType::Float
            | InferredType::Color
            | InferredType::Vector2
            | InferredType::Vector3
            | InferredType::GString => {}
        }
    }
}

impl BasicLerp<Variant> for CustomBasicLerper {
    #[inline]
    fn spire_lerp(&mut self, from: &Variant, to: &Variant, weight: f64) -> Variant {
        match &mut self.lerp_fn {
            Some(func) => func.spire_lerp(from, to, weight),
            None => {
                let weight_as_var = weight.to_variant();
                godot::global::lerp(from, to, &weight_as_var)
            }
        }
    }
}

impl BasicLerp<Variant> for CustomLerper {
    #[inline]
    fn spire_lerp(&mut self, from: &Variant, to: &Variant, weight: f64) -> Variant {
        self.base.spire_lerp(from, to, weight)
    }
}

impl SpireLerp<Variant> for CustomLerper {
    #[inline]
    fn add_relative(
        &mut self,
        current_at_obj: &Variant,
        previous_relative: &Variant,
        new_relative: &Variant,
    ) -> Variant {
        macro_rules! default_relative_fn_for_ty {
            ($Ty:ty) => {{
                let Some(current_at_obj_var) = current_at_obj.try_to_relaxed::<$Ty>().log_if_err()
                else { return current_at_obj.clone() };

                let Some(previous_relative_var) = previous_relative.try_to_relaxed::<$Ty>().log_if_err()
                else { return current_at_obj.clone() };

                let Some(new_relative_var) = new_relative.try_to_relaxed::<$Ty>().log_if_err()
                else { return current_at_obj.clone() };

                <() as SpireLerp<$Ty>>::add_relative(
                    &mut (),
                    &current_at_obj_var,
                    &previous_relative_var,
                    &new_relative_var,
                )
                .to_variant()
            }};
        }

        if self.relative_fn.is_valid() {
            self.relative_fn
                .call(&[current_at_obj.clone(), previous_relative.clone(), new_relative.clone()])
        } else {
            self.base.assimilate_ty_inference(current_at_obj);
            self.base.assimilate_ty_inference(previous_relative);
            self.base.assimilate_ty_inference(new_relative);

            match self.base.inferred_ty {
                InferredType::None => {
                    godot_error!(
                        "Cannot perform relative-addition for `Variant` values `{:?}`, `{:?}`, `{:?}` because the \
                         type could not be inferred and no custom relative-addition function was provided.",
                        current_at_obj,
                        previous_relative,
                        new_relative
                    );

                    current_at_obj.clone()
                }
                InferredType::Int => default_relative_fn_for_ty!(i64),
                InferredType::Float => default_relative_fn_for_ty!(f64),
                InferredType::Color => default_relative_fn_for_ty!(Color),
                InferredType::Vector2 => default_relative_fn_for_ty!(Vector2),
                InferredType::Vector3 => default_relative_fn_for_ty!(Vector3),
                InferredType::GString => default_relative_fn_for_ty!(GString),
            }
        }
    }

    fn spire_step(&mut self, from: &Variant, to: &Variant, speed: f64, weight: f64) -> (Variant, StepResult) {
        macro_rules! default_step_fn_for_ty {
            ($Ty:ty) => {{
                let Some(from_var) = from.try_to_relaxed::<$Ty>().log_if_err()
                else { return (from.clone(), StepResult::Finished { excess_time: 0. }) };

                let Some(to_var) = to.try_to_relaxed::<$Ty>().log_if_err()
                else { return (from.clone(), StepResult::Finished { excess_time: 0. }) };

                let (value, step_result) =
                    <() as SpireLerp<$Ty>>::spire_step(&mut (), &from_var, &to_var, speed, weight);

                return (value.to_variant(), step_result);
            }};
        }

        if !self.step_fn.is_valid() {
            self.base.assimilate_ty_inference(from);
            self.base.assimilate_ty_inference(to);

            match self.base.inferred_ty {
                InferredType::None => {
                    godot_error!(
                        "Cannot perform step for `Variant` values `{:?}` and `{:?}` because the type could not be \
                         inferred and no custom step function was provided.",
                        from,
                        to
                    );

                    return (from.clone(), StepResult::Finished { excess_time: 0. });
                }
                InferredType::Int => default_step_fn_for_ty!(i64),
                InferredType::Float => default_step_fn_for_ty!(f64),
                InferredType::Color => default_step_fn_for_ty!(Color),
                InferredType::Vector2 => default_step_fn_for_ty!(Vector2),
                InferredType::Vector3 => default_step_fn_for_ty!(Vector3),
                InferredType::GString => default_step_fn_for_ty!(GString),
            }
        };

        let result = self
            .step_fn
            .call(&[from.clone(), to.clone(), speed.to_variant(), weight.to_variant()]);

        let Ok(dict) = result.try_to_relaxed::<Dictionary>()
        else {
            return (from.clone(), StepResult::Finished { excess_time: 0. });
        };

        let Some(value) = dict.get("value")
        else {
            godot_warn!(
                "Expected lerp step callable `{:?}` 's returned Dictionary to contain a 'value' key of type `Variant`.",
                self.step_fn
            );
            return (from.clone(), StepResult::Finished { excess_time: 0. });
        };

        let Some(is_finished) = dict.get("is_finished").and_then(|v| v.try_to_relaxed::<bool>().ok())
        else {
            godot_warn!(
                "Expected lerp step callable `{:?}` 's returned Dictionary to contain an 'is_finished' key of type \
                 `bool`.",
                self.step_fn
            );
            return (value, StepResult::Unfinished { accumulated_time: 0. });
        };

        let Some(time) = dict.get("fuel").and_then(|v| v.try_to_relaxed::<f64>().ok())
        else {
            godot_warn!(
                "Expected lerp step callable `{:?}` 's returned Dictionary to contain a 'fuel' key of type `f64`.",
                self.step_fn
            );
            return (
                value,
                if is_finished {
                    StepResult::Finished { excess_time: 0. }
                } else {
                    StepResult::Unfinished { accumulated_time: 0. }
                },
            );
        };

        if is_finished {
            (value, StepResult::Finished { excess_time: time })
        } else {
            (value, StepResult::Unfinished { accumulated_time: time })
        }
    }

    fn spire_distance(&mut self, from: &Variant, to: &Variant) -> f64 {
        macro_rules! default_distance_fn_for_ty {
            ($Ty:ty) => {{
                let Some(from_var) = from.try_to_relaxed::<$Ty>().log_if_err()
                else { return 0. };

                let Some(to_var) = to.try_to_relaxed::<$Ty>().log_if_err()
                else { return 0. };

                <() as SpireLerp<$Ty>>::spire_distance(&mut (), &from_var, &to_var)
            }};
        }

        if self.distance_fn.is_valid() {
            self.distance_fn
                .call(&[from.clone(), to.clone()])
                .try_to_relaxed::<f64>()
                .map_err(|err| {
                    godot_error!(
                        "Expected distance callable `{:?}` to return a `f64`, got conversion error: {:?}",
                        self.distance_fn,
                        err
                    );
                })
                .unwrap_or_default()
        } else {
            self.base.assimilate_ty_inference(from);
            self.base.assimilate_ty_inference(to);

            match self.base.inferred_ty {
                InferredType::None => {
                    godot_error!(
                        "Cannot perform distance calculation for `Variant` values `{:?}` and `{:?}` because the type \
                         could not be inferred and no custom distance function was provided.",
                        from,
                        to
                    );

                    0.
                }
                InferredType::Int => default_distance_fn_for_ty!(i64),
                InferredType::Float => default_distance_fn_for_ty!(f64),
                InferredType::Color => default_distance_fn_for_ty!(Color),
                InferredType::Vector2 => default_distance_fn_for_ty!(Vector2),
                InferredType::Vector3 => default_distance_fn_for_ty!(Vector3),
                InferredType::GString => default_distance_fn_for_ty!(GString),
            }
        }
    }
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

        result.into_iter().take(new_len).collect::<String>().into()
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

        result.into()
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

        (value.into(), step_result)
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
