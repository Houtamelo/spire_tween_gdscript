use super::*;

impl ILerpable for Variant {
    type BasicLerper = CustomBasicLerper;
    type Lerper = CustomLerper;
}

/// Implements [`BasicLerp<Variant>`] via a user-supplied [`Callable`], used by
/// [`SpireTween::<LerpMethodData<Variant>>::new_custom`].
///
/// With no custom callable, uses the typed built-in implementation for supported
/// numbers, vectors, colors, strings, rotations, and rectangles. Other types fall
/// back to `godot::global::lerp`. Mixed integer/float endpoints use `f64`.
///
/// The callable signature is `func(from: Variant, to: Variant, weight: f64) -> Variant`.
#[derive(Default)]
pub struct CustomBasicLerper {
    /// User-supplied lerp function. `None` selects built-in interpolation.
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
    Vector4,
    Quaternion,
    Rect2,
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
                    VariantType::VECTOR4 => InferredType::Vector4,
                    VariantType::QUATERNION => InferredType::Quaternion,
                    VariantType::RECT2 => InferredType::Rect2,
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
            | InferredType::Vector4
            | InferredType::Quaternion
            | InferredType::Rect2
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
                // Variant tweens share the typed families' interpolation rules.
                macro_rules! typed_lerp {
                    ($ty:ty) => {{
                        let Some(from) = from.try_to_relaxed::<$ty>().log_if_err() else { return from.clone() };
                        let Some(to) = to.try_to_relaxed::<$ty>().log_if_err() else { return from.to_variant() };
                        return ().spire_lerp(&from, &to, weight).to_variant();
                    }};
                }
                match from.get_type() {
                    VariantType::INT if to.get_type() == VariantType::FLOAT => typed_lerp!(f64),
                    VariantType::INT => typed_lerp!(i64),
                    VariantType::FLOAT => typed_lerp!(f64),
                    VariantType::VECTOR2 => typed_lerp!(Vector2),
                    VariantType::VECTOR2I => typed_lerp!(Vector2i),
                    VariantType::VECTOR3 => typed_lerp!(Vector3),
                    VariantType::VECTOR3I => typed_lerp!(Vector3i),
                    VariantType::VECTOR4 => typed_lerp!(Vector4),
                    VariantType::COLOR => typed_lerp!(Color),
                    VariantType::STRING | VariantType::STRING_NAME | VariantType::NODE_PATH => typed_lerp!(GString),
                    VariantType::QUATERNION => typed_lerp!(Quaternion),
                    VariantType::RECT2 => typed_lerp!(Rect2),
                    _ => {}
                }
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
                InferredType::Vector4 => default_relative_fn_for_ty!(Vector4),
                InferredType::Quaternion => default_relative_fn_for_ty!(Quaternion),
                InferredType::Rect2 => default_relative_fn_for_ty!(Rect2),
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
                InferredType::Vector4 => default_step_fn_for_ty!(Vector4),
                InferredType::Quaternion => default_step_fn_for_ty!(Quaternion),
                InferredType::Rect2 => default_step_fn_for_ty!(Rect2),
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
                InferredType::Vector4 => default_distance_fn_for_ty!(Vector4),
                InferredType::Quaternion => default_distance_fn_for_ty!(Quaternion),
                InferredType::Rect2 => default_distance_fn_for_ty!(Rect2),
                InferredType::GString => default_distance_fn_for_ty!(GString),
            }
        }
    }
}
