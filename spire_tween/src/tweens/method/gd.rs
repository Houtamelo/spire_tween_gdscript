use super::*;

#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct InternalMethodTween {}

#[godot_api]
impl InternalMethodTween {
    #[func]
    pub fn instantiate(
        owner: Gd<Object>,
        method: StringName,
        from: Variant,
        to: Variant,
        duration: f64,
        lerp_fn: Callable,
    ) -> Gd<Self> {
        let handle = Self::new_gd();

        let tween = MethodTween::from(SpireTween::<MethodData<Variant>>::new(
            method,
            ObjectOrNode::from_unchecked_object(owner),
            from,
            to,
            duration,
            AutoPlay(true),
            move |from: &Variant, to: &Variant, t: f64| {
                lerp_fn
                    .call(&[from.to_variant(), to.to_variant(), t.to_variant()])
                    .try_to()
                    .unwrap_or_default()
            },
        ));

        register_gd_handle! { tween, handle }
    }

    #[func(gd_self)]
    pub fn get_method_name(this: Gd<Self>) -> StringName {
        tween_method_ref(gd_handle_id!(this), |tween| tween.get_method_name())
            .log_if_err()
            .unwrap_or_else(|| StringName::from("NotAMethodTween"))
    }

    #[func(gd_self)]
    pub fn set_method_name(this: Gd<Self>, method: StringName) {
        tween_method_mut(gd_handle_id!(this), |tween| tween.set_method_name(method)).log_if_err();
    }

    #[func(gd_self)]
    pub fn get_owner(this: Gd<Self>) -> Option<Gd<Object>> {
        tween_method_ref(gd_handle_id!(this), |tween| tween.get_owner().clone().into_object())
            .log_if_err()
    }

    #[func(gd_self)]
    pub fn set_owner(this: Gd<Self>, new_owner: Gd<Object>) {
        tween_method_mut(gd_handle_id!(this), |tween| {
            tween.set_owner(ObjectOrNode::from_unchecked_object(new_owner))
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn get_duration(this: Gd<Self>) -> f64 {
        tween_method_mut(gd_handle_id!(this), |tween| tween.get_duration())
            .log_if_err()
            .unwrap_or_default()
    }

    #[func(gd_self)]
    pub fn set_duration(this: Gd<Self>, duration: f64) {
        tween_method_mut(gd_handle_id!(this), |tween| {
            tween.set_duration(duration);
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn get_ease(this: Gd<Self>) -> Variant {
        tween_method_ref(gd_handle_id!(this), |tween| tween.get_ease().to_variant())
            .log_if_err()
            .unwrap_or_default()
    }

    #[func(gd_self)]
    pub fn set_ease(this: Gd<Self>, ease: Variant) {
        match ease.try_to::<Ease>() {
            Ok(ok) => {
                tween_method_mut(gd_handle_id!(this), |tween| tween.set_ease(ok)).log_if_err();
            }
            Err(err) => godot_error!("Could not convert Variant to `Ease`: {err}"),
        }
    }

    #[func(gd_self)]
    pub fn get_start_value(this: Gd<Self>) -> Variant {
        tween_method_mut(gd_handle_id!(this), |tween| tween.get_start_value())
            .log_if_err()
            .unwrap_or_default()
    }

    #[func(gd_self)]
    pub fn set_start_value(this: Gd<Self>, value: Variant) {
        tween_method_mut(gd_handle_id!(this), |tween| {
            tween.set_start_value(value).log_if_err();
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn get_final_value(this: Gd<Self>) -> Variant {
        tween_method_mut(gd_handle_id!(this), |tween| tween.get_final_value())
            .log_if_err()
            .unwrap_or_default()
    }

    #[func(gd_self)]
    pub fn set_final_value(this: Gd<Self>, value: Variant) {
        tween_method_mut(gd_handle_id!(this), |tween| {
            tween.set_final_value(value).log_if_err();
        })
        .log_if_err();
    }
}

define_base_gd_methods! { InternalMethodTween }

macro_rules! constructors {
    (
        $({ $fn_name:ident, $val:ty })*
    ) => {
        #[godot_api(secondary)]
        impl InternalMethodTween {
            $(
                #[func]
                pub fn $fn_name(
                    owner: Gd<Object>,
                    method: StringName,
                    from: $val,
                    to: $val,
                    duration: f64,
                ) -> Gd<Self> {
                    let handle = Self::new_gd();

                    let tween = MethodTween::from(SpireTween::<MethodData::<$val>>::new(
                        method,
                        ObjectOrNode::from_unchecked_object(owner),
                        from,
                        to,
                        duration,
                        AutoPlay(true),
                    ));

                    register_gd_handle! { tween, handle }
                }
            )*
        }
    };
}

constructors! {
    { new_i32, i32 }
    { new_i64, i64 }
    { new_f32, f32 }
    { new_f64, f64 }
    { new_string, GString }
    { new_vector2, Vector2 }
    { new_vector3, Vector3 }
    { new_color, Color }
}
