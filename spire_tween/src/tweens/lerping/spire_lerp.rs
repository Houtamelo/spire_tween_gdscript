use super::*;

/// The minimum lerping operation: take `from`, `to`, and a weight, return the
/// interpolated value.
///
/// Built-in numeric and vector types use the unit `()` as their lerper — see the
/// many `impl BasicLerp<…> for ()` blocks in this module. Implement `BasicLerp` on a
/// new type when you want to plug a custom data type into [`LerpMethodData`] (the
/// minimum requirement for a method tween).
///
/// # Contract
///
/// `spire_lerp(from, to, weight)` should return:
/// - `from` when `weight == 0.0`
/// - `to` when `weight == 1.0`
/// - the value halfway between when `weight == 0.5`
/// - extrapolated values when `weight` falls outside `0.0 ~ 1.0` (this can happen
///   under [`LoopMode::Incremental`] or with custom easing functions)
///
/// For numbers and vectors the canonical formula is `from + (to - from) * weight`.
/// For non-arithmetic types (strings, custom enums, …) the formula is up to you, as
/// long as the contract above holds.
pub trait BasicLerp<T> {
    fn spire_lerp(&mut self, from: &T, to: &T, t: f64) -> T;
}

/// Extends [`BasicLerp`] with the three additional operations Spire needs to power
/// **relative** ([`LerpMode::Relative`]) and **speed-based** ([`LerpMode::SpeedBased`])
/// property tweens.
///
/// You only need to implement `SpireLerp` (instead of just [`BasicLerp`]) when:
/// - the type is going to be plugged into [`LerpPropertyData`] (the property tween),
///   and
/// - users may want to use that type in relative or speed-based mode.
///
/// For [`LerpMethodData`] (method tweens), only `BasicLerp` is required.
///
/// The four operations Spire calls per mode:
///
/// | Mode             | Calls                                     |
/// |------------------|-------------------------------------------|
/// | [`LerpMode::Absolute`]    | `spire_lerp` only                |
/// | [`LerpMode::Relative`]    | `spire_lerp` + `add_relative`    |
/// | [`LerpMode::SpeedBased`]  | `spire_step` + `spire_distance`  |
///
/// See each method's documentation for the contract.
pub trait SpireLerp<T>: BasicLerp<T> {
    /// **Used in [`LerpMode::Relative`].**
    ///
    /// A relative tween needs to add an *increment* to the property's current value
    /// each tick — without overwriting other forces affecting the property. The
    /// challenge: how do you do that generically for any `T` (including types like
    /// `GString` where you can't multiply by a `f64`)?
    ///
    /// Spire solves this by computing two interpolation samples per tick:
    /// - `previous_relative = lerp(from, to, previous_weight)`
    /// - `new_relative      = lerp(from, to, current_weight)`
    ///
    /// Then `next_value = current_at_obj + (new_relative - previous_relative)`. The
    /// difference `(new - previous)` is the increment for this tick; adding it to
    /// `current_at_obj` lets the tween blend with whatever else has moved the
    /// property since last frame.
    ///
    /// For numeric / vector types the implementation is literally
    /// `current_at_obj + new_relative - previous_relative`. For exotic types you
    /// need an analogous "add the delta" operation.
    ///
    /// # Train/passenger analogy
    ///
    /// Imagine a train whose speed varies. A passenger inside walks toward the
    /// front at a constant 2 m/s *relative to the train*. Each frame the
    /// passenger's world position changes by both the train's movement *and* their
    /// own. We don't want to model the train (other forces); we just want to add
    /// our own 2 m/s × delta_time to the passenger's current world position.
    /// `add_relative` is "add my own movement to wherever they are now".
    fn add_relative(&mut self, current_at_obj: &T, previous_relative: &T, new_relative: &T) -> T;

    /// **Used in [`LerpMode::SpeedBased`].**
    ///
    /// Step `from` toward `to` by at most `speed * step` units of progress. Returns
    /// the new value plus a [`StepResult`] indicating whether the target was reached.
    ///
    /// Naive implementation `current += speed * step` fails for integer types: when
    /// `speed * step < 0.5`, the increment rounds to `0` and the value never
    /// changes — or rounds to `1` and overshoots. Spire models `speed * step` as
    /// "fuel" that callers can save up between ticks via [`StepResult::Unfinished::accumulated_time`]
    /// and spend later when there's enough for a whole-unit move.
    ///
    /// **Contract:**
    /// - Never overshoot — return `to` (or as close as your unit allows) and stop.
    /// - When the result equals `to`, return [`StepResult::Finished`] with the unspent
    ///   fuel as `excess_time`. Spire feeds that excess into the parent sequence so
    ///   it doesn't drift.
    /// - Otherwise return [`StepResult::Unfinished`] with the saved-up fuel as
    ///   `accumulated_time` (so the next call can spend it).
    /// - Spire trusts you on `is_finished`: it does not double-check by comparing
    ///   `value == to`.
    fn spire_step(&mut self, from: &T, to: &T, speed: f64, step: f64) -> (T, StepResult);

    /// **Used in [`LerpMode::SpeedBased`].**
    ///
    /// "How far apart are these two values?" — used both to compute speed-based
    /// progress (for easing) and to know when the tween has reached its target.
    ///
    /// For numbers: `(to - from).abs()`. For vectors: Euclidean distance. For
    /// `Color`: 4D distance treating `(r,g,b,a)` as a vector. For `GString`: the
    /// number of characters that differ (Spire's built-in impl).
    fn spire_distance(&mut self, from: &T, to: &T) -> f64;
}

