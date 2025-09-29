use super::*;

#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct InternalSequenceGd {}

#[godot_api]
impl InternalSequenceGd {
    #[func]
    pub fn instantiate() -> Gd<Self> {
        let this = Self::new_gd();
        let sequence = SpireSequence::new();
        let id = gd_handle_id!(this);
        TWEENS.register_with_id(id, sequence.into());
        this
    }

    #[func(gd_self)]
    pub fn append(this: Gd<Self>, handle: Gd<RefCounted>) {
        let handle_id = gd_handle_id!(handle);
        let Some(to_append) = TWEENS.get_or_make_shared(handle_id)
        else {
            godot_error!("Tween with ID {:?} not found.", handle_id);
            return;
        };

        tween_sequence_mut(gd_handle_id!(this), |sequence| {
            sequence.append_shared(to_append);
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn append_call(this: Gd<Self>, func: Callable) {
        tween_sequence_mut(gd_handle_id!(this), |sequence| {
            sequence.append_callable(func);
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn append_interval(this: Gd<Self>, seconds: f64) {
        tween_sequence_mut(gd_handle_id!(this), |sequence| {
            sequence.append_interval(seconds);
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn append_many(this: Gd<Self>, tweens: VariantArray) {
        tween_sequence_mut(gd_handle_id!(this), |sequence| {
            for var in tweens.iter_shared() {
                if let Ok(callable) = var.try_to::<Callable>() {
                    sequence.append_callable(callable);
                } else if let Ok(handle) = var.try_to::<Gd<RefCounted>>() {
                    if let Some(tween) = TWEENS.get_or_make_shared(gd_handle_id!(handle)) {
                        sequence.append_shared(tween);
                    }
                } else {
                    godot_error!("Expected Callable or tween object, got: {:?}", var.get_type());
                }
            }
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn join(this: Gd<Self>, handle: Gd<RefCounted>) {
        let Some(to_join) = TWEENS.get_or_make_shared(gd_handle_id!(handle))
        else {
            godot_error!("Tween with ID {:?} not found.", gd_handle_id!(handle));
            return;
        };

        tween_sequence_mut(gd_handle_id!(this), |sequence| {
            sequence.join_shared(to_join);
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn join_call(this: Gd<Self>, func: Callable) {
        tween_sequence_mut(gd_handle_id!(this), |sequence| {
            sequence.join_callable(func);
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn join_many(this: Gd<Self>, tweens: VariantArray) {
        tween_sequence_mut(gd_handle_id!(this), |sequence| {
            for var in tweens.iter_shared() {
                if let Ok(callable) = var.try_to::<Callable>() {
                    sequence.join_callable(callable);
                } else if let Ok(handle) = var.try_to::<Gd<RefCounted>>() {
                    if let Some(tween) = TWEENS.get_or_make_shared(gd_handle_id!(handle)) {
                        sequence.join_shared(tween);
                    } else {
                        godot_error!("Tween with ID {:?} not found.", gd_handle_id!(handle));
                    }
                } else {
                    godot_error!("Expected Callable or tween object, got: {:?}", var.get_type());
                }
            }
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn insert_call(this: Gd<Self>, time_offset: f64, func: Callable) {
        tween_sequence_mut(gd_handle_id!(this), |sequence| {
            sequence.insert_callable(time_offset, func);
        })
        .log_if_err();
    }

    #[func(gd_self)]
    pub fn insert(this: Gd<Self>, time_offset: f64, handle: Gd<RefCounted>) {
        let Some(to_insert) = TWEENS.get_or_make_shared(gd_handle_id!(handle))
        else {
            godot_error!("Tween with ID {:?} not found.", gd_handle_id!(handle));
            return;
        };

        tween_sequence_mut(gd_handle_id!(this), |sequence| {
            sequence.insert_shared(time_offset, to_insert);
        })
        .log_if_err();
    }
}

define_base_gd_methods! { InternalSequenceGd }
