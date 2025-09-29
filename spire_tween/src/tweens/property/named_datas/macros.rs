macro_rules! nested_try_from_path_and_object {
    ($path_var: ident, $obj_var: ident, $($Tys:ty),*) => {{
        $(
            if let Some(data) = <$Tys>::try_from_path_and_object($path_var, $obj_var.clone()) {
                return data.into();
            }
        )*
    }};
}

pub(crate) use nested_try_from_path_and_object;

macro_rules! gd_property_tween {
    ($GdName:ident, $T:ty, $P:ident) => {
        #[derive(GodotClass)]
        #[class(base = RefCounted, init)]
        pub struct $GdName {}

        #[godot_api]
        impl $GdName {
            #[func(gd_self)]
            pub fn get_property_path(this: Gd<Self>) -> NodePath {
                tween_property_ref(gd_handle_id!(this), |tween| tween.get_property_path().clone())
                    .log_if_err()
                    .unwrap_or_default()
            }

            #[func(gd_self)]
            pub fn set_property_path(this: Gd<Self>, property_path: NodePath) {
                tween_property_mut(gd_handle_id!(this), |tween| {
                    tween.set_property_path(property_path)
                })
                .log_if_err();
            }

            #[func(gd_self)]
            pub fn get_owner(this: Gd<Self>) -> Option<Gd<Object>> {
                tween_property_ref(gd_handle_id!(this), |tween| tween.get_owner().to_object())
                    .log_if_err()
            }

            #[func(gd_self)]
            pub fn set_owner(this: Gd<Self>, owner: Gd<Object>) {
                tween_property_mut(gd_handle_id!(this), |tween| {
                    tween.set_owner(ObjectOrNode::from_unchecked_object(owner))
                })
                .log_if_err();
            }

            #[func(gd_self)]
            pub fn get_ease(this: Gd<Self>) -> Variant {
                tween_property_ref(gd_handle_id!(this), |tween| tween.get_ease().to_variant())
                    .log_if_err()
                    .unwrap_or_default()
            }

            #[func(gd_self)]
            pub fn set_ease(this: Gd<Self>, ease: Variant) {
                match ease.try_to::<Ease>() {
                    Ok(ok) => {
                        tween_property_mut(gd_handle_id!(this), |tween| tween.set_ease(ok))
                            .log_if_err();
                    }
                    Err(err) => godot_error!("Could not convert Variant to `Ease`: {err}"),
                }
            }

            #[func(gd_self)]
            pub fn set_start_value(this: Gd<Self>, value: $T) {
                tween_property_mut(gd_handle_id!(this), |tween| {
                    match tween {
                        PropertyTween::$P(t) => {
                            t.set_begin_value(value);
                        }
                        other => {
                            godot_error!(
                                "Expected tween to be a `{}` tween, got: {:?}.",
                                stringify!($P),
                                other.inner_type_name()
                            );
                        }
                    }
                })
                .log_if_err();
            }

            #[func(gd_self)]
            pub fn get_final_value(this: Gd<Self>) -> $T {
                tween_property_mut(gd_handle_id!(this), |tween| {
                    if let PropertyTween::$P(t) = tween {
                        t.t.to.eval()
                    } else {
                        godot_error!(
                            "Expected tween to be a `{}` tween, got: {:?}.",
                            stringify!($P),
                            tween.inner_type_name()
                        );
                        <$T>::default()
                    }
                })
                .log_if_err()
                .unwrap_or_default()
            }

            #[func(gd_self)]
            pub fn set_final_value(this: Gd<Self>, value: $T) {
                tween_property_mut(gd_handle_id!(this), |tween| {
                    if let PropertyTween::$P(t) = tween {
                        t.t.to = Evaluator::Static(value);
                    } else {
                        godot_error!(
                            "Expected tween to be a color tween, got: {:?}.",
                            tween.inner_type_name()
                        );
                    }
                })
                .log_if_err();
            }

            #[func(gd_self)]
            pub fn get_duration(this: Gd<Self>) -> f64 {
                tween_property_mut(gd_handle_id!(this), |tween| tween.get_duration())
                    .log_if_err()
                    .unwrap_or(0.0)
            }

            #[func(gd_self)]
            pub fn set_duration(this: Gd<Self>, duration: f64) {
                tween_property_mut(gd_handle_id!(this), |tween| {
                    tween.set_duration(duration);
                })
                .log_if_err();
            }

            #[func(gd_self)]
            pub fn is_absolute(this: Gd<Self>) -> bool {
                tween_property_ref(gd_handle_id!(this), |tween| tween.is_absolute())
                    .log_if_err()
                    .unwrap_or(false)
            }

            #[func(gd_self)]
            pub fn is_relative(this: Gd<Self>) -> bool {
                tween_property_ref(gd_handle_id!(this), |tween| tween.is_relative())
                    .log_if_err()
                    .unwrap_or(false)
            }

            #[func(gd_self)]
            pub fn is_speed_based(this: Gd<Self>) -> bool {
                tween_property_ref(gd_handle_id!(this), |tween| tween.is_speed_based())
                    .log_if_err()
                    .unwrap_or(false)
            }

            #[func(gd_self)]
            pub fn set_absolute(this: Gd<Self>) {
                tween_property_mut(gd_handle_id!(this), |tween| {
                    tween.set_absolute();
                })
                .log_if_err();
            }

            #[func(gd_self)]
            pub fn set_speed_based(this: Gd<Self>, speed: f64) {
                tween_property_mut(gd_handle_id!(this), |tween| {
                    tween.set_speed_based(speed);
                })
                .log_if_err();
            }

            #[func(gd_self)]
            pub fn set_relative(this: Gd<Self>, relative_to: $T) {
                tween_mut::<LerpPropertyData<$T>, _>(gd_handle_id!(this), |tween| {
                    tween.set_relative(relative_to);
                })
                .log_if_err();
            }
        }

        define_base_gd_methods! { $GdName }
    };
}