/// Outcome of one [`SpireLerp::spire_step`] call. Carries leftover "fuel" so the
/// stepper can integer-quantize without losing time across ticks.
#[derive(Debug, Clone, Copy)]
pub enum StepResult {
    /// The target wasn't reached on this tick. `accumulated_time` is the unspent
    /// fuel that should be saved and added to the next tick's `step` argument.
    Unfinished { accumulated_time: f64 },
    /// The target was reached on this tick. `excess_time` is the overshoot that
    /// Spire forwards to the parent sequence (for clean handoff to the next block)
    /// or to the next loop.
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

/// Implements [`BasicLerp<Variant>`] via a user-supplied [`Callable`], used by
/// [`SpireTween::<LerpMethodData<Variant>>::new_custom`].
///
/// If the callable is missing or invalid, falls back to `godot::global::lerp` with
/// runtime type inference (good for built-in numeric types; produces a
/// `godot_error!` if it can't infer).
///
/// The callable signature is `func(from: Variant, to: Variant, weight: f64) -> Variant`.
#[derive(Default)]
pub struct CustomBasicLerper {
    /// User-supplied lerp function. `None` falls back to inference + `godot::global::lerp`.
    pub lerp_fn: Option<Callable>,
    inferred_ty: InferredType,
}

impl CustomBasicLerper {
    /// Wraps `lerp_fn` so it can be plugged into [`LerpMethodData`].
    pub fn new(lerp_fn: Callable) -> Self {
        Self {
            lerp_fn: Some(lerp_fn),
            inferred_ty: Default::default(),
        }
    }
}

/// Implements [`SpireLerp<Variant>`] via four user-supplied [`Callable`]s — used by
/// [`SpireTween::<LerpPropertyData<Variant>>::new_custom`] for property tweens whose
/// type isn't natively supported.
///
/// The four callables, with their signatures and modes that consume them:
///
/// | Field           | Signature                                                               | Used by                                                             |
/// |-----------------|-------------------------------------------------------------------------|---------------------------------------------------------------------|
/// | [`base`](Self::base)`.lerp_fn`        | `func(from, to, weight: f64) -> T`                                      | absolute, relative                                                  |
/// | [`relative_fn`](Self::relative_fn) | `func(current, previous_lerp, next_lerp) -> T`                          | relative                                                            |
/// | [`step_fn`](Self::step_fn)         | `func(from, to, speed: f64, fuel: f64) -> Dictionary{value, is_finished, fuel}` | speed-based                                                         |
/// | [`distance_fn`](Self::distance_fn) | `func(from, to) -> f64`                                                 | speed-based                                                         |
///
/// See [`SpireLerp`]'s per-method docs for the contract each callable must satisfy.
///
/// **Fallbacks:** any invalid `Callable` (including `Callable::invalid()`) triggers
/// type inference + the built-in `()` impl for the inferred type. Inference looks at
/// the [`VariantType`] of values it sees; if it can't infer (e.g. a custom Godot
/// resource), Spire emits a `godot_error!`. Pass valid callables for the modes you'll
/// actually use; pass `Callable::invalid()` for the rest.
pub struct CustomLerper {
    pub base: CustomBasicLerper,
    pub relative_fn: Callable,
    pub step_fn: Callable,
    pub distance_fn: Callable,
}

impl CustomLerper {
    /// Constructs a lerper from all four callables. Pass `Callable::invalid()` for
    /// any operation you don't need — Spire will fall back to inference + the
    /// built-in impl for the inferred type if inference succeeds.
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
                let Some(current_at_obj_var) = current_at_obj.try_to_relaxed::<$Ty>().log_if_err() else {
                    return current_at_obj.clone()
                };

                let Some(previous_relative_var) = previous_relative.try_to_relaxed::<$Ty>().log_if_err() else {
                    return current_at_obj.clone()
                };

                let Some(new_relative_var) = new_relative.try_to_relaxed::<$Ty>().log_if_err() else {
                    return current_at_obj.clone()
                };

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
                let Some(from_var) = from.try_to_relaxed::<$Ty>().log_if_err() else {
                    return (from.clone(), StepResult::Finished { excess_time: 0. })
                };

                let Some(to_var) = to.try_to_relaxed::<$Ty>().log_if_err() else {
                    return (from.clone(), StepResult::Finished { excess_time: 0. })
                };

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

        let Ok(dict) = result.try_to_relaxed::<VarDictionary>() else {
            return (from.clone(), StepResult::Finished { excess_time: 0. });
        };

        let Some(value) = dict.get("value") else {
            godot_warn!(
                "Expected lerp step callable `{:?}` 's returned Dictionary to contain a 'value' key of type `Variant`.",
                self.step_fn
            );
            return (from.clone(), StepResult::Finished { excess_time: 0. });
        };

        let Some(is_finished) = dict.get("is_finished").and_then(|v| v.try_to_relaxed::<bool>().ok()) else {
            godot_warn!(
                "Expected lerp step callable `{:?}` 's returned Dictionary to contain an 'is_finished' key of type \
                 `bool`.",
                self.step_fn
            );
            return (value, StepResult::Unfinished { accumulated_time: 0. });
        };

        let Some(time) = dict.get("fuel").and_then(|v| v.try_to_relaxed::<f64>().ok()) else {
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
                let Some(from_var) = from.try_to_relaxed::<$Ty>().log_if_err() else { return 0. };

                let Some(to_var) = to.try_to_relaxed::<$Ty>().log_if_err() else { return 0. };

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
