#![allow(non_snake_case)]
use std::{fmt::Display, sync::LazyLock};

use godot::sys::UtilityFunctionBind;

use super::*;

pub trait LogIfErr<T> {
    type Output;

    fn log_if_err(self) -> Option<Self::Output>;
}

impl<T, E: Debug> LogIfErr<T> for Result<T, E> {
    type Output = T;

    fn log_if_err(self) -> Option<T> {
        match self {
            Err(err) => {
                godot_error!("{err:?}");
                None
            }
            Ok(ok) => Some(ok),
        }
    }
}

#[allow(dead_code)]
pub trait LogNullArg {
    fn log_null_arg<F, D>(self, arg_name: F) -> Self
    where
        Self: Sized,
        F: FnOnce() -> D,
        D: Display;
}

#[allow(dead_code)]
impl<C: GodotClass> LogNullArg for Option<Gd<C>> {
    fn log_null_arg<F, D>(self, arg_name: F) -> Self
    where
        Self: Sized,
        F: FnOnce() -> D,
        D: Display,
    {
        if self.is_none() {
            godot_error!("Parameter `{}`: expected non-null value, got `null`.", arg_name());
        }

        self
    }
}

pub trait LogBadArg {
    type Output;

    fn log_bad_spire_arg<F, D>(self, arg_name: F) -> Option<Self::Output>
    where
        Self: Sized,
        F: FnOnce() -> D,
        D: Display;
}

impl LogBadArg for Option<Gd<RefCounted>> {
    type Output = AnyTween;

    fn log_bad_spire_arg<F, D>(self, arg_name: F) -> Option<Self::Output>
    where
        F: FnOnce() -> D,
        D: Display,
    {
        let Some(handle) = self else {
            godot_error!("Parameter `{}`: expected Spire type, got `null`.", arg_name());
            return None;
        };

        handle.log_bad_spire_arg(arg_name)
    }
}

impl LogBadArg for Gd<RefCounted> {
    type Output = AnyTween;

    fn log_bad_spire_arg<F, D>(self, arg_name: F) -> Option<Self::Output>
    where
        F: FnOnce() -> D,
        D: Display,
    {
        tween_from_gd_handle(self).log_bad_spire_arg(arg_name)
    }
}

impl LogBadArg for Result<AnyTween, Gd<RefCounted>> {
    type Output = AnyTween;

    fn log_bad_spire_arg<F, D>(self, arg_name: F) -> Option<AnyTween>
    where
        F: FnOnce() -> D,
        D: Display,
    {
        match self {
            Ok(tween) => Some(tween),
            Err(non_spire) => {
                godot_error!("Parameter `{}`: expected Spire type, got `{}`.", arg_name(), non_spire.get_class());
                None
            }
        }
    }
}

pub trait TryVarAt {
    #[allow(unused)]
    fn try_var_at<T: FromGodot>(&self, key: &str) -> Result<T, ConvertError>;
}

impl TryVarAt for VarDictionary {
    fn try_var_at<T: FromGodot>(&self, key: &str) -> Result<T, ConvertError> {
        let key_var = GString::from(key).to_variant();
        self.get(&key_var)
            .ok_or_else(|| ConvertError::new("Expected Dictionary to contain key `{key}`."))?
            .try_to_relaxed()
    }
}

static IS_INSTANCE_ID_VALID_FN: LazyLock<UtilityFunctionBind> =
    LazyLock::new(|| unsafe { godot::sys::utility_function_table().is_instance_id_valid });

pub fn is_instance_id_valid(id: i64) -> bool {
    type CallRet = bool;
    type CallParams = (i64,);
    let args = (id,);
    unsafe {
        let utility_fn = *IS_INSTANCE_ID_VALID_FN;
        Signature::<CallParams, CallRet>::out_utility_ptrcall(utility_fn, "is_instance_id_valid", args)
    }
}

#[macro_export]
macro_rules! print_every_nth_frame {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        let frames = godot::classes::Engine::singleton().get_physics_frames();
        if frames % 60 == 0 {
            let str = format!($fmt $(, $args)*);
            let str_with_frames = format!("[{frames}]{str}");
            godot_print!("{str_with_frames}");
        }
    }};
}

#[macro_export]
macro_rules! print_with_frame {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        let str = format!($fmt $(, $args)*);
        let str_with_frames = format!("[{}]{str}", godot::classes::Engine::singleton().get_physics_frames());
        godot_print!("{str_with_frames}");
    }};
}
