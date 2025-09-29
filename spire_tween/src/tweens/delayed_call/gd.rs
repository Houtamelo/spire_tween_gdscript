use super::*;

#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct InternalDelayedCallGd {}

#[godot_api]
impl InternalDelayedCallGd {
    #[func]
    pub fn instantiate(func: Callable, delay: f64, auto_play: bool) -> Gd<Self> {
        let this = Self::new_gd();
        let tween = SpireTween::<DelayedCallData>::new_callable(func, delay, AutoPlay(auto_play));
        register_gd_handle! { tween, this }
    }
}

define_base_gd_methods! { InternalDelayedCallGd }