use super::*;

gd_property_tween! { InternalTweenPropertyI32, i32, i32  }
gd_property_tween! { InternalTweenPropertyI64, i64, i64  }
gd_property_tween! { InternalTweenPropertyF32, f32, f32  }
gd_property_tween! { InternalTweenPropertyF64, f64, f64  }
gd_property_tween! { InternalTweenPropertyString, GString, GString  }
gd_property_tween! { InternalTweenPropertyColor, Color, Color  }
gd_property_tween! { InternalTweenPropertyVector2, Vector2, Vector2  }
gd_property_tween! { InternalTweenPropertyVector3, Vector3, Vector3  }
gd_property_tween! { InternalTweenProperty, Variant, Variant  }

macro_rules! define_instantiate_fn {
    ($GdName:ident, $T:ty) => {
        #[godot_api(secondary)]
        impl $GdName {
            #[func]
            pub fn instantiate(
                owner: Gd<Object>,
                property_path: NodePath,
                to: $T,
                duration: f64,
                auto_play: bool,
            ) -> Gd<Self> {
                let property_str = &property_path.to_string();

                let data = <$T as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
                let tween = SpireTween::<LerpPropertyData<$T>>::new(
                    data,
                    Evaluator::Static(to),
                    duration,
                    AutoPlay(auto_play),
                );

                let this = Self::new_gd();
                register_gd_handle! { tween, this }
            }
        }
    };
}

define_instantiate_fn! { InternalTweenPropertyI32, i32 }
define_instantiate_fn! { InternalTweenPropertyI64, i64 }
define_instantiate_fn! { InternalTweenPropertyF32, f32 }
define_instantiate_fn! { InternalTweenPropertyF64, f64 }
define_instantiate_fn! { InternalTweenPropertyString, GString }
define_instantiate_fn! { InternalTweenPropertyColor, Color }
define_instantiate_fn! { InternalTweenPropertyVector2, Vector2 }
define_instantiate_fn! { InternalTweenPropertyVector3, Vector3 }

#[godot_api(secondary)]
impl InternalTweenProperty {
    #[func]
    pub fn instantiate(
        owner: Gd<Object>,
        property_path: NodePath,
        to: Variant,
        duration: f64,
        auto_play: bool,
        lerp_fn: Callable,
        relative_fn: Callable,
        step_fn: Callable,
        distance_fn: Callable,
    ) -> Gd<Self> {
        let tween = SpireTween::<LerpPropertyData<Variant>>::new(
            &property_path,
            ObjectOrNode::from_unchecked_object(owner),
            Evaluator::Static(to),
            duration,
            AutoPlay(auto_play),
            CowLerpFn::Boxed(Box::new(move |from: &Variant, to: &Variant, t: f64| -> Variant {
                lerp_fn.call(&[from.clone(), to.clone(), t.to_variant()])
            })),
            CowRelativeFn::Boxed(Box::new(
                move |current_at_obj: &Variant,
                      previous_relative: &Variant,
                      new_relative: &Variant|
                      -> Variant {
                    relative_fn.call(&[
                        current_at_obj.clone(),
                        previous_relative.clone(),
                        new_relative.clone(),
                    ])
                },
            )),
            CowStepFn::Boxed(Box::new(
                move |from: &Variant, to: &Variant, speed: f64, t: f64| -> (Variant, StepResult) {
                    macro_rules! default {
                        () => {
                            (Variant::nil(), StepResult::Finished { excess_time: 0.0 })
                        };
                    }

                    let Some(dict) = step_fn
                        .call(&[from.clone(), to.clone(), speed.to_variant(), t.to_variant()])
                        .try_to::<Dictionary>()
                        .log_if_err()
                    else {
                        return default!();
                    };

                    let Some(result) = dict.try_var_at::<Variant>("result").log_if_err()
                    else {
                        return default!();
                    };

                    if let Ok(accumulated_time) = dict.try_var_at::<f64>("accumulated_time") {
                        (result, StepResult::Unfinished { accumulated_time })
                    } else if let Ok(excess_time) = dict.try_var_at::<f64>("excess_time") {
                        (result, StepResult::Finished { excess_time })
                    } else {
                        godot_error!(
                            "Expected dictionary to contain one of the following keys: `accumulated_time` or `excess_time`."
                        );
                        default!()
                    }
                },
            )),
            CowDistanceFn::Boxed(Box::new(move |from: &Variant, to: &Variant| -> f64 {
                distance_fn
                    .call(&[from.clone(), to.clone()])
                    .try_to::<f64>()
                    .log_if_err()
                    .unwrap_or_default()
            })),
        );

        let this = Self::new_gd();
        register_gd_handle! { tween, this }
    }
}

use crate::cow_fn::CowRelativeFn;
