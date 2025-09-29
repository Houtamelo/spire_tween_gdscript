use std::sync::LazyLock;

use godot::{meta::Signature, sys::UtilityFunctionBind};

use super::*;

pub trait LogIfErr<T> {
    fn log_if_err(self) -> Option<T>;
}

impl<T, E: Debug> LogIfErr<T> for Result<T, E> {
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

pub trait TryVarAt {
    fn try_var_at<T: FromGodot>(&self, key: &str) -> Result<T, ConvertError>;
}

impl TryVarAt for Dictionary {
    fn try_var_at<T: FromGodot>(&self, key: &str) -> Result<T, ConvertError> {
        self.get(key.to_godot())
            .ok_or_else(|| ConvertError::new("Expected Dictionary to contain key `{key}`."))?
            .try_to()
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
        Signature::<CallParams, CallRet>::out_utility_ptrcall(
            utility_fn,
            "is_instance_id_valid",
            args,
        )
    }
}
