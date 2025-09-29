use super::*;

#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct InternalCallableTweenGd {}

#[godot_api]
impl InternalCallableTweenGd {
    #[func]
    pub fn instantiate(
        callable: Callable,
        from: Variant,
        to: Variant,
        duration: f64,
        auto_play: bool,
        lerp_fn: Callable,
    ) -> Gd<Self> {
        let this = Self::new_gd();
        let tween = SpireTween::<CallableData>::new(
            callable,
            from,
            to,
            duration,
            AutoPlay(auto_play),
            lerp_fn,
        );

        let id = gd_handle_id!(this);
        TWEENS.register_with_id(id, tween.into());
        this
    }

    #[func(gd_self)]
    pub fn get_callable(this: Gd<Self>) -> Callable {
        tween_callable_ref(gd_handle_id!(this), |tween| tween.t.callable.clone())
            .log_if_err()
            .unwrap_or_else(Callable::invalid)
    }

    #[func(gd_self)]
    pub fn set_callable(this: Gd<Self>, callable: Callable) {
        tween_callable_mut(gd_handle_id!(this), |tween| {
            tween.t.callable = callable;
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn get_start_value(this: Gd<Self>) -> Variant {
        tween_callable_ref(gd_handle_id!(this), |tween| tween.t.from.clone())
            .log_if_err()
            .unwrap_or_else(Variant::nil)
    }

    #[func(gd_self)]
    pub fn set_start_value(this: Gd<Self>, from: Variant) {
        tween_callable_mut(gd_handle_id!(this), |tween| {
            tween.t.from = from;
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn get_final_value(this: Gd<Self>) -> Variant {
        tween_callable_ref(gd_handle_id!(this), |tween| tween.t.to.clone())
            .log_if_err()
            .unwrap_or_else(Variant::nil)
    }

    #[func(gd_self)]
    pub fn set_final_value(this: Gd<Self>, to: Variant) {
        tween_callable_mut(gd_handle_id!(this), |tween| {
            tween.t.to = to;
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn get_duration(this: Gd<Self>) -> f64 {
        tween_callable_ref(gd_handle_id!(this), |tween| tween.t.duration)
            .log_if_err()
            .unwrap_or_default()
    }

    #[func(gd_self)]
    pub fn set_duration(this: Gd<Self>, duration: f64) {
        tween_callable_mut(gd_handle_id!(this), |tween| {
            tween.t.duration = duration;
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn get_ease(this: Gd<Self>) -> Ease {
        tween_callable_ref(gd_handle_id!(this), |tween| tween.t.ease.clone())
            .log_if_err()
            .unwrap_or_default()
    }

    #[func(gd_self)]
    pub fn set_ease(this: Gd<Self>, ease: Ease) {
        tween_callable_mut(gd_handle_id!(this), |tween| {
            tween.t.ease = ease;
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn get_lerp_fn(this: Gd<Self>) -> Callable {
        tween_callable_ref(gd_handle_id!(this), |tween| tween.t.lerp_fn.clone())
            .log_if_err()
            .unwrap_or_else(Callable::invalid)
    }

    #[func(gd_self)]
    pub fn set_lerp_fn(this: Gd<Self>, lerp_func: Callable) {
        tween_callable_mut(gd_handle_id!(this), |tween| {
            tween.t.lerp_fn = lerp_func;
        })
        .log_if_err();
    }
}

define_base_gd_methods! { InternalCallableTweenGd }
